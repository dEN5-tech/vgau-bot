import React from "react";
import { Menu, Transition } from "@headlessui/react";

const ExportScreen = () => {
  return (
    <div>
      <h2 className="mb-4 text-2xl font-bold">Экспорт / Импорт данных</h2>
      <div className="p-6 bg-white rounded-lg shadow-sm">
        <h3 className="mb-4 text-lg font-medium">Экспорт JSON</h3>
        <p className="mb-4 text-gray-600">
          Выгрузите данные бота в JSON-файл для резервного копирования
          или переноса на другой сервер.
        </p>
        <Menu as="div" className="inline-block relative text-left">
          <Menu.Button className="px-4 py-2 text-white bg-blue-500 rounded transition-colors hover:bg-blue-600">
            Экспорт данных
          </Menu.Button>
          <Transition
            enter="transition duration-100 ease-out"
            enterFrom="transform scale-95 opacity-0"
            enterTo="transform scale-100 opacity-100"
            leave="transition duration-75 ease-out"
            leaveFrom="transform scale-100 opacity-100"
            leaveTo="transform scale-95 opacity-0"
          >
            <Menu.Items className="absolute left-0 mt-2 w-48 bg-white rounded-md divide-y divide-gray-100 ring-1 ring-black ring-opacity-5 shadow-lg origin-top-left focus:outline-none">
              <div className="px-1 py-1">
                <Menu.Item>
                  {({ active }) => (
                    <button
                      className={`${
                        active ? 'text-blue-700 bg-blue-50' : 'text-gray-900'
                      } group flex rounded-md items-center w-full px-2 py-2 text-sm`}
                    >
                      Скачать JSON
                    </button>
                  )}
                </Menu.Item>
                <Menu.Item>
                  {({ active }) => (
                    <button
                      className={`${
                        active ? 'text-blue-700 bg-blue-50' : 'text-gray-900'
                      } group flex rounded-md items-center w-full px-2 py-2 text-sm`}
                    >
                      Скачать CSV
                    </button>
                  )}
                </Menu.Item>
              </div>
            </Menu.Items>
          </Transition>
        </Menu>
      </div>
    </div>
  );
};

export default ExportScreen; 