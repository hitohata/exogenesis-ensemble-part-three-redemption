import * as path from "node:path";
import * as cdk from "aws-cdk-lib";
import { RemovalPolicy } from "aws-cdk-lib";
import * as s3 from "aws-cdk-lib/aws-s3";
import { RustFunction } from "cargo-lambda-cdk";
import type { Construct } from "constructs";
import type { StageTypes } from "../util/types";

const APP_NAME = "exogenesis-ensemble";

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
		apiFunction.addFunctionUrl();

		const s3HookFunction = new RustFunction(this, "S3HookFunction", {
			functionName: `${APP_NAME}-s3-hook-app-${stage}`,
			manifestPath: path.join(__dirname, "../../lambdas/s3-hook-app"),
			runtime: "provided.al2",
		});

		// Access Control

		// standard s3 bucket
		standardS3Bucket.grantReadWrite(apiFunction);
		standardS3Bucket.grantRead(s3HookFunction);
	}
}
