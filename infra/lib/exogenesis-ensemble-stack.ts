import * as path from "node:path";
import * as cdk from "aws-cdk-lib";
import { RemovalPolicy } from "aws-cdk-lib";
import { LambdaRestApi } from "aws-cdk-lib/aws-apigateway";
import * as dynamodb from "aws-cdk-lib/aws-dynamodb";
import { AttributeType, BillingMode } from "aws-cdk-lib/aws-dynamodb";
import * as iam from "aws-cdk-lib/aws-iam";
import { Effect, type PolicyStatement } from "aws-cdk-lib/aws-iam";
import * as s3 from "aws-cdk-lib/aws-s3";
import * as ssm from "aws-cdk-lib/aws-ssm";
import { RustFunction } from "cargo-lambda-cdk";
import type { Construct } from "constructs";
import type { StageTypes } from "../util/types";

const APP_NAME = "exogenesis-ensemble";
const NOTIFICATION_ARN_PARAMETER_NAME = "/arn/notification/event-bus";

interface IProps extends cdk.StackProps {
	stage: StageTypes;
}

export class ExogenesisEnsembleStack extends cdk.Stack {
	private readonly stage: StageTypes;
	constructor(scope: Construct, id: string, props: IProps) {
		super(scope, id, props);

		this.stage = props.stage;

		// Dynamo Table
		const dynamoTable = this.dynamoDb();

		// bucket
		const standardS3Bucket = this.s3StandardBucket();

		// functions
		const [apiFunction, s3HookFunction] = this.lambdaFunctions({
			standardBucketName: standardS3Bucket.bucketName,
			tableName: dynamoTable.tableName,
		});

		// api gateway
		new LambdaRestApi(this, "WebAPIFunctionGateway", {
			restApiName: `${APP_NAME}-api-gateway`,
			handler: apiFunction,
		});

		// Access Control

		// standard s3 bucket
		standardS3Bucket.grantReadWrite(apiFunction);
		standardS3Bucket.grantRead(s3HookFunction);

		// DynamoTable
		dynamoTable.grantReadData(apiFunction);
		dynamoTable.grantReadWriteData(s3HookFunction);

		// add permission to lambdas to put events
		apiFunction.addToRolePolicy(this.getNotificationServicePolicyStatement());
		s3HookFunction.addToRolePolicy(
			this.getNotificationServicePolicyStatement(),
		);
	}

	/**
	 * The policy statement to put events to the notification service
	 * To call notification service, a service must be attached this policy
	 * In this function, get the notification services' ARN then create a statement
	 * { @link https://github.com/hitohata/my-notification-service }
	 * @private
	 */
	private getNotificationServicePolicyStatement(): PolicyStatement {
		const eventBusArn = ssm.StringParameter.valueForStringParameter(
			this,
			NOTIFICATION_ARN_PARAMETER_NAME,
		);

		return new iam.PolicyStatement({
			effect: Effect.ALLOW,
			actions: ["events:PutEvents"],
			resources: [eventBusArn],
		});
	}

	private dynamoDb(): dynamodb.Table {
		return new dynamodb.Table(this, "DynamoDB", {
			tableName: `${APP_NAME}-table-${this.stage}`,
			removalPolicy:
				this.stage === "dev" ? RemovalPolicy.DESTROY : RemovalPolicy.RETAIN,
			partitionKey: {
				name: "PK",
				type: AttributeType.STRING,
			},
			sortKey: {
				name: "SK",
				type: AttributeType.NUMBER,
			},
			billingMode: BillingMode.PAY_PER_REQUEST,
		});
	}

	/**
	 * The rust functions
	 * The return value is tuple.
	 * The first one is s3 hook, the second one is s3 hook
	 * @private
	 */
	private lambdaFunctions({
		standardBucketName,
		tableName,
	}: { standardBucketName: string; tableName: string }): [
		RustFunction,
		RustFunction,
	] {
		const webApi = new RustFunction(this, "WebAPIFunction", {
			functionName: `${APP_NAME}-web-api-app-${this.stage}`,
			manifestPath: path.join(__dirname, "../../lambdas/web-api-app"),
			runtime: "provided.al2",
			environment: {
				STANDARD_BUCKET_NAME: standardBucketName,
				TABLE_NAME: tableName,
			},
		});

		const s3Hook = new RustFunction(this, "S3HookFunction", {
			functionName: `${APP_NAME}-s3-hook-app-${this.stage}`,
			manifestPath: path.join(__dirname, "../../lambdas/s3-hook-app"),
			runtime: "provided.al2023",
			environment: {
				TABLE_NAME: tableName,
			},
		});

		return [webApi, s3Hook];
	}

	private s3StandardBucket(): s3.Bucket {
		return new s3.Bucket(this, "StandardBucket", {
			bucketName: `${APP_NAME}-bucked-${this.stage}`,
			removalPolicy:
				this.stage === "prod" ? RemovalPolicy.RETAIN : RemovalPolicy.DESTROY,
			autoDeleteObjects: this.stage === "dev", // when the stage is the dev, objects will be removed.
		});
	}
}
