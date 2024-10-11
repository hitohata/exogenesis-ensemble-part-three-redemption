import { TauriEvent, listen } from "@tauri-apps/api/event";
import { type ReactNode, useEffect, useState } from "react";
import type { MODE } from "../constant/links.ts";
import { DesktopLayout } from "./DesktopLayout/DesktopLayout.tsx";
import { MobileLayout } from "./MobileLayout/MobileLayout.tsx";

/**
 * This component wrapper of the main contents
 * This component watches the window size and returns an appropriate layout for mobile size or
 * @param children
 * @param setMode
 * @constructor
 */
export function Layout({ children }: { children: ReactNode }) {
	const [windowWidth, setWindowWidth] = useState<number>(1000);

	useEffect(() => {
		listen<TauriEvent.WINDOW_RESIZED>(TauriEvent.WINDOW_RESIZED, (event) => {
			if (event.payload.width && typeof event.payload.width === "number") {
				setWindowWidth(event.payload.width);
			}
		});
	}, []);

	return (
		<div>
			{windowWidth < 480 ? (
				<MobileLayout>{children}</MobileLayout>
			) : (
				<DesktopLayout>{children}</DesktopLayout>
			)}
		</div>
	);
}
