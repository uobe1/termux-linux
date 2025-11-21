<!-- OPENSPEC:START -->
# OpenSpec Instructions

These instructions are for AI assistants working in this project.

Always open `@/openspec/AGENTS.md` when the request:
- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding
- When synchronizing the proposal completion status with `task.md`

Use `@/openspec/AGENTS.md` to learn:
- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Keep this managed block so 'openspec update' can refresh the instructions.

<!-- OPENSPEC:END -->
<!-- SPEC2025 -->
# Spec 2025
## RuleF
- 使用中文交流沟通
- 禁止未经用户同意并双重确认的Agent调用

## OpenSpecB
- 当完成开发后**MUST**更新task.md
<!-- SPEC2025:END -->
# TermuxForLinux 项目

## 项目概述

TermuxForLinux 是一个用于在 Android Termux 环境中安装和运行多种 Linux 发行版的工具集。该项目使用 Rust 实现，支持多系统并行安装、自定义命名和精细化配置。可以轻松地在 Termux 中安装、管理和卸载不同的 Linux 发行版，包括 Ubuntu、Kali、Debian、CentOS 和 Fedora。Rust 版本提供了更好的性能、内存安全性和类型安全。

## 项目结构

```
/root/TermuxForLinux/

├── Cargo.toml               # Rust 项目配置文件
├── Cargo.lock               # Rust 依赖锁定文件
├── .gitignore               # Git 忽略文件
├── README.md                # 说明文档
├── IFLOW.md                 # 项目文档
├── openspec/                # OpenSpec 规范目录
│   ├── AGENTS.md           # OpenSpec 代理指南
│   ├── project.md          # 项目约定
│   ├── specs/              # 当前规范
│   └── changes/            # 变更提案
│       └── enhance-ui-visuals/  # UI 增强提案
├── src/                     # Rust 源代码目录
│   ├── main.rs              # 主程序入口
│   ├── cli.rs               # 命令行参数处理和用户交互模块
│   ├── distro.rs            # Linux 发行版管理模块
│   ├── utils.rs             # 通用工具函数模块
│   ├── installer.rs         # 系统安装逻辑模块
│   ├── system.rs            # 系统管理和元数据模块
│   ├── config.rs            # 配置文件管理模块
│   └── ui.rs                # 用户界面和显示模块
├── target/                  # Rust 编译输出目录
│   └── release/             # 发布版本输出
│       └── termux-linux-install  # Rust 版本可执行文件
└── $HOME/Ostermux/          # 运行时生成目录（首次运行自动创建）
    └── config               # 统一配置文件（首次运行自动创建）
```

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
- **进度反馈**: 安装过程显示实时进度条

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

### 4. 配置文件管理

通过 `config` 文件统一管理所有发行版的配置：
- 支持自定义镜像源URL
- 支持自定义下载链接
- 默认使用国内优化镜像源
- 首次运行自动创建默认配置文件

配置文件格式：
```bash
# 镜像源配置
ubuntu-mirror = https://mirrors.ustc.edu.cn/ubuntu/
debian-mirror = https://mirrors.163.com/debian/
kali-mirror = http://http.kali.org/kali/
centos-mirror = https://mirrors.aliyun.com/centos/
fedora-mirror = https://mirrors.tuna.tsinghua.edu.cn/fedora/

# 自定义下载链接配置（可选）
# ubuntu-link = https://custom-mirror.com/ubuntu-rootfs-arm64.tar.xz
# debian-link = https://custom-mirror.com/debian-rootfs-arm64.tar.xz
# kali-link = https://custom-mirror.com/kali-rootfs-arm64.tar.xz
# centos-link = https://custom-mirror.com/centos-rootfs-arm64.tar.xz
# fedora-link = https://custom-mirror.com/fedora-rootfs-arm64.tar.xz
```

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
cd $HOME/Ostermux/debian1
./start.sh

# 或直接传递命令
./start.sh "ls -la"
```

### 配置文件

编辑 `$HOME/Ostermux/config` 文件：
```bash
# 镜像源配置
ubuntu-mirror = https://mirrors.ustc.edu.cn/ubuntu/
debian-mirror = https://mirrors.163.com/debian/

# 自定义下载链接（可选）
ubuntu-link = https://your-custom-mirror.com/ubuntu-rootfs-arm64.tar.xz
```

**注意**：
- 首次运行程序会自动创建默认配置文件
- 配置文件位置：`$HOME/Ostermux/config`

## 技术实现

- **多系统架构**: 通过独立目录和元数据管理多个系统实例
- **容器化技术**: 使用 proot 实现用户空间的容器化，无需 root 权限
- **元数据管理**: 通过 `meta.txt` 存储系统信息和配置
- **响应式界面**: 自适应终端宽度的显示效果
- **配置驱动**: 通过统一 `config` 文件管理镜像源和下载链接
- **优化的ID生成**: 使用哈希集合快速生成唯一实例ID
- **自动回退机制**: 自定义链接失效时自动使用默认下载源
- **权限处理**: 解压时使用 `--no-same-owner` 参数避免权限问题

### 技术特点

- **内存安全**: Rust 的所有权系统确保内存安全
- **零成本抽象**: 高级抽象不会带来运行时开销
- **类型安全**: 强类型系统在编译时捕获错误
- **模块化架构**: 清晰的模块分离，便于维护和扩展
- **标准库优先**: 仅使用标准库，减少依赖复杂度
- **性能优化**: 发布版本经过 LTO 和其他优化技术处理

### 模块化架构优势

1. **可维护性**: 代码结构清晰，便于定位和修改问题
2. **可扩展性**: 新功能可以通过添加新模块实现，不影响现有代码
3. **可重用性**: 模块可以在不同项目中重用
4. **团队协作**: 不同开发者可以独立开发不同模块
5. **测试友好**: 每个模块可以独立测试，提高代码质量
6. **性能优化**: 模块化设计允许针对特定模块进行性能优化

## 开发约定

- **多系统支持**: 所有功能支持多实例并行运行
- **统一接口**: 命令行和交互式界面功能一致
- **配置驱动**: 关键配置通过文件管理，便于定制
- **响应式设计**: 界面适配不同屏幕尺寸
- **向后兼容**: 保持与旧版本的基本兼容性

### 代码结构约定

- **main.rs**: 主程序入口，仅负责模块初始化和程序启动
- **cli.rs**: 命令行参数处理、交互式菜单、用户输入处理
- **distro.rs**: Linux 发行版定义、系统元数据、安装配置
- **utils.rs**: 通用工具函数、文件操作、命令执行
- **installer.rs**: 系统安装流程、镜像下载、解压和配置
- **system.rs**: 系统管理、卸载逻辑、系统实例管理
- **config.rs**: 配置文件管理、镜像源配置
- **ui.rs**: 用户界面显示、格式化输出、终端适配、进度条组件
- **错误处理**: 统一使用 `Result<T, E>` 类型
- **代码风格**: 遵循 Rust 官方代码风格和命名约定

### 模块化设计原则

1. **单一职责**: 每个模块只负责一个特定功能领域
2. **低耦合**: 模块间依赖最小化，通过公共接口通信
3. **高内聚**: 相关功能集中在同一模块内
4. **可扩展性**: 新功能可以通过添加新模块或扩展现有模块实现
5. **可测试性**: 每个模块都可以独立测试

## 注意事项

1. 安装过程需要稳定的网络连接
2. 首次安装会下载较大的根文件系统压缩包
3. 各发行版独立存储在 `$HOME/Ostermux/` 目录下
4. 系统ID自动生成，格式为 `{DISTRO}{NUMBER}`
5. 支持小屏幕终端操作，界面自适应宽度
6. 配置文件支持自定义，默认使用国内优化源
7. 首次运行会自动创建 `$HOME/Ostermux/config` 配置文件
8. 自定义下载链接失效时会自动回退到默认源

## 版本更新记录

### v0.1.2 (当前版本)
- ✅ UI界面优化：移除边框，添加进度条
- ✅ 改进文本排版和视觉层次
- ✅ 增强用户操作反馈