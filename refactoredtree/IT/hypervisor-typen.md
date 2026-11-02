# Hypervisor-Typen

Ein **Hypervisor** (Virtual Machine Monitor, VMM) verwaltet virtuelle Maschinen (VMs) und teilt ihnen Hardware-Ressourcen wie CPU, RAM, Storage und Netzwerk zu. Er sorgt für **Isolation** zwischen den virtuellen Maschinen.

## Typ 1 – Bare-Metal-Hypervisor

Läuft **direkt auf der Hardware**, ohne ein allgemeines Host-Betriebssystem dazwischen.

- **Funktionsweise:** Der Hypervisor ist selbst das Betriebssystem für die Virtualisierung
- **Vorteile:** Geringer Overhead, hohe Performance, direkter Hardwarezugriff, bessere Ressourcennutzung, kleinerer Angriffsvektor
- **Nachteile:** Spezialisierte Installation, teilweise eingeschränkte Hardware-Kompatibilität, Verwaltung meist über separates Management-Tool
- **Einsatz:** Rechenzentren, Serverkonsolidierung, Produktivbetrieb, Clusterumgebungen
- **Beispiele:** VMware ESXi, Microsoft Hyper-V, Proxmox VE (KVM), Xen

## Typ 2 – Hosted-Hypervisor

Läuft **als Anwendung auf einem bestehenden Host-Betriebssystem**.

- **Funktionsweise:** Nutzt das Host-OS für Treiber und Hardwarezugriff
- **Vorteile:** Einfache Installation, läuft auf handelsüblicher Hardware, ideal zum Ausprobieren und Testen
- **Nachteile:** Höherer Overhead, abhängig vom Host-OS, weniger performant, größerer Angriffsvektor, weniger geeignet für Hochverfügbarkeit
- **Einsatz:** Entwicklung, Schulung, Tests, lokale Laborumgebungen
- **Beispiele:** Oracle VirtualBox, VMware Workstation, VMware Fusion

## Direkter Vergleich

| Kriterium | **Typ 1 (Bare-Metal)** | **Typ 2 (Hosted)** |
|---|---|---|
| **Grundlage** | Direkt auf der Hardware | Auf einem Host-Betriebssystem |
| **Overhead** | Gering | Höher (durch Host-OS) |
| **Performance** | Höher, näher an nativer Performance | Etwas geringer |
| **Sicherheit** | Geringerer Angriffsvektor | Höherer Angriffsvektor (Host-OS zusätzlich angreifbar) |
| **Einsatz** | **Produktivbetrieb** | **Entwicklung & Tests** |
| **Skalierbarkeit** | Sehr gut (z. B. für Cluster, Live-Migration) | Eingeschränkt |
| **Verwaltung** | Zentral über Management-Konsole | Über Hypervisor-Anwendung im Host-OS |

## IHK-Merksätze

- **Typ 1 = Bare-Metal** – Kein separates Host-OS zwischen Hypervisor und Hardware
- **Typ 2 = Hosted** – Benötigt ein vollständiges Host-Betriebssystem
- **Für Servervirtualisierung im Rechenzentrum:** Immer **Typ 1**
- **Für lokale VM-Tests:** **Typ 2** vollkommen ausreichend
- **Hypervisor = Vermittler** zwischen VM und physischer Hardware, garantiert **Ressourcenteilung & Isolation**

## Verwandt

- [VM-Container-Server](vm-container-server.md) – Vergleich Bare Metal, VM, Container
- [Live-Migration-VMs](live-migration-vms.md) – vMotion/Live-Migration zwischen Hosts
- [Skalierbarkeit-Elastizität](skalierbarkeit-elastizitaet.md) – vertikal/horizontal
- [Cloud Computing](cloud-computing.md) – IaaS nutzt meist Typ-1-Hypervisoren
- [Virtualisierung](virtualisierung.md) – Grundlagen der Virtualisierung

