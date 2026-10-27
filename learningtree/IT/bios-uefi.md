# BIOS-UEFI

# UEFI/BIOS – Härtung & Kontrolle (IHK 2026)

## Wichtigkeit
- **Boot-Root-of-Trust**: Erster Code, der läuft; Basis für Sicherheit.
- **Secure Boot**: Nur signierte OS-Loader starten (Malware-Prävention).
- **Hardware-Initialisierung**: CPU, RAM, Peripherie.
- **Schnittstelle**: Zugriff auf System-Konfiguration (z. B. Boot-Reihenfolge, Passwörter).

## Kontrolle (Wie prüft man?)
1. **UEFI-Setup aufrufen**:
   - Tasten: F2, Del, F10, Esc (geräteabhängig).
   - Menüpfade: Advanced → Security → Secure Boot, Boot → Boot Order.
2. **UEFI-Tools**:
   - `efibootmgr` (Linux): Boot-Einträge verwalten.
   - `msinfo32` (Windows): BIOS-Modus prüfen (UEFI vs Legacy).
   - `dmidecode` (Linux): BIOS-Datenbank abfragen.

## Härtungen (Wichtig für DSGVO/Sicherheit)
| Maßnahme | Beschreibung | IHK-Relevanz |
|----------|--------------|--------------|
| **Secure Boot aktivieren** | Nur signierte Images starten | Schutz vor Bootkiten |
| **BIOS-Passwort** | Zugriff auf Setup sperren | Zugriffsschutz (DSGVO) |
| **TPM 2.0 aktivieren** | Verschlüsselungsspeicher für BitLocker/Full-Disk-Encryption | DSGVO-konforme Datenverschlüsselung |
| **UEFI-Update auf neueste Version** | Patches für Sicherheitslücken | Schwachstellen-Management |
| **Legacy-Optionen deaktivieren** (CSM, Legacy-Boot) | Verhindert Umgehung von Secure Boot | Boot-Sicherheit |
| **Boot-Reihenfolge festlegen** | Kein Boot von externen Medien | Schutz vor unbefugtem Zugriff |

## Neue Features (2026)
- **Network Boot (PXE)**: Booten per Netzwerk (Virtualisierung, Fleet-Management).
- **UEFI-Apps**: Anwendungen direkt im BIOS (z. B. Hardware-Diagnose, Recovery).
- **Secure Boot erweitert**: Support für Custom Keys (z. B. Unternehmens-CA).
- **Firmware-Updates per UEFI-App**: Kein DOS-Flash nötig.

## GRUB & Boot-Prozess
- **GRUB 2**: Lädt sich über UEFI-Boot-Manager (nicht Legacy-MBR).
- **Konfiguration**: `/boot/grub/grub.cfg` – Menü-Einträge, Timeout, Default-OS.
- **Secure Boot + GRUB**: GRUB muss signiert sein (z. B. `shim`-Loader für Linux).
- **Boot-Reihenfolge im UEFI**:
  1. UEFI-Boot-Manager listet `.efi`-Dateien auf.
  2. GRUB (bzw. `shim`) wird geladen.
  3. GRUB lädt Kernel → System startet.

## IHK-Prüfung: 3-Linien-Format
1. **UEFI vs. BIOS**: UEFI ist modern, supports Secure Boot, 64-bit, Graphik-Oberfläche; BIOS ist legacy, 16-bit, textbasiert.
2. **Härtung**: Secure Boot, BIOS-Passwort, TPM, Updates – schützen vor Bootkiten und unbefugtem Zugriff (DSGVO).
3. **GRUB**: Lädt sich nur über UEFI-Boot-Entry; muss für Secure Boot signiert sein (z. B. `shim`).

## Fazit
- **UEFI ist Pflicht** für moderne Systeme (Sicherheit, DSGVO).
- **Härtung**: Konfiguration im Setup sichern, Updates durchführen, Legacy-Boot deaktivieren.
- **GRUB**: Nur mit Secure Boot kompatibel, wenn signiert (z. B. `shim`).
