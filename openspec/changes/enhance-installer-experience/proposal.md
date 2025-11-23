# Change: 增强安装器体验与UI现代化

## Why
当前安装器存在以下问题：
1. 默认安装过多软件包（tmux、git等），不符合最小化安装原则
2. 缺乏详细的日志和错误处理机制，使用2>&1重定向掩盖错误
3. 颜色使用不足，仅少数界面使用颜色提示
4. 颜色方案单一且视觉效果不佳，影响用户体验

## What Changes
- **删除默认软件包**: 移除所有发行版的default_packages列表，实现真正最小化安装
- **增强日志系统**: 添加结构化日志记录，删除所有2>&1重定向，提供详细的错误上下文
- **统一颜色使用**: 在所有UI组件中应用颜色主题，包括进度条、状态消息、错误提示等
- **现代化颜色方案**: 采用更美观的颜色方案，区分不同信息级别（成功、错误、警告、信息、进度）

## Impact
- **Affected specs**: installer, ui, config
- **Affected code**: 
  - 发行版定义文件（debian_core.rs, arch_core.rs等）
  - 安装器核心逻辑（installer/core.rs, installer/interactive.rs）
  - 日志和命令执行工具（utils/cmd.rs, utils/fs_core.rs, utils/net_core.rs）
  - UI组件（ui/colors.rs, ui/display.rs, ui/progress*.rs）
  - 配置管理（config/mod.rs）
- **User impact**: 安装过程更干净、错误信息更明确、视觉体验更现代
- **Breaking changes**: 默认不再安装常用工具包，用户需手动安装所需软件