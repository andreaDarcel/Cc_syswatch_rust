use sysinfo::{System, SystemExt, ProcessExt, CpuExt};
use crate::{SystemSnapshot, CpuInfo, MemInfo, ProcessInfo};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CollectorError {
    SysinfoError(String),
}

impl fmt::Display for CollectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CollectorError::SysinfoError(s) => write!(f, "Sysinfo error: {}", s),
        }
    }
}

impl Error for CollectorError {}

pub fn collect_snapshot() -> Result<SystemSnapshot, CollectorError> {
    let mut sys = System::new_all();
    sys.refresh_all();

    // CPU: average or first CPU
    let cpus = sys.cpus().iter().enumerate().map(|(i, c)| {
        CpuInfo { name: format!("cpu{}", i), usage: c.cpu_usage() }
    }).collect::<Vec<_>>();

    let total_mem = sys.total_memory() * 1024; // sysinfo returns KB
    let used_mem = (sys.total_memory() - sys.available_memory()) * 1024;

    let mem = MemInfo { total: total_mem, used: used_mem };

    let mut procs = sys.processes().values().map(|p| {
        ProcessInfo {
            pid: p.pid().as_u32() as i32,
            name: p.name().to_string(),
            cpu: p.cpu_usage(),
            mem: p.memory(),
        }
    }).collect::<Vec<_>>();

    procs.sort_by(|a, b| b.cpu.partial_cmp(&a.cpu).unwrap_or(std::cmp::Ordering::Equal));
    procs.truncate(5);

    Ok(SystemSnapshot { cpus, mem, processes: procs })
}
