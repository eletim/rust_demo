# rust_demo

このプロジェクトは、Rustの所有権・借用の基本を確認するための最小デモです。
Ubuntu 22.04 上での動作を前提としています。

---

## 動作環境

- OS: Ubuntu 22.04
- Rust: stable（rustup でインストール）

---

## Rustのインストール

このプロジェクトをビルドする前に、Rustをインストールしてください。

### rustup を使ったインストール（推奨）

```bash
sudo apt update
sudo apt install -y curl build-essential

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
````

インストール後、環境変数を反映します。

```bash
source ~/.cargo/env
```

正しくインストールされているか確認します。

```bash
rustc --version
cargo --version
```

---

## ビルドと実行

プロジェクトのルートディレクトリで以下を実行してください。

```bash
cargo build
cargo run
```

---

## このデモで確認できること

* 構造体がデータを「所有」するとはどういうことか
* 不変参照と可変参照の違い
* Rustがコンパイル時にメモリ安全性を保証する仕組み

