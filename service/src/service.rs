use windows_service::service::{
    ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus, ServiceType,
};
use windows_service::service_control_handler::ServiceStatusHandle;

pub trait ServiceStatusHandlerExtension {
    fn set_status_running(&self, accepted: ServiceControlAccept) -> windows_service::Result<()>;
    fn set_status_stop_pending(&self) -> windows_service::Result<()>;
    fn set_status_stopped(&self, exit_code: u32) -> windows_service::Result<()>;
}

impl ServiceStatusHandlerExtension for ServiceStatusHandle {
    fn set_status_running(&self, accepted: ServiceControlAccept) -> windows_service::Result<()> {
        self.set_service_status(ServiceStatus {
            current_state: ServiceState::Running,
            controls_accepted: accepted,
            ..get_default_service_status()
        })
    }

    fn set_status_stop_pending(&self) -> windows_service::Result<()> {
        self.set_service_status(ServiceStatus {
            current_state: ServiceState::StopPending,
            ..get_default_service_status()
        })
    }

    fn set_status_stopped(&self, exit_code: u32) -> windows_service::Result<()> {
        self.set_service_status(ServiceStatus {
            current_state: ServiceState::Stopped,
            exit_code: ServiceExitCode::Win32(exit_code),
            ..get_default_service_status()
        })
    }
}

fn get_default_service_status() -> ServiceStatus {
    ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Default::default(),
        process_id: None,
    }
}
