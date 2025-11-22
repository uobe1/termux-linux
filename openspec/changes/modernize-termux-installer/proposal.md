## Why
当前 insOs 仅支持 5 个 Linux 发行版，且存在多个用户体验和代码质量问题。用户需要支持 UrlList.txt 中定义的全部发行版和架构（通过手动硬编码实现），同时需要修复 panic 错误、移除硬编码数据、改进进度条显示、添加颜色支持等，以提供更专业、更可靠的 Termux Linux 安装体验。

## What Changes
- **BREAKING**: 重构发行版系统，手动硬编码 UrlList.txt 中定义的所有发行版（adelie, deepin, debian, chimera, opensuse, artix, manjaro, archlinux, void, fedora, ubuntu, rockylinux, alpine, pardus）和多架构（ARM64, ARM32, x86_64, i686, riscv64）
- 修复 CLI 退出时的 index out of bounds panic 错误
- 移除硬编码的用户组、权限、时间等信息，改为动态获取真实数据
- 在配置文件中增加 {distro}-init 配置项，支持发行版安装时的自定义初始化命令
- 安装完成后询问用户是否立即启动系统
- 实现真实的下载和解压进度条，替代当前模拟实现
- 模块化重构代码，确保每个文件不超过 5KB
- 为 CLI 界面添加颜色支持（保留 --no-color 选项）
- 将"最小化安装"重命名为"最小安装"

## Impact
- 新增 capability: `distro` - 发行版定义和管理
- 受影响的 specs: `installer`, `cli`, `config`, `ui`
- 受影响的代码文件:
  - `src/distro/definitions.rs` - 发行版定义重构
  - `src/installer/` - 安装逻辑和进度条
  - `src/cli/` - CLI 界面和交互
  - `src/config/` - 配置解析和 {distro}-init 支持
  - `src/ui/` - 颜色支持和显示改进
  - `src/system/` - 动态数据获取
