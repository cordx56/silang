# SILang
SILang - Simple Interpreter Language

大学の講義で作成したインタプリタの改善・拡張版。

元のプログラムはC++で書かれていたが、Rustで書き直した。

言語の試用は[https://silang.cordx.net/](https://silang.cordx.net/)から可能。

## Run
Running interpreter
```bash
$ cargo run
```

Running file
```bash
$ cargo run file.sil
```

## Language specification
### BNF
```
<program>    := (<multispace>* <statement> <multispace>*)*
<block>      := "{" <multispace>* <program> <multispace>* "}"
<statement>  := <multispace>* expression <space>* "\n"
<expression> := <factor> (<space>+ <factor>)*
<factor>     := <string> | <number> | <identifier> ("[" <expression> "]")? | "(" <multispace>* <expression>? <multispace>* ")" | <block>
```
