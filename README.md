# editor-in-rust
在Windows下用Rust语言编写一个编辑器。

先要安装好Rust Windows (GNU ABI) (.msi) 64bit 版本


我的项目目录为 /e/git/editor-in-rust
```
$cd /e/git/editor-in-rust
```

创建一个 project 作为 rust cargo 项目目录
```
$cargo new project --bin
```

直接运行就可以显示: Hello, World!
```
$cargo run
```

编辑 /e/git/editor-in-rust/project/src/main.rs 为一个 GUI 版本的 Hello World!

需要添加 [winapi](https://crates.io/crates/winapi), [user32](https://crates.io/crates/user32-sys) 外部库

在 Cargo.toml 的 dependencies 下添加
```
winapi = "0.2.8"
user32-sys = "0.2.0"
```
