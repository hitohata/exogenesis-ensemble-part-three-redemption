export type DocumentSectionType = {
	section: string;
	documents: DocLinkType[];
};

export type DocLinkType = {
	documentName: string;
	body: string;
	link: string;
};

const addRepoName = (resource: string) => {
	return `/ExogenesisEnsemble-Part3-Redemption${resource.charAt(0) === "/" ? resource : `/${resource}`}`;
};

export const DOC_LINKS: DocumentSectionType[] = [
	{
		section: "Overview",
		documents: [
			{
				documentName: "Project Document",
				body: "Project Document. This document contains high-level design of this system.",
				link: addRepoName("/project/index.html"),
			},
			{
				documentName: "Top Page (Here)",
				body: "Just the entry point of this system document.",
				link: addRepoName("/top-page/index.html"),
			},
		],
	},
];
