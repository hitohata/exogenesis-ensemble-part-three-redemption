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
	return `${import.meta.env.BASE_URL}${resource.charAt(0) === "/" ? resource : `/${resource}`}`;
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
	{
		section: "API Documents",
		documents: [
			{
				documentName: "Native Back end API",
				body: "Native Application's back end code",
				link: addRepoName("/api/native/back-end/index.html"),
			},
		],
	},
];
