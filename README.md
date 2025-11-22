# insOs

insOs 是一个用于在 Android Termux 环境中安装和运行多种 Linux 发行版的工具集。使用 Rust 实现，支持多系统并行安装、自定义命名和精细化配置。

## 功能特性

- **多发行版支持**: Ubuntu、Kali、Debian、CentOS、Fedora
- **交互式安装**: 可视化菜单选择，支持自定义系统名称
- **系统管理**: 查询、卸载已安装系统
- **配置灵活**: 支持自定义镜像源、下载链接和登录 Shell
- **现代 UI**: ASCII 艺术 Logo、进度条、彩色输出
- **国际化**: 支持中英文界面

## 安装

### 前置要求

- Termux 环境
- 已安装 Git
- 网络连接

### 编译安装

```bash
# 克隆项目
git clone <repository-url>
cd insOs

# 编译发布版本
cargo build --release

# 安装到系统路径
cp target/release/insOs $PREFIX/bin/
```

## 使用

### 交互式模式

```bash
insOs
```

启动后，您将看到交互式菜单：
1. 安装系统
2. 卸载系统
3. 查询已安装系统
4. 退出

### 命令行模式

```bash
# 列出已安装系统
insOs --list

# 安装指定发行版
insOs --install ubuntu
insOs --install kali
insOs --install debian
insOs --install centos
insOs --install fedora

# 自定义系统名称
insOs --install ubuntu --name my-ubuntu

# 卸载系统
insOs --uninstall <system-id>

# 禁用颜色输出
insOs --no-color

# 指定语言
insOs --lang en
insOs --lang zh
```

### 启动已安装系统

安装完成后，使用显示的命令启动系统：

```bash
cd $HOME/termos/<system-id> && ./start.sh
```

## 配置

### 配置文件位置

`$HOME/termos/config`

### 配置示例

```bash
# 镜像源配置
ubuntu-mirror = https://mirrors.ustc.edu.cn/ubuntu/
debian-mirror = https://mirrors.163.com/debian/
kali-mirror = http://http.kali.org/kali/
centos-mirror = https://mirrors.aliyun.com/centos/
fedora-mirror = https://mirrors.tuna.tsinghua.edu.cn/fedora/

# 自定义下载链接（可选）
# ubuntu-link = https://custom-mirror.com/ubuntu-rootfs-arm64.tar.xz
# debian-link = https://custom-mirror.com/debian-rootfs-arm64.tar.xz
# kali-link = https://custom-mirror.com/kali-rootfs-arm64.tar.xz
# centos-link = https://custom-mirror.com/centos-rootfs-arm64.tar.xz
# fedora-link = https://custom-mirror.com/fedora-rootfs-arm64.tar.xz

# Shell 配置（可选）
# 自定义登录 shell 命令，默认为 /bin/bash --login
# shell = /bin/zsh --login
```

### 发行版默认软件包

每个发行版都有推荐的默认软件包：

- **Ubuntu**: build-essential, curl, wget, git, vim, htop
- **Kali**: kali-linux-headless, metasploit-framework, nmap, wireless-tools, aircrack-ng, john, hydra, sqlmap
- **Debian**: build-essential, devscripts, curl, wget, git, vim, htop, tmux
- **CentOS**: epel-release, vim, curl, wget, git, htop, net-tools, lsof
- **Fedora**: @development-tools, curl, wget, git, vim, htop, tmux, dnf-plugins-core

## 开发

### 项目结构

```
src/
├── main.rs              # 程序入口
├── cli/                 # 命令行接口
├── config/              # 配置管理
├── distro/              # 发行版管理
├── installer/           # 安装器
├── system/              # 系统管理
├── ui/                  # UI 组件
├── utils/               # 工具函数
└── i18n/                # 国际化
```

### 构建

```bash
# 调试构建
cargo build

# 发布构建（优化）
cargo build --release

# 运行测试
cargo test

# 运行代码检查
cargo clippy
```

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

## 许可证

[许可证信息]

## 贡献

欢迎提交 Pull Request 或 Issue！

## 更新日志

### v0.2.0
- 重构为多模块架构
- 添加国际化支持
- 增强 UI 体验
- 支持自定义 Shell 配置
- 优化默认系统设置
- 重命名为 insOs
