# Project Context

## Purpose
TermuxForLinux (termux-linux-install) 是一个在 Android Termux 环境中安装和运行多种 Linux 发行版的 Rust 工具集。项目目标：
- 支持多系统并行安装（同一发行版可安装多个实例）
- 提供简洁友好的命令行和交互式界面
- 无需 root 权限，使用 proot 技术实现容器化
- 针对中国网络环境优化（支持国内镜像源）
- 支持 Ubuntu, Kali, Debian, CentOS, Fedora 等主流发行版

## Tech Stack
- **Rust 2021 Edition** - 核心编程语言
- **标准库优先** - 仅使用 Rust 标准库，无外部依赖
- **proot** - 用户空间容器化技术（外部工具）
- **架构**: 支持 ARM64 架构（Android Termux 环境）
- **构建优化**: LTO、单编译单元、panic=abort、二进制瘦身

## Project Conventions

### Code Style
- 遵循 Rust 官方代码风格指南（rustfmt）
- 命名约定：
  - 模块名：小写蛇形命名（snake_case）
  - 函数名：小写蛇形命名（snake_case）
  - 结构体/枚举：大驼峰命名（PascalCase）
  - 常量：大写蛇形命名（SCREAMING_SNAKE_CASE）
- 错误处理：统一使用 `Result<T, E>` 类型
- 注释：使用中文注释说明复杂逻辑
- 避免编译器警告：所有代码应无 warnings

### Architecture Patterns
- **模块化架构**：按功能划分独立模块
  - `main.rs` - 程序入口（仅负责初始化和启动）
  - `cli.rs` - 命令行参数处理、交互式菜单
  - `distro.rs` - Linux 发行版定义和元数据
  - `utils.rs` - 通用工具函数（文件操作、命令执行）
  - `installer.rs` - 系统安装流程（下载、解压、配置）
  - `system.rs` - 系统管理和卸载逻辑
  - `config.rs` - 配置文件管理
  - `ui.rs` - 用户界面显示和格式化输出

- **设计原则**：
  - 单一职责：每个模块专注一个功能领域
  - 低耦合：模块间依赖最小化，通过公共接口通信
  - 高内聚：相关功能集中在同一模块
  - 可扩展性：新功能通过添加或扩展模块实现

- **数据管理**：
  - 元数据存储：每个系统实例使用 `meta.txt` 存储元信息
  - 配置驱动：通过 `$HOME/Ostermux/config` 统一管理镜像源
  - 实例命名：格式为 `{DISTRO}{NUMBER}`（如 debian1、ubuntu2）

### Testing Strategy
- 当前阶段：手动测试为主
- 测试环境：Android Termux on ARM64
- 重点测试场景：
  - 多系统并行安装和运行
  - 配置文件解析和镜像源切换
  - 小屏幕终端界面适配
  - 异常处理和错误恢复
- 未来计划：添加单元测试和集成测试

### Git Workflow
- 主分支：`main`
- 提交规范：
  - `feat:` 新功能
  - `fix:` 错误修复
  - `refactor:` 代码重构
  - `docs:` 文档更新
  - `test:` 测试相关
  - `chore:` 构建/工具链更新
- 提交语言：中文或英文均可

## Domain Context
- **Termux 环境**：Android 终端模拟器，无 root 权限
- **proot 技术**：用户空间 chroot 实现，关键参数：
  - `--link2symlink` - 将硬链接转换为符号链接
  - `--delay-directory-restore` - 延迟目录权限恢复
  - `--preserve-permissions` - 保留权限
  - `--exclude='dev'` - 排除设备文件
- **rootfs 结构**：标准 Linux 文件系统层次（FHS）
- **镜像源**：优先使用国内镜像（USTC、163、阿里云、清华等）
- **压缩格式**：tar.xz（高压缩比）

## Important Constraints
- **无 root 权限**：所有操作必须在用户空间完成
- **架构限制**：主要支持 ARM64，需考虑跨架构兼容性
- **存储限制**：Android 存储空间有限，需优化安装包大小
- **网络限制**：中国大陆网络环境，需提供国内镜像源
- **终端限制**：小屏幕适配（最小宽度支持 40 字符）
- **性能约束**：移动设备性能有限，需优化启动速度
- **权限问题**：解压时必须使用 `--no-same-owner` 避免权限错误

## External Dependencies
- **proot** - 必需的外部工具（用户需在 Termux 中安装）
- **tar** - 解压 rootfs 压缩包（Termux 自带）
- **wget/curl** - 下载镜像文件（Termux 自带）
- **screenfetch** - 可选的系统信息显示工具
- **镜像源**：
  - Ubuntu: mirrors.ustc.edu.cn
  - Debian: mirrors.163.com
  - Kali: http.kali.org
  - CentOS: mirrors.aliyun.com
  - Fedora: mirrors.tuna.tsinghua.edu.cn
