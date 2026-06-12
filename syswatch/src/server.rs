use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{BufReader, BufRead, Write};
use crate::SystemSnapshot;
use crate::formatter::format_response;
use crate::collector::collect_snapshot;
use chrono::Local;
use std::fs::OpenOptions;
use std::time::Duration;

pub fn start_server(shared_snap: Arc<Mutex<SystemSnapshot>>) -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:7878")?;
    println!("Listening on 7878");
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                let snap = Arc::clone(&shared_snap);
                thread::spawn(move || {
                    if let Err(e) = handle_client(s, snap) {
                        eprintln!("Client error: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("Listener error: {}", e),
        }
    }
    Ok(())
}

fn handle_client(stream: TcpStream, snap: Arc<Mutex<SystemSnapshot>>) -> std::io::Result<()> {
    let peer = stream.peer_addr().map(|a| a.to_string()).unwrap_or_default();
    log_event(&format!("CONNECT {}", peer))?;

    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = stream;
    writeln!(writer, "Welcome to SysWatch. Type 'help'")?;

    loop {
        let mut line = String::new();
        let bytes = reader.read_line(&mut line)?;
        if bytes == 0 { break; } // connexion fermee par le client

        let cmd = line.trim();
        log_event(&format!("{} CMD {}", peer, cmd))?;

        if cmd == "quit" {
            writeln!(writer, "Goodbye")?;
            break;
        }

        // Clone du snapshot pour relacher le verrou immediatement
        // (on ne garde JAMAIS le lock pendant l'ecriture reseau).
        // unwrap_or_else gere le cas d'un Mutex empoisonne (panic dans un autre thread).
        let snapshot = snap
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone();

        let resp = format_response(&snapshot, cmd);
        writer.write_all(resp.as_bytes())?;
        writer.flush()?;
    }

    log_event(&format!("DISCONNECT {}", peer))?;
    Ok(())
}

fn log_event(s: &str) -> std::io::Result<()> {
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open("syswatch.log")?;
    let ts = Local::now().to_rfc3339();
    writeln!(f, "{} {}", ts, s)?;
    Ok(())
}

/// Thread de rafraichissement : toutes les 5 s, collecte un nouveau
/// snapshot et remplace le contenu du Arc<Mutex<...>>.
pub fn start_refresher(shared_snap: Arc<Mutex<SystemSnapshot>>) {
    thread::spawn(move || loop {
        match collect_snapshot() {
            Ok(new) => {
                let mut guard = shared_snap
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner());
                *guard = new;
            }
            Err(e) => eprintln!("Refresh error: {}", e),
        }
        thread::sleep(Duration::from_secs(5));
    });
}
