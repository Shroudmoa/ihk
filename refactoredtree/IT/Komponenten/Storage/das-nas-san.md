# DAS vs. NAS vs. SAN – Speicherlösungen

## In 30 Sekunden

- **DAS** = Speicher direkt am Server (intern, USB, SAS-Gehäuse), **Blockebene**.
- **NAS** = Dateiserver im Netz, **Dateiebene** über **SMB/NFS**. Einfach, günstig, für Freigaben und Backups.
- **SAN** = eigenes Speichernetz, **Blockebene** über **Fibre Channel oder iSCSI**. Server sehen LUNs wie lokale Platten, typisch für Virtualisierung, Cluster und Datenbanken.
- RAID organisiert die Platten **innerhalb** eines Systems und ist weder DAS noch NAS noch SAN.
- Geprüft: SAN-Kapazität und Betriebszeit 2021W, 2023S, 2024W. NAS-Benchmark-Verfälschung 2022S.

## Kernwissen

### 1. DAS – Direct Attached Storage

**DAS** ist direkt an einen einzelnen Computer oder Server angeschlossener Speicher, ohne dass der Speicherzugriff über ein Netzwerk erfolgt.

- Verbindung: SATA, SAS, USB, PCIe/NVMe
- Zugriff: Blockebene
- Beispiele: Interne HDD, SATA-SSD, NVMe-SSD, externe USB-Festplatte
- Vorteile: Einfach, geringe Latenz, meist kostengünstig
- Nachteile: Nicht automatisch zentral über das Netzwerk verfügbar, schwieriger gemeinsam zu nutzen
- Einsatz: PC-Speicher, Serverlaufwerke, lokale Datenbanken

**Beispiel:** Ein Server verwendet eine intern eingebaute NVMe-SSD für sein Betriebssystem und seine Anwendungen.

### 2. NAS – Network Attached Storage

Ein NAS stellt Dateien über ein Netzwerk für mehrere Clients bereit.

- Verbindung: Ethernet/LAN
- Zugriff: Dateiebene
- Protokolle: SMB/CIFS, NFS
- Vorteile: Gemeinsame Dateifreigaben, zentrale Verwaltung, einfache Nutzung
- Nachteile: Netzwerk und NAS können Engpässe oder Ausfallpunkte sein
- Einsatz: Teamordner, Dateiserver, Medien, Backups

**Beispiel:** Mitarbeiter speichern Dokumente in einem gemeinsamen Netzwerkordner.

### 3. SAN – Storage Area Network

Ein SAN stellt Servern über ein Speichernetzwerk zentralen Speicher auf Blockebene bereit.

- Verbindung: Fibre Channel oder iSCSI über Ethernet/IP
- Zugriff: Blockebene
- Typische Begriffe: LUN, Volume, Multipathing
- Vorteile: Zentraler Speicher, hohe Leistung und Verfügbarkeit bei entsprechender Auslegung
- Nachteile: Höhere Komplexität und häufig höhere Kosten
- Einsatz: Virtualisierung, Datenbanken, Unternehmensserver

**Beispiel:** Mehrere Virtualisierungshosts greifen über ein SAN auf einen zentralen VM-Datastore zu.

### 4. Vergleich

| Eigenschaft | DAS | NAS | SAN |
|---|---|---|---|
| Vollständiger Name | Direct Attached Storage | Network Attached Storage | Storage Area Network |
| Anschluss | Direkt am Rechner | Netzwerk | Speichernetzwerk |
| Zugriff | Block | Datei | Block |
| Typische Protokolle | SATA, SAS, NVMe | SMB, NFS | iSCSI, Fibre Channel |
| Gemeinsame Nutzung | Nicht standardmäßig über Netzwerk | Einfach für mehrere Clients | Für mehrere Server, mit passender Konfiguration |
| Komplexität | Niedrig | Mittel | Häufig hoch |
| Typische Kosten | Niedrig bis mittel | Niedrig bis hoch | Mittel bis hoch |
| Einsatz | Lokale Laufwerke | Gemeinsame Dateien | VM-Storage, Datenbanken |

### Merksätze für die IHK

- **DAS:** Speicher direkt am Rechner.
- **NAS:** Dateien über das Netzwerk.
- **SAN:** Speicherblöcke über ein Speichernetzwerk.

### Benchmark eines NAS richtig messen (2022S)

| Verfälschung | Warum | Abhilfe |
|---|---|---|
| Netzwerk als Engpass | 1-Gbit/s-Client misst max. ca. 110 MB/s, auch wenn das NAS 10 Gbit/s kann | Test mit passender Anbindung |
| Cache-Effekt | Testdatei kleiner als der RAM-Cache des NAS → es wird der RAM gemessen | Testdatei deutlich größer als den Cache wählen |
| zu wenig Parallelität | ein einzelner Thread lastet Controller und Platten nicht aus | mehrere Threads, höhere Queue-Tiefe |

## Prüfungsfrage

**Frage:** Für fünf Virtualisierungshosts wird zentraler Speicher benötigt. Begründen Sie, ob ein NAS oder ein SAN besser geeignet ist.

**Musterantwort:** Ein SAN ist besser geeignet. Es stellt den Hosts Speicher auf Blockebene bereit (z. B. per iSCSI oder Fibre Channel). Die Hosts binden die LUNs wie lokale Datenträger ein und können darauf ein Cluster-Dateisystem für die VMs anlegen. Alle Hosts sehen dieselben Daten, was Live-Migration und Failover ermöglicht. Ein eigenes Speichernetz trennt den Storage-Verkehr vom Benutzernetz und bietet hohe Leistung und geringe Latenz. Ein NAS arbeitet auf Dateiebene (SMB/NFS) und eignet sich eher für Dateifreigaben und Backups.

## Falle

- DAS bedeutet nicht zwingend eine interne Festplatte: Eine direkt per USB angeschlossene externe Festplatte ist ebenfalls DAS.
- NAS und SAN sind Speicherarchitekturen, keine bestimmten Festplattentypen. Intern können sie HDDs, SSDs oder NVMe verwenden.
- NAS und SAN sind nicht automatisch schneller als DAS. Die tatsächliche Leistung hängt von Hardware, Netzwerk, Protokoll und Auslastung ab.
- RAID ist weder DAS noch NAS noch SAN, sondern eine Methode zur Organisation mehrerer Laufwerke.
- RAID ersetzt kein Backup.

## Merksatz

**„DAS hängt dran, NAS teilt Dateien, SAN verleiht Blöcke.“**

## Verwandt

- [RAID](../../FiSi-Administrator/Server/raid.md)
- [Kapazitätsplanung](kapazitaetsplanung.md) – SAN-Bedarf und Betriebszeit berechnen
- [Dateisysteme](dateisysteme.md)
- [Speicherhierarchie](speicherhierarchie.md)
- [Backup-Strategien](../../FiSi-Administrator/Server/backup/backup-strategien.md) – NAS als Backup-Ziel
- [Cluster](../../FiSi-Administrator/Server/cluster.md) – Shared Storage
- [Lektion 02 Speicher, Backup, RAID](../../analyse-der-exams/02_Speicher_Backup_RAID.md)
