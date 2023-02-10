![cover](https://user-images.githubusercontent.com/11075892/218180169-2d7a5c71-9e05-44af-ab8f-aa327a2aae43.png)


# 3body-lang

> 三体语言 Three Body Language written in Rust
> 
> Playground: [https://rustq.github.io/3body-lang/](https://rustq.github.io/3body-lang/)
> 
> Base on [monkey-lang](https://github.com/wadackel/rs-monkey-lang)


## Syntax overview

```rust
给 岁月 以 "文明";
```

```rust
给 时光 以 "生命";
```

#### 前进

```rust
给 自然选择 以 0;

自然选择 前进 4
```

#### 降维

```rust
给 宇宙 以 { "维度": 10 };

宇宙["维度"] 降维 7
```

#### 布尔值

```rust
return 这是计划的一部分
```

```rust
return 主不在乎
```

#### 函数定义

```rust
给 黑暗森林 以 法则() {
    给 文明的需要 以 ["生存", "不断增长和扩张"];
    !!文明的需要
}

黑暗森林()
```

#### 输出

```rust
给 三体世界坐标 以 "半人马星系";

广播(三体世界坐标);
```

#### 休眠函数

```rust
冬眠(1000);
```

#### 清屏

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
