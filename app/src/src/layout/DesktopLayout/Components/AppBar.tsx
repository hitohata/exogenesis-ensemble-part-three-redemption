import { LINKS } from "../../../constant/links.ts";

export function AppBar() {
	return (
		<>
			{LINKS.map((link) => (
				<p key={link.url}>{link.name}</p>
			))}
		</>
	);
}
