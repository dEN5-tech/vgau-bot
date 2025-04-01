import React from "react";
import { Transition } from "@headlessui/react";

const MenuScreen = ({ editingNode, setEditingNode }) => {
  return (
    <div>
      <h2 className="mb-4 text-2xl font-bold">Структура меню бота</h2>
      {editingNode && (
        <Transition
          show={!!editingNode}
          enter="transition duration-100 ease-out"
          enterFrom="transform scale-95 opacity-0"
          enterTo="transform scale-100 opacity-100"
          leave="transition duration-75 ease-out"
          leaveFrom="transform scale-100 opacity-100"
          leaveTo="transform scale-95 opacity-0"
        >
          <div className="p-3 mb-4 bg-blue-50 rounded-r border-l-4 border-l-blue-500">
            <div className="flex justify-between items-center">
              <div>
                <div className="text-lg font-medium">Редактирование пункта меню</div>
                <div className="text-gray-600">{editingNode.text}</div>
              </div>
              <button 
                className="p-2 text-gray-500 bg-white rounded transition-colors hover:text-gray-700 hover:bg-gray-100"
                onClick={() => setEditingNode(null)}
              >
                Отменить
              </button>
            </div>
          </div>
        </Transition>
      )}
    </div>
  );
};

export default MenuScreen; 