export type StageTypes = "prod" | "dev";

/**
 * Generate a stage name.
 * The stage name will be determined by the env value.
 * If the env value is not provided, this function returns the "dev" string.
 */
export const detectStage = (): StageTypes => {
	if (process.env.STAGE === "prod") {
		return "prod";
	}
	return "dev";
};
