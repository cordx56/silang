# SILang - Simple Interpreter Language

大学の講義で作成したインタプリタの改善・拡張版。

元のプログラムはC++で書かれていたが、Rustで書き直した。

## 実行
インタプリタの実行
```bash
$ cargo run
```

ファイルの実行
```bash
$ cargo run file.sil
```

## 言語仕様
### BNF
```
Program := Statement*
Statement := Expression "\n" | Expression? "{" Statement* "}"
Expression := Factor (Whitespace+ Factor)*
Factor := String | Number | Identifier | "(" Expression ")"
```
