<h1 align="center">
    Rust-Package-Practice
</h1>

## About
Rustのmodとかの勉強をする。Rustのmodだとかは、ややこしい。

- [x] ディレクトリ構成（あるある）
- [x] modを使う。（pubあり）
- [x] 外部のクレートを使用する
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
- use パッケージ名/モジュール名;
- パッケージ内部のライブラリクレートを使用する際は、Cargo.tomlは特にいじらない。

## 外部のクレートを使用する。
mainAppで、以下のパッケージを使用する。
```zsh
sub
├── Cargo.toml
└── src
    └── lib.rs
```
sub/src/Cargo.toml
```
[package]
name = "sub"
version = "0.1.0"
edition = "2024"

[dependencies]
```
sub/src/lib.rs
足し算をするもの。
```Rust
pub mod ccc {
    pub fn adder(x: i32, y: i32) -> i32 {
        return x + y;
    }
}
```
これをmainAppで使用する。
Cargo.toml
```diff
[package]
name = "practice_rust"
version = "0.1.0"
edition = "2024"

[dependencies]
+ sub = { path = "../sub" }
```
- pathでは、`cargo run`した場所からパッケージまでの相対パスを指定する。
- subの部分では、使用するパッケージのCargo.tomlに記述された、\[package\]のnameを書く。

mainApp/main.rs
```Rust
use practice_rust::bbb;
use sub::ccc; // これを追加

fn main() {
    bbb::hello_from_bbb();
    let num1: i32 = 3;
    let num2: i32 = 5;
    let num = ccc::adder(num1 , num2);
    println!("3+5={:?}", num);
}

```
