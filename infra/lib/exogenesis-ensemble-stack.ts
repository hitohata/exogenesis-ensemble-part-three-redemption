import * as path from "node:path";
import * as cdk from "aws-cdk-lib";
import { RustFunction } from "cargo-lambda-cdk";
import type { Construct } from "constructs";
import type { StageTypes } from "../util/types";

interface IProps extends cdk.StackProps {
	stage: StageTypes;
}

export class ExogenesisEnsembleStack extends cdk.Stack {
	constructor(scope: Construct, id: string, props: IProps) {
		super(scope, id, props);

		const { stage } = props;

		const apiFunction = new RustFunction(this, "WebAPIFunction", {
			functionName: `exogenesis-ensemble-web-api-app-${stage}`,
			manifestPath: path.join(__dirname, "../../lambdas/web-api-app"),
			runtime: "provided.al2",
		});
		apiFunction.addFunctionUrl();

		const s3HookFunction = new RustFunction(this, "S3HookFunction", {
			functionName: `exogenesis-ensemble-s3-hook-app-${stage}`,
			manifestPath: path.join(__dirname, "../../lambdas/s3-hook-app"),
			runtime: "provided.al2",
		});
	}
}
