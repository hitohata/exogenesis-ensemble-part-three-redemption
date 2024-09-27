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
		section: "Native APP API Documents",
		documents: [
			{
				documentName: "Native Back End API",
				body: "Native Application's back end",
				link: addRepoName("/native/api/back-end/doc/app/index.html"),
			},
			{
				documentName: "Native Front End API",
				body: "Native Application's front end",
				link: addRepoName("/native/api/front-end/index.html"),
			},
		],
	},
	{
		section: "Web API Documents",
		documents: [
			{
				documentName: "WEB API",
				body: "Back end WEB API",
				link: addRepoName("/web-api/index.html"),
			}
		],
	},
];
