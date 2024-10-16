import { invoke } from "@tauri-apps/api/core";
import { TauriEvent, listen } from "@tauri-apps/api/event";
import { useEffect } from "react";

export function UploadPage() {
	useEffect(() => {
		listen<{ paths: string[] }>(TauriEvent.DRAG_DROP, (event) => {
			for (const path of event.payload.paths) {
				invoke("select_file", { path }).then((f) => f);
			}
		});
	}, []);

	return (
		<div className="h-full">
			<p>Drop file here!</p>
		</div>
	);
}
