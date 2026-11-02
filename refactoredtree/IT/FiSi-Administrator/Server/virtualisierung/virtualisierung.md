# Virtualisierung

## In 30 Sekunden

- Virtualisierung **abstrahiert** Hardware: Ein **Hypervisor** teilt einen physischen Host in mehrere isolierte **virtuelle Maschinen** (VMs) mit eigenem Betriebssystem.
- **Typ 1** (Bare-Metal) läuft direkt auf der Hardware: ESXi, Hyper-V, KVM/Proxmox. **Typ 2** (Hosted) läuft auf einem Betriebssystem: VirtualBox, VMware Workstation → [Hypervisor-Typen](hypervisor-typen.md).
- Vorteile: **Konsolidierung** (Green IT), schnelle Bereitstellung, Snapshots, Live-Migration, Isolation, einfachere Wiederherstellung.
- Nachteile: Host als **Single Point of Failure**, Overhead, Komplexität, Lizenzkosten, Noisy Neighbor.
- Außer Servern lassen sich Desktops, Speicher, Netzwerke und Anwendungen virtualisieren.
- Geprüft: Vor-/Nachteile und Hypervisor-Typen 2022S, 2025S. Zweck des Hypervisors und Containerisierung 2023W.

## Kernwissen

### Zweck des Hypervisors (2023W)

| Aufgabe | Erklärung |
|---|---|
| **Abstraktion** | stellt jeder VM virtuelle Hardware bereit (vCPU, vRAM, virtuelle Platte, vNIC) |
| **Ressourcenverteilung** | teilt CPU, RAM, Storage und Netz zu, Limits und Reservierungen |
| **Isolation** | VMs sehen sich gegenseitig nicht. Ein Absturz oder eine Kompromittierung bleibt idealerweise auf eine VM beschränkt |
| **Verwaltung** | Anlegen, Klonen, Snapshots, Live-Migration, Hochverfügbarkeit |

### Arten der Virtualisierung

| Art | Was wird virtualisiert? | Beispiel |
|---|---|---|
| **Server** | ganze Server als VMs | 20 Server-VMs auf 3 Hosts |
| **Desktop (VDI)** | Arbeitsplätze im RZ | Windows-Desktops für Thin Clients → [VDI & Thin Client](vdi-thin-client.md) |
| **Betriebssystem/Container** | isolierte Prozesse auf gemeinsamem Kernel | Docker → [Container & Docker](container-docker.md) |
| **Storage** | viele physische Datenträger als gemeinsamer Pool | SAN-LUNs, vSAN, Storage Spaces |
| **Netzwerk** | logische Netze auf gemeinsamer Hardware | VLAN, virtueller Switch, SDN |
| **Anwendung** | Programm läuft gekapselt, ohne klassische Installation | App-V, Citrix Virtual Apps |

### Vor- und Nachteile (2025S: je drei)

| Vorteile | Nachteile |
|---|---|
| **Konsolidierung:** höhere Auslastung, weniger Hardware, Strom, Kühlung und Platz | **SPOF:** Fällt der Host aus, fallen alle VMs aus → Cluster nötig |
| **Flexibilität:** VMs in Minuten anlegen, klonen, per Vorlage ausrollen | **Overhead:** Hypervisor braucht Ressourcen, Engpässe bei zu dichter Belegung (Noisy Neighbor) |
| **Snapshots und Live-Migration:** Rollback vor Updates, Wartung ohne Ausfall | **Komplexität:** Know-how, Management, Monitoring, Backup der VMs |
| **Isolation:** Dienste getrennt, Fehler bleiben auf eine VM begrenzt | **Lizenzkosten:** Hypervisor, Management, Gast-OS nach Kernen |
| **Hardwareunabhängigkeit:** VM läuft auf anderem Host, einfacher Hardwaretausch | **Sicherheitsrisiko** bei Fehlkonfiguration oder Hypervisor-Lücke (VM-Escape) |
| **Disaster Recovery:** VMs als Dateien sichern und replizieren | **Ressourcenkonkurrenz** um Storage-IOPS und Netz |

### Begriffe

| Begriff | Bedeutung |
|---|---|
| **Host / Gast** | physischer Server / VM darauf |
| **Overcommitment** | mehr vCPU/vRAM vergeben als physisch vorhanden, funktioniert, solange nicht alle VMs gleichzeitig Volllast fahren |
| **Thin Provisioning** | virtuelle Platte belegt nur den tatsächlich genutzten Platz |
| **Template / Klon** | Vorlage bzw. Kopie einer VM |
| **Paravirtualisierung** | Gast-OS weiß, dass es virtualisiert ist, und nutzt optimierte Treiber (VirtIO, VMware Tools) |
| **Hardwareunterstützung** | Intel VT-x / AMD-V im BIOS/UEFI aktivieren |

### Rechenweg: Konsolidierung

12 physische Server, im Mittel 10 % CPU-Last, je 32 GB RAM, davon 16 GB genutzt.
- RAM-Bedarf: 12 · 16 GB = 192 GB (+ 20 % Reserve ≈ 230 GB)
- Ein Host mit 256 GB RAM würde reichen, mit N+1 sind es **2 Hosts** mit je 256 GB.
- Stromersparnis → [Energieeffizienz & Stromkosten](../energieeffizienz-stromkosten.md).

## Prüfungsfrage

**Frage:** Ein Unternehmen will seine Server virtualisieren. Erläutern Sie je zwei Vor- und Nachteile.

**Musterantwort:** Vorteil 1: Konsolidierung. Mehrere VMs teilen sich einen Host, die Hardware wird besser ausgelastet, und es werden weniger physische Server, weniger Strom, Kühlung und Platz benötigt. Vorteil 2: Flexibilität. VMs lassen sich aus Vorlagen in Minuten bereitstellen, vor Updates per Snapshot sichern und per Live-Migration ohne Ausfall auf einen anderen Host verschieben. Nachteil 1: Der Host wird zum Single Point of Failure. Fällt er aus, sind alle VMs betroffen, daher braucht man einen Cluster mit mehreren Hosts. Nachteil 2: Komplexität und Kosten steigen, etwa durch Lizenzen für Hypervisor und Management sowie zusätzliches Know-how für Betrieb, Monitoring und Backup.

## Falle

- „Virtualisierung erhöht automatisch die Verfügbarkeit.“ Mit nur einem Host sinkt sie sogar (alle Dienste an einem Gerät).
- Typ 1 und Typ 2 vertauschen. Typ **1** = direkt auf der Hardware.
- Snapshot als Backup verkaufen → [Snapshots-Backups](snapshots-backups.md).
- Container als Unterart der VM beschreiben. Container haben keinen eigenen Kernel.
- Ressourcen beliebig überbuchen. RAM-Overcommitment führt zu Auslagerung und Leistungseinbruch.

## Merksatz

**„Viele VMs auf einem Host sparen Hardware, legen aber alle Eier in einen Korb, also Cluster einplanen.“**

## Verwandt

- [Hypervisor-Typen](hypervisor-typen.md) – Typ 1 vs. Typ 2
- [VM-Container-Server](vm-container-server.md) – Bare Metal, VM, Container
- [Container & Docker](container-docker.md)
- [VDI & Thin Client](vdi-thin-client.md)
- [Snapshots-Backups](snapshots-backups.md)
- [Live-Migration-VMs](live-migration-vms.md)
- [Skalierbarkeit-Elastizität](skalierbarkeit-elastizitaet.md)
- [Cluster](../cluster.md)
- [Lizenzierung](../lizenzierung.md) – Core-Lizenzen und VMs
- [Cloud Computing](../Cloud/cloud-computing.md) – IaaS basiert auf Virtualisierung
- [Lektion L13 Virtualisierung](../../../analyse-der-exams/L13_Virtualisierung.md)
