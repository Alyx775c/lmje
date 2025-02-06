import React, { useState } from "react";
import { Dropdown, List, MenuProps, Select, Space } from "antd"; // Ensure this is the correct import
import { Skill } from "../bindings/Skill";
import { DownOutlined } from "@ant-design/icons";
import "./Geditor.css";
import { ItemType } from "antd/es/menu/interface";

const GEdit: React.FC<{
	skillMap: Array<Skill> | Skill[];
	mItems: ItemType[],
	setMItems: (items: ItemType[]) => void,
}> = ({ skillMap, mItems, setMItems }) => {
	//	let [curSelectedSkill, setCurSkill] = useState<Skill | null>(null);
	let [curGak, setCur] = useState<number>(0);
	let [curGEdit, setCurGEdit] = useState<React.ReactNode>(<> </>);

	function renderSkill(skill: Skill) {
		return (
			<List.Item key={skill.id.toString()}>
				<div className="graphicItem">
					<div>{skill.id.toString()}</div>
					<div className="subItems">
						{(() => {
							let items: MenuProps["items"] = [];

							skill.skillData.forEach((sd) => {
								items.push({
									label: sd.gaksungLevel.toString(),
									key: sd.gaksungLevel.toString(),
									onClick: (e) => {
										setCur(Number(e.key));
									},
								});
							});

							return (
								<Dropdown menu={{ items }}>
									<Space>
										Uptie: {curGak.toString()}
										<DownOutlined />
									</Space>
								</Dropdown>
							);
						})()}
					</div>
				</div>
			</List.Item>
		);
	}

	const Selector: React.FC<{}> = ({}) => {
		return (
			<Select
				showSearch
				className="skillSelect"
				placeholder="Select a skill"
				onSelect={(e) => {
					let skill: Skill | null = null;

					skillMap.forEach((sk) => {
						if (sk.id == Number(e)) {
							skill = sk;
						}
					});

					if (skill) {
						setCurGEdit(renderSkill(skill));
					}
				}}
				options={new Array(skillMap.length).fill(null).map((_, i) => {
					let skillMapped = skillMap[i] as Skill;

					return {
						label: skillMapped.id.toString(),
						value: skillMapped.id.toString(),
					};
				})}
			/>
		);
	};

	let locMItems = mItems;
	locMItems.push({
		label: "Add Item",
		key: "add",
		onClick: () => {
			console.log('test')
		},
	});
	setMItems(locMItems);

	//return <List>{renderSkills()}</List>;
	return (
		<div id="gedit">
			<Selector />
			<List>{curGEdit}</List>
		</div>
	);
};

export default GEdit;
