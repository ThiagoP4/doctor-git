#!/usr/bin/env node

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import { execSync } from 'child_process';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const args = process.argv.slice(2);
const command = args[0];

if (command === 'setup') {
    console.log("Configurando o ambiente...");

    const gitDir = path.resolve(process.cwd(), '.git');
    const hooksDir = path.join(gitDir, 'hooks');
    const hookFile = path.join(hooksDir, 'post-commit');

    if (!fs.existsSync(gitDir)) {
        console.error("Erro: Você não está em um repositório Git! Rode 'git init' primeiro.");
        process.exit(1);
    }

    const brainPath = path.join(__dirname, 'doctor-git-brain', 'brain.js');

    const brainPathFixed = brainPath.replace(/\\/g, '/');

    const HookContent = `#!/bin/sh
    # Hook do Doctor Git
    node "${brainPathFixed}" &
    `;

    try {
        fs.writeFileSync(hookFile, HookContent, { mode: 0o755 });
        console.log("Hook de post-commit instalado com sucesso!");
    } catch (err) {
        console.error("Erro ao instalar o hook:", err);
    }
} else {
    console.log("Comandos disponíveis:");
    console.log("  doctor-git setup  -> Instala o monitoramento no repositório atual");
}