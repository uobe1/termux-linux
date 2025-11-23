## REMOVED Requirements

### Requirement: 发行版默认软件包配置
发行版定义中的default_packages字段及其所有相关逻辑SHALL被移除。

#### Scenario: Debian发行版配置
- **WHEN** 查看Debian发行版定义
- **THEN** 不包含build-essential、devscripts、curl、wget、git、vim、htop、tmux等默认包
- **AND** default_packages字段为空列表或已被移除

#### Scenario: Ubuntu发行版配置
- **WHEN** 查看Ubuntu发行版定义
- **THEN** 不包含build-essential、curl、wget、git、vim、htop等默认包
- **AND** default_packages字段为空列表或已被移除

#### Scenario: Arch Linux发行版配置
- **WHEN** 查看Arch Linux发行版定义
- **THEN** 不包含vim、curl、wget等默认包
- **AND** default_packages字段为空列表或已被移除

#### Scenario: 其他发行版配置
- **WHEN** 查看Deepin、Pardus、Manjaro、Artix、Void、Chimera、Adelie等发行版定义
- **THEN** 所有default_packages字段为空列表或已被移除
- **AND** 安装器不尝试安装任何预定义软件包

## MODIFIED Requirements

### Requirement: 配置管理器初始化命令
配置管理器SHALL支持获取发行版特定的初始化命令，但不再包含软件包安装命令。

#### Scenario: 获取Debian初始化命令
- **WHEN** 调用get_init_commands_for_distro("debian")
- **THEN** 返回的初始化命令不包含apt-get install
- **AND** 仅包含环境配置和系统设置命令（如果有）

#### Scenario: 获取Arch Linux初始化命令
- **WHEN** 调用get_init_commands_for_distro("archlinux")
- **THEN** 返回的初始化命令不包含pacman -S
- **AND** 仅包含环境配置和系统设置命令（如果有）

### Requirement: 安装模式配置
安装模式配置SHALL反映最小化安装理念，移除标准安装和自定义安装中的软件包安装选项。

#### Scenario: 最小安装模式
- **WHEN** 用户选择最小安装模式
- **THEN** 系统仅安装基础rootfs
- **AND** 不执行任何额外软件包安装
- **AND** 日志记录"最小化安装完成"

#### Scenario: 标准安装模式
- **WHEN** 用户选择标准安装模式
- **THEN** 系统仅安装基础rootfs
- **AND** 不执行任何额外软件包安装（与最小安装相同）
- **AND** 日志记录"标准安装完成"

#### Scenario: 自定义安装模式
- **WHEN** 用户选择自定义安装模式
- **THEN** 系统仅安装基础rootfs
- **AND** 提示用户进入系统后手动安装所需软件包
- **AND** 日志记录"自定义安装完成，请手动安装所需软件包"