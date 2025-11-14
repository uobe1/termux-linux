# TermuxForLinux - Rust版本

这是TermuxForLinux项目的Rust移植版本，用于在Android Termux环境中安装和运行多种Linux发行版。

## 功能特性

- 安装/卸载 Ubuntu
- 安装/卸载 Kali Linux
- 安装/卸载 Debian
- 安装/卸载 CentOS
- 安装/卸载 Fedora
- 查询已安装的系统

## 构建和运行

### 构建项目

```bash
cargo build --release
```

### 运行程序

```bash
./target/release/Ostermux-install
```

## 项目结构

```
src/
├── main.rs      # 主程序入口和用户界面
├── distro.rs    # Linux发行版管理模块
└── utils.rs     # 工具函数模块
```

## 技术特点

- 使用纯Rust编写，无外部依赖
- 类型安全的错误处理
- 内存安全的系统操作
- 高性能的文件操作

## 使用说明

1. 确保在Termux环境中运行
2. 程序会自动安装必要的依赖包（如screenfetch）
3. 按照菜单提示选择要安装的Linux发行版
4. 安装完成后，可在`$HOME/Ostermux/`目录下找到对应的系统
5. 使用相应的启动脚本启动已安装的Linux系统

## 注意事项

- 安装过程需要稳定的网络连接
- 首次安装会下载较大的根文件系统压缩包
- 各发行版独立存储在`$HOME/Ostermuxu/`目录下

## 项目特点

本项目使用Rust编写，提供了安装和管理多种Linux发行版的功能，具有优秀的性能和安全性。