use crate::detect;
use crate::InitSystem;
use crate::Service;
use tokio::process::Command;

// ======== InitSystem Service utils ========
// mainly for run systemd/openrc/SysV commands
async fn run(cmd: &str, args: &[&str]) -> i32 {
    // 继承父进程 stdio，以便直接看到被封装命令的输出
    let status = Command::new(cmd).args(args).status().await;

    match status {
        Ok(st) if st.success() => 0,
        Ok(st) => {
            eprintln!(
                "Command failed: {} {} (exit: {})",
                cmd,
                args.join(" "),
                st.code().unwrap_or(-1)
            );
            st.code().unwrap_or(1)
        }
        Err(err) => {
            eprintln!("Failed to execute '{} {}': {err}", cmd, args.join(" "));
            1
        }
    }
}

// systemd
async fn systemd(service: &str, act: Service) -> i32 {
    match act {
        Service::Start => run("systemctl", &["start", service]).await,
        Service::Stop => run("systemctl", &["stop", service]).await,
        Service::Enable => run("systemctl", &["enable", service]).await,
        Service::Disable => run("systemctl", &["disable", service]).await,
        Service::Restart => run("systemctl", &["restart", service]).await,
    }
}

// OpenRC
async fn openrc(service: &str, act: Service) -> i32 {
    match act {
        Service::Start => run("rc-service", &["-q", service, "start"]).await,
        Service::Stop => run("rc-service", &["-q", service, "stop"]).await,
        Service::Restart => run("rc-service", &["-q", service, "restart"]).await,
        Service::Enable => run("rc-update", &["add", service, "default"]).await,
        Service::Disable => run("rc-update", &["del", service, "default"]).await,
    }
}

// runit
async fn runit(service: &str, act: Service) -> i32 {
    match act {
        Service::Start => run("sv", &["start", service]).await,
        Service::Stop => run("sv", &["stop", service]).await,
        Service::Restart => run("sv", &["restart", service]).await,
        Service::Enable => {
            // ln -s /etc/sv/<service> /etc/service/
            let target = format!("/etc/sv/{service}");
            let link = format!("/etc/service/{service}");
            run("ln", &["-s", &target, &link]).await
        }
        Service::Disable => {
            // rm /etc/service/<service>
            let link = format!("/etc/service/{service}");
            run("rm", &["-f", &link]).await
        }
    }
}

// SysV（Deb 系倾向用 update-rc.d；若不可用则回退到 chkconfig）
async fn sysv(service: &str, act: Service) -> i32 {
    match act {
        Service::Start => run("service", &["--full-restart", service, "start"]).await, // 尽量兼容
        Service::Stop => run("service", &[service, "stop"]).await,
        Service::Restart => run("service", &[service, "restart"]).await,
        Service::Enable => {
            // 先尝试 update-rc.d
            let rc = run("update-rc.d", &[service, "defaults"]).await;
            if rc == 0 {
                return rc;
            }
            // 尝试 chkconfig
            run("chkconfig", &[service, "on"]).await
        }
        Service::Disable => {
            // Debian/Ubuntu
            let rc = run("update-rc.d", &["-f", service, "remove"]).await;
            if rc == 0 {
                return rc;
            }
            // RHEL/CentOS
            run("chkconfig", &[service, "off"]).await
        }
    }
}

pub(crate) async fn handle_service(svc: Service) -> i32 {
    let which_init: InitSystem = match detect() {
        Ok(init) => init,
        Err(e) => {
            eprintln!("Failed to detect init system: {e}");
            return 0;
        }
    };

    // Your unit/service name
    let service_name = "synchron";

    match which_init {
        InitSystem::Systemd => systemd(service_name, svc).await,
        InitSystem::OpenRc => openrc(service_name, svc).await,
        InitSystem::SysV => sysv(service_name, svc).await,
        _ => {
            eprintln!(
                "Unsupported init system for `synchron service`: {:?}",
                which_init
            );
            2
        }
    }
}
