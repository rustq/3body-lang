# 3body-lang

[![License](https://img.shields.io/badge/license-MIT%20License-blue.svg)](https://opensource.org/licenses/MIT)
[![Package version](https://img.shields.io/crates/v/three_body_lang.svg)](https://crates.io/crates/three_body_lang)
[![Workflow](https://img.shields.io/github/actions/workflow/status/rustq/3body-lang/CI.yml?branch=main)](https://github.com/rustq/3body-lang/actions)


三体编程语言 Three Body Language written in Rust

![carbon](https://user-images.githubusercontent.com/11075892/218237230-18000cfe-8db1-4bf7-979d-a11695039f35.png)

目前三体编程语言已经实现了如 "面壁"、"破壁"、"思想钢印"、"冬眠" 等语法，不过仍然是一个概念级的编程语言。

解释器的设计均来自于作者 Thorsten Ball 的原书，同时很感谢多个优秀开源仓库为本项目带来的灵感启发。

All from the book [Writing An Interpreter In Go](https://interpreterbook.com/)

Inspired by [wadackel/rs-monkey-lang](https://github.com/wadackel/rs-monkey-lang) + [flaneur2020/pua-lang](https://github.com/flaneur2020/pua-lang) which also inspired [Monkey-Rust-2021-Edition](https://github.com/meloalright/Monkey-Rust-2021-Edition)

## ⚡️ Installation

```shell
$ brew install three-body
```

`Program in REPL`

```shell
$ 3body
```

`Get help message`

```shell
$ 3body -h
```

## Syntax overview

### Variable bindings

##### Format

```
给 <identifier> 以 <expression>;
```

##### Example

```rust
给 岁月 以 "文明";

给 时光 以 "生命";
```

### Constant bindings

##### Format

```
思想钢印 <identifier> = <expression>;
```

##### Example

```rust
思想钢印 水 = "剧毒的";
```

### Operators

##### 前进(+)运算符

```rust
给 自然选择 以 0;

自然选择 前进 4

// > 4
```

##### 降维(-)运算符

```rust
给 宇宙 以 { "维度": 10 };

宇宙.维度 降维 7

// > 3
```

### Boolean

```rust
这是计划的一部分

// > true
```

```rust
主不在乎

// > false
```

### Function

##### Format

```
法则 (<parameter one>, <parameter two>, ...) { <block statement> };
```

##### Example

```rust
给 黑暗森林 以 法则() {
    给 基本公理 以 ["生存是文明的第一需要", "文明不断增长和扩张，但宇宙中的物质总量保持不变"];
    基本公理
}

黑暗森林()
```

### Loop

##### Format

```
面壁 (<expression>) { <block statement> };
```

##### Example

```rust
给 危机纪年 以 3;
给 面壁者 以 ["泰勒", "雷迪亚兹", "希恩斯", "罗辑"];

面壁 (危机纪年 < 400) {

    危机纪年 = 危机纪年 + 1;

    if (危机纪年 == 8) {
        面壁者 = rest(面壁者);
        延绪;
    }
    if (危机纪年 == 23) {
        面壁者 = rest(面壁者);
        延绪;
    }
    if (危机纪年 == 205) {
        面壁者 = rest(面壁者);
    }

    if (危机纪年 == 205) {
        破壁;
    }
}

面壁者
```

## Built-in Functions

##### Print 广播

```rust
给 三体世界坐标 以 "半人马星系";

广播(三体世界坐标);

// > "半人马星系"
```

##### Sleep 冬眠

```rust
冬眠(1000);
```

##### Clear 二向箔清理

```rust
二向箔清理();
```

##### Exit 毁灭

```rust
毁灭();
```

##### Deep Equal 没关系的都一样

```rust
没关系的都一样([1, [2, 3], { "4": 5 }], [1, [2, 3], { "4": 5 }]);

// > true
```

## Summary

|Token|3body-lang|Explanation|
|---|---|---|
|let|给|"give"|
|=|以|"as"|
|const|思想钢印|"thoughtcontrou"|
|+|前进|"go forward"|
|-|降维|"dimension reduction"|
|true|这是计划的一部分|"It's part of the plan."|
|false|主不在乎|"The Lord doesn't care."|
|fn|法则|"rule"|
|while|面壁|"face the wall"|
|break|破壁|"break the wall"|
|continue|延绪|"continue"|
|print|广播|"broadcast"|
|sleep|冬眠|"hibernation"|
|clear|二向箔清理|"two-way foil cleaning"|
|exit|毁灭|"destroy"|
|deep-equal|没关系的都一样|"It's okay. It's all the same."|

## Development

```bash
$ git clone https://github.com/rustq/3body-lang.git

$ cd 3body-lang

$ cargo run --features="repl"
```

```
$ make test
```

有更多建议和想法 💡

Create issues: [issues](https://github.com/rustq/3body-lang/issues)

## License

[MIT](https://opensource.org/licenses/MIT)
