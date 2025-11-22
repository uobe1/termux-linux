# Project Context

## Purpose
insOs 是一个用于在 Android Termux 环境中安装和运行多种 Linux 发行版的工具集。该项目使用 Rust 实现，支持多系统并行安装、自定义命名和精细化配置。主要目标是为 Termux 用户提供简单易用的 Linux 环境安装和管理解决方案。

## Tech Stack
- **Rust 2021 Edition**: 主要编程语言，提供内存安全和高性能
- **TOML**: 配置文件格式，用于系统配置管理
- **标准库优先**: 仅使用 Rust 标准库和 toml 依赖，减少外部依赖
- **proot 技术**: 用户空间容器化技术，无需 root 权限运行 Linux
- **Termux 环境**: 目标运行环境，Android 上的 Linux 终端模拟器

## Project Conventions

### Code Style
- **命名约定**:
  - 模块名：小写蛇形命名 (snake_case)
  - 函数名：小写蛇形命名 (snake_case)
  - 结构体/枚举：大驼峰命名 (PascalCase)
  - 常量：大写蛇形命名 (SCREAMING_SNAKE_CASE)
- **模块化设计**: 每个功能领域独立模块，清晰分离职责
- **错误处理**: 使用 Result<T, Box<dyn std::error::Error>> 进行统一错误处理
- **代码格式**: 使用 rustfmt 进行代码格式化
- **文档注释**: 为公共 API 提供详细的文档注释

### Architecture Patterns
- **模块化架构**: 按功能领域划分模块 (cli, config, distro, installer, system, ui, utils, i18n)
- **单一职责原则**: 每个模块专注一个功能领域
- **低耦合高内聚**: 模块间依赖最小化，相关功能集中
- **依赖注入**: 通过参数传递依赖，提高可测试性
- **配置驱动**: 通过外部配置文件管理系统行为

### Testing Strategy
- **单元测试**: 使用 Rust 内置测试框架
- **临时文件测试**: 使用 tempfile 依赖进行文件系统测试
- **集成测试**: 测试模块间交互和完整工作流
- **错误路径测试**: 确保所有错误情况都有适当的处理
- **测试命令**: `cargo test` 运行所有测试

### Git Workflow
- **主分支**: main 分支用于稳定版本
- **提交消息**: 使用清晰、简洁的中文提交消息
- **OpenSpec 规范**: 使用 OpenSpec 管理开发任务和变更
- **顺序开发**: 按照任务列表顺序进行开发，不允许乱序
- **状态同步**: 完成任务后必须更新相应文件中的状态

## Domain Context
- **Linux 发行版管理**: 支持 Ubuntu、Kali、Debian、CentOS、Fedora 等主流发行版
- **ARM64 架构**: 主要针对 Android Termux 的 ARM64 环境
- **容器化技术**: 使用 proot 实现用户空间容器，无需 root 权限
- **镜像源管理**: 支持自定义镜像源，优化下载速度
- **系统元数据**: 每个系统实例维护独立的元数据信息
- **国际化支持**: 支持中英文界面切换

## Important Constraints
- **Termux 环境**: 必须在 Android Termux 环境中运行
- **ARM64 架构**: 主要支持 ARM64 架构的系统镜像
- **存储空间**: 需要足够的存储空间（建议至少 2GB 可用空间）
- **网络连接**: 需要稳定的网络连接下载系统镜像
- **proot 依赖**: 用户必须预先安装 proot 工具
- **权限限制**: 某些功能可能需要适当的 Termux 权限

## External Dependencies
- **proot**: 必需的外部工具，用于用户空间容器化
- **pkg**: Termux 包管理器，用于安装依赖
- **tar**: 系统自带工具，用于解压 rootfs 压缩包
- **wget/curl**: 系统自带工具，用于下载镜像文件
- **screenfetch**: 可选工具，用于显示系统信息
