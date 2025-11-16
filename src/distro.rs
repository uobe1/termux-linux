use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

use crate::utils::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DistroType {
    Ubuntu,
    Kali,
    Debian,
    CentOS,
    Fedora,
}

impl fmt::Display for DistroType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DistroType::Ubuntu => write!(f, "Ubuntu"),
            DistroType::Kali => write!(f, "Kali"),
            DistroType::Debian => write!(f, "Debian"),
            DistroType::CentOS => write!(f, "CentOS"),
            DistroType::Fedora => write!(f, "Fedora"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SystemMeta {
    pub name: String,
    pub os_type: String,
    pub created_at: String,
    pub user_group: String,
    pub permissions: String,
    pub mirror_url: Option<String>,
}

impl SystemMeta {
    pub fn new(name: String, os_type: String) -> Self {
        // 使用标准库获取当前时间
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            name,
            os_type,
            created_at: format!("2025-11-14T{:02}:{:02}:{:02}Z", 
                (now / 3600) % 24, 
                (now / 60) % 60, 
                now % 60),
            user_group: "root".to_string(),
            permissions: "755".to_string(),
            mirror_url: None,
        }
    }
    
    pub fn to_string(&self) -> String {
        let mut result = format!("name = {}\n", self.name);
        result.push_str(&format!("os_type = {}\n", self.os_type));
        result.push_str(&format!("created_at = {}\n", self.created_at));
        result.push_str(&format!("user_group = {}\n", self.user_group));
        result.push_str(&format!("permissions = {}\n", self.permissions));
        if let Some(mirror) = &self.mirror_url {
            result.push_str(&format!("mirror_url = {}\n", mirror));
        }
        result
    }
    
    pub fn from_string(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut map = HashMap::new();
        
        for line in content.lines() {
            if let Some((key, value)) = line.split_once('=') {
                map.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
        
        Ok(SystemMeta {
            name: map.get("name").unwrap_or(&"".to_string()).clone(),
            os_type: map.get("os_type").unwrap_or(&"".to_string()).clone(),
            created_at: map.get("created_at").unwrap_or(&"".to_string()).clone(),
            user_group: map.get("user_group").unwrap_or(&"root".to_string()).clone(),
            permissions: map.get("permissions").unwrap_or(&"755".to_string()).clone(),
            mirror_url: map.get("mirror_url").cloned(),
        })
    }
}

pub struct LinuxDistro {
    distro_type: DistroType,
    instance_id: Option<String>,
    custom_name: Option<String>,
}


impl LinuxDistro {
    pub fn new(distro_type: DistroType) -> Self {
        Self { 
            distro_type,
            instance_id: None,
            custom_name: None,
        }
    }
    
    pub fn with_name(distro_type: DistroType, custom_name: String) -> Self {
        Self { 
            distro_type,
            instance_id: None,
            custom_name: Some(custom_name),
        }
    }
    
    #[allow(dead_code)]
    pub fn new_with_instance(distro_type: DistroType, instance_id: String, custom_name: Option<String>) -> Self {
        Self { 
            distro_type,
            instance_id: Some(instance_id),
            custom_name,
        }
    }
    
    pub fn install(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = self.get_config();
        
        // 生成实例ID和名称
        let instance_id = self.generate_instance_id()?;
        let system_name = self.custom_name.clone().unwrap_or_else(|| instance_id.clone());
        
        println!("生成的实例ID: {}", instance_id);
        println!("系统名称: {}", system_name);
        
        // 检查是否有自定义下载链接
        let config_manager = crate::config::ConfigManager::new()?;
        let custom_link = config_manager.get_download_link_for_distro(&config.os_name.to_lowercase())?;
        
        let use_custom_link = custom_link.is_some();
        
        if let Some(link) = &custom_link {
            println!("\n正在从自定义链接下载 Rootfs 镜像文件，请耐心等待");
            self.download_from_custom_link(link, &config.tarball)?;
        } else {
            println!("\n正在从码云下载 Rootfs 镜像文件，请耐心等待");
            // 检查镜像目录是否已存在
            if PathBuf::from(&config.image_dir).exists() {
                println!("镜像目录已存在，跳过下载");
            } else {
                run_command(&format!("git clone \"{}\"", config.repo_url))?;
            }
        }
        
        println!("\n正在解压镜像 请耐心等待");
        let install_dir = get_home_dir()?.join("Ostermux").join(&instance_id);
        let filesys_dir = install_dir.join("filesys");
        fs::create_dir_all(&filesys_dir)?;
        
        println!("安装目录: {:?}", install_dir);
        println!("文件系统目录: {:?}", filesys_dir);
        
        // 确保解压目录存在
        if !filesys_dir.exists() {
            fs::create_dir_all(&filesys_dir)?;
        }
        
        // 检查镜像文件是否存在
        let tarball_path = if use_custom_link {
            PathBuf::from(&config.tarball)
        } else {
            PathBuf::from(&config.image_dir).join(&config.tarball)
        };
        
        if !tarball_path.exists() {
            return Err(format!("镜像文件不存在: {:?}", tarball_path).into());
        }
        
        let extract_cmd = if config.exclude_dev {
            format!("proot --link2symlink tar -xJf {} -C {} --exclude=dev ||:",
                    tarball_path.display(), filesys_dir.display())
        } else {
            format!("proot --link2symlink tar -xJf {} -C {} ||:",
                    tarball_path.display(), filesys_dir.display())
        };

        println!("执行解压命令: {}", extract_cmd);

        run_command(&extract_cmd)?;
        
        // 检查解压是否成功
        if !filesys_dir.join("bin").exists() {
            return Err("解压失败：未找到 bin 目录".into());
        }
        
        println!("\n解压完成 正在删除已下载的镜像");
        if use_custom_link {
            run_command(&format!("rm -rf {}", config.tarball))?;
        } else {
            run_command(&format!("rm -rf {}", config.image_dir))?;
        }
        
        println!("\n正在创建元数据");
        let mut meta = SystemMeta::new(system_name.clone(), config.os_name.to_lowercase());
        let mirror_url = config_manager.get_mirror_for_distro(&config.os_name.to_lowercase())?;
        meta.mirror_url = Some(mirror_url.clone());
        let meta_content = meta.to_string();
        fs::write(install_dir.join("meta.txt"), meta_content)?;
        
        println!("\n正在优化系统设置");
        self.setup_system_new(&config, &install_dir, &filesys_dir)?;
        
        run_command(&format!("screenfetch -A \"{}\" -L", config.screenfetch_name))?;
        
        println!("\n   {} (ID: {}) 安装成功", system_name, instance_id);
        println!("\n    祝您使用愉快\n");
        
        Ok(())
    }
    
    fn generate_instance_id(&self) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(id) = &self.instance_id {
            println!("使用预设的实例ID: {}", id);
            return Ok(id.clone());
        }
        
        let installed_systems = get_installed_systems()?;
        let base_name = self.distro_type.to_string().to_lowercase();
        
        // 使用正则表达式找出同类型系统的最大数字后缀
        use std::collections::HashSet;
        let mut numbers = HashSet::new();
        
        for system in &installed_systems {
            if system.starts_with(&base_name) {
                if let Some(num_str) = system.strip_prefix(&base_name) {
                    if let Ok(num) = num_str.parse::<u32>() {
                        numbers.insert(num);
                    }
                }
            }
        }
        
        // 找到最大的数字，然后+1
        let mut max_num = 0;
        for num in &numbers {
            if *num > max_num {
                max_num = *num;
            }
        }
        
        let instance_id = format!("{}{}", base_name, max_num + 1);
        println!("生成的实例ID: {}", instance_id);
        
        Ok(instance_id)
    }
    
    #[allow(dead_code)]
    pub fn uninstall(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = self.get_config();
        
        println!("\n正在卸载 {} 请耐心等待", config.os_name);
        
        if config.needs_chmod {
            run_command(&format!("chmod 777 -R $HOME/Ostermux/{}", config.os_name))?;
        }
        
        run_command(&format!("rm -rf $HOME/Ostermux/{}", config.os_name))?;
        println!("\n卸载完成!");
        
        Ok(())
    }
    
    fn get_config(&self) -> DistroConfig {
        match self.distro_type {
            DistroType::Ubuntu => DistroConfig {
                os_name: "Ubuntu".to_string(),
                folder_name: "ubuntu-fs".to_string(),
                shname: "start-ubuntu.sh".to_string(),
                image_dir: "termux-ubuntu".to_string(),
                tarball: "ubuntu-rootfs-arm64.tar.xz".to_string(),
                repo_url: "https://gitee.com/sqlsec/termux-ubuntu.git".to_string(),
                screenfetch_name: "Ubuntu".to_string(),
                exclude_dev: true,
                needs_chmod: false,
                default_mirror: "https://mirrors.ustc.edu.cn/ubuntu/".to_string(),
            },
            DistroType::Kali => DistroConfig {
                os_name: "Kali".to_string(),
                folder_name: "kali-fs".to_string(),
                shname: "start-kali.sh".to_string(),
                image_dir: "termux-kali".to_string(),
                tarball: "kali-rootfs-arm64.tar.xz".to_string(),
                repo_url: "https://gitee.com/sqlsec/termux-kali".to_string(),
                screenfetch_name: "Kali Linux".to_string(),
                exclude_dev: false,
                needs_chmod: false,
                default_mirror: "http://http.kali.org/kali/".to_string(),
            },
            DistroType::Debian => DistroConfig {
                os_name: "Debian".to_string(),
                folder_name: "debian-fs".to_string(),
                shname: "start-debian.sh".to_string(),
                image_dir: "termux-debian".to_string(),
                tarball: "debian-rootfs-arm64.tar.xz".to_string(),
                repo_url: "https://gitee.com/sqlsec/termux-debian".to_string(),
                screenfetch_name: "Debian".to_string(),
                exclude_dev: false,
                needs_chmod: false,
                default_mirror: "https://mirrors.163.com/debian/".to_string(),
            },
            DistroType::CentOS => DistroConfig {
                os_name: "CentOS".to_string(),
                folder_name: "centos-fs".to_string(),
                shname: "start-centos.sh".to_string(),
                image_dir: "termux-centos".to_string(),
                tarball: "centos-rootfs-arm64.tar.xz".to_string(),
                repo_url: "https://gitee.com/sqlsec/termux-centos".to_string(),
                screenfetch_name: "CentOS".to_string(),
                exclude_dev: true,
                needs_chmod: true,
                default_mirror: "https://mirrors.aliyun.com/centos/".to_string(),
            },
            DistroType::Fedora => DistroConfig {
                os_name: "Fedora".to_string(),
                folder_name: "fedora-fs".to_string(),
                shname: "start-fedora.sh".to_string(),
                image_dir: "termux-fedora".to_string(),
                tarball: "fedora-rootfs-arm64.tar.xz".to_string(),
                repo_url: "https://gitee.com/sqlsec/termux-fedora".to_string(),
                screenfetch_name: "Fedora".to_string(),
                exclude_dev: false,
                needs_chmod: true,
                default_mirror: "https://mirrors.tuna.tsinghua.edu.cn/fedora/".to_string(),
            },
        }
    }
    
    #[allow(dead_code)]
    fn setup_system(&self, config: &DistroConfig) -> Result<(), Box<dyn std::error::Error>> {
        let home = get_home_dir()?;
        let install_dir = home.join("Ostermux").join(&config.os_name);
        let fs_dir = install_dir.join(&config.folder_name);
        
        // 创建绑定目录
        fs::create_dir_all(install_dir.join("binds"))?;
        
        // 复制启动脚本
        let source_script = PathBuf::from(config.os_name.to_lowercase()).join(&config.shname);
        let dest_script = install_dir.join(&config.shname);
        fs::copy(&source_script, &dest_script)?;
        
        // 修复脚本权限
        run_command(&format!("termux-fix-shebang {}", dest_script.display()))?;
        run_command(&format!("chmod +x {}", dest_script.display()))?;
        
        // 特殊设置
        match self.distro_type {
            DistroType::Ubuntu | DistroType::Kali | DistroType::Debian => {
                // 替换 sources.list
                let sources_list = fs_dir.join("etc/apt/sources.list");
                if sources_list.exists() {
                    fs::remove_file(&sources_list)?;
                }
                let source_sources = PathBuf::from(config.os_name.to_lowercase()).join("sources.list");
                fs::copy(source_sources, sources_list)?;
            }
            DistroType::CentOS => {
                // CentOS 特殊设置
                fs::create_dir_all(fs_dir.join("tmp"))?;
                fs::write(fs_dir.join("etc/hosts"), "127.0.0.1 localhost\n")?;
                fs::write(fs_dir.join("etc/resolv.conf"), "nameserver 8.8.8.8\nnameserver 8.8.4.4\n")?;
            }
            DistroType::Fedora => {
                // Fedora 特殊设置
                fs::write(fs_dir.join("etc/hosts"), "127.0.0.1 localhost\n")?;
                fs::write(fs_dir.join("etc/resolv.conf"), "nameserver 8.8.4.4\nnameserver 8.8.4.4\n")?;
                
                // 复制 repo 文件
                let repos_dir = fs_dir.join("etc/yum.repos.d");
                if repos_dir.exists() {
                    for entry in fs::read_dir(&repos_dir)? {
                        let entry = entry?;
                        fs::remove_file(entry.path())?;
                    }
                }
                
                for entry in fs::read_dir("fedora")? {
                    let entry = entry?;
                    let path = entry.path();
                    if let Some(filename) = path.file_name() {
                        if let Some(filename_str) = filename.to_str() {
                            if filename_str.ends_with(".repo") {
                                fs::copy(&path, repos_dir.join(filename))?;
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn setup_system_new(&self, config: &DistroConfig, install_dir: &PathBuf, filesys_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        // 创建启动脚本
        let start_script_content = self.generate_start_script(config, filesys_dir)?;
        fs::write(install_dir.join("start.sh"), start_script_content)?;
        run_command(&format!("termux-fix-shebang {}", install_dir.join("start.sh").display()))?;
        run_command(&format!("chmod +x {}", install_dir.join("start.sh").display()))?;
        
        // 特殊设置
        match self.distro_type {
            DistroType::Ubuntu | DistroType::Kali | DistroType::Debian => {
                // 替换 sources.list
                let sources_list = filesys_dir.join("etc/apt/sources.list");
                if sources_list.exists() {
                    fs::remove_file(&sources_list)?;
                }
                
                // 尝试从多个可能的路径复制 sources.list
                let source_paths = vec![
                    PathBuf::from(config.os_name.to_lowercase()).join("sources.list"),  // 当前目录
                    PathBuf::from("/root/TermuxForLinux").join(config.os_name.to_lowercase()).join("sources.list"),  // 项目根目录
                ];
                
                let mut copied = false;
                for source_path in source_paths {
                    if source_path.exists() {
                        if let Ok(_) = fs::copy(&source_path, &sources_list) {
                            copied = true;
                            break;
                        }
                    }
                }
                
                if !copied {
                    // 如果无法复制源文件，至少创建一个空的 sources.list
                    fs::write(&sources_list, "")?;
                }
            }
            DistroType::CentOS => {
                // CentOS 特殊设置
                fs::create_dir_all(filesys_dir.join("tmp"))?;
                fs::write(filesys_dir.join("etc/hosts"), "127.0.0.1 localhost\n")?;
                fs::write(filesys_dir.join("etc/resolv.conf"), "nameserver 8.8.8.8\nnameserver 8.8.4.4\n")?;
            }
            DistroType::Fedora => {
                // Fedora 特殊设置
                fs::write(filesys_dir.join("etc/hosts"), "127.0.0.1 localhost\n")?;
                fs::write(filesys_dir.join("etc/resolv.conf"), "nameserver 8.8.4.4\nnameserver 8.8.4.4\n")?;
                
                // 复制 repo 文件
                let repos_dir = filesys_dir.join("etc/yum.repos.d");
                if repos_dir.exists() {
                    for entry in fs::read_dir(&repos_dir)? {
                        let entry = entry?;
                        fs::remove_file(entry.path())?;
                    }
                }
                
                // 尝试从多个可能的路径复制 repo 文件
                let fedora_dirs = vec![
                    PathBuf::from("fedora"),  // 当前目录
                    PathBuf::from("/root/TermuxForLinux/fedora"),  // 项目根目录
                ];
                
                for fedora_dir in fedora_dirs {
                    if fedora_dir.exists() {
                        if let Ok(entries) = fs::read_dir(&fedora_dir) {
                            for entry in entries {
                                if let Ok(entry) = entry {
                                    let path = entry.path();
                                    if let Some(filename) = path.file_name() {
                                        if let Some(filename_str) = filename.to_str() {
                                            if filename_str.ends_with(".repo") {
                                                let _ = fs::copy(&path, repos_dir.join(filename));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn generate_start_script(&self, _config: &DistroConfig, _filesys_dir: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
        let script = r#"#!/data/data/com.termux/files/usr/bin/bash
cd $(dirname $0)
## unset LD_PRELOAD in case termux-exec is installed
unset LD_PRELOAD
command="proot"
command+=" --link2symlink"
command+=" -0"
command+=" -r filesys"
if [ -n "$(ls -A binds)" ]; then
    for f in binds/* ;do
      . $f
    done
fi
command+=" -b /dev"
command+=" -b /proc"
command+=" -b filesys/root:/dev/shm"
## uncomment the following line to have access to the home directory of termux
#command+=" -b /data/data/com.termux/files/home:/root"
## uncomment the following line to mount /sdcard directly to /
#command+=" -b /sdcard"
command+=" -w /root"
command+=" /usr/bin/env -i"
command+=" HOME=/root"
command+=" PATH=/usr/local/sbin:/usr/local/bin:/bin:/usr/bin:/sbin:/usr/sbin:/usr/games:/usr/local/games"
command+=" TERM=$TERM"
command+=" LANG=C.UTF-8"
command+=" /bin/bash --login"
com="$@"
if [ -z "$1" ];then
    exec $command
else
    $command -c "$com"
fi
"#;
        Ok(script.to_string())
    }
}

impl LinuxDistro {
    fn download_from_custom_link(&self, link: &str, tarball: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("正在下载: {}", link);
        run_command(&format!("wget -O {} {}", tarball, link))?;
        
        // 检查下载是否成功
        if !PathBuf::from(tarball).exists() {
            return Err("下载失败：文件不存在".into());
        }
        
        Ok(())
    }
}

#[allow(dead_code)]
struct DistroConfig {
    os_name: String,
    folder_name: String,
    shname: String,
    image_dir: String,
    tarball: String,
    repo_url: String,
    screenfetch_name: String,
    exclude_dev: bool,
    needs_chmod: bool,
    default_mirror: String,
}