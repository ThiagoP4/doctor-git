// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod brain; // <--- Importa o módulo que criamos

use std::env;
use tauri::Manager; // Adicionado para poder gerenciar as dimensões e janelas

fn main() {
    // 1. Verifica os argumentos
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];
        if command == "--brain" {
            brain::run_analysis();
            return;
        } else if command == "init" {
            run_init();
            return;
        }
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

// Lógica de injeção automática no repositório atual
fn run_init() {
    let git_dir = std::path::Path::new(".git");
    if !git_dir.exists() || !git_dir.is_dir() {
        eprintln!("❌ ERRO: Nenhuma pasta '.git' detectada! Execute o comando 'doctor-git init' apenas na raiz de projetos Git que voce deseja instalar este rastreador.");
        std::process::exit(1);
    }
    
    let hooks_dir = git_dir.join("hooks");
    if !hooks_dir.exists() {
        if let Err(e) = std::fs::create_dir_all(&hooks_dir) {
            eprintln!("❌ ERRO ao criar a pasta hooks na estrutura do Git: {}", e);
            std::process::exit(1);
        }
    }
    
    let hook_file = hooks_dir.join("post-commit");
    
    // Detecta o caminho absoluto onde este executável está instalado para injetá-lo no hook
    // sem depender de variáveis PATH locais do SO do usuário.
    let current_exe = std::env::current_exe().unwrap_or_else(|_| std::path::PathBuf::from("doctor-git"));
    let exe_path_str = current_exe.to_string_lossy().replace("\\", "/"); // Linux/Bash format

    // Roteiro do ShellScript em Bash que acionará o executavel em Background silencioso
    let hook_content = format!(r#"#!/bin/sh
# ----------------------------------------------------
# DOCTOR GIT - Auto Gerador de Conquistas Gamificadas
# ----------------------------------------------------
# Gerado via 'doctor-git init'. Caminho travado para segurança.

"{}" --brain >> doctor_git_log.txt 2>&1 &
"#, exe_path_str);

    if let Err(e) = std::fs::write(&hook_file, hook_content) {
        eprintln!("❌ ERRO ao injetar a automação dentro do post-commit: {}", e);
        std::process::exit(1);
    }
    
    println!("🚀 [Doctor Git] Injeção de Telemetria Git Automática executada com sucesso!");
    println!("✅ Agora o arquivo '.git/hooks/post-commit' foi interceptado.");
    println!("🕹️  A partir de hoje, a sua IA vai vigiar ativamente esta pasta e gerar Conquistas dos seus Diff!");
}