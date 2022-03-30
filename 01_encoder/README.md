# Encoders

## Setup

1. `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`
1. `sudo apt-get install binaryen wabt`
1. `cargo install wasm-snip`
1. `cargo install just`

## Usage

Execute 
    -  First time: `just setup`
    -  `just listen`

## If change package name:

1. Alter the `README.md` title
1. Alter the `index.html` in `<script defer src="./pkg/<package_name>.js"></script>`
1. Alter the `main.js` in `await wasm_bindgen("./pkg/<package_name>_bg.wasm");`

## Dependency techs

1. **binaryen and wabt** `sudo apt-get install binaryen wabt`
1. **wasm-snip** - `https://github.com/rustwasm/wasm-snip`

## References

[Shrinking .wasm Code Size](https://rustwasm.github.io/docs/book/reference/code-size.html#shrinking-wasm-code-size)

[Create a New Rust/Webpack Project using the rust-webpack Template](https://egghead.io/lessons/webpack-create-a-new-rust-webpack-project-using-the-rust-webpack-template) - [Git](https://github.com/nikgraf/webassembly-rust-course/blob/master/lesson-5/crate/Cargo.toml)
[Serde](https://rustwasm.github.io/docs/wasm-bindgen/reference/arbitrary-data-with-serde.html)
[Style](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/radio)