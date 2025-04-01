use tg_menu_editor_wasm::MenuEditorApp;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // If you're building for native, include this part
    use eframe::NativeOptions;
    
    let native_options = NativeOptions {
        initial_window_size: Some(eframe::egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };

    eframe::run_native(
        "ВГАУ Бот Меню Редактор",
        native_options,
        Box::new(|cc| Box::new(MenuEditorApp::new(cc))),
    ).unwrap();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // For WebAssembly target, we just need to initialize the panic hook
    // The actual initialization happens in lib.rs with the start_app function
    console_error_panic_hook::set_once();
} 