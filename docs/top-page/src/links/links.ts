export type DocumentSectionType = {
	section: string;
	documents: DocLinkType[];
};

export type DocLinkType = {
	documentName: string;
	body: string;
	link: string;
};

export const DOC_LINKS: DocumentSectionType[] = [
	{
		section: "Overview",
		documents: [
			{
				documentName: "Project Document",
				body: "Project Document. This document contains high-level design of this system.",
				link: "/project/index.html",
			},
			{
				documentName: "Top Page (Here)",
				body: "Just the entry point of this system document.",
				link: "/top-page/index.html",
			},
		],
	},
];
