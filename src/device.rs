use crate::domain::DeviceProfile;
#[cfg(target_os = "linux")]
use std::fs;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use std::process::Command;
#[cfg(target_os = "macos")]
use std::{ffi::CString, ptr};

#[cfg(target_os = "macos")]
unsafe extern "C" {
    fn sysctlbyname(
        name: *const std::ffi::c_char,
        old_value: *mut std::ffi::c_void,
        old_length: *mut usize,
        new_value: *mut std::ffi::c_void,
        new_length: usize,
    ) -> std::ffi::c_int;
}

pub fn detect(runtime_reachable: bool) -> DeviceProfile {
    DeviceProfile {
        os: std::env::consts::OS.into(),
        architecture: std::env::consts::ARCH.into(),
        logical_cpus: std::thread::available_parallelism()
            .map(usize::from)
            .unwrap_or(1),
        total_memory_bytes: total_memory_bytes(),
        runtime_reachable,
    }
}

fn total_memory_bytes() -> Option<u64> {
    #[cfg(target_os = "linux")]
    {
        let content = fs::read_to_string("/proc/meminfo").ok()?;
        let kb = content
            .lines()
            .find(|line| line.starts_with("MemTotal:"))?
            .split_whitespace()
            .nth(1)?
            .parse::<u64>()
            .ok()?;
        return Some(kb * 1024);
    }
    #[cfg(target_os = "macos")]
    {
        if let Some(value) = macos_total_memory_bytes() {
            return Some(value);
        }
        let output = Command::new("sysctl")
            .args(["-n", "hw.memsize"])
            .output()
            .ok()?;
        return String::from_utf8(output.stdout).ok()?.trim().parse().ok();
    }
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                "(Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory",
            ])
            .output()
            .ok()?;
        return String::from_utf8(output.stdout).ok()?.trim().parse().ok();
    }
    #[allow(unreachable_code)]
    None
}

#[cfg(target_os = "macos")]
fn macos_total_memory_bytes() -> Option<u64> {
    let name = CString::new("hw.memsize").ok()?;
    let mut value = 0_u64;
    let mut length = std::mem::size_of::<u64>();
    // SAFETY: `name` is a valid nul-terminated string. `value` and `length`
    // point to writable objects of the exact size requested by hw.memsize,
    // and no new value is supplied because this is a read-only query.
    let status = unsafe {
        sysctlbyname(
            name.as_ptr(),
            (&mut value as *mut u64).cast(),
            &mut length,
            ptr::null_mut(),
            0,
        )
    };
    (status == 0 && length == std::mem::size_of::<u64>()).then_some(value)
}
