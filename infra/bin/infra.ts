#!/usr/bin/env node
import "source-map-support/register";
import * as cdk from "aws-cdk-lib";
import { ExogenesisEnsembleStack } from "../lib/exogenesis-ensemble-stack";
import { detectStage } from "../util/types";

const stage = detectStage();

const app = new cdk.App();
new ExogenesisEnsembleStack(app, `ExogenesisEnsemblePart3Redemption-${stage}`, {
	stage,
});
