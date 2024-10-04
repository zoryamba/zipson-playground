# zipson-playground

This is my Capstone Project for [2024 Summer RustCamp](https://www.linkedin.com/posts/ukrainian-rust-community_we-are-excited-to-announce-the-enrollment-activity-7175432591154503680-fcdf?utm_source=share&utm_medium=member_desktop) provided by [Ukrainian Rust Community](https://www.linkedin.com/company/ukrainian-rust-community/).

[serde_zipson](https://github.com/zoryamba/serde_zipson) is [serde](https://serde.rs/)-compatible Rust implementation of [zipson](https://www.npmjs.com/package/zipson) compression format.

This simple web-playground allows to convert JSON data to zipson and vice versa in realtime. It is built on top of [Leptos](https://leptos.dev/) ([WASM](https://developer.mozilla.org/en-US/docs/WebAssembly)-based reactive front-end framework) and [Tailwind](https://tailwindcss.com/) (utility-first CSS framework)

This playground is available on [Github Pages](https://zoryamba.github.io/zipson-playground/)

### Prerequisites

1. Install [Rust](https://www.rust-lang.org/tools/install)

2. Install [Trunk](https://trunkrs.dev/#install)

3. Add `wasm` target to Rust toolchain: 
   ```
   rustup target add wasm32-unknown-unknown
   ```

### Build project
```
trunk build
```

### Or run dev server
```
trunk serve --port <port> [--open]
```

### Known issues

 - initial conversion for some reason takes 40+ ms while consequent conversions take up to 1 ms 
 - `serde_zipson` panics on integer overflow
 - `serde_zipson` repeat feature not working yet, so `[1,1,1,1,1,1,1,1,1]` ends up in `|ÊÊÊÊÊÊÊÊÊ÷` instead of `|Êþþþ^5÷`
 - `serde_zipson` object template feature not working yet, so `[{"key":"value1"},{"key":"value2"}]` ends up in `|{¨key¨¨value1¨}{ß0¨value2¨}÷` instead of `|¦¨key¨‡¨value1¨¨value2¨—÷`

### Further improvements

 - try to attach some JS code editor with syntax highlighting (e.g. [Ace Editor](https://ace.c9.io/)) to JSON input using [wasm-bindgen](https://book.leptos.dev/web_sys.html#using-js-libraries-with-wasm-bindgen)
 - set up [GitHub Actions](https://medium.com/@mpaternostro/how-to-deploy-pages-on-github-using-actions-a9281d03b345) to deploy to pages