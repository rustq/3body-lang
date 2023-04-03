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

##源码解读 (Source code interpretation)

###目录结构
