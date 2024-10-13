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
		<div className="fixed bottom-0 left-0 z-40 w-full h-16 bg-white border-t border-gray-light">
			<div className="grid h-full max-w-lg grid-cols-3 mx-auto">
				{LINKS.map((item) => (
					<button
						type="button"
						className="border-none"
						key={item.mode}
						onClick={() => changeMode(item.mode)}
					>
						<img src={item.iconPath} alt={item.name} className="h-full w-full"/>
					</button>
				))}
			</div>
		</div>
	);
}
