const { invoke } = window.__TAURI__.core;

// Função utilitária para transformar o setTimeout em Promessa (await)
const delay = (ms) => new Promise((res) => setTimeout(res, ms));

async function init() {

  const text = await invoke("get_achievement");
  const itemElement = document.getElementById("item-name");
  itemElement.innerText = text;

  // Ajustar tamanho se o texto for muito grande
  if (text.length > 15) {
    itemElement.style.fontSize = "1.0rem";
  }

  // 1. O Popup fica visível por 6 segundos
  await delay(6000);
    
  // 2. Prepara a animação (CSS)
  document.body.style.transition = "opacity 0.5s ease";
  requestAnimationFrame(() => {
    document.body.style.opacity = "0"; // Esmaece na tela
  });

  // 3. Espera a animação de "sumiço" terminar (0.5s)
  await delay(500);
  
  // 4. Diz pro código Rust matar o processo completamente
  invoke("close_window");
}

window.addEventListener("DOMContentLoaded", () => {
  init();
});
