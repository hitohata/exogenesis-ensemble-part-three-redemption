export type ModesType = {
	mode: MODE;
	name: string;
	icon: string;
}[];

export type MODE = "UPLOAD" | "BACK_UP" | "LOCAL";

/**
 * The links of the contents
 */
export const LINKS: ModesType = [
	{
		mode: "UPLOAD",
		name: "Upload",
		icon: "",
	},
	{
		mode: "BACK_UP",
		name: "Buck Up",
		icon: "",
	},
	{
		mode: "LOCAL",
		name: "Local",
		icon: "",
	},
];
