# BIOS/UEFI-Kernel

# BIOS/UEFI, Kernel und Speicherbereiche

## Bootprozess

UEFI/BIOS → Bootloader → Kernel → Systemdienste → Anwendungen

### BIOS / UEFI
Firmware, die beim Einschalten startet und die Hardware für den Betriebssystemstart vorbereitet.

- POST: Prüft grundlegende Hardwarefunktionen.
- Initialisiert Hardware und wählt ein Bootmedium.
- Startet den Bootloader.

**BIOS** ist die ältere Firmware-Technologie; **UEFI** ist der moderne Nachfolger.

### Kernel
Zentraler Bestandteil des Betriebssystems. Verwaltet Hardware-Ressourcen und stellt Anwendungen kontrollierte Schnittstellen bereit.

- CPU-Zeit und Prozessverwaltung
- RAM und virtueller Speicher
- Dateisysteme
- Netzwerk und Gerätezugriff
- Gerätetreiber und Berechtigungen

### Kernel Space
Privilegierter Speicher- und Ausführungsbereich des Kernels. Fehler im Kernel können das gesamte System beeinträchtigen.

### User Space
Hier laufen normale Anwendungen und viele Systemdienste. Direkter Zugriff auf geschützten Kernel-Speicher ist nicht erlaubt.

**System Calls:** Kontrollierte Schnittstelle, über die Anwendungen Kernel-Funktionen anfordern, z. B. Dateien öffnen oder Netzwerkdaten senden.

## Kernel-Architekturen

| Betriebssystem | Kernel | Architektur |
|---|---|---|
| Linux | Linux-Kernel | Monolithisch, modular |
| Windows | Windows NT | Hybrid |
| macOS | XNU | Hybrid, Mach + BSD |
| FreeBSD | FreeBSD-Kernel | Monolithisch, modular |

## IHK-Merksätze

- **Kernel ≠ Kernel Space:** Kernel = Betriebssystemkern; Kernel Space = privilegierter Ausführungsbereich.
- **User Space:** Begrenzte Rechte; Kernel-Funktionen werden über System Calls angefordert.
- **UEFI/BIOS:** Firmware, die vor dem Betriebssystem ausgeführt wird.
- **System Call:** Kontrollierter Übergang vom User Space in den Kernel Space.

## 3-Zeilen-IHK-Antwort

UEFI initialisiert die Hardware und startet den Bootloader, der den Kernel lädt.
Der Kernel läuft im privilegierten Kernel Space und verwaltet Ressourcen wie CPU, RAM und Geräte.
Anwendungen laufen im User Space und greifen über System Calls kontrolliert auf Kernel-Funktionen zu.


# Kernel, Partitionen und Bootloader – Teil 2

## Kernel: Linux vs. Windows

Der Kernel verwaltet CPU, RAM, Prozesse, Geräte, Dateisysteme und Zugriffsrechte. Er vermittelt zwischen Hardware und Anwendungen.

| Konzept | Linux | Windows |
|---|---|---|
| Kernel | Linux-Kernel | Windows NT Kernel |
| Architektur | Monolithisch, modular | Hybrid |
| Treiber | Kernel- oder User-Space möglich | Kernel- und User-Mode-Treiber |
| System Calls | Linux-System-Call-Schnittstelle | Windows-API/NT-Systemdienste |
| Konfiguration | Häufig Textdateien | Registry, Dienste und Systemtools |
| Kernel-Module | Ladbare Kernel-Module | Kernel-Treiber und andere Kernel-Komponenten |

**Wichtig:** Die Windows API ist nicht dasselbe wie ein System Call. Anwendungen nutzen APIs, die bei Bedarf Kernel-Funktionen aufrufen.

### Was macht der Kernel konkret?

- **Prozessverwaltung:** Prozesse starten, beenden und CPU-Zeit verteilen.
- **Speicherverwaltung:** RAM zuweisen, virtuellen Speicher verwalten und Prozesse isolieren.
- **Geräteverwaltung:** Treiber und Hardwarezugriffe koordinieren.
- **Dateisysteme:** Dateioperationen und Zugriffsrechte bereitstellen.
- **Netzwerk:** Netzwerkprotokolle und Kommunikation unterstützen.

## Partitionen und Dateisysteme

Eine Partition ist ein logisch abgegrenzter Bereich eines Datenträgers. Ein Dateisystem organisiert darin Dateien und Verzeichnisse.

**Beispiele:** Linux verwendet häufig ext4 oder XFS, Windows meist NTFS.

### Typische Linux-Partitionen

| Mountpoint / Bereich | Funktion |
|---|---|
| `/boot` | Kernel-Dateien und weitere Dateien für den Bootvorgang |
| `/` (Root) | Wurzel des Linux-Dateisystems; enthält das Betriebssystem |
| `swap` | Auslagerungsbereich für Speicherseiten; kann Partition oder Datei sein |
| `/home` | Benutzerdaten und persönliche Konfigurationen |
| EFI-Systempartition (ESP) | Bootloader-Dateien für UEFI-Systeme |

**Wichtig:** `/` ist nicht dasselbe wie `/root`. `/` ist das Wurzelverzeichnis; `/root` ist das Home-Verzeichnis des Administrators `root`.

Nicht jede Linux-Installation benötigt eine separate `/boot`- oder Swap-Partition. Bei UEFI-Systemen wird üblicherweise eine EFI-Systempartition benötigt.

### Wie macht Windows das?

Windows verwendet ebenfalls Partitionen, aber andere Bezeichnungen und Strukturen:

- **EFI-Systempartition:** Enthält UEFI-Bootdateien.
- **C:**:** Üblicherweise die Windows-Systempartition.
- **Recovery-Partition:** Enthält Wiederherstellungswerkzeuge.
- **System-reservierte Partition:** Kann bei BIOS/MBR-Systemen Bootdateien enthalten.
- **Auslagerungsdatei (`pagefile.sys`):** Dient als virtueller Speicher; normalerweise keine separate Swap-Partition.

## Bootloader

Ein Bootloader startet den Betriebssystem-Kernel oder übergibt die Kontrolle an einen weiteren Bootloader.

### GRUB

**GRUB (GRand Unified Bootloader)** ist ein verbreiteter Bootloader für Linux.

- Zeigt ein Bootmenü für mehrere Betriebssysteme oder Kernel.
- Lädt den Linux-Kernel und gegebenenfalls eine initiale RAM-Disk (`initramfs`).
- Ermöglicht die Auswahl verschiedener Kernel oder Recovery-Einträge.

### Andere Bootloader

| Bootloader | Typischer Einsatz |
|---|---|
| Windows Boot Manager | Windows-Systeme |
| systemd-boot | Linux-Systeme mit UEFI |
| U-Boot | Embedded-Systeme |
| Apple Bootprozess | macOS auf Apple-Hardware |

### BIOS vs. UEFI: Bootvorgang

**BIOS/MBR:** Firmware → Bootcode im MBR → Bootloader → Kernel.

**UEFI:** Firmware → EFI-Anwendung auf der EFI-Systempartition → Bootloader → Kernel.

Bei Linux lädt GRUB häufig zusätzlich `initramfs`, das benötigte Treiber und Werkzeuge für den frühen Systemstart enthält.

## IHK-Merksätze

- **Partition:** Logische Aufteilung eines Datenträgers.
- **Dateisystem:** Organisiert Dateien und Verzeichnisse.
- **Bootloader:** Lädt den Kernel oder übergibt die Kontrolle an ihn.
- **Swap/Pagefile:** Unterstützt virtuellen Speicher, ersetzt aber keinen ausreichend dimensionierten RAM.
- **GRUB:** Häufiger Linux-Bootloader; Windows verwendet normalerweise den Windows Boot Manager.

## 3-Zeilen-IHK-Antwort

Der Bootloader lädt den Betriebssystem-Kernel und gegebenenfalls eine initiale RAM-Disk.
Der Kernel verwaltet Ressourcen und stellt kontrollierte Schnittstellen für Anwendungen bereit.
Partitionen strukturieren den Datenträger; Dateisysteme organisieren die darauf gespeicherten Daten.

## Siehe auch

- [CPU](cpu.md) – Zusammenspiel mit der CPU
- [VM-Container-Server](vm-container-server.md) – Kernel im Container
- [Hypervisor-Typen](hypervisor-typen.md) – Firmware unter der Virtualisierung

