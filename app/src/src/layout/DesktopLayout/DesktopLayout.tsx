import type { ReactNode } from "react";
import { AppBar } from "./Components/AppBar.tsx";

/**
 * Desktop layout
 * @param children
 * @constructor
 */
export function DesktopLayout({ children }: { children: ReactNode }) {
	return (
		<>
			<AppBar />
			{children}
		</>
	);
}
