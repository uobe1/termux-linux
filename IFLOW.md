# TermuxForLinux 项目

## 项目概述

TermuxForLinux 是一个用于在 Android Termux 环境中安装和运行多种 Linux 发行版的工具集。该项目使用 Rust 实现，支持多系统并行安装、自定义命名和精细化配置。可以轻松地在 Termux 中安装、管理和卸载不同的 Linux 发行版，包括 Ubuntu、Kali、Debian、CentOS 和 Fedora。Rust 版本提供了更好的性能、内存安全性和类型安全。

## 项目结构

```
/root/TermuxForLinux/

├── Cargo.toml               # Rust 项目配置文件
├── .gitignore               # Git 忽略文件
├── README.md                # 说明文档
├── IFLOW.md                 # 项目文档
├── mirrors.conf             # 镜像源配置文件
├── src/                     # Rust 源代码目录
│   ├── main.rs              # 主程序入口和用户界面
│   ├── distro.rs            # Linux 发行版管理模块
│   └── utils.rs             # 工具函数模块
├── target/                  # Rust 编译输出目录
│   └── release/             # 发布版本输出
│       └── termux-linux-install  # Rust 版本可执行文件
```

## 新版本特性

### 🆕 多系统支持
- 支持同一发行版的多个实例并行安装
- 通过 `{OS_NAME}{NUMBER}` 方式区分（如 `debian1`, `debian2`）
- 每个系统独立运行，互不干扰

### 🆕 自定义命名
- 支持为每个系统自定义名称
- 通过 `meta.txt` 文件管理系统元数据
- 默认名称为系统ID（如 `debian1`），自定义安装可设置

### 🆕 新目录结构
```
$HOME/Ostermux/            # 默认安装目录，自定义安装可设置
├── debian1/
│   ├── start.sh           # 统一启动脚本
│   ├── meta.txt           # 系统元数据
│   └── filesys/           # 文件系统目录
│       ├── bin/
│       ├── etc/
│       ├── home/
│       └── ...
├── ubuntu1/
│   ├── start.sh
│   ├── meta.txt
│   └── filesys/
└── kali1/
    ├── start.sh
    ├── meta.txt
    └── filesys/
```

### 🆕 美化系统列表
- 适配小屏幕终端的响应式显示
- 显示系统名称、ID、创建时间、用户组、权限等信息
- 清晰的边框和布局设计

### 🆕 命令行支持
- 完整的命令行参数支持
- 支持批处理和脚本自动化
- 保留友好的交互式界面

### 🆕 配置文件换源
- 通过 `mirrors.conf` 统一管理镜像源
- 支持自定义镜像源URL
- 针对中国网络环境优化

## 核心功能

### 1. 主程序

#### termux-linux-install
- **全新架构**: 支持多系统并行管理
- **命令行支持**: 完整的CLI参数支持
- **交互式界面**: 简化的4选项主菜单
- **自定义命名**: 支持系统自定义名称
- **精细化安装**: 三种安装模式（最小化、标准、自定义）
- **无外部依赖**: 仅使用Rust标准库
- **小屏幕适配**: 优化移动终端显示效果

### 2. 命令行接口

```bash
# 交互式界面
./termux-linux-install

# 列出已安装系统
./termux-linux-install --list

# 安装指定发行版
./termux-linux-install --install ubuntu --name "开发环境"

# 最小化安装
./termux-linux-install --install debian --minimal

# 卸载指定系统
./termux-linux-install --uninstall debian1

# 显示帮助
./termux-linux-install --help
```

### 3. 启动脚本

统一的启动脚本 `start.sh`，使用 proot 技术在 Termux 环境中创建容器化的 Linux 环境：
- 设置 proot 环境
- 挂载必要的系统目录 (/dev, /proc)
- 配置环境变量 (PATH, HOME, TERM, LANG)
- 启动 bash shell
- 支持命令参数传递

### 4. 镜像源配置

通过 `mirrors.conf` 文件管理所有发行版的镜像源：
- 支持自定义镜像源URL
- 默认使用国内优化镜像源
- 格式：`{distro}-mirror = {URL}`

## 使用方法

### 安装依赖

```bash
pkg install screenfetch -y
```

### 编译和运行

```bash
# 克隆项目
git clone <repository-url>
cd TermuxForLinux

# 编译发布版本
cargo build --release

# 运行程序
./target/release/termux-linux-install
```

### 交互式安装

1. 运行程序选择"安装系统"
2. 选择要安装的Linux发行版
3. 输入自定义系统名称（可选）
4. 选择安装模式：
   - 最小化安装
   - 标准安装
   - 自定义安装

### 启动已安装的系统

```bash
cd $HOME/Termux-Linux/debian1
./start.sh

# 或直接传递命令
./start.sh "ls -la"
```

### 配置镜像源

编辑 `mirrors.conf` 文件：
```bash
# Ubuntu 镜像源
ubuntu-mirror = https://mirrors.ustc.edu.cn/ubuntu/

# Debian 镜像源
debian-mirror = https://mirrors.163.com/debian/
```

## 技术实现

- **多系统架构**: 通过独立目录和元数据管理多个系统实例
- **容器化技术**: 使用 proot 实现用户空间的容器化，无需 root 权限
- **元数据管理**: 通过 `meta.txt` 存储系统信息和配置
- **响应式界面**: 自适应终端宽度的显示效果
- **配置驱动**: 通过配置文件管理镜像源和其他设置

### Rust 版本技术特点

- **内存安全**: Rust 的所有权系统确保内存安全
- **零成本抽象**: 高级抽象不会带来运行时开销
- **类型安全**: 强类型系统在编译时捕获错误
- **模块化架构**: 清晰的模块分离，便于维护和扩展
- **标准库优先**: 仅使用标准库，减少依赖复杂度
- **性能优化**: 发布版本经过 LTO 和其他优化技术处理

## 开发约定

- **多系统支持**: 所有功能支持多实例并行运行
- **统一接口**: 命令行和交互式界面功能一致
- **配置驱动**: 关键配置通过文件管理，便于定制
- **响应式设计**: 界面适配不同屏幕尺寸
- **向后兼容**: 保持与旧版本的基本兼容性

### 代码结构约定

- **main.rs**: 主程序入口、命令行处理、用户界面
- **distro.rs**: 发行版管理、系统元数据、安装逻辑
- **utils.rs**: 工具函数、文件操作、显示辅助
- **错误处理**: 统一使用 `Result<T, E>` 类型
- **代码风格**: 遵循 Rust 官方代码风格和命名约定

## 注意事项

1. 安装过程需要稳定的网络连接
2. 首次安装会下载较大的根文件系统压缩包
3. 各发行版独立存储在 `$HOME/Termux-Linux/` 目录下
4. 系统ID自动生成，格式为 `{OS_NAME}{NUMBER}`
5. 支持小屏幕终端操作，界面自适应宽度
6. 镜像源配置文件支持自定义，默认使用国内优化源

