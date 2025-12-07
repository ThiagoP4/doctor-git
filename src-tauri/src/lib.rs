

// 1. Criamos um comando que o JavaScript pode chamar
#[tauri::command]
fn get_achievement() -> String {
    // Pega os argumentos do sistema (ex: doctor-git.exe "Texto Aqui")
    let args: Vec<String> = std::env::args().collect();
    
    // Se tiver um argumento (índice 1), retorna ele. Se não, retorna um teste.
    args.get(1).cloned().unwrap_or_else(|| "TEST MODE".to_string())
}

// 2. Criamos um comando para fechar a janela
#[tauri::command]
fn close_window(app_handle: tauri::AppHandle) {
    app_handle.exit(0); // Fecha o programa completamente
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Registramos os comandos aqui para o JS poder usar
        .invoke_handler(tauri::generate_handler![get_achievement, close_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}