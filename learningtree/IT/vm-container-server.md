# VM-Container-Server

# Bare Metal vs. VM vs. Container

## 1. Bare Metal (Physischer Server)

**Prinzip:** Ein Betriebssystem läuft direkt auf der Hardware. Keine Virtualisierungsschicht erforderlich.

Früher oft: `1 Dienst → 1 Server`

**Vorteile**
- Hohe Performance, kein Hypervisor-Overhead.
- Direkter Hardwarezugriff, geeignet für I/O-intensive Workloads.
- Volle Kontrolle über Hardware und Betriebssystem.

**Nachteile**
- Schlechtere Ressourcenauslastung, wenn der Dienst wenig Last erzeugt.
- Skalierung und Migration sind aufwendiger.
- Hardwareausfälle können den Dienst vollständig unterbrechen.

**Einsatz:** Hochperformante Datenbanken, Storage-Systeme und spezielle Hardware-Workloads.

**Beispiel:** Ein Datenbankserver verarbeitet Millionen I/O-Operationen und benötigt dedizierte Ressourcen.

## 2. Virtuelle Maschinen (VMs)

**Prinzip:** Ein Hypervisor virtualisiert CPU, RAM, Storage und Netzwerk. Jede VM besitzt ein eigenes vollständiges Gastbetriebssystem inklusive Kernel.

**Typ 1:** Läuft direkt auf der Hardware, z. B. VMware ESXi, Hyper-V oder KVM.

**Typ 2:** Läuft auf einem Host-Betriebssystem, z. B. VirtualBox.

**Vorteile**
- Mehrere Betriebssysteme auf einem physischen Server.
- Flexible Ressourcenzuweisung und bessere Hardwareauslastung.
- Snapshots, Klonen und Migration möglich (abhängig von Plattform und Konfiguration).
- Stärkere Isolation als bei Containern.

**Nachteile**
- Mehr RAM- und Storage-Bedarf durch separate Gastbetriebssysteme.
- Hypervisor-Overhead.
- Startzeit meist länger als bei Containern.

**Beispiel:** Ein Server betreibt drei VMs: Windows Server für Active Directory, Linux für einen Webserver und Linux für eine Datenbank.

**Wichtig:** 20 Alpine-VMs benötigen 20 Gastbetriebssysteme und 20 Kernel.

## 3. Container

**Prinzip:** Container isolieren Anwendungen und Prozesse, teilen sich aber den Kernel des Host-Betriebssystems. Sie enthalten die Anwendung sowie ihre benötigten Bibliotheken und Abhängigkeiten.

**Technologien:** Docker, Podman, containerd; Orchestrierung mit Kubernetes.

**Vorteile**
- Geringer Ressourcenbedarf im Vergleich zu vollständigen VMs.
- Schneller Start und einfache Skalierung.
- Reproduzierbare Deployments durch Images.
- Ideal für Microservices und CI/CD.

**Nachteile**
- Gemeinsamer Kernel; eine Kernel-Schwachstelle kann mehrere Container betreffen.
- Isolation hängt stark von Konfiguration und Sicherheitsmechanismen ab.
- Container sind standardmäßig keine vollständigen virtuellen Maschinen.
- Persistente Daten benötigen geeignete Volumes oder externen Storage.

**Beispiel:** Eine Website läuft mit drei Containern: Nginx als Webserver, einer API und einer PostgreSQL-Datenbank.

**Wichtig:** Container sind nicht automatisch sicher oder zustandslos. Datenbanken benötigen weiterhin persistente Speicherung und Backups.

## 4. Vergleich

| Kriterium | Bare Metal | VM | Container |
|---|---|---|---|
| Startzeit | Langsam | Mittel | Sehr schnell |
| Performance | Sehr hoch | Hoch | Nahezu nativ, abhängig vom Workload |
| Isolation | Physische Trennung möglich | Stark | Prozess-/Kernel-basiert |
| Ressourceneffizienz | Gering bis hoch, je nach Auslastung | Mittel | Hoch |
| Flexibilität | Gering | Hoch | Sehr hoch |
| Betriebssystem | Direkt auf Hardware | Eigenes Gastbetriebssystem | Gemeinsamer Host-Kernel |
| Skalierung | Aufwendig | Flexibel | Sehr flexibel |
| Typischer Einsatz | Spezialserver, I/O-intensive Workloads | Serverkonsolidierung, unterschiedliche OS | Microservices, Web-Apps, CI/CD |

## 5. Real-World-Beispiele

- **Bare Metal:** Eine große Datenbank benötigt maximale I/O-Leistung und dedizierte Hardware.
- **VM:** Ein Unternehmen betreibt Windows- und Linux-Server auf derselben Hardware.
- **Container:** Ein Onlineshop skaliert Webserver und API-Instanzen abhängig von der Anzahl der Anfragen.

## 6. IHK-Prüfungswissen

**Hypervisor:** Schicht zur Virtualisierung und Verwaltung virtueller Hardware.

**Container-Runtime:** Software zum Erstellen und Ausführen von Containern, z. B. containerd.

**Overhead:** Zusätzlicher Ressourcenverbrauch durch Virtualisierung oder Verwaltung.

**Skalierung:**
- Vertikal: CPU/RAM einer bestehenden Instanz erhöhen.
- Horizontal: Weitere Instanzen hinzufügen.

**Migration:** Verschieben einer VM auf einen anderen Host. Live-Migration ist mit geeigneter Infrastruktur teilweise ohne längere Unterbrechung möglich.

**Snapshot vs. Backup:**
- Snapshot: Zustand zu einem bestimmten Zeitpunkt, häufig abhängig vom ursprünglichen Datenträger.
- Backup: Separate Sicherung zur Wiederherstellung nach Datenverlust.

**Sicherheit:** Updates, Zugriffskontrolle, Netzwerksegmentierung und Backups bleiben bei allen drei Varianten notwendig.

## Merksatz

- **Bare Metal:** Maximale Hardwarekontrolle.
- **VM:** Eigenes Betriebssystem und starke Isolation.
- **Container:** Schlanke, portable Anwendungsumgebung mit gemeinsamem Kernel.

Die Entscheidung hängt von Performance, Isolation, Ressourceneffizienz, Betriebskosten und Skalierbarkeit ab.

## 7. Wann verwendet man was?

- **Bare Metal:** Wenn ein Dienst konstante Performance, hohe I/O-Leistung oder direkten Hardwarezugriff benötigt.  
  *Beispiel:* Datenbankserver mit hohem Transaktionsvolumen.

- **VM:** Wenn Isolation, unterschiedliche Betriebssysteme oder getrennte Umgebungen benötigt werden.  
  *Beispiel:* Test- und Produktionssysteme oder mehrere Abteilungen auf einem Host.

- **Container:** Wenn Anwendungen schnell gestartet, aktualisiert und horizontal skaliert werden müssen.  
  *Beispiel:* Webdienste und APIs mit mehreren Anwendungsversionen für Tests und Rollouts.

## Siehe auch

- [Hypervisor-Typen](hypervisor-typen.md) – Typ 1 vs. Typ 2
- [Live-Migration-VMs](live-migration-vms.md) – laufende VMs verschieben
- [BIOS/UEFI-Kernel](bios-uefi-kernel.md) – Kernel und User Space
- [Snapshots-Backups](snapshots-backups.md) – Sicherung von VMs

