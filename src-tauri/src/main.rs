#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod brain; // <--- Importa o módulo que criamos

use std::env;

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