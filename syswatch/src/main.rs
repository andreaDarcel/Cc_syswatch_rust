mod collector;
mod formatter;
mod server;

use std::sync::{Arc, Mutex};

/// Types partagés au niveau racine du crate,
/// pour que `use crate::SystemSnapshot;` fonctionne dans les sous-modules.
#[derive(Debug, Clone)]
pub struct CpuInfo {
    pub name: String,
    pub usage_percent: f32,
}

#[derive(Debug, Clone)]
pub struct MemInfo {
    pub total: u64, // en kB
    pub used: u64,  // en kB
}

#[derive(Debug, Clone)]
pub struct ProcInfo {
    pub pid: u32,
    pub name: String,
    pub mem_kb: u64,
}

#[derive(Debug, Clone)]
pub struct SystemSnapshot {
    pub cpus: Vec<CpuInfo>,
    pub mem: MemInfo,
    pub processes: Vec<ProcInfo>,
}

impl Default for SystemSnapshot {
    fn default() -> Self {
        SystemSnapshot {
            cpus: vec![],
            mem: MemInfo { total: 0, used: 0 },
            processes: vec![],
        }
    }
}

fn main() {
    // Snapshot initial (Default::default() si la collecte échoue)
    let initial = collector::collect_snapshot().unwrap_or_default();
    let shared = Arc::new(Mutex::new(initial));

    // Étape 4a : thread de rafraîchissement toutes les 5 s
    server::start_refresher(Arc::clone(&shared));

    // Étape 4b : serveur TCP bloquant sur 0.0.0.0:7878
    if let Err(e) = server::start_server(shared) {
        eprintln!("Erreur fatale du serveur : {}", e);
        std::process::exit(1);
    }
}
