<h1 align="center">
    Rust-Package-Practice
</h1>

## About
Rustのmodとかの勉強をする。Rustのmodだとかは、ややこしい。

- [x] ディレクトリ構成（あるある）
- [x] modを使う。（pubあり）
- [ ] 外部のクレートを使用する
- [ ] modの中から、さらに別のクレートを使用する。

## ディレクトリ構成
クレートのあるある
```zsh
.
├── Cargo.toml
├── README.md
└── src
    ├── lib.rs // ライブラリクレートのエントリポイント
    └── main.rs // バイナリクレートのエントリポイント
```

## バイナリクレートを使用する。
src/lib.rs
```Rust
pub mod bbb {
    pub fn hello_from_bbb() {
	println!("hello from bbb.");
    }
}
```
- 関数を外部から使用したい場合は、pubを関数にもつける。
src/main.rs
```Rust
use practice_rust::bbb;

fn main() {
    bbb::hello_from_bbb();
}
```
- use (ファイルパス)/パッケージ名/モジュール名;
- パッケージ内部のライブラリクレートを使用する際は、Cargo.tomlは特にいじらない。

## 
