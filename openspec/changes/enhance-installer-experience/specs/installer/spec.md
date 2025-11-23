## MODIFIED Requirements

### Requirement: 发行版安装流程
系统SHALL提供最小化安装，不预装任何额外软件包。

#### Scenario: 安装Debian系统
- **WHEN** 用户选择安装Debian系统
- **THEN** 系统仅安装基础rootfs，不安装build-essential、git、vim等额外包
- **AND** 安装日志记录每个步骤的详细信息

#### Scenario: 安装Arch Linux系统
- **WHEN** 用户选择安装Arch Linux系统
- **THEN** 系统仅安装基础rootfs，不安装vim、curl、wget等额外包
- **AND** 所有命令执行错误都包含详细上下文信息

### Requirement: 安装器日志记录
安装器SHALL记录所有操作的详细日志，包括命令执行、文件操作和网络请求。

#### Scenario: 下载rootfs镜像
- **WHEN** 系统开始下载镜像文件
- **THEN** 日志记录下载URL、目标路径和进度信息
- **AND** 任何网络错误都包含完整的错误详情和HTTP状态（如适用）

#### Scenario: 解压rootfs归档
- **WHEN** 系统解压tar.xz文件
- **THEN** 日志记录每个解压的文件名和进度
- **AND** 解压错误包含归档文件路径和具体错误原因

### Requirement: 命令执行错误处理
所有外部命令执行SHALL分离stdout和stderr，提供详细的错误上下文。

#### Scenario: 命令执行失败
- **WHEN** 执行外部命令返回非零退出码
- **THEN** 错误信息包含命令字符串、stdout内容、stderr内容和退出码
- **AND** 不使用2>&1重定向掩盖错误输出

#### Scenario: tar命令解压失败
- **WHEN** tar命令解压归档文件失败
- **THEN** 错误信息包含归档文件路径、tar命令输出和错误原因
- **AND** 日志记录完整的错误上下文