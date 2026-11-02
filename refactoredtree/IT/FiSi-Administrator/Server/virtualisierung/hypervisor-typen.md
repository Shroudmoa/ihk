# Hypervisor-Typen

## In 30 Sekunden

- Ein **Hypervisor** (Virtual Machine Monitor) erzeugt und verwaltet VMs, teilt ihnen CPU, RAM, Storage und Netz zu und isoliert sie voneinander.
- **Typ 1 (Bare-Metal):** läuft **direkt auf der Hardware** → Rechenzentrum, Produktivbetrieb. ESXi, Hyper-V, KVM/Proxmox, Xen.
- **Typ 2 (Hosted):** läuft **als Anwendung auf einem Betriebssystem** → Test, Schulung, Entwicklerrechner. VirtualBox, VMware Workstation.
- Geprüft: identische Tabellenaufgabe (Erläuterung, Einsatzbeispiel, Produkt) 2022S und 2025S, Zweck des Hypervisors 2023W.

## Kernwissen

Ein **Hypervisor** (Virtual Machine Monitor, VMM) verwaltet virtuelle Maschinen (VMs) und teilt ihnen Hardware-Ressourcen wie CPU, RAM, Storage und Netzwerk zu. Er sorgt für **Isolation** zwischen den virtuellen Maschinen.

### Typ 1 – Bare-Metal-Hypervisor

Läuft **direkt auf der Hardware**, ohne ein allgemeines Host-Betriebssystem dazwischen.

- **Funktionsweise:** Der Hypervisor ist selbst das Betriebssystem für die Virtualisierung
- **Vorteile:** Geringer Overhead, hohe Performance, direkter Hardwarezugriff, bessere Ressourcennutzung, kleinerer Angriffsvektor
- **Nachteile:** Spezialisierte Installation, teilweise eingeschränkte Hardware-Kompatibilität, Verwaltung meist über separates Management-Tool
- **Einsatz:** Rechenzentren, Serverkonsolidierung, Produktivbetrieb, Clusterumgebungen
- **Beispiele:** VMware ESXi, Microsoft Hyper-V, Proxmox VE (KVM), Xen

### Typ 2 – Hosted-Hypervisor

Läuft **als Anwendung auf einem bestehenden Host-Betriebssystem**.

- **Funktionsweise:** Nutzt das Host-OS für Treiber und Hardwarezugriff
- **Vorteile:** Einfache Installation, läuft auf handelsüblicher Hardware, ideal zum Ausprobieren und Testen
- **Nachteile:** Höherer Overhead, abhängig vom Host-OS, weniger performant, größerer Angriffsvektor, weniger geeignet für Hochverfügbarkeit
- **Einsatz:** Entwicklung, Schulung, Tests, lokale Laborumgebungen
- **Beispiele:** Oracle VirtualBox, VMware Workstation, VMware Fusion

### Direkter Vergleich

| Kriterium | **Typ 1 (Bare-Metal)** | **Typ 2 (Hosted)** |
|---|---|---|
| **Grundlage** | Direkt auf der Hardware | Auf einem Host-Betriebssystem |
| **Overhead** | Gering | Höher (durch Host-OS) |
| **Performance** | Höher, näher an nativer Performance | Etwas geringer |
| **Sicherheit** | Geringerer Angriffsvektor | Höherer Angriffsvektor (Host-OS zusätzlich angreifbar) |
| **Einsatz** | **Produktivbetrieb** | **Entwicklung & Tests** |
| **Skalierbarkeit** | Sehr gut (z. B. für Cluster, Live-Migration) | Eingeschränkt |
| **Verwaltung** | Zentral über Management-Konsole | Über Hypervisor-Anwendung im Host-OS |

**Hinweis Hyper-V:** Auch wenn Hyper-V als Rolle in Windows installiert wird, ist es ein Typ-1-Hypervisor. Er schiebt sich unter das Windows, das danach selbst als privilegierte „Root-Partition“ läuft.

## Prüfungsfrage

**Frage:** Vervollständigen Sie für Hypervisor Typ 1 und Typ 2 je eine Erläuterung, ein Einsatzbeispiel und ein Produkt.

**Musterantwort:**

| | Typ 1 (Bare-Metal) | Typ 2 (Hosted) |
|---|---|---|
| Erläuterung | läuft direkt auf der Hardware ohne Host-Betriebssystem, dünne Schicht, geringer Overhead, hohe Leistung und Isolation | läuft als Anwendung auf einem installierten Betriebssystem und nutzt dessen Treiber, mehr Overhead |
| Einsatzbeispiel | Servervirtualisierung im Rechenzentrum, Cluster mit Live-Migration | Testumgebung oder Schulungs-VMs auf dem Arbeitsplatzrechner |
| Produkt | VMware ESXi, Microsoft Hyper-V, Proxmox VE | Oracle VirtualBox, VMware Workstation |

## Falle

- Typ 1 und Typ 2 vertauschen. **1 = direkt** auf der Hardware.
- Hyper-V als Typ 2 einordnen, weil es „unter Windows“ installiert wird.
- Docker als Hypervisor nennen. Das ist eine Container-Engine ohne eigene Gast-Kernel.
- Typ 2 für produktive Server empfehlen. Das Host-OS ist zusätzliche Angriffs- und Ausfallquelle.

## Merksatz

**„Typ 1 sitzt auf dem Blech, Typ 2 sitzt auf dem Betriebssystem.“**

## Verwandt

- [VM-Container-Server](vm-container-server.md) – Vergleich Bare Metal, VM, Container
- [Live-Migration-VMs](live-migration-vms.md) – vMotion/Live-Migration zwischen Hosts
- [Skalierbarkeit-Elastizität](skalierbarkeit-elastizitaet.md) – vertikal/horizontal
- [Cloud Computing](../Cloud/cloud-computing.md) – IaaS nutzt meist Typ-1-Hypervisoren
- [Virtualisierung](virtualisierung.md) – Grundlagen, Vor- und Nachteile
- [Container & Docker](container-docker.md)
- [Lizenzierung](../lizenzierung.md) – Core-Lizenzen und VMs
- [Lektion L13 Virtualisierung](../../../analyse-der-exams/L13_Virtualisierung.md)

