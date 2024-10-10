import type { ReactNode } from "react";
import { BottomMenu } from "./Components/BottomMenu.tsx";

/**
 * mobile's layout
 */
export function MobileLayout({ children }: { children: ReactNode }) {
	return (
		<>
			{children}
			<BottomMenu />
		</>
	);
}
