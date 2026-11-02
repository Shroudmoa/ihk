# BIOS/UEFI – Firmware und Härtung

## In 30 Sekunden

- Die **Firmware** (BIOS oder UEFI) läuft als erstes nach dem Einschalten: **POST**, Hardware-Initialisierung, Auswahl des Bootmediums, Start des Bootloaders.
- **UEFI** ist der Nachfolger des BIOS: GPT-Datenträger > 2 TiB, **Secure Boot**, Netzwerk-Boot, grafische Oberfläche, Boot über `.efi`-Dateien auf der EFI-Systempartition.
- **Härtung** (2023S, 2026S): **Admin-Passwort** für das Setup, **Secure Boot** an, **Boot von externen Medien sperren** und Bootreihenfolge festlegen, Legacy/CSM aus, **Firmware aktuell**, TPM aktivieren, ungenutzte Schnittstellen deaktivieren.
- Dieses Blatt behandelt Firmware und Härtung. Bootablauf, Kernel, Partitionen und GRUB stehen in [BIOS/UEFI-Kernel](bios-uefi-kernel.md).

## Kernwissen

### BIOS vs. UEFI

| Merkmal | BIOS (Legacy) | UEFI |
|---|---|---|
| Partitionstabelle beim Boot | MBR (max. 2 TiB, 4 primäre Partitionen) | **GPT** (> 2 TiB, 128 Partitionen) |
| Bootvorgang | Bootcode im ersten Sektor (MBR) | Bootloader als `.efi`-Datei auf der EFI-Systempartition (FAT32) |
| Sicherheit | keine Signaturprüfung | **Secure Boot**: nur signierte Bootloader/Kernel starten |
| Betriebsmodus | 16-Bit-Real-Mode | 32/64-Bit, modular, eigene Treiber |
| Oberfläche | Text, Tastatur | oft grafisch, Maus |
| Netzwerk | PXE als Erweiterung | Netzwerk-Boot (PXE/HTTP-Boot) integriert |
| Kompatibilität | – | **CSM** emuliert BIOS für alte Systeme (unterläuft Secure Boot) |

### Härtungsmaßnahmen

| Maßnahme | Wirkung |
|---|---|
| **Setup-/Administrator-Passwort** | verhindert, dass jemand Bootreihenfolge, Secure Boot oder TPM abschaltet |
| **Boot von USB/DVD/Netz sperren**, feste Bootreihenfolge | kein Start eines Live-Systems zum Umgehen von Passwörtern oder Abziehen von Daten |
| **Secure Boot aktivieren** | nur signierte Bootloader starten → Schutz vor **Bootkits/Rootkits** |
| **CSM/Legacy-Boot deaktivieren** | verhindert das Umgehen von Secure Boot |
| **TPM 2.0 aktivieren** | sicherer Schlüsselspeicher für BitLocker, Measured Boot, Windows 11 |
| **Firmware-Updates** einspielen | schließt Lücken in der Firmware (z. B. UEFI-Schwachstellen), nur signierte Updates |
| **Ungenutzte Schnittstellen abschalten** | z. B. USB, serielle Ports, WLAN/Bluetooth am Server |
| **Gehäuseöffnungs-Erkennung** (Chassis Intrusion) | meldet physische Manipulation |
| **BMC/iDRAC/iLO absichern** | eigenes Management-Netz, Standardpasswort ändern |

### Secure Boot in Kürze

1. Die Firmware enthält Datenbanken mit vertrauenswürdigen Schlüsseln (**PK**, **KEK**, **db**) und eine Sperrliste (**dbx**).
2. Vor dem Start prüft sie die Signatur des Bootloaders gegen `db`.
3. Linux-Distributionen starten über einen von Microsoft signierten **shim**, der dann den signierten GRUB und Kernel prüft.
4. Eigene Schlüssel (Unternehmens-CA) lassen sich hinterlegen.

### Nützliche Werkzeuge

- Windows: `msinfo32` (BIOS-Modus UEFI oder Legacy, Secure-Boot-Status), `Confirm-SecureBootUEFI` (PowerShell), `tpm.msc`.
- Linux: `efibootmgr` (Booteinträge), `mokutil --sb-state` (Secure-Boot-Status), `dmidecode` (Firmware-Infos).

## Prüfungsfrage

**Frage:** Nennen und erläutern Sie zwei Einstellungen im UEFI/BIOS, mit denen ein Server gehärtet wird.

**Musterantwort:** Erstens wird ein Administrator-Passwort für das UEFI-Setup gesetzt, und der Boot von externen Medien wird deaktiviert, sodass nur noch von der Systemplatte gestartet wird. Damit kann niemand mit physischem Zugang ein Live-System von USB booten, um Passwörter zu umgehen oder Daten zu kopieren, und die Einstellungen lassen sich nicht unbefugt ändern. Zweitens wird Secure Boot aktiviert (bei deaktiviertem CSM). Die Firmware startet dann nur Bootloader und Kernel mit gültiger Signatur, was manipulierte Bootloader und Bootkits verhindert.

## Falle

- Secure Boot mit Verschlüsselung verwechseln. Secure Boot prüft Signaturen, verschlüsselt aber nichts (das macht BitLocker/LUKS mit TPM).
- Härtung nur mit „Passwort setzen“ beantworten, ohne zu sagen, **wogegen** es schützt.
- CSM aktiviert lassen. Dann ist Secure Boot wirkungslos bzw. nicht verfügbar.
- Firmware-Updates vergessen. Auch die Firmware hat Sicherheitslücken.
- Das BIOS-Passwort für einen Ersatz der Festplattenverschlüsselung halten. Wer die Platte ausbaut, liest sie in einem anderen Rechner.

## Merksatz

**„Passwort aufs Setup, Riegel vor USB, Secure Boot an, CSM aus, Firmware aktuell.“**

## Verwandt

- [BIOS/UEFI-Kernel](bios-uefi-kernel.md) – Bootablauf, Kernel, Partitionen, GRUB
- [BitLocker](bitlocker.md) – TPM und Festplattenverschlüsselung
- [OS-Härtung](os-haertung.md) – Härtung des Betriebssystems
- [Dateisysteme](../../Komponenten/Storage/dateisysteme.md) – MBR vs. GPT
- [Server-Hardware](../../Komponenten/server-hardware.md) – BMC/Fernwartung
- [OS-Sicherheitsmerkmale](../../nice-to-know/OS-sicherheitsmerkmale.md)
- [Lektion L28 GA1 Praxisthemen](../../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md)
