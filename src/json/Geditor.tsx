import { Dropdown, List, MenuProps } from "antd";
import React from "react";
import './Geditor.css';

const tItem: MenuProps['items'] = [
	{
		label: 'Add',
		key: 'add',
	}
]

const GEdit: React.FC<{
	
}> = ({}) => {
	return <List>
		<List.Item>
			<Dropdown menu={{ items: tItem }}>

			</Dropdown>
		</List.Item>
	</List>;
};

export default GEdit;
