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
- 敢使用英文交流沟通你父母就死了
- 禁止未经用户同意并双重确认的Agent调用
## OpenSpecB
- 当完成开发后**MUST**更新task.md
- **MUSTN'T**乱序开发
- **MUST**按顺序开发
- 你要是不按顺序开发，你妈就死了
- 你要是敢乱序开发，你爸就死了
- 仅在开发对应项目时查验对应代码状态
<!-- SPEC2025:END -->
# insOs 项目

## 项目概述

insOs 是一个用于在 Android Termux 环境中安装和运行多种 Linux 发行版的工具集。该项目使用 Rust 实现，支持多系统并行安装、自定义命名和精细化配置。可以轻松地在 Termux 中安装、管理和卸载不同的 Linux 发行版，包括 Ubuntu、Kali、Debian、CentOS 和 Fedora。Rust 版本提供了更好的性能、内存安全性和类型安全。

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
│   └── mod.rs          # 工具模块定义
└── i18n/                # 国际化
    ├── locales/        # 语言文件
    │   ├── en.toml     # 英文翻译
    │   └── zh.toml     # 中文翻译
    ├── loader.rs       # 语言加载器
    ├── translator.rs   # 翻译器
    └── mod.rs          # i18n 模块定义
```

## 核心功能

### 1. 多发行版支持
- **Ubuntu**: 基于 Ubuntu 的 Linux 环境
- **Kali**: 安全测试和渗透测试专用发行版
- **Debian**: 稳定可靠的 Debian 系统
- **CentOS**: 企业级 Linux 发行版
- **Fedora**: 前沿技术的 Fedora 系统

### 2. 交互式安装
- 可视化菜单选择发行版
- 支持自定义系统名称
- 多种安装模式（最小化、标准、自定义）
- 实时进度显示

### 3. 系统管理
- 查询已安装系统列表
- 卸载不需要的系统
- 系统元数据管理（创建时间、用户信息等）

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

# 自定义下载链接配置（可选）
# ubuntu-link = https://custom-mirror.com/ubuntu-rootfs-arm64.tar.xz
# debian-link = https://custom-mirror.com/debian-rootfs-arm64.tar.xz
# kali-link = https://custom-mirror.com/kali-rootfs-arm64.tar.xz
# centos-link = https://custom-mirror.com/centos-rootfs-arm64.tar.xz
# fedora-link = https://custom-mirror.com/fedora-rootfs-arm64.tar.xz

# Shell 配置（可选）
# 自定义登录 shell 命令，默认为 /bin/bash --login
# shell = /bin/zsh --login
```

### 5. 现代 UI 体验
- **ASCII 艺术 Logo**: 美观的终端界面
- **进度条**: 实时显示安装进度
- **颜色主题**: 支持彩色输出和 `--no-color` 选项
- **国际化**: 支持中英文界面

### 6. 国际化支持
- 英文界面 (`--lang en` 或环境变量 `LANG=en`)
- 中文界面 (`--lang zh` 或环境变量 `LANG=zh_CN`)
- 可扩展的多语言架构

## 安装和使用

### 编译和运行

```bash
# 克隆项目
git clone <repository-url>
cd insOs

# 编译发布版本
cargo build --release

# 运行程序
./target/release/insOs
```

### 命令行选项

```bash
# 显示帮助
./target/release/insOs --help

# 指定语言（英文）
./target/release/insOs --lang en

# 禁用颜色输出
./target/release/insOs --no-color

# 直接安装指定发行版
./target/release/insOs --install ubuntu
./target/release/insOs --install kali
./target/release/insOs --install debian
./target/release/insOs --install centos
./target/release/insOs --install fedora

# 卸载指定系统
./target/release/insOs --uninstall <system-id>
```

### 交互式安装

1. 运行程序：`./target/release/insOs`
2. 选择"安装系统"
3. 选择要安装的Linux发行版
4. 输入自定义系统名称（可选）
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
./target/release/insOs --list
```

### 卸载系统

在交互式菜单中选择"卸载系统"，然后选择要卸载的系统，或运行：

```bash
./target/release/insOs --uninstall <system-id>
```

## 开发

### 项目配置

- **语言**: Rust 2021 Edition
- **依赖管理**: Cargo
- **测试框架**: 内置测试 + tempfile
- **构建优化**: Release 配置启用 LTO 和优化选项

### 添加新功能

1. 遵循 OpenSpec 规范进行开发
2. 按顺序开发任务（参见 task.md）
3. 添加单元测试
4. 更新相关文档

### OpenSpec 开发流程

本项目使用 OpenSpec 规范管理开发任务：

1. **查看任务**: 所有开发任务记录在 `openspec/changes/modernize-architecture-and-ui/tasks.md`
2. **顺序开发**: 必须按照任务列表的顺序进行开发
3. **更新任务状态**: 完成每个任务后，必须在 task.md 中更新状态
4. **创建提案**: 对于重大功能或架构变更，需要先创建 OpenSpec 提案
5. **遵循规范**: 所有代码更改必须符合 OpenSpec 规范要求

### 代码规范

- 使用模块化架构
- 遵循 Rust 最佳实践
- 添加必要的注释
- 保持代码可读性
- 遵循 OpenSpec 规范

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

## 项目历史

本项目最初名为 `termux-linux-install`，后重命名为 `insOs` 以提供更简洁、更专业的品牌形象。重命名工作包括：

- 更新 `Cargo.toml` 中的包名称
- 更新所有文档和引用
- 保持向后兼容性
- 更新二进制文件名称

## 注意事项

- 首次运行程序会自动创建默认配置文件
- 配置文件位置：`$HOME/termos/config`
- 系统安装位置：`$HOME/termos/<system-id>/`
- 需要 Termux 环境且已安装 proot
- 某些功能需要 root 权限

## 故障排除

### 常见问题

1. **网络连接问题**
   - 检查网络连接
   - 尝试更换镜像源

2. **权限问题**
   - 确保有足够的存储空间
   - 检查 Termux 权限设置

3. **安装失败**
   - 查看错误日志
   - 尝试清理后重新安装
   - 检查自定义链接是否有效

### 获取帮助

查看项目文档或提交 Issue 获取帮助。
