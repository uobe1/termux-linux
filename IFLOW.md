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
# TermuxForLinux 项目

## 项目概述

TermuxForLinux 是一个用于在 Android Termux 环境中安装和运行多种 Linux 发行版的工具集。该项目使用 Rust 实现，支持多系统并行安装、自定义命名和精细化配置。可以轻松地在 Termux 中安装、管理和卸载不同的 Linux 发行版，包括 Ubuntu、Kali、Debian、CentOS 和 Fedora。Rust 版本提供了更好的性能、内存安全性和类型安全。

## 项目结构
不知道

## 核心功能
前面的不知道
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
不知道
### 编译和运行

```bash
# 克隆项目
git clone <repository-url>
cd TermuxForLinux

# 编译发布版本
cargo build --release

# 运行程序
不知道
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
不知道

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