import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { TauriEvent } from "@tauri-apps/api/event";
import { useContext, useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import "./index.css";
import "./App.css";
import { ModeContext } from "./src/context/ModeContext.tsx";
import { Layout } from "./src/layout/Layout.tsx";
import {UploadPage} from "./src/pages/upload/uploadPage.tsx";

function App() {
	const { mode } = useContext(ModeContext);

	const [greetMsg, setGreetMsg] = useState("");
	const [name, setName] = useState("hoge-");

	async function greet() {
		// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
		setGreetMsg(await invoke("greet", { name }));
	}

	useEffect(() => {
		console.log(mode);
	}, [mode]);

	// useEffect(() => {
	// 	listen<TauriEvent.DRAG_DROP>(TauriEvent.DRAG_DROP, async (event) => {
	// 		console.log(event);
	// 	});
	// 	listen(TauriEvent.WINDOW_FOCUS, (event) => {
	// 		console.log(event.payload);
	// 	});
	// 	listen(TauriEvent.DRAG_LEAVE, async (event) => {
	// 		console.log(event);
	// 	});
	// }, []);

	return (
		<Layout>
			{/*<div className="container">*/}

				{ mode === "UPLOAD" && <UploadPage />}

				{/*<h1>Welcome to Tauri!</h1>*/}

				{/*<div className="row">*/}
				{/*	{ mode === "UPLOAD" && <p>vite</p> }*/}
				{/*	<a href="https://vitejs.dev" target="_blank" rel="noreferrer">*/}
				{/*		<img src="/vite.svg" className="logo vite" alt="Vite logo" />*/}
				{/*	</a>*/}
				{/*	{ mode === "BACK_UP" && <p>tauri</p> }*/}
				{/*	<a href="https://tauri.app" target="_blank" rel="noreferrer">*/}
				{/*		<img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />*/}
				{/*	</a>*/}
				{/*	{ mode === "LOCAL" && <p>React</p> }*/}
				{/*	<a href="https://reactjs.org" target="_blank" rel="noreferrer">*/}
				{/*		<img src={reactLogo} className="logo react" alt="React logo" />*/}
				{/*	</a>*/}
				{/*</div>*/}

				{/*<p>Click on the Tauri, Vite, and React logos to learn more.</p>*/}

				{/*<form*/}
				{/*	className="row"*/}
				{/*	onSubmit={(e) => {*/}
				{/*		e.preventDefault();*/}
				{/*		greet();*/}
				{/*	}}*/}
				{/*>*/}
				{/*	<input*/}
				{/*		id="greet-input"*/}
				{/*		onChange={(e) => setName(e.currentTarget.value)}*/}
				{/*		placeholder="Enter a name..."*/}
				{/*	/>*/}
				{/*	<button type="submit">Greet</button>*/}
				{/*</form>*/}

				{/*<p>{greetMsg}</p>*/}
			{/*</div>*/}
		</Layout>
	);
}

export default App;
