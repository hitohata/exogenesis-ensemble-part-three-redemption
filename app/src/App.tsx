import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { TauriEvent } from "@tauri-apps/api/event"
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("hoge-");
  const [triggered, setTriggered] = useState<boolean>(false);

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  useEffect(() => {
      console.log("App.tsx");
  })
    useEffect(() => {
        greet().then();
    }, []);

    useEffect(() => {
        // let unlisten = null;
        if (window.__TAURI__ && !triggered) {
            setTriggered(true);

            listen<TauriEvent.WINDOW_FILE_DROP>(TauriEvent.WINDOW_FILE_DROP, async (event) => {
                console.log(event);
                // if (event.payload.type === 'hover') {
                //     console.log('User hovering', event.payload.paths);
                //  } else if (event.payload.type === 'drop') {
                //    console.log('User dropped', event.payload.paths);
                //  } else {
                //    console.log('File drop cancelled');
                //  }
            })
            listen(TauriEvent.WINDOW_FOCUS, (event) => {
                console.log(event);
            })
            listen(TauriEvent.WINDOW_FILE_DROP_CANCELLED, async (event) => {
                console.log(event);
            })
            // .then(e => unlisten = e)
            // .catch(e => console.log(e));
            // unlisten = appWindow.onFileDropEvent((event) => {
            //     if (event.payload.type === 'hover') {
            //         console.log('User hovering', event.payload.paths);
            //      } else if (event.payload.type === 'drop') {
            //        console.log('User dropped', event.payload.paths);
            //      } else {
            //        console.log('File drop cancelled');
            //      }
            // })
            //
            // registerEvent();

            setTimeout(() => {
                setTriggered(false);
            }, 3000)
        }
        // return () => { unlisten && unlisten() }
    }, []);

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>

      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
