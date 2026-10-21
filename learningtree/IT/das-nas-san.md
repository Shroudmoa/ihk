# DAS-NAS-SAN

# DAS vs. NAS vs. SAN – Speicherlösungen

## 1. DAS – Direct Attached Storage

**DAS** ist direkt an einen einzelnen Computer oder Server angeschlossener Speicher, ohne dass der Speicherzugriff über ein Netzwerk erfolgt.

- Verbindung: SATA, SAS, USB, PCIe/NVMe
- Zugriff: Blockebene
- Beispiele: Interne HDD, SATA-SSD, NVMe-SSD, externe USB-Festplatte
- Vorteile: Einfach, geringe Latenz, meist kostengünstig
- Nachteile: Nicht automatisch zentral über das Netzwerk verfügbar, schwieriger gemeinsam zu nutzen
- Einsatz: PC-Speicher, Serverlaufwerke, lokale Datenbanken

**Beispiel:** Ein Server verwendet eine intern eingebaute NVMe-SSD für sein Betriebssystem und seine Anwendungen.

## 2. NAS – Network Attached Storage

Ein NAS stellt Dateien über ein Netzwerk für mehrere Clients bereit.

- Verbindung: Ethernet/LAN
- Zugriff: Dateiebene
- Protokolle: SMB/CIFS, NFS
- Vorteile: Gemeinsame Dateifreigaben, zentrale Verwaltung, einfache Nutzung
- Nachteile: Netzwerk und NAS können Engpässe oder Ausfallpunkte sein
- Einsatz: Teamordner, Dateiserver, Medien, Backups

**Beispiel:** Mitarbeiter speichern Dokumente in einem gemeinsamen Netzwerkordner.

## 3. SAN – Storage Area Network

Ein SAN stellt Servern über ein Speichernetzwerk zentralen Speicher auf Blockebene bereit.

- Verbindung: Fibre Channel oder iSCSI über Ethernet/IP
- Zugriff: Blockebene
- Typische Begriffe: LUN, Volume, Multipathing
- Vorteile: Zentraler Speicher, hohe Leistung und Verfügbarkeit bei entsprechender Auslegung
- Nachteile: Höhere Komplexität und häufig höhere Kosten
- Einsatz: Virtualisierung, Datenbanken, Unternehmensserver

**Beispiel:** Mehrere Virtualisierungshosts greifen über ein SAN auf einen zentralen VM-Datastore zu.

## 4. Vergleich

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

## 5. Merksätze für die IHK

- **DAS:** Speicher direkt am Rechner.
- **NAS:** Dateien über das Netzwerk.
- **SAN:** Speicherblöcke über ein Speichernetzwerk.

## 6. Wichtige Prüfungsfallen

- DAS bedeutet nicht zwingend eine interne Festplatte: Eine direkt per USB angeschlossene externe Festplatte ist ebenfalls DAS.
- NAS und SAN sind Speicherarchitekturen, keine bestimmten Festplattentypen. Intern können sie HDDs, SSDs oder NVMe verwenden.
- NAS und SAN sind nicht automatisch schneller als DAS. Die tatsächliche Leistung hängt von Hardware, Netzwerk, Protokoll und Auslastung ab.
- RAID ist weder DAS noch NAS noch SAN, sondern eine Methode zur Organisation mehrerer Laufwerke.
- RAID ersetzt kein Backup.
