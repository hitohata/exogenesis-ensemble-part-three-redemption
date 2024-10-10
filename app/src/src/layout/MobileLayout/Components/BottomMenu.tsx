import { LINKS } from "../../../constant/links.ts";

/**
 * The bottom menu for the Mobile Layout
 * @constructor
 */
export function BottomMenu() {
	return (
		<>
			{LINKS.map((item) => (
				<p key={item.url}>{item.name}</p>
			))}
		</>
	);
}
