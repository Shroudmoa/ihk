# Dateisysteme (NTFS, ReFS, FAT32, exFAT, ext4)

## In 30 Sekunden

- Ein Dateisystem organisiert Daten in einer Partition: Dateinamen, Ordner, Rechte, freie Blöcke.
- **NTFS** ist der Windows-Standard (Rechte/ACL, Journaling, Verschlüsselung, Kompression). **ReFS** ist für große Windows-Datenspeicher mit Prüfsummen und Selbstheilung gedacht.
- **FAT32**: maximal **4 GiB pro Datei**, keine Rechte → nur noch für USB-Sticks und EFI-Partitionen. **exFAT** ist der Nachfolger für Wechseldatenträger (große Dateien, keine Rechte).
- **ext4** ist der Linux-Standard, **XFS** für große Dateien und Server, **Btrfs/ZFS** bringen Snapshots, Prüfsummen und Copy-on-Write.
- **Clustergröße** (Zuordnungseinheit): klein bei vielen kleinen Dateien (wenig Verschnitt), groß bei großen Dateien (weniger Verwaltung, schneller). Geprüft 2021W und 2023W.
- **GPT** statt MBR: Datenträger > 2 TiB, bis 128 Partitionen, nötig für UEFI-Boot.

## Kernwissen

### Vergleich

| Dateisystem | OS | max. Dateigröße | Rechte | Journaling | Besonderheiten / Einsatz |
|---|---|---|---|---|---|
| **FAT32** | alle | **4 GiB − 1 Byte** | nein | nein | USB-Sticks, EFI-Systempartition, maximale Kompatibilität |
| **exFAT** | alle (modern) | praktisch unbegrenzt | nein | nein | SD-Karten, USB-Datenträger mit großen Videos |
| **NTFS** | Windows | 16 TiB (klassisch), mehr mit großen Clustern | **ACL** | ja | Windows-System und Datenpartitionen, EFS, Kompression, Kontingente |
| **ReFS** | Windows Server | sehr groß | ACL | Integrität per Prüfsumme | Storage Spaces, Hyper-V-Datenspeicher, Backup-Repositories. Kein Bootlaufwerk |
| **ext4** | Linux | 16 TiB | `rwx` + ACL | ja | Linux-Standard |
| **XFS** | Linux | 8 EiB | `rwx` + ACL | ja | große Dateien, Server (RHEL-Standard) |
| **Btrfs / ZFS** | Linux / BSD, NAS | sehr groß | ja | Copy-on-Write | Snapshots, Prüfsummen, integriertes RAID, Deduplizierung |
| **APFS** | macOS | sehr groß | ja | Copy-on-Write | Apple-Geräte, Snapshots, Verschlüsselung |

**Journaling:** Änderungen werden zuerst in ein Journal geschrieben. Nach einem Absturz oder Stromausfall bleibt das Dateisystem konsistent, und es entfällt ein stundenlanger Prüflauf (chkdsk/fsck).

### Clustergröße und Verschnitt (Slack)

Eine Datei belegt immer ganze Cluster. Der ungenutzte Rest des letzten Clusters heißt **Slack** (Verschnitt).

**Rechenweg:** 10.000 Dateien à 1 KiB

| Clustergröße | belegt pro Datei | Verschnitt pro Datei | Verschnitt gesamt |
|---:|---:|---:|---:|
| 4 KiB | 4 KiB | 3 KiB | 30.000 KiB ≈ **29,3 MiB** |
| 64 KiB | 64 KiB | 63 KiB | 630.000 KiB ≈ **615 MiB** |

Eine Datei mit 10 KiB belegt bei 4-KiB-Clustern 3 Cluster = 12 KiB (2 KiB Slack).

| Clustergröße | sinnvoll für | Begründung |
|---|---|---|
| **klein** (4 KiB) | viele kleine Dateien: Office, Logs, Quellcode | wenig Verschnitt |
| **groß** (64 KiB und mehr) | wenige große Dateien: Videos, VM-Images, Datenbanken, Backups | weniger Verwaltungsaufwand, weniger Fragmentierung, schnellere sequenzielle Zugriffe |

### Partitionstabellen

| | MBR | GPT |
|---|---|---|
| max. Datenträgergröße | 2 TiB (bei 512-Byte-Sektoren) | 8 ZiB (praktisch unbegrenzt) |
| Partitionen | 4 primäre (oder 3 + erweiterte) | 128 (Windows) |
| Boot | BIOS (Legacy) | UEFI |
| Ausfallsicherheit | eine Tabelle | Kopie am Ende des Datenträgers, CRC-Prüfsumme |

## Prüfungsfrage

**Frage:** Für zwei neue Volumes ist die Größe der Zuordnungseinheit festzulegen. Auf Volume A liegen Office-Dokumente und Logdateien, auf Volume B Videoaufnahmen und Datenbanksicherungen. Geben Sie je eine sinnvolle Clustergröße an und begründen Sie.

**Musterantwort:** Volume A erhält eine kleine Zuordnungseinheit, z. B. 4 KiB. Bei vielen kleinen Dateien bleibt im jeweils letzten Cluster wenig Platz ungenutzt, der Verschnitt ist gering. Volume B erhält eine große Zuordnungseinheit, z. B. 64 KiB. Große Dateien bestehen dann aus weniger Clustern. Das senkt den Verwaltungsaufwand des Dateisystems, verringert die Fragmentierung und beschleunigt sequenzielle Zugriffe. Der Verschnitt fällt bei großen Dateien kaum ins Gewicht.

## Falle

- FAT32 für ein 6-GiB-Image vorschlagen. Die Datei lässt sich nicht speichern.
- „NTFS = schnell, FAT = langsam“ als Hauptargument. Der Kernunterschied sind **Rechte und Journaling**.
- Große Cluster für viele kleine Dateien. Der Verschnitt frisst Kapazität.
- ReFS als Systemlaufwerk für Windows-Clients einplanen. Davon kann nicht gebootet werden.
- Datenträger > 2 TiB mit MBR partitionieren. Der Rest bleibt ungenutzt.

## Merksatz

**„Viele Kleine → kleine Cluster, wenige Große → große Cluster. FAT32 endet bei 4 GiB.“**

## Verwandt

- [NTFS- und Freigabeberechtigungen](../../FiSi-Administrator/Server/ntfs-freigabeberechtigungen.md)
- [chmod](../../FiSi-Administrator/chmod.md) – Rechte unter ext4
- [BIOS/UEFI-Kernel](../../FiSi-Administrator/Server/bios-uefi-kernel.md) – Partitionen, EFI-Systempartition
- [USB-SSD-HDD](../usb-ssd-hdd.md) – Datenträger
- [Deduplizierung & Komprimierung](deduplizierung-komprimierung.md)
- [Kapazitätsplanung](kapazitaetsplanung.md)
- [Lektion 02 Speicher, Backup, RAID](../../analyse-der-exams/02_Speicher_Backup_RAID.md)
- [Lektion L28 GA1 Praxisthemen](../../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md)
