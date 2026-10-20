# Hypervisor-Typen

# Hypervisoren

Ein Hypervisor verwaltet virtuelle Maschinen (VMs) und teilt ihnen Hardware-Ressourcen wie CPU, RAM, Storage und Netzwerk zu.

## Typ 1 – Bare-Metal-Hypervisor

Läuft direkt auf der Hardware, ohne allgemeines Host-Betriebssystem dazwischen. Optimiert für Serverbetrieb, Ressourcenverwaltung und Verfügbarkeit.

- **Vorteile:** Weniger zusätzliche Software-Schichten, geringer Overhead, zentrale VM-Verwaltung.
- **Nachteile:** Spezialisierte Installation und Hardware-Kompatibilität erforderlich.
- **Einsatz:** Rechenzentren, Servervirtualisierung, Dauerbetrieb.
- **Beispiele:** VMware ESXi, Microsoft Hyper-V, KVM-basierte Virtualisierung.

## Typ 2 – Hosted-Hypervisor

Läuft als Anwendung auf einem Host-Betriebssystem. Hardwarezugriffe erfolgen über den Hypervisor und teilweise über das Host-OS.

- **Vorteile:** Einfache Installation, ideal für lokale Test- und Entwicklungsumgebungen.
- **Nachteile:** Zusätzlicher Ressourcenverbrauch und potenziell mehr Overhead durch das Host-OS.
- **Einsatz:** Softwaretests, Schulungen, lokale Entwicklungsumgebungen.
- **Beispiele:** Oracle VirtualBox, VMware Workstation.

## Vergleich

| Kriterium | Typ 1 | Typ 2 |
|---|---|---|
| Grundlage | Direkte Hardware-Nutzung | Host-Betriebssystem |
| Overhead | Typischerweise geringer | Typischerweise höher |
| Einsatz | Produktivbetrieb | Entwicklung und Tests |
| Abhängigkeit | Hypervisor und Hardware | Zusätzlich Host-OS |

**IHK-Merksatz:** Typ 1 eignet sich besonders für produktive Serverumgebungen, Typ 2 für lokale Entwicklungs- und Testsysteme.

**Wichtig:** Typ 1 ist nicht automatisch stabiler oder sicherer. Entscheidend sind Konfiguration, Updates, Hardware und Betrieb.

## Siehe auch

- [VM-Container-Server](vm-container-server.md) – Vergleich Bare Metal, VM, Container
- [Live-Migration-VMs](live-migration-vms.md) – Migration zwischen Hosts

