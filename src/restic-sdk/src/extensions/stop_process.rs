use log::warn;
use std::io;
use std::time::Duration;
use tokio::process::Child;
use tokio::time::timeout;

/// https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
pub const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
pub const CREATE_NEW_CONSOLE: u32 = 0x00000010;

const ATTACH_PARENT_PROCESS: u32 = 0xFFFFFFFF;
const GRACEFUL_STOP_TIMEOUT: Duration = Duration::from_secs(30);

/// Starts the process stop operation (using `CTRL_BREAK_EVENT`).
/// - If this process has a console attached, this function will first detach from the console,
///   attach to the child process's console, and re-attach to the parent console.
/// - If the process does not have a console attached, this function will simply attach
///   to the child process's console and then free the console.
/// - If the process does not exist within [GRACEFUL_STOP_TIMEOUT], the process will be killed.
/// - It is expected to wait for the process after calling this function.
///
/// Ensure the process is started with '[CREATE_NEW_PROCESS_GROUP] | [CREATE_NEW_CONSOLE]'.
#[cfg(windows)]
pub async fn start_stop_process(child: &mut Child) -> Result<(), io::Error> {
    // Stop the process by sending CTRL_BREAK_EVENT.
    // This requires a console to be attached to the process (and be spawned in a different process group).
    // As we might be running in a service without a console, we need to switch to the console of the PID.
    // Then we can send the signal.
    // After switching, we detach from the console to avoid catching the signal ourselves.
    // Finally, we re-attach to the parent console if we had one before.
    if let Some(pid) = child.id() {
        unsafe {
            use windows_sys::Win32::Foundation::GetLastError;
            use windows_sys::Win32::Foundation::HWND;
            use windows_sys::Win32::System::Console;
            use windows_sys::core::BOOL;
            const FAILURE: BOOL = 0;

            let has_console = !HWND::is_null(Console::GetConsoleWindow());

            Console::FreeConsole();

            if Console::AttachConsole(pid) != FAILURE {
                if Console::GenerateConsoleCtrlEvent(Console::CTRL_BREAK_EVENT, pid) == FAILURE {
                    warn!(
                        "Failed to send signal to {pid}. Error Code: {}",
                        GetLastError()
                    );
                }
                Console::FreeConsole();
            } else {
                warn!("Failed to attach to {pid}. Error Code: {}", GetLastError());
            }

            if has_console && Console::AttachConsole(ATTACH_PARENT_PROCESS) == FAILURE {
                warn!(
                    "Failed to re-attach to parent console. Error Code: {}",
                    GetLastError()
                );
            }
        }
    }

    match timeout(GRACEFUL_STOP_TIMEOUT, child.wait()).await {
        Ok(_) => {}
        Err(_) => {
            warn!("Timeout of {GRACEFUL_STOP_TIMEOUT:?} reached, force killing process.");
            child.start_kill()?
        }
    };
    Ok(())
}
