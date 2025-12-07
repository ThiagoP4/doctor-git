import ollama from 'ollama';
import { spawn } from 'child_process';
import { exec } from 'child_process';
import util from 'util';
import sqlite3 from 'sqlite3';
import { open } from 'sqlite';
import path from 'path';
import { fileURLToPath } from 'url';


const execAsync = util.promisify(exec); // Promisify exec for async/await usage

// --- CONFIGURAÇÕES ---
const pathToModel = "C:\\Users\\Thiago Silva\\Projetos\\doctor-git\\src-tauri\\target\\release\\doctor-git.exe";
const AIModel = 'llama3.2';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const DB_PATH = path.join(__dirname, 'Stone.db');


async function registrarConquista(nomeConquista) {
    try {
        const db = await open({
            filename: DB_PATH,
            driver: sqlite3.Database
        });

        await db.exec(`
            CREATE TABLE IF NOT EXISTS conquest (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                date TEXT,
                timestamp INTEGER
            )
        `);

        await db.run(
            'INSERT INTO conquest (name, date, timestamp) VALUES (?, ?, ?)',
            nomeConquista,
            new Date().toLocaleString('pt-BR'),
            Date.now()
        );
        console.log(`Conquista "${nomeConquista}" registrada no banco de dados.`);
        await db.close();
    } catch (error) {
        console.error("Erro ao registrar conquista no banco de dados:", error);
    }
}


async function analisarCodigo() {
    try {
        console.log("Escaneando alterações no código...");

        // 1. Obtém as alterações do último commit
        // --stat: mostra resumo de arquivos (ex: main.js +20 lines)
        // -p: mostra o código (patch)
        // HEAD^ HEAD: compara o agora com o anterior
        const { stdout, stderr } = await execAsync('git diff HEAD~1 HEAD --stat -p');
        
        if (!stdout || stdout.length < 10){
            console.log("Nenhuma alteração significativa detectada no último commit.");
            return;
        }

        // 2. Limita o tamanho para não "explodir" a cabeça da IA
        // Pegamos os primeiros 3000 caracteres. É suficiente para ver nomes de arquivos e lógica principal.
        const diffTruncated = stdout.substring(0, 4000); // Limita a 3000 caracteres

        console.log("Diferenças do Git obtidas:\n", diffTruncated);

        const response = await ollama.chat({
            model: AIModel,
            messages: [{ 
                role: 'system', 
                content: `Você é um gerador de conquistas para programadores. Você é uma API que retorna APENAS dados crus. NÃO converse. NÃO explique. NÃO use markdown. Retorne APENAS o nome da conquista.`
            }, { 
                role: 'user', 
                content: ` Analise este git diff e identifique a mudança técnica mais importante.
                
                REGRAS CRÍTICAS:
                1. Use APENAS termos do código real analisado (nomes de funções, variáveis, tecnologias)
                2. NÃO use os exemplos que vou mostrar - eles são apenas formato
                3. Se for mudança trivial (typo, docs, formatação) → retorne: FALSE
                4. Máximo 3 palavras técnicas em inglês

                FORMATO (NÃO COPIE O CONTEÚDO, apenas o estilo):
                [Tecnologia] [Ação] [Contexto]

                Exemplos de FORMATO (invente o seu baseado no diff real):
                - Se adicionou SQLite: "SQLite Integration"
                - Se mudou de fetch para axios: "Axios Migrated"
                - Se criou hook React: "Custom Hook Created"
                - Se corrigiu memory leak: "Memory Leak Fixed"

                IMPORTANTE: Os exemplos acima são APENAS para mostrar o formato. Você DEVE criar um nome novo baseado no diff abaixo.

                DIFF:
                ${diffTruncated}
                Responda APENAS com o nome da conquista ou FALSE:`
            }],
            options: {
                temperature: 0.3,
                top_p: 0.9,
                num_predict: 20,
            }
            
        });

    let result = response.message.content.trim();

    if (result.includes("Conquista:")) {
        result = result.split("Conquista:")[1].trim();
    }

    result = result.replace(/\*\*/g, "").replace(/\*/g, "").trim();

    result = result.split("\n")[0].trim();

    if (result.endsWith(".")) {
        result = result.slice(0, -1);
    }

    console.log(`🤖 Resposta Bruta da IA: "${response.message.content.slice(0, 50)}..."`);
    console.log(`✨ Conquista Processada: "${result}"`);

    const conquest = result;

    if (conquest.toUpperCase().includes("FALSE") || conquest.length < 2) {
        console.log("Nenhuma conquista relevante gerada pela IA.");
    } else {
        console.log("Conquista gerada pela IA:", conquest);

        await registrarConquista(conquest);

        const child = spawn(pathToModel, [conquest], {
                detached: true,   // Permite que o Node feche enquanto o pop-up fica aberto
                stdio: 'ignore'   // Não trava o terminal esperando resposta
            });
        child.unref();
    }
    } catch (error) {
        console.error("Erro ao analisar o código ou gerar conquista:", error);
    }
}
analisarCodigo();