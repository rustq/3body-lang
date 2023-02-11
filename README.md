# 3Body Lang

[![Package version](https://img.shields.io/crates/v/three_body_lang.svg)](https://crates.io/crates/three_body_lang)
[![License](https://img.shields.io/badge/license-MIT%20License-blue.svg)](https://github.com/rustq/three_body_lang/blob/master/LICENSE)

三体编程语言 Three Body Language written in Rust

Playground: [https://rustq.github.io/3body-lang/](https://rustq.github.io/3body-lang/)

Base on [monkey-lang](https://github.com/wadackel/rs-monkey-lang)

## Try 3Body Lang !

### With REPL

![carbon](https://user-images.githubusercontent.com/11075892/218237230-18000cfe-8db1-4bf7-979d-a11695039f35.png)


### With Online Playground

![playground](https://user-images.githubusercontent.com/11075892/218237993-c128c439-8048-406c-b043-0abcd33d4833.png)

Working with Wasm!! 主很在乎 👏🏻

[https://rustq.github.io/3body-lang/](https://rustq.github.io/3body-lang/)

## Syntax overview

### Variable bindings

Variable bindings, such as those supported by many programming languages, are implemented. Variables can be defined using the let keyword.

##### Format

```
给 <identifier> 以 <expression>;
```

##### Example

```rust
给 岁月 以 "文明";

给 时光 以 "生命";
```

### Operators

##### + 运算符

```rust
给 自然选择 以 0;

自然选择 前进 4

// > 4
```

#### - 运算符

```rust
给 宇宙 以 { "维度": 10 };

宇宙["维度"] 降维 7

// > 3
```

### Boolean

```rust
return 这是计划的一部分

// > true
```

```rust
return 主不在乎

// > false
```

### Function

#### Format

```
法则 (<parameter one>, <parameter two>, ...) { <block statement> };
```

#### Example

```rust
给 黑暗森林 以 法则() {
    给 文明的需要 以 ["生存", "不断增长和扩张"];
    !!文明的需要
}

黑暗森林()
```

## Built-in Functions

### Print

#### Format

```
广播(<arg1>, <arg2>, ...): void
```

#### Example

```rust
给 三体世界坐标 以 "半人马星系";

广播(三体世界坐标);

// > "半人马星系"
```

### Sleep

#### Format

```
冬眠(<arg1>): void
```

#### Example

```rust
冬眠(1000);
```

### Clear

```rust
二向箔清理();
```

## Summary

|Monkey|3body-lang|Explanation|
|---|---|---|
|let|给|"give"|
|=|以|"as"|
|+|前进|"go forward"|
|-|降维|"dimension reduction"|
|true|这是计划的一部分|"It's part of the plan."|
|false|主不在乎|"The Lord doesn't care."|
|fn|法则|"rule"|
|print|广播|"broadcast"|
|sleep|冬眠|"hibernation"|
|clear|二向箔清理|"two-way foil cleaning"|
|exit|破壁|"break the wall"|

## Development

```bash
$ git clone https://github.com/rustq/3body-lang.git

$ cd 3body-lang

$ make repl
```

```
$ make build_wasm
```

```
$ make test
```

## Contributors

| Author |
| ----------- |
| ![meloalright](https://avatars.githubusercontent.com/u/11075892?s=96&amp;v=4)       |
| [meloalright](https://github.com/meloalright)        |

## License

[MIT](https://opensource.org/licenses/MIT)
