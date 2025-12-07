const { invoke } = window.__TAURI__.core;

async function init() {

  const text = await invoke("get_achievement");
  const itemElement = document.getElementById("item-name");
  itemElement.innerText = text;

  if (text.length > 15) {
    itemElement.style.fontSize = "1.0rem";
  }

  setTimeout(async () => {
    
    document.body.style.transition = "opacity 0.5s ease";

    requestAnimationFrame(() => {
      document.body.style.opacity = "0";
    });

    setTimeout(() => {
      invoke("close_window");
    }, 500); 
  }, 6000);
}
    

  window.addEventListener("DOMContentLoaded", () => {
    init();
  });
