import upload from "./icons/upload.svg";
import backUp from "./icons/download.svg";
import disk from "./icons/disk.svg";

export type ModesType = {
	mode: MODE;
	name: string;
	iconPath: string;
}[];

export type MODE = "UPLOAD" | "BACK_UP" | "LOCAL";

/**
 * The links of the contents
 */
export const LINKS: ModesType = [
	{
		mode: "UPLOAD",
		name: "Upload",
		iconPath: upload,
	},
	{
		mode: "BACK_UP",
		name: "Buck Up",
		iconPath: backUp,
	},
	{
		mode: "LOCAL",
		name: "Local",
		iconPath: disk,
	},
];
