# Betriebssystem-Sicherheitsmerkmale und Härtung

> Abgrenzung: Dieses Blatt sammelt **eingebaute Sicherheitsfunktionen** der Betriebssysteme und eine Härtungs-Checkliste für Prüfungsantworten. Das ausführliche Härtungsblatt ist [OS-Härtung](../FiSi-Administrator/Server/os-haertung.md).

## In 30 Sekunden

- Prüfung 2022S GA1: **Sicherheitsmerkmale eines Betriebssystems** nennen – z. B. Benutzer- und Rechteverwaltung (UAC/sudo), Festplattenverschlüsselung (BitLocker/LUKS), Virenschutz, Firewall, Updates, Protokollierung.
- Prüfung 2022S, 2023S, 2025S, 2026S: **Härtungsmaßnahmen** – unnötige Dienste abschalten, Standardkonten umbenennen/deaktivieren, Secure Boot, BIOS/UEFI-Passwort, Least Privilege, Updates.
- Grundidee der Härtung: **Angriffsfläche verkleinern** – was nicht läuft, kann nicht angegriffen werden.
- Härtung ist ein **Prozess**: nach Updates und Änderungen erneut prüfen.

## Kernwissen

**Eingebaute Sicherheitsmerkmale**

| Merkmal | Windows | Linux | macOS | Schützt vor |
|---|---|---|---|---|
| Rechteverwaltung | UAC, NTFS-Rechte, AD-Gruppen | Benutzer/Gruppen, `chmod`/`chown`, `sudo` | Benutzerkonten, Admin-Bestätigung | Systemänderungen durch normale Benutzer/Malware |
| Datenträgerverschlüsselung | **BitLocker** (mit TPM) | **LUKS** | FileVault | Datendiebstahl bei Verlust/Diebstahl (Data at Rest) |
| Zwangszugriffskontrolle | – (AppLocker/WDAC als Anwendungssteuerung) | **SELinux, AppArmor** | SIP (System Integrity Protection) | kompromittierte Dienste greifen nicht über ihre Grenzen |
| Malwareschutz | Microsoft Defender | z. B. ClamAV, EDR-Agenten | XProtect, Gatekeeper | Schadsoftware |
| Firewall | Windows Defender Firewall | nftables/iptables, ufw, firewalld | Application Firewall | unerwünschte Verbindungen |
| Updates | Windows Update/WSUS | Paketverwaltung (apt, dnf) | Softwareupdate | bekannte Lücken |
| Protokollierung | Ereignisanzeige, Überwachungsrichtlinien | journald, auditd, syslog | Unified Logging | Nachvollziehbarkeit, Angriffserkennung |
| Startschutz | **Secure Boot**, Measured Boot mit TPM | Secure Boot | Secure Boot (Apple-Chips) | manipulierte Bootloader/Rootkits |
| Netzwerkfreigaben | SMB-Signing, SMBv1 abschalten | – | – | Man-in-the-Middle auf Freigaben |

**Härtungs-Checkliste (Prüfung 2023S, gegliedert)**

| Bereich | Maßnahmen |
|---|---|
| Hardware/Firmware | **UEFI/BIOS-Passwort**, **Secure Boot** aktivieren, Bootreihenfolge festlegen (kein Start von USB), ungenutzte Schnittstellen deaktivieren, TPM aktivieren |
| Installation/Konfiguration | nur benötigte Rollen und Software installieren, Updates einspielen und automatisieren, getrennte System- und Datenpartitionen, Servernamen ohne Rückschluss auf Funktion |
| Dienste | **nicht benötigte Dienste und Ports abschalten**, unsichere Protokolle ersetzen (Telnet → SSH, FTP → SFTP, SMBv1 aus), Autostart prüfen, Skriptausführung einschränken |
| Konten und Anmeldung | **Standardkonten umbenennen oder deaktivieren**, Standardpasswörter ändern, starke Passwortrichtlinie, **MFA**, Kontosperre, Anmeldezeiten einschränken, Fernzugriff nur über VPN bzw. definierte IPs |
| Administration | **Least Privilege**, getrennte Admin-Konten, Rechte auf Verzeichnisse und Systemeinstellungen eng vergeben, **Protokollierung** aktivieren und zentral sammeln |
| Netzwerk | Host-Firewall mit Whitelisting (alles verboten, was nicht erlaubt ist), Management-Zugriff nur aus dem Admin-Netz |

## Prüfungsfrage

**Frage:** Nennen Sie vier Maßnahmen, um einen neu installierten Windows-Server zu härten, und begründen Sie zwei davon.

**Musterantwort:** Erstens werden nicht benötigte Dienste und Rollen deaktiviert, weil jeder laufende Dienst eine mögliche Angriffsfläche bietet. Zweitens wird das Standardkonto Administrator umbenannt bzw. deaktiviert und durch persönliche Admin-Konten mit starken Passwörtern und MFA ersetzt, weil Angreifer den Namen kennen und gezielt Passwörter dafür ausprobieren. Drittens werden alle aktuellen Updates eingespielt und regelmäßig über WSUS verteilt. Viertens werden Secure Boot aktiviert und ein UEFI-Passwort gesetzt, damit weder manipulierte Bootloader starten noch Unbefugte die Bootreihenfolge ändern können.

## Falle

- „Antivirus installieren“ als einzige Härtungsmaßnahme → Härtung heißt vor allem **reduzieren und einschränken**.
- Secure Boot mit BIOS-Passwort verwechseln: Secure Boot prüft **Signaturen** des Bootloaders, das Passwort schützt die **Einstellungen**.
- Standardkonto nur umbenennen, aber mit Standardpasswort lassen.

## Merksatz

**„Was nicht läuft, kann nicht angegriffen werden – abschalten, umbenennen, einschränken, aktualisieren.“**

## Verwandt

- [OS-Härtung](../FiSi-Administrator/Server/os-haertung.md)
- [BIOS und UEFI](../FiSi-Administrator/Server/bios-uefi.md)
- [BitLocker](../FiSi-Administrator/Server/bitlocker.md)
- [Least Privilege](../FiSi-Administrator/Sicherheit/least-privilege.md)
- [Patchmanagement](../FiSi-Administrator/Netzwerk-Administration/patchmanagement-tu-su-pu.md)
- [Malware und Ransomware](../FiSi-Administrator/Sicherheit/hacking/malware-ransomware.md)
- [Zugriffskontrollmodelle](access-control.md) – MAC mit SELinux
- [Lektion 28 GA1-Praxisthemen (OS-Härtung)](../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md)
