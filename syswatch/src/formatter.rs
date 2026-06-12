use crate::{SystemSnapshot, CpuInfo, MemInfo, ProcessInfo};

pub fn format_response(snapshot: &SystemSnapshot, command: &str) -> String {
match command.trim() {
"cpu" => {
let mut s = String::new();
for c in &snapshot.cpus {
let bars = bar_of(c.usage);
s.push_str(&format!("{} {:>6.2}% {}\n", c.name, c.usage, bars));
}
s
}
"mem" => {
let used = snapshot.mem.used as f64 / 1024.0 / 1024.0;
let total = snapshot.mem.total as f64 / 1024.0 / 1024.0;
let pct = used / total * 100.0;
format!("Memory: {:.1} / {:.1} MB ({:.1}%)\n{}\n", used, total, pct, bar_of(pct as f32))
}
"ps" => {
let mut s = String::from("PID NAME CPU% MEM(KB)\n");
for p in &snapshot.processes {
s.push_str(&format!("{:>6} {:<20} {:>6.2} {:>10}\n", p.pid, p.name, p.cpu, p.mem));
}
s
}
"all" => format!("{}", snapshot),
"help" => String::from("Commands: cpu, mem, ps, all, help, quit\n"),
"quit" => String::from("quit\n"),
other => format!("Unknown command: {}\n", other),
}
}

fn bar_of(value: f32) -> String {
let max = 20;
let filled = ((value / 100.0) * max as f32).round().clamp(0.0, max as f32) as usize;
let mut s = String::new();
s.push('[');
s.push_str(&"#".repeat(filled));
s.push_str(&" ".repeat(max - filled));
s.push(']');
s
}
