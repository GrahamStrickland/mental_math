const { invoke } = window.__TAURI__.core;

let dateInputEl;
let dateMsgEl;
let dateBtnEl;

async function randomDate() {
  dateMsgEl.textContent = await invoke("generate_date");
}

async function checkAnswer() {
  dateMsgEl.textContent = await invoke("check_answer", { answer: dateInputEl.value, rand_date: dateMsgEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  dateMsgEl = document.querySelector("#date-msg");
  dateInputEl = document.querySelector("#date-input");
  dateBtnEl = document.querySelector("#date-btn");

  document.querySelector("#date-btn").addEventListener("submit", (e) => {
    e.preventDefault();
    randomDate();
  });

  document.querySelector("#date-form").addEventListener("submit", (e) => {
    e.preventDefault();
    checkAnswer();
  });
});
