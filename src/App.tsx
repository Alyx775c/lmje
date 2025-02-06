import "./App.css";
import { ConfigProvider, theme } from "antd";
import Menubar from "./Menubar";
import { useState } from "react";
import GEdit from "./json/Geditor";
import { Skill } from "./bindings/Skill";
import { ItemType } from "antd/es/menu/interface";

function App() {
	const [curFolder, setCurFolder] = useState("");
	const [curSkillSet, setCurSkillSet] = useState<Skill[]>([]);
	const [additionalItems, setItems] = useState<ItemType[]>([]);

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
					curItems={additionalItems}
				/>
				<GEdit
					skillMap={curSkillSet}
					mItems={additionalItems}
					setMItems={setItems}
				/>
			</main>
		</ConfigProvider>
	);
}

export default App;
