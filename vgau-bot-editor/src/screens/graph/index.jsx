import React, { useEffect, useRef } from "react";
import "./graph.css";

const GraphScreen = () => {
  const containerRef = useRef(null);
  const wasmInitialized = useRef(false);

  useEffect(() => {
    if (containerRef.current && !wasmInitialized.current) {
      // Create a canvas element for the WASM application
      const canvas = document.createElement('canvas');
      canvas.id = 'editor_canvas';
      canvas.style.width = '100%';
      canvas.style.height = '100%';
      
      // Create a loading indicator
      const loadingDiv = document.createElement('div');
      loadingDiv.id = 'loading';
      loadingDiv.textContent = 'Загрузка приложения...';
      loadingDiv.style.position = 'absolute';
      loadingDiv.style.top = '50%';
      loadingDiv.style.left = '50%';
      loadingDiv.style.transform = 'translate(-50%, -50%)';
      loadingDiv.style.textAlign = 'center';
      loadingDiv.style.fontSize = '18px';
      loadingDiv.style.color = '#555';
      
      // Clear previous content and add new elements
      containerRef.current.innerHTML = '';
      containerRef.current.style.position = 'relative';
      containerRef.current.appendChild(loadingDiv);
      containerRef.current.appendChild(canvas);
      
      // Dynamic import of the WASM module
      const initWasm = async () => {
        try {
          // Use the correct path to your WASM module
          const wasmModule = await import('../../assets/wasm/tg_menu_editor_wasm.js');
          await wasmModule.default();
          wasmModule.start_app("editor_canvas");
          loadingDiv.style.display = 'none';
          wasmInitialized.current = true;
        } catch (e) {
          console.error("Error initializing WASM application:", e);
          loadingDiv.textContent = 'Ошибка загрузки: ' + e.message;
        }
      };
      
      initWasm();
    }
    // Cleanup function
    return () => {
      wasmInitialized.current = false;
    };
  }, []);

  return (
    <div className="h-full">
      <h2 className="mb-4 text-2xl font-bold text-white">Редактор графа узлов</h2>
      <div 
        ref={containerRef} 
        className="p-4 bg-gray-900 rounded-lg shadow-sm h-[calc(100vh-150px)] overflow-hidden"
      >
        {/* WASM canvas will be inserted here */}
      </div>
      <div className="mt-4 text-sm text-gray-400">
        <ul className="pl-5 space-y-1 list-disc">
          <li>Перетащите узлы, чтобы изменить их положение</li>
          <li>Соедините порты, перетаскивая от выхода к входу</li>
          <li>Щелкните правой кнопкой мыши для открытия контекстного меню</li>
          <li>Удерживайте Ctrl и перетащите для выбора нескольких узлов</li>
        </ul>
      </div>
    </div>
  );
};

export default GraphScreen; 