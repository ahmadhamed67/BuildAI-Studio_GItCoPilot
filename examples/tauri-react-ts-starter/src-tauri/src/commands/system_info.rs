use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemInfo {
    pub platform: String,
    pub arch: String,
    pub cpu_count: usize,
    pub total_memory: u64,
    pub free_memory: u64,
}

#[tauri::command]
pub fn get_system_info() -> Result<SystemInfo, String> {
    // These would typically use system information libraries
    // For demonstration, returning placeholder values
    Ok(SystemInfo {
        platform: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        cpu_count: num_cpus::get(),
        total_memory: 0,
        free_memory: 0,
    })
}

#[tauri::command]
pub fn get_app_version() -> Result<String, String> {
    Ok(env!("CARGO_PKG_VERSION").to_string())
}
