const { invoke } = window.__TAURI__.core;

let dateInputEl;
let dateMsgEl;
let dateDisplayEl;
let dateBtnEl;
let helpMsgEl;
let helpDisplayed = false;

async function randomDate() {
  dateMsgEl.textContent = "Try calculate the day of the week corresponding to:";
  dateDisplayEl.textContent = await invoke("generate_date");
}

async function checkAnswer() {
  dateMsgEl.textContent = await invoke("check_answer",
    { answer: dateInputEl.value, randDate: dateDisplayEl.value }
  );
}

async function displayHelp() {
  if (helpDisplayed) {
    helpMsgEl.textContent = "";
    helpDisplayed = false;
  } else {
    helpMsgEl.textContent = await invoke("help");
    helpDisplayed = true;
  }
}

window.addEventListener("DOMContentLoaded", () => {
  dateInputEl = document.querySelector("#date-input");
  dateMsgEl = document.querySelector("#date-msg");
  dateDisplayEl = document.querySelector("#date-display");
  dateBtnEl = document.querySelector("#date-btn");
  helpMsgEl = document.querySelector("#help");

  randomDate();

  document.querySelector("#date-btn").addEventListener("click", (e) => {
    e.preventDefault();
    randomDate();
  });

  document.querySelector("#date-form").addEventListener("submit", (e) => {
    e.preventDefault();
    checkAnswer();
  });

  document.querySelector("#help-btn").addEventListener("click", (e) => {
    e.preventDefault();
    displayHelp();
  })
});
