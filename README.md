# Cc_syswatch_rust
Moniteur système en réseau développé en Rust (L4 Génie Logiciel). Serveur TCP interactif et multi-threadé qui collecte les métriques réelles de la machine (CPU, RAM, top 5 processus) via la crate sysinfo, gère le partage d'état sécurisé (Arc/Mutex) et intègre une journalisation des commandes.
