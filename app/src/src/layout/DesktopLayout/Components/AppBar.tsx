import { useContext } from "react";
import { LINKS } from "../../../constant/links.ts";
import { ModeContext } from "../../../context/ModeContext.tsx";

export function AppBar() {
	const { changeMode } = useContext(ModeContext);

	return (
		<div className="py-5 border-b-4 width-limiter">
			<ul className="flex justify-center content-center space-x-20">
				{LINKS.map((link) => (
					<li key={link.mode}>
						<button
							type="button"
							className="min-h-full bg-inherit border-0 shadow-none"
							onClick={() => changeMode(link.mode)}
						>
							<p>{link.name}</p>
						</button>
					</li>
				))}
			</ul>
		</div>
	);
}
