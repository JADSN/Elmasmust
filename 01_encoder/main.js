"use strict";

const inpUserText = document.querySelector("input#inp-user-text");
const h3Sha1Code = document.querySelector("h3#h3-sha1-code");

const onInpUserTextKeyUp = (event) => {
  const value = event.target.value;

  // * getValueOfCurrentRadioButtonSelectedByName - functions.js
  const optionChosen = getValueOfCurrentRadioButtonSelectedByName("encode");

  // * dispatch - functions.js
  h3Sha1Code.textContent = dispatch(optionChosen, value);
};

const onRadioButtonEncodeOptionClick = (event) => {
  const optionChosenValue = event.target.value;
  const valueNormalized = inpUserText.value;

  // * dispatch - functions.js
  h3Sha1Code.textContent = dispatch(optionChosenValue, valueNormalized);
};

const main = async () => {
  await wasm_bindgen("./pkg/sha2encoder_bg.wasm");

  console.log("main.js - START");

  // * Set default value: ""(empty string)

  // * getValueOfCurrentRadioButtonSelectedByName - functions.js
  const optionChosen = getValueOfCurrentRadioButtonSelectedByName("encode");
  // * dispatch - functions.js
  h3Sha1Code.textContent = dispatch(optionChosen, inpUserText.value);

  // * Add click event for each encoded option.
  inpUserText.addEventListener("keyup", onInpUserTextKeyUp);

  const inputs = document.querySelectorAll(
    `#div-encode-options > div > input[type='radio']`,
  );
  const inputsArr = Array.from(inputs);

  inputsArr.forEach((input) => {
    input.addEventListener("click", onRadioButtonEncodeOptionClick);
  });

  console.log("main.js - END");
};

main();
