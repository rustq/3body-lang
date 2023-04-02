## System Libraries

### Rand

Utilities for random number generation

用于随机数生成

#### Chaotic Era and Stable Era

Default to be Chaotic Era, custom to be Stable Era.

默认自带乱纪元属性随机返回 0~3 之间，允许自定义初始恒星个数进入恒纪元。

#### Quick Start

```rust
给 rand 以 import("./system/3body/rand");
```

```rust
给 乱纪元 以 rand["default"]();

乱纪元["getSuns"]() // 0 | 1 | 2 | 3
```

```rust
给 恒纪元 以 rand["custom"](1);

恒纪元["getSuns"](); // 1
```

## source read (整体项目结构源码简单解读)

### evaluator mod

求值器：将 3bodylang 语言的库中一些基本函数（方法），比如 "广播" ，还有一些基本变量如 Int String Bool ,以及基本的语句 break等 用 rust 语言重新做定义，创建基本的语言环境

### lexer mod
词法分析：
读取识别3bodylang语言语句标志符号 fn  法则   if  else 等，识别基本的字符 是啥 比如是空格（is_whitespace） 或者是表情符（is_emoji_like）等等

词法分析器（Lexer）它的目的是读取一系列字符（即源代码）并将其转换为标记流，即具有不同语法含义的有意义的代码单元。

### parser mod
定义了一个结构体Parser，它需要一个 Lexer 类型的参数，通过该参数初始化 Parser 对象的 lexer 字段，最终生成ast


### wasm mod

定义3bodylang语言一些基本函数如 print sleep clear等


### format mod 

用于格式化用户输入：格式化不同类型的表达式和语句

### bin 目录

main 函数的执行目录：

1.首先，代码创建了一个 Env 对象并使用其构建新的内置函数上下文。然后，Evaluator 对象被创建（使用 Env 参数），用于评估表达式和执行代码。 MonkeyHelper 对象被创建，它将作为 Rustyline REPL 库的帮助程序。将配置项提供给 Config 构造器，此后将创建 Editor 对象作为代码 REPL。

2.如果程序以文件路径参数运行，则文件的内容被读取并解析与评估。否则开始一个死循环，等待用户输入命令。用户的输入将被送到 Lexer 和 Parser, 进行语法分析并生成一颗 AST (抽象语法树)。如果存在错误，将打印错误信息并让用户重新输入；否则，AST 将被传递给 Evaluator 被评估为 Rust 代码。


