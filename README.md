# Competitive Programming (Rust)

## コマンド

| コマンド                          | 説明                                   |
|-----------------------------------|----------------------------------------|
| `cargo run`                       | `src/main.rs`を実行                    |
| `cargo run --bin <target>`        | 指定した問題を実行（例: `abc431_a`）   |
| `cargo fmt`                       | コードフォーマット                     |
| `cargo clippy`                    | リンター実行                           |

## ファイル構成

```text
rust/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   └── bin/
│       ├── abc431_a.rs
│       ├── abc431_b.rs
│       └── abc431_c.rs
├── Cargo.toml
└── Cargo.lock
```
