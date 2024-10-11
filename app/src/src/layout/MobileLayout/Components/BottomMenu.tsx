import { useContext } from "react";
import { LINKS } from "../../../constant/links.ts";
import { ModeContext } from "../../../context/ModeContext.tsx";

/**
 * The bottom menu for the Mobile Layout
 * @constructor
 */
export function BottomMenu() {
	const { changeMode } = useContext(ModeContext);

	return (
		<div className="fixed bottom-0 left-0">
			<div className="grid h-full">
				{LINKS.map((item) => (
					<button
						type="button"
						key={item.mode}
						onClick={() => changeMode(item.mode)}
					>
						<p>{item.name}</p>
					</button>
				))}
			</div>
		</div>
	);
}
