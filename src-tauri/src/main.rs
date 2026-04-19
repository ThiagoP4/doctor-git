// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod brain; // <--- Importa o módulo que criamos

use std::env;
use tauri::Manager; // Adicionado para poder gerenciar as dimensões e janelas

fn main() {
    // 1. Verifica os argumentos
    let args: Vec<String> = env::args().collect();

    // 2. Se tiver a flag "--brain", vira o analisador e encerra
    if args.len() > 1 && args[1] == "--brain" {
        brain::run_analysis();
        return; // <--- IMPORTANTE: Sai aqui e NÃO inicia o Tauri
    }

    // 3. Se não tiver flag, inicia a Interface Gráfica (Tauri)
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Pegar a janela inicial
            if let Some(window) = app.get_webview_window("main") {
                // Pegar o monitor principal
                if let Ok(Some(monitor)) = window.primary_monitor() {
                    let size = monitor.size();
                    let scale_factor = monitor.scale_factor();
                    
                    // Pegar tamanho da janela ou forçar 500x350
                    let win_size = window.outer_size().unwrap_or(tauri::PhysicalSize::new(
                        (500.0 * scale_factor) as u32,
                        (350.0 * scale_factor) as u32,
                    ));
                    
                    let margin = (20.0 * scale_factor) as u32;
                    let taskbar_space = (40.0 * scale_factor) as u32;
                    
                    let x = size.width.saturating_sub(win_size.width + margin);
                    let y = size.height.saturating_sub(win_size.height + margin + taskbar_space);
                    
                    let _ = window.set_position(tauri::PhysicalPosition::new(x, y));
                }
                
                // Mostrar apenas após ter colocado no lugar certo!
                let _ = window.show();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_achievement, close_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Comandos antigos (mantenha os que você já tinha ou use estes)
#[tauri::command]
fn get_achievement() -> String {
    let args: Vec<String> = env::args().collect();
    // Pega o argumento 1 (se não for --brain) como o nome da conquista
    args.get(1).cloned().unwrap_or_else(|| "TEST MODE".to_string())
}

#[tauri::command]
fn close_window(app_handle: tauri::AppHandle) {
    app_handle.exit(0);
}