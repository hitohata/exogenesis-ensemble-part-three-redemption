import * as path from "node:path";
import * as cdk from "aws-cdk-lib";
import { RemovalPolicy } from "aws-cdk-lib";
import * as iam from "aws-cdk-lib/aws-iam";
import { Effect, type PolicyStatement } from "aws-cdk-lib/aws-iam";
import * as s3 from "aws-cdk-lib/aws-s3";
import * as ssm from "aws-cdk-lib/aws-ssm";
import { RustFunction } from "cargo-lambda-cdk";
import type { Construct } from "constructs";
import type { StageTypes } from "../util/types";
import {LambdaRestApi} from "aws-cdk-lib/aws-apigateway";

const APP_NAME = "exogenesis-ensemble";
const NOTIFICATION_ARN_PARAMETER_NAME = "/arn/notification/event-bus";

interface IProps extends cdk.StackProps {
	stage: StageTypes;
}

export class ExogenesisEnsembleStack extends cdk.Stack {
	constructor(scope: Construct, id: string, props: IProps) {
		super(scope, id, props);

		const { stage } = props;

		const standardS3Bucket = new s3.Bucket(this, "StandardBucket", {
			bucketName: `${APP_NAME}-bucked-${stage}`,
			removalPolicy:
				stage === "prod" ? RemovalPolicy.RETAIN : RemovalPolicy.DESTROY,
			autoDeleteObjects: stage === "dev", // when the stage is the dev, objects will be removed.
		});

		const apiFunction = new RustFunction(this, "WebAPIFunction", {
			functionName: `${APP_NAME}-web-api-app-${stage}`,
			manifestPath: path.join(__dirname, "../../lambdas/web-api-app"),
			runtime: "provided.al2",
			environment: {
				STANDARD_BUCKET_NAME: standardS3Bucket.bucketName,
			},
		});

		const s3HookFunction = new RustFunction(this, "S3HookFunction", {
			functionName: `${APP_NAME}-s3-hook-app-${stage}`,
			manifestPath: path.join(__dirname, "../../lambdas/s3-hook-app"),
			runtime: "provided.al2",
		});

		// api gateway
		new LambdaRestApi(this, "WebAPIFunctionGateway", {
			restApiName: `${APP_NAME}-api-gateway`,
			handler: apiFunction
		})

		// Access Control

		// standard s3 bucket
		standardS3Bucket.grantReadWrite(apiFunction);
		standardS3Bucket.grantRead(s3HookFunction);

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
}
