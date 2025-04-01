import React, { useState } from "react";
import { Tab } from "@headlessui/react";

// Импорт компонентов экранов
import MenuScreen from "./screens/menu";
import GraphScreen from "./screens/graph";
import FAQScreen from "./screens/faq";
import ExportScreen from "./screens/export";
import SettingsScreen from "./screens/settings";

function App() {
	const [selectedIndex, setSelectedIndex] = useState(0);
	const [editingNode, setEditingNode] = useState(null);

	const handleEditNode = (node) => {
		setEditingNode(node);
	};

	const sections = [
		{ id: "menu", name: "Структура меню" },
		{ id: "graph", name: "Визуализация меню" },
		{ id: "faq", name: "FAQ" },
		{ id: "export", name: "Экспорт / Импорт" },
		{ id: "settings", name: "Настройки" }
	];

	return (
		<div className="flex h-screen bg-gray-100">
			<Tab.Group vertical selectedIndex={selectedIndex} onChange={setSelectedIndex} className="flex w-full">
				{/* Sidebar */}
				<aside className="flex flex-col w-64 bg-white shadow-md">
					<div className="p-4 border-b">
						<h1 className="text-xl font-bold">ВГАУ Бот Редактор</h1>
					</div>
					
					<Tab.List className="p-4 space-y-2">
						{sections.map((section) => (
							<Tab
								key={section.id}
								className={({ selected }) => 
									`p-2 w-full text-left hover:bg-gray-100 rounded cursor-pointer transition-colors duration-150 ${
										selected ? "bg-blue-50 text-blue-700 font-medium" : ""
									}`
								}
							>
								{section.name}
							</Tab>
						))}
					</Tab.List>
				</aside>
				
				{/* Main content */}
				<main className="overflow-auto flex-1 p-6 ml-4 bg-white rounded-lg shadow-sm">
					<Tab.Panels>
						<Tab.Panel>
							<MenuScreen editingNode={editingNode} setEditingNode={setEditingNode} />
						</Tab.Panel>
						
						<Tab.Panel>
							<GraphScreen />
						</Tab.Panel>
						
						<Tab.Panel>
							<FAQScreen />
						</Tab.Panel>
						
						<Tab.Panel>
							<ExportScreen />
						</Tab.Panel>
						
						<Tab.Panel>
							<SettingsScreen />
						</Tab.Panel>
					</Tab.Panels>
				</main>
			</Tab.Group>
		</div>
	);
}

export default App;
