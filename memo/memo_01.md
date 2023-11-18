# スケルトンの作成

## 手順

- スケルトンの作成

  ```bash
  nodenv local 20.8.0
  npm init rust-webpack walk-the-dog
  cd walk-the-dog
  npm i
  ```

- サーバーの起動

  ```bash
  npm run start
  ```

- `Cargo.toml` の `package.edition` を修正する

  ```diff
  # You must change these to your own details.
  [package]
  name = "rust-webpack-template"
  description = "My super awesome Rust, WebAssembly, and Webpack project!"
  version = "0.1.0"
  authors = ["You <you@example.com>"]
  categories = ["wasm"]
  readme = "README.md"
  - edition = "2018"
  + edition = "2021"

  # ...snip...
  ```