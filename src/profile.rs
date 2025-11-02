use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub cpu_cores: u32,
    pub cpu_breakdown: Option<String>,
    pub memory_gb: u32,
    pub npu_count: Option<u32>,
    pub npu_breakdown: Option<String>,
    pub npu_tops: Option<u32>,
    pub gpu_count: Option<u32>,
    pub gpu_breakdown: Option<String>,
    pub gpu_tops: Option<u32>,
    pub network_mbps: u32,
    pub wireless: bool,
}
