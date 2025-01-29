import "./App.css";
import { ConfigProvider , theme } from "antd";
import Menubar from "./Menubar";
import { useState } from "react";

function App() {
	/*const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }*/
	const [curFolder, setCurFolder] = useState('folderContext');

	return (
		<ConfigProvider theme={{
			algorithm: theme.darkAlgorithm
		}}>
			<main className="container">
				<Menubar setCurFolder={setCurFolder}/>
				{curFolder}
			</main>
		</ConfigProvider>
	);
}

export default App;
