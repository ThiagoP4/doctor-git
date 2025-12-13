use std::process::Command; // Permite rodar comandos do terminal (como o 'git')
use rusqlite::{params, Connection}; // Biblioteca para o SQLite (conexão e parâmetros)
use chrono::Local; // Biblioteca para manipulação de datas e horas
use serde::{Deserialize}; // Transforma Texto em JSON e vice-versa
use serde_json::json; // Ajuda a criar objetos JSON

// Estrutura para ler a resposta do Ollama
#[derive(Deserialize)] // Rust transformará o texto em JSON nesta estrutura
struct OllamaResponse { 
    message: OllamaMessage, // Esperamos um campo chamado 'message' que seja do tipo 'OllamaMessage'
}

#[derive(Deserialize)]
struct OllamaMessage {
    content: String, // Esperamos um campo chamado 'content' que seja do tipo String
}

// &str: "String Slice". É um tipo de texto ultra-rápido e imutável gravado direto na memória do programa.
// Diferente de 'String' (que é gerenciável e pode crescer), &str é fixo.
const AI_MODEL: &str = "llama3.2";

    // Função principal que analisa o código
    pub fn run_analysis() {
        println!("Iniciando análise de código com o modelo AI...");

        // 1. Git diff para obter as mudanças de código
        let output = Command::new("git") // (Por exemplo: digita o comando 'git' no terminal)
            // .args(...): Passa uma lista (array) de argumentos para o comando.
            // &[]: Cria uma fatia de array (slice) com os textos.
            .args(&["diff", "HEAD~1", "HEAD", "--stat", "-p"])
            // "HEAD~1": Penúltimo commit.
            // "HEAD": Último commit.
            // "-p": Patch (mostra o código que mudou).
            .output(); // Gatilho de envio para execução do comando.
            // Ele pausa o programa e espera o Git terminar de rodar.

        // --- BLOCO DE DECISÃO (MATCH) ---

        // O Match atua como um switch case, mas obrigado a tratar TUDO
        match output {  // Estamos abrindo a caixa 'output' para ver o que tem dentro.
            Ok(o) => { // Ok(o): "Se deu certo, chame o resultado de 'o'"
                // String::from_utf8_lossy: O terminal devolve Bytes, não texto.
                // Essa função converte Bytes para String legível.
                // "lossy" (com perdas) significa: "Se tiver um caractere estranho que o Rust não entende,
                // troque por '' em vez de travar o programa".
                // &o.stdout: Pegamos a saída padrão (o texto do git) por referência (&).
                let stdout = String::from_utf8_lossy(&o.stdout);
                if stdout.len() < 10 {
                    println!("Nenhuma mudança significativa detectada.");
                    return;
                }
                // stdout.chars(): Transforma o texto em um iterador de caracteres (letra por letra).
                // .take(4000): Pega apenas os primeiros 4000 caracteres e descarta o resto.
                // .collect(): Junta esses caracteres de volta numa nova String.
                let diff_truncated: String = stdout.chars().take(4000).collect();
                // Chama a próxima função (que cria a requisição HTTP), 
                // passando o texto truncado por referência (&).
                analyze_with_ollama(&diff_truncated);
            }
            Err(e) => {
                println!("Erro ao executar git diff: {}", e);
            }
        }
    }

    // Diff: &str Recebe uma referência para uma String Slice (texto imutável) ao texto do diff
    fn analyze_with_ollama(diff: &str) {
        println!("Enviando mudanças para análise do modelo AI ({})", diff.len());

        // Request HTTP para o Ollama
        // blocking atua como um await, pausando o programa até receber a resposta.
        let client = reqwest::blocking::Client::new();
        // !format é uma macro que mistura o texto com váriáveis.
        let prompt = format!(
        "Tarefa: Analise o código (git diff) e crie um Nome de Conquista RPG (Inglês, Max 3 palavras). \
        Regras: 1. ANALISE IMPORTS/FUNÇÕES. 2. USE TERMOS TÉCNICOS. 3. Trivial = FALSE. \
        Retorne APENAS o nome. \
        CÓDIGO: {}", diff
        );
        // No Rust, você não pode simplesmente escrever um objeto { chave: valor } solto como no JS. 
        // Você precisa usar essa macro para criar um objeto JSON válido que será enviado pela rede.
        let body = json!({
            "model": AI_MODEL,
            "messages": [
                { "role": "system", "content": "Você é um analista de código. Retorne APENAS o nome da conquista." },
                { "role": "user", "content": prompt }
            ],
            "options": { "temperature": 0.4 },
            "stream": false // Envia todo o texto de uma vez, não em partes
        });

        // client.post(...): Define o endereço e o método (POST).
        // .json(&body): Anexa o pacote que criamos acima no corpo da mensagem.
        match client.post("http://localhost:11434/api/chat").json(&body).send() {
            Ok(res) => {
                // res.json::<OllamaResponse>(): Tenta pegar o texto que voltou e forçar
                // ele a caber dentro daquela Struct 'OllamaResponse' que criamos no começo.
                // if let Ok(json): É um jeito curto de fazer um match.
                // Se a conversão para JSON der certo, chame o resultado de 'json' e entre no bloco.
                if let Ok(json) = res.json::<OllamaResponse>() {
                    // Passa apenas o texto da mensagem para a próxima etapa.
                    process_result(json.message.content);
                }
            }
            Err(e) => println!("Erro ao conectar com Ollama: {}", e),
        }

    }

    fn process_result(raw_result: String) {
        let mut result = raw_result.trim().to_string(); // Remove espaços em branco extras
        if result.contains("Conquista:") {
          result = result.split("Conquista:").nth(1).unwrap_or("").trim().to_string();
        }

        result = result.replace(&['"', '*'][..], "");

        result = result.lines().next().unwrap_or("").trim().to_string();

        println!("IA: {}", result);

        if result.to_uppercase().contains("FALSE") || result.len() < 3 {
            println!("Análise considerada trivial. Nenhuma conquista será registrada.");
        } else {
            println!("🎉 CONQUISTA: {}", result);
            save_to_db(&result);
            trigger_popup(&result);
        }
    }

    fn save_to_db(conquest_name: &str) {
        let exe_path = std::env::current_exe().unwrap();
        let db_path = exe_path.parent().unwrap().join("Stone.db");

        let conn = Connection::open(db_path).unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS conquest (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            date TEXT,
            timestamp INTEGER
        )",
        [],
        ).unwrap();

        let now = Local::now();

        conn.execute(
            "INSERT INTO conquest (name, date, timestamp) VALUES (?1, ?2, ?3)",
            params![conquest_name, now.format("%d/%m/%Y %H:%M:%S").to_string(), now.timestamp()],
        ).unwrap();

        println!("Salvo no banco de dados.");
    }

    fn trigger_popup(conquest_name: &str) {

        let exe_path = std::env::current_exe().unwrap();
        Command::new(exe_path)
        .arg(conquest_name) // Passa o nome para a janela ler
        .spawn() // Abre e libera (não espera fechar)
        .expect("Falha ao abrir o popup");

        /*
        #[cfg(target_os = "windows")]
        {
            use tauri::api::notification::Notification;

            Notification::new("com.seuapp.stone")
                .title("Nova Conquista Desbloqueada!")
                .body(conquest_name)
                .show()
                .unwrap();
        }

        #[cfg(target_os = "macos")]
        {
            use tauri::api::notification::Notification;

            Notification::new("com.seuapp.stone")
                .title("Nova Conquista Desbloqueada!")
                .body(conquest_name)
                .show()
                .unwrap();
        }

        #[cfg(target_os = "linux")]
        {
            use tauri::api::notification::Notification;

            Notification::new("com.seuapp.stone")
                .title("Nova Conquista Desbloqueada!")
                .body(conquest_name)
                .show()
                .unwrap();
        } */
       
    }