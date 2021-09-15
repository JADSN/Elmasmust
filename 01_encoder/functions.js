"use strict";

console.log("functions.js - START");

const dispatch = (command, data) => {
  const { entrypoint } = wasm_bindgen;

  const json = JSON.stringify({
    "cmd": command,
    "data": data,
  });

  const result = entrypoint(json);

  return result;
};

// * Get value of the current radio button selected by name
const getValueOfCurrentRadioButtonSelectedByName = (name) => {
  const radioButtons = document.getElementsByName(name);
  let optionChosen = "";
  for (let i = 0; i < radioButtons.length; i++) {
    if (radioButtons[i].checked == true) {
      optionChosen = radioButtons[i].value;
    }
  }

  return optionChosen;
};

console.log("functions.js - END");
