Foremost Rule ↓
# > 用户高于一切 <
Main Rule ↑
<!-- OPENSPEC:START -->
# OpenSpec Instructions

These instructions are for AI assistants working in this project.

Always open `@/openspec/AGENTS.md` when the request:
- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/openspec/AGENTS.md` to learn:
- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Keep this managed block so 'openspec update' can refresh the instructions.

<!-- OPENSPEC:END -->
<!-- SPEC2025 -->
# Spec 2025
## RuleF
- **MUST**使用中文交流沟通
  - **禁止**英文交流沟通
- **禁止**使用 Agent 和 Task 工具
- **尽量不要**使用Shell命令进行文件读写
  - **可以**使用tool代替Shell命令，Tool列表如下:
    ```List
    - Edit
    - exit_plan_mode
    - glob
    - list_directory
    - multi_edit
    - plan
    - read plan
    - read_file
    - read_many_files
    - save_memory
    - Search # 无法使用则手动使用ripgrep搜索
    - Shell # 尽量不要用它进行文件读写
    - task # 禁止使用
    - web_fetch
    - web_search
    - write_file
    - xml_escape
    ```

## OpenSpecB
- **MUST**按顺序开发
  - **禁止**乱序开发
  - **禁止**跳序开发
- **ONLY**开发对应任务时查验对应代码实现状态
  - **禁止**未开发任务时查验代码实现状态
  - **禁止**开发任务时查验非与其对应的代码实现状态
- 当完成单个任务(如: 1.1 2.3 4.1)的开发后:
  **MUST**更新 `task.md` 以反映修改
    - **禁止**不更新 `task.md` 直接提交代码
  **MUST**使用 `git add . && git commit -m "fact: " -m "other..." ` 提交修改，如果提交到达上限则使用 `git push`
    - **禁止**提交代码后不向用户报告当前工作
  **MUST**暂停开发，向用户报告当前工作(已完成步骤内容、完成的步骤和正在开发的步骤)并征求**用户的**下一步意见
      - **禁止**不向用户征求下一步意见就继续开发

> Keep this managed block <
<!-- SPEC2025:END -->

# insOs 项目

## 项目概述

insOs 是一个用于在 Android Termux 环境中安装和运行多种 Linux 发行版的工具集该项目使用 Rust 实现，支持多系统并行安装、自定义命名和精细化配置可以轻松地在 Termux 中安装、管理和卸载不同的 Linux 发行版，包括 Ubuntu、Kali、Debian、CentOS 和 FedoraRust 版本提供了更好的性能、内存安全性和类型安全

## 项目结构

```
src/
├── main.rs              # 程序入口
├── cli/                 # 命令行接口
│   ├── args.rs         # 命令行参数处理
│   ├── interactive.rs  # 交互式菜单
│   └── mod.rs          # CLI 模块定义
├── config/              # 配置管理
│   ├── defaults.rs     # 默认配置
│   ├── parser.rs       # 配置文件解析
│   └── mod.rs          # 配置模块定义
├── distro/              # 发行版管理
│   ├── definitions.rs  # 发行版定义和安装逻辑
│   └── mod.rs          # 发行版模块定义
├── installer/           # 安装器
│   ├── interactive.rs  # 交互式安装
│   └── mod.rs          # 安装器模块定义
├── system/              # 系统管理
│   ├── manage.rs       # 系统管理功能
│   ├── permissions.rs  # 权限管理
│   └── mod.rs          # 系统模块定义
├── ui/                  # UI 组件
│   ├── colors.rs       # 颜色主题
│   ├── display.rs      # 显示功能
│   ├── progress.rs     # 进度条
│   └── mod.rs          # UI 模块定义
├── utils/               # 工具函数
│   ├── cmd.rs          # 命令执行
│   ├── fs.rs           # 文件系统操作
│   ├── net.rs          # 网络操作
│   ├── permissions.rs  # 权限管理
│   └── mod.rs          # 工具模块定义
└── i18n/                # 国际化
    ├── locales/        # 语言文件
    │   ├── en.toml     # 英文翻译
    │   └── zh.toml     # 中文翻译
    ├── loader.rs       # 语言加载器
    ├── translator.rs   # 翻译器
    └── mod.rs          # i18n 模块定义

openspec/
├── project.md          # 项目上下文和技术规范
├── specs/              # 模块规范文档
│   ├── cli/
│   ├── config/
│   ├── installer/
│   └── ui/
└── changes/            # 变更记录和提案
    └── archive/

target/
├── debug/              # 调试构建输出
└── release/            # 发布构建输出
```

## 核心功能

### 1. **多发行版支持**
- **Ubuntu**: 基于 Ubuntu 的 Linux 环境，默认安装 build-essential, curl, wget, git, vim, htop
- **Kali**: 安全测试和渗透测试专用发行版，默认安装 kali-linux-headless, metasploit-framework, nmap, wireless-tools, aircrack-ng, john, hydra, sqlmap
- **Debian**: 稳定可靠的 Debian 系统，默认安装 build-essential, devscripts, curl, wget, git, vim, htop, tmux
- **CentOS**: 企业级 Linux 发行版，默认安装 epel-release, vim, curl, wget, git, htop, net-tools, lsof
- **Fedora**: 前沿技术的 Fedora 系统，默认安装 @development-tools, curl, wget, git, vim, htop, tmux, dnf-plugins-core

### 2. **交互式安装**
- 可视化菜单选择发行版
- 支持自定义系统名称
- 多种安装模式(最小化、标准、自定义)
- 实时进度显示
- 命令行参数支持(`--name`, `--minimal`)
- ASCII 艺术 Logo 和现代化界面设计

### 3. **系统管理**
- 查询已安装系统列表
- 卸载不需要的系统
- 系统元数据管理(创建时间、用户信息、权限等)
- 支持多系统并行运行
- 系统实例自动命名(如 ubuntu1, ubuntu2)

### 4. 配置文件管理

通过 `config` 文件统一管理所有发行版的配置：
- 支持自定义镜像源URL
- 支持自定义下载链接
- 支持自定义登录 Shell
- 默认使用国内优化镜像源
- 首次运行自动创建默认配置文件

配置文件位置：`$HOME/termos/config`

配置文件格式：
```bash
# 镜像源配置
ubuntu-mirror = https://mirrors.ustc.edu.cn/ubuntu/
debian-mirror = https://mirrors.163.com/debian/
kali-mirror = http://http.kali.org/kali/
centos-mirror = https://mirrors.aliyun.com/centos/
fedora-mirror = https://mirrors.tuna.tsinghua.edu.cn/fedora/

# 自定义下载链接配置(可选)
# ubuntu-link = https://custom-mirror.com/ubuntu-rootfs-arm64.tar.xz
# debian-link = https://custom-mirror.com/debian-rootfs-arm64.tar.xz
# kali-link = https://custom-mirror.com/kali-rootfs-arm64.tar.xz
# centos-link = https://custom-mirror.com/centos-rootfs-arm64.tar.xz
# fedora-link = https://custom-mirror.com/fedora-rootfs-arm64.tar.xz

# Shell 配置(可选)
# 自定义登录 shell 命令，默认为 /bin/bash --login
# shell = /bin/zsh --login
```

### 5. **现代 UI 体验**
- **ASCII 艺术 Logo**: 美观的终端界面
- **进度条**: 实时显示安装进度
- **颜色主题**: 不支持彩色输但支持 `--no-color` 选项
- **国际化**: 支持中英文界面
- **响应式设计**: 适配小屏幕终端(最小支持 40 字符宽度)
- **统一的消息格式**: 成功(✓)、错误(✗)、信息(ℹ)图标

### 6. 国际化支持
- 英文界面 (`--lang en` 或环境变量 `LANG=en`)
- 中文界面 (`--lang zh` 或环境变量 `LANG=zh_CN`)
- 可扩展的多语言架构
- 语言文件位置：`src/i18n/locales/`
- 支持运行时语言切换

## 安装和使用

### 编译和运行

```bash
# 克隆项目
git clone git@github.com:uobe1/termux-linux.git
cd tl

# 编译发布版本
cargo build --release

# 运行程序
./target/release/insOs

# 或者安装到系统路径
cp target/release/insOs $PREFIX/bin/
```

### 命令行选项

```bash
# 显示帮助
insOs --help

# 指定语言(英文)
insOs --lang en

# 指定语言(中文)
insOs --lang zh

# 禁用颜色输出
insOs --no-color

# 直接安装指定发行版
insOs --install ubuntu
insOs --install kali
insOs --install debian
insOs --install centos
insOs --install fedora

# 自定义系统名称安装
insOs --install ubuntu --name my-ubuntu

# 最小化安装
insOs --install ubuntu --minimal

# 卸载指定系统
insOs --uninstall <system-id>

# 列出已安装系统
insOs --list
```

### 交互式安装

1. 运行程序：`insOs`
2. 选择"安装系统"
3. 选择要安装的Linux发行版
4. 输入自定义系统名称(可选)
5. 选择安装模式：
   - 最小化安装
   - 标准安装
   - 自定义安装

### 启动已安装的系统

安装完成后，系统会显示启动命令：

```bash
cd $HOME/termos/<system-id> && ./start.sh
```

### 查询已安装系统

在交互式菜单中选择"查询已安装系统"，或运行：

```bash
insOs --list
```

### 卸载系统

在交互式菜单中选择"卸载系统"，然后选择要卸载的系统，或运行：

```bash
insOs --uninstall <system-id>
```

## 开发

### 项目配置

- **语言**: Rust 2021 Edition
- **依赖管理**: Cargo
- **测试框架**: 内置测试 + tempfile
- **构建优化**: Release 配置启用 LTO 和优化选项
- **版本**: 0.2.0

### 构建和测试

```bash
# 调试构建
cargo build

# 发布构建(优化)
cargo build --release

# 运行测试
cargo test

# 运行代码检查
cargo clippy

# 格式化代码
cargo fmt
```

### 添加新功能

1. 遵循 OpenSpec 规范进行开发
2. 按顺序开发任务(参见 openspec/changes/ 目录)
3. 添加单元测试
4. 更新相关文档

### OpenSpec 开发流程

本项目使用 OpenSpec 规范管理开发任务：

1. **查看项目上下文**: `openspec/project.md` 包含项目技术规范和约定
2. **模块规范**: `openspec/specs/` 目录包含各模块的详细规范
3. **变更记录**: `openspec/changes/` 目录记录所有变更和提案
4. **顺序开发**: 必须按照任务列表的顺序进行开发
5. **更新任务状态**: 完成每个任务后，必须在相应文件中更新状态
6. **创建提案**: 对于重大功能或架构变更，需要先创建 OpenSpec 提案
7. **遵循规范**: 所有代码更改必须符合 OpenSpec 规范要求

### 代码规范

- 使用模块化架构
- 遵循 Rust 最佳实践
- 添加必要的注释
- 保持代码可读性
- 遵循 OpenSpec 规范
- 命名约定：
  - 模块名：小写蛇形命名(snake_case)
  - 函数名：小写蛇形命名(snake_case)
  - 结构体/枚举：大驼峰命名(PascalCase)
  - 常量：大写蛇形命名(SCREAMING_SNAKE_CASE)

## 技术架构

### 核心技术栈
- **Rust 2021 Edition**: 内存安全和高性能
- **标准库优先**: 仅使用 Rust 标准库和 toml 依赖
- **proot 技术**: 用户空间容器化，无需 root 权限
- **模块化设计**: 清晰的代码组织和职责分离
- **构建优化**: LTO、单编译单元、panic=abort、二进制瘦身

### 架构模式
- **单一职责**: 每个模块专注一个功能领域
- **低耦合**: 模块间依赖最小化，通过公共接口通信
- **高内聚**: 相关功能集中在同一模块
- **可扩展性**: 新功能通过添加或扩展模块实现

### 数据管理
- **元数据存储**: 每个系统实例使用 `meta.txt` 存储元信息
- **配置驱动**: 通过 `$HOME/termos/config` 统一管理镜像源
- **实例命名**: 格式为 `{DISTRO}{NUMBER}`(如 debian1、ubuntu2)
- **权限管理**: 自动检测和设置适当的文件权限

### 外部依赖
- **proot**: 必需的外部工具(用户需在 Termux 中安装)
- **tar**: 解压 rootfs 压缩包(Termux 自带)
- **wget/curl**: 下载镜像文件(Termux 自带)
- **screenfetch**: 可选的系统信息显示工具

## 配置示例

### 自定义镜像源

编辑 `$HOME/termos/config` 文件：

```bash
# 使用阿里云镜像
ubuntu-mirror = https://mirrors.aliyun.com/ubuntu/
debian-mirror = https://mirrors.aliyun.com/debian/

# 使用清华镜像
fedora-mirror = https://mirrors.tuna.tsinghua.edu.cn/fedora/
```

### 自定义下载链接

```bash
# 使用自定义 rootfs
ubuntu-link = https://your-mirror.com/ubuntu-rootfs-arm64.tar.xz
```

### 自定义 Shell

```bash
# 使用 Zsh 作为默认 Shell
shell = /bin/zsh --login

# 使用 Fish Shell
shell = /usr/bin/fish
```

### 版本更新

**v0.2.0**
- 重构为多模块架构
- 添加国际化支持(中英文界面)
- 增强 UI 体验(颜色主题、进度条)
- 支持自定义 Shell 配置
- 优化默认系统设置
- 添加 OpenSpec 规范管理
- 完善错误处理和用户反馈
- 添加命令行参数支持(`--name`, `--minimal`)
- 改进配置文件管理和默认值

## 注意事项

- 首次运行程序会自动创建默认配置文件
- 配置文件位置：`$HOME/termos/config`
- 系统安装位置：`$HOME/termos/<system-id>/`
- 需要 Termux 环境且已安装 proot
- 某些功能需要 root 权限
- 主要支持 ARM64 架构(Android Termux 环境)
- 需要稳定的网络连接以下载镜像文件

## 故障排除

### 常见问题

1. **网络连接问题**
   - 检查网络连接
   - 尝试更换镜像源

2. **权限问题**
   - 确保有足够的存储空间
   - 检查 Termux 权限设置

### 3. **安装失败**
   - 查看错误日志
   - 尝试清理后重新安装
   - 检查自定义链接是否有效
   - 确保有足够的存储空间(建议至少 2GB 可用空间)

### 4. **命令行参数问题**
   - 使用 `insOs --help` 查看所有可用选项
   - 确保参数顺序正确
   - 检查发行版名称拼写是否正确

### 5. **性能问题**
   - 使用国内镜像源提高下载速度
   - 考虑使用最小化安装模式
   - 在安装过程中避免运行其他大型程序

### 获取帮助

- 查看项目文档 `openspec/project.md`
- 提交 Issue 获取帮助
- 参考 README.md 获取使用示例
- 使用 `insOs --help` 查看命令行选项
