import { type ReactNode, createContext, useState } from "react";
import type { MODE } from "../constant/links.ts";

/**
 * This context manages the mode of this application.
 * The mode will be switched by the layout.
 */
export const ModeContext = createContext<{
	mode: MODE;
	changeMode: (mode: MODE) => void;
}>({
	mode: "UPLOAD",
	changeMode: (_: MODE) => {},
});

export const ModeContextProvider = ({ children }: { children: ReactNode }) => {
	const [mode, setMode] = useState<MODE>("UPLOAD");

	const changeMode = (mode: MODE) => setMode(mode);

	return (
		<ModeContext.Provider value={{ mode, changeMode }}>
			{children}
		</ModeContext.Provider>
	);
};
