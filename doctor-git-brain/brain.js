import ollama from 'ollama';
import { exec } from 'child_process';
import util from 'util';


const execAsync = util.promisify(exec); // Promisify exec for async/await usage

const pathToModel = "C:\\Users\\Thiago\\Desktop\\doctor-git\\src-tauri\\target\\release\\doctor-git.exe";

const AIModel = 'llama3.2';

async function analisarCodigo() {

    try {

        console.log("Iniciando análise de código com o modelo AI...");

        // 1. Obtém as alterações do último commit
        // --stat: mostra resumo de arquivos (ex: main.js +20 lines)
        // -p: mostra o código (patch)
        // HEAD^ HEAD: compara o agora com o anterior
        const { stdout, stderr } = await execAsync('git diff HEAD^ HEAD --stat -p');

        if (!stdout || stdout.length < 10){
            console.log("Nenhuma alteração significativa detectada no último commit.");
            return;
        }

        // 2. Limita o tamanho para não "explodir" a cabeça da IA
        // Pegamos os primeiros 3000 caracteres. É suficiente para ver nomes de arquivos e lógica principal.
        const diffTruncated = stdout.substring(0, 3000); // Limita a 3000 caracteres

        console.log("Diferenças do Git obtidas:\n", diffTruncated);

    const response = await ollama.chat({
            model: MODELO_IA,
            messages: [{ 
                role: 'user', 
                content: `
                Você é um juiz de código fonte.
                Analise o seguinte DIFF (alterações de código) de um projeto git:

                --- INICIO DO DIFF ---
                ${diffTruncated}
                --- FIM DO DIFF ---

                Instruções:
                1. Identifique se houve uma implementação real de funcionalidade, uma refatoração complexa ou uma adição importante (Ex: novas funções, lógica de banco de dados, nova tela).
                2. Se for algo relevante, retorne um NOME CURTO e ÉPICO para a conquista (Max 3 palavras). Ex: "Database Connected", "New Physics Engine", "Auth System".
                3. Se for apenas correção de texto, formatação, imports ou mudanças triviais, retorne APENAS a palavra "FALSE".
                
                Responda APENAS o nome ou FALSE.
                `
            }],
        });

    const result = response.message.content.trim();

    const conquest = result.replace(/["']/g, ""); // Remove aspas se houver

    if (conquest.toUpperCase().includes === "FALSE" || conquest.length < 2) {
        console.log("Nenhuma conquista relevante gerada pela IA.");
    } else {
        console.log("Conquista gerada pela IA:", conquest);
        exec(`start "" "${pathToModel}" --achievement "${conquest}"`);
    }
    } catch (error) {
        console.error("Erro ao analisar o código ou gerar conquista:", error);
    }
}
analisarCodigo();