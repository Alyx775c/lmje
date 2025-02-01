import "./App.css";
import { ConfigProvider , theme } from "antd";
import Menubar from "./Menubar";
import { useState } from "react";
import GEdit from "./json/Geditor";

function App() {
	const [curFolder, setCurFolder] = useState('');

	return (
		<ConfigProvider theme={{
			algorithm: theme.darkAlgorithm
		}}>
			<main className="container">
				<Menubar currentFolder={curFolder} setCurFolder={setCurFolder}/>
				<GEdit/>
				{curFolder}
			</main>
		</ConfigProvider>
	);
}

export default App;
