import "./App.css";
import { ConfigProvider, theme } from "antd";
import Menubar from "./Menubar";
import { useState } from "react";
import GEdit from "./json/Geditor";
import { Skill } from "./bindings/Skill";

function App() {
	const [curFolder, setCurFolder] = useState("");
	const [curSkillSet, setCurSkillSet] = useState<Skill[]>([]);

	return (
		<ConfigProvider
			theme={{
				algorithm: theme.darkAlgorithm,
			}}
		>
			<main className="container">
				<Menubar
					currentFolder={curFolder}
					setCurFolder={setCurFolder}
					setCurSkillSet={setCurSkillSet}
				/>
				<GEdit skillMap={curSkillSet} />
				{curFolder}
			</main>
		</ConfigProvider>
	);
}

export default App;
