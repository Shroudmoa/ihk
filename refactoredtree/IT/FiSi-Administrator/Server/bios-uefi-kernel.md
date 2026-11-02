# Bootprozess, Kernel, Partitionen und GRUB

## In 30 Sekunden

- Bootkette: **Firmware (UEFI/BIOS) → Bootloader (GRUB, Windows Boot Manager) → Kernel (+ initramfs) → Init-System (systemd) → Dienste → Anmeldung**.
- Der **Kernel** verwaltet CPU, RAM, Geräte, Dateisysteme und Rechte. Er läuft im privilegierten **Kernel Space**, Anwendungen laufen im **User Space** und rufen ihn über **System Calls** auf.
- **GRUB** ist der verbreitete Linux-Bootloader: Bootmenü, Auswahl von Kernel oder Betriebssystem, lädt Kernel und initramfs. Konfiguration in `/boot/grub/grub.cfg`, erzeugt aus `/etc/default/grub`.
- Typische Linux-Partitionen: EFI-Systempartition, `/boot`, `/`, `swap`, `/home`.
- Firmware-Härtung und Secure Boot stehen in [BIOS/UEFI – Firmware und Härtung](bios-uefi.md).
- Geprüft: GRUB-Funktion und Fehler „file not found“ 2026S.

## Kernwissen

### Bootablauf im Detail

| Schritt | BIOS/MBR | UEFI/GPT |
|---|---|---|
| 1. Firmware | POST, Hardware-Initialisierung | POST, Hardware-Initialisierung, Secure-Boot-Prüfung |
| 2. Bootloader finden | Bootcode im MBR (erster Sektor) | `.efi`-Datei auf der EFI-Systempartition (z. B. `\EFI\ubuntu\shimx64.efi`) |
| 3. Bootloader | GRUB / Windows Boot Manager | GRUB (über shim) / Windows Boot Manager |
| 4. Kernel | wird mit initramfs in den RAM geladen | ebenso |
| 5. Init | systemd startet Dienste (Linux) bzw. Windows-Sitzungsmanager | ebenso |

**initramfs:** kleines Start-Dateisystem im RAM mit Treibern (z. B. für RAID, LVM, verschlüsselte Platten), damit der Kernel die echte Root-Partition einhängen kann.

### Kernel

| Aufgabe | Beispiel |
|---|---|
| Prozessverwaltung | Prozesse starten, beenden, CPU-Zeit verteilen (Scheduler) |
| Speicherverwaltung | RAM zuweisen, virtueller Speicher, Prozesse voneinander isolieren |
| Geräteverwaltung | Treiber, Hardwarezugriffe koordinieren |
| Dateisysteme | Dateioperationen, Zugriffsrechte |
| Netzwerk | TCP/IP-Stack |

| Betriebssystem | Kernel | Architektur |
|---|---|---|
| Linux | Linux-Kernel | monolithisch, modular (ladbare Module) |
| Windows | Windows NT | Hybrid |
| macOS | XNU | Hybrid (Mach + BSD) |
| FreeBSD | FreeBSD-Kernel | monolithisch, modular |

- **Kernel Space:** privilegierter Bereich. Ein Fehler hier kann das ganze System anhalten (Kernel Panic, Bluescreen).
- **User Space:** normale Anwendungen mit begrenzten Rechten. Sie fordern Kernel-Funktionen per **System Call** an (z. B. Datei öffnen, Netzwerkdaten senden). Unter Windows rufen Anwendungen die Windows-API auf, die intern die NT-Systemdienste nutzt.
- Container teilen sich den Kernel des Hosts → [Container & Docker](virtualisierung/container-docker.md).

### Partitionen

| Mountpoint / Bereich | Funktion |
|---|---|
| EFI-Systempartition (ESP) | Bootloader-Dateien für UEFI, FAT32, ca. 100–500 MB |
| `/boot` | Kernel, initramfs, GRUB-Dateien |
| `/` (Root) | Wurzel des Dateisystems, Betriebssystem |
| `swap` | Auslagerungsbereich (Partition oder Datei) |
| `/home` | Benutzerdaten |

`/` ist nicht `/root`: `/root` ist das Home-Verzeichnis des Benutzers root. Nicht jede Installation hat ein separates `/boot` oder eine Swap-Partition.

**Windows:** EFI-Systempartition, MSR (reserviert), **C:** als Systempartition, Wiederherstellungspartition (WinRE). Statt Swap-Partition wird die Auslagerungsdatei `pagefile.sys` verwendet.

### GRUB

- Zeigt ein Menü (mehrere Kernel, Recovery-Modus, Dual-Boot mit Windows).
- Findet den Kernel über Datenträger, Partition und Pfad, z. B. `(hd0,gpt2)/vmlinuz`, heute meist über die **UUID** des Dateisystems.
- Konfiguration nie direkt in `grub.cfg` bearbeiten: Einstellungen in `/etc/default/grub` und `/etc/grub.d/`, danach `update-grub` (Debian/Ubuntu) bzw. `grub2-mkconfig -o /boot/grub2/grub.cfg` (RHEL).
- Andere Bootloader: Windows Boot Manager, systemd-boot, U-Boot (Embedded).

### GRUB-Fehler „file not found“ / `grub rescue>` (2026S)

**Ursachen:** GRUB findet seine Dateien oder den Kernel nicht, etwa weil Partitionen verschoben oder neu angelegt wurden (geänderte UUID), `/boot` beschädigt oder voll ist, ein Update abgebrochen wurde oder `grub.cfg` auf einen falschen Pfad zeigt.

**Behebung:**
1. In der Rescue-Konsole: `ls` zeigt Partitionen. Dann `set root=(hd0,gpt2)`, `set prefix=(hd0,gpt2)/boot/grub`, `insmod normal`, `normal` → System starten.
2. Dauerhaft: `grub-install` (auf den richtigen Datenträger bzw. die ESP) und `update-grub` ausführen.
3. Startet nichts mehr: von einem Live-System booten, Root-Partition mounten, per `chroot` wechseln und Schritt 2 ausführen.

## Prüfungsfrage

**Frage:** Nach einer Änderung an den Partitionen startet ein Linux-Server nicht mehr. GRUB meldet „error: file not found“. Erläutern Sie Funktion von GRUB, eine mögliche Ursache und das Vorgehen zur Behebung.

**Musterantwort:** GRUB ist der Bootloader. Er wird von der Firmware gestartet, zeigt ein Bootmenü und lädt den gewählten Linux-Kernel samt initramfs in den Arbeitsspeicher. Danach übergibt er die Kontrolle an den Kernel. Durch die Änderung der Partitionen stimmen vermutlich Partitionsnummer oder UUID in der GRUB-Konfiguration nicht mehr. GRUB findet deshalb seine Moduldateien oder den Kernel nicht. Zur Behebung bootet man ein Live-System, hängt die Root- und Boot-Partition ein, wechselt per chroot in das installierte System und führt `grub-install` sowie `update-grub` aus. Damit wird GRUB neu installiert und die Konfiguration mit den aktuellen UUIDs neu erzeugt.

## Falle

- Bootloader und Kernel gleichsetzen. GRUB **lädt** den Kernel, er ist kein Teil des Betriebssystemkerns.
- `grub.cfg` von Hand ändern. Beim nächsten Update wird sie überschrieben.
- Swap als Ersatz für fehlenden RAM verkaufen. Auslagern ist um Größenordnungen langsamer.
- Kernel und Kernel Space gleichsetzen. Das eine ist die Software, das andere der privilegierte Ausführungsbereich.
- Nach einem Wechsel BIOS ↔ UEFI erwarten, dass der alte Bootloader weiter funktioniert.

## Merksatz

**„Firmware weckt, GRUB lädt, Kernel herrscht, systemd startet den Rest.“**

## Verwandt

- [BIOS/UEFI – Firmware und Härtung](bios-uefi.md) – Secure Boot, Setup-Passwort
- [Linux-Grundlagen](linux-grundlagen.md) – Verzeichnisse und Befehle
- [Dateisysteme](../../Komponenten/Storage/dateisysteme.md) – MBR/GPT, ext4, NTFS
- [CPU](../../Komponenten/cpu.md) – Zusammenspiel mit der CPU
- [VM-Container-Server](virtualisierung/vm-container-server.md) – Kernel im Container
- [Hypervisor-Typen](virtualisierung/hypervisor-typen.md) – Firmware unter der Virtualisierung
- [Lektion L28 GA1 Praxisthemen](../../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md)
