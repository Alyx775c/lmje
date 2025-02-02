import React, { useState } from "react";
import { Dropdown, List, MenuProps, Space } from "antd"; // Ensure this is the correct import
import { Skill } from "../bindings/Skill";
import "./Geditor.css";
import { DownOutlined } from "@ant-design/icons";

const GEdit: React.FC<{
	skillMap: Array<Skill> | Skill[];
}> = ({ skillMap }) => {
	function renderSkills() {
		return skillMap.map((e) => {
			return (
				<List.Item key={e.id.toString()}>
					<div className="graphicItem">
						<div>{e.id.toString()}</div>
						<div className="subItems">
							{(() => {
								let [curGak, setCur] = useState(0);
								let items: MenuProps["items"] = [];

								e.skillData.forEach((sd) => {
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
											Uptie: {curGak}
											<DownOutlined />
										</Space>
									</Dropdown>
								);
							})()}
						</div>
					</div>
				</List.Item>
			);
		});
	}

	return <List>{renderSkills()}</List>;
};

export default GEdit;
