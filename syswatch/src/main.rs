use std::fmt;
mod collector;
use collector::collect_snapshot;

#[derive(Debug, Clone)]
pub struct CpuInfo {
    pub name: String,
    pub usage: f32,
}

impl fmt::Display for CpuInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:<10} {:>6.2}%", self.name, self.usage)
    }
}

#[derive(Debug, Clone)]
pub struct MemInfo {
    pub total: u64,
    pub used: u64,
}

impl fmt::Display for MemInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let used_pct = (self.used as f64 / self.total as f64) * 100.0;
        write!(f, "RAM: {}/{} MB ({:.1}%)", self.used / 1024 / 1024, self.total / 1024 / 1024, used_pct)
    }
}

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: i32,
    pub name: String,
    pub cpu: f32,
    pub mem: u64,
}

impl fmt::Display for ProcessInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:>6} {:<20} {:>6.2}% {:>8} KB", self.pid, self.name, self.cpu, self.mem / 1024)
    }
}

#[derive(Debug, Clone)]
pub struct SystemSnapshot {
    pub cpus: Vec<CpuInfo>,
    pub mem: MemInfo,
    pub processes: Vec<ProcessInfo>,
}

impl fmt::Display for SystemSnapshot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "CPUs:")?;
        for c in &self.cpus { writeln!(f, "  {}", c)?; }
        writeln!(f, "")?;
        writeln!(f, "{}", self.mem)?;
        writeln!(f, "")?;
        writeln!(f, "Top processes:")?;
        for p in &self.processes { writeln!(f, "  {}", p)?; }
        Ok(())
    }
}

fn main() {
    let snapshot = SystemSnapshot {
        cpus: vec![
            CpuInfo { name: "cpu0".into(), usage: 12.3 },
            CpuInfo { name: "cpu1".into(), usage: 4.7 },
        ],
        mem: MemInfo { total: 8 * 1024 * 1024 * 1024u64, used: 3 * 1024 * 1024 * 1024u64 },
        processes: vec![
            ProcessInfo { pid: 1, name: "init".into(), cpu: 0.1, mem: 10240 },
            ProcessInfo { pid: 234, name: "bash".into(), cpu: 2.5, mem: 20480 },
        ],
    };

    match collect_snapshot() {
        Ok(snap) => println!("{}", snap),
        Err(e) => eprintln!("Collect error: {}", e),
    }
    println!("{}", snapshot);
}
