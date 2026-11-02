# Linux-Grundlagen und Befehle

## In 30 Sekunden

- Alles hängt unter **einem** Wurzelverzeichnis `/`. Laufwerksbuchstaben gibt es nicht, Datenträger werden **eingehängt** (`mount`).
- Admin-Konto ist **root** (UID 0). Im Alltag arbeitet man als normaler Benutzer und nutzt `sudo` für einzelne Befehle (Least Privilege).
- Rechte: `rwx` für Besitzer, Gruppe und andere → [chmod](../chmod.md).
- Prozesse beenden: `kill <PID>` sendet **SIGTERM** (15, sauber), `kill -9 <PID>` sendet **SIGKILL** (sofort, ohne Aufräumen).
- Dienste steuert **systemd** (`systemctl`), Logs zeigt `journalctl`, Pakete installiert `apt` bzw. `dnf`.
- Geprüft: chmod/ls -l 2023W, GRUB 2026S, Prozess beenden 2026S, Dateiausgaben lesen (Analyse-Aufgaben).

## Kernwissen

### Verzeichnisstruktur (FHS)

| Pfad | Inhalt |
|---|---|
| `/` | Wurzel des gesamten Dateisystems |
| `/bin`, `/usr/bin` | Programme für alle Benutzer |
| `/sbin`, `/usr/sbin` | Systemprogramme für Administration |
| `/etc` | **Konfigurationsdateien** (z. B. `/etc/passwd`, `/etc/fstab`, `/etc/ssh/sshd_config`) |
| `/home/<name>` | Benutzerverzeichnisse |
| `/root` | Home-Verzeichnis von root (nicht dasselbe wie `/`) |
| `/var` | veränderliche Daten, z. B. `/var/log` (Logs), `/var/www` |
| `/tmp` | temporäre Dateien |
| `/boot` | Kernel, initramfs, GRUB |
| `/dev` | Gerätedateien (`/dev/sda1`, `/dev/nvme0n1p1`) |
| `/proc`, `/sys` | virtuelle Dateisysteme mit Kernel- und Prozessinfos |
| `/mnt`, `/media` | Einhängepunkte für Datenträger |

### Wichtige Befehle

| Zweck | Befehl | Beispiel |
|---|---|---|
| Navigation | `pwd`, `cd`, `ls -la` | `ls -la /etc` zeigt Rechte, Besitzer, versteckte Dateien |
| Dateien | `cp`, `mv`, `rm`, `mkdir`, `touch` | `cp -r /daten /backup/` |
| Inhalt | `cat`, `less`, `head`, `tail -f` | `tail -f /var/log/syslog` (live mitlesen) |
| Suchen | `find`, `grep` | `grep -i error /var/log/syslog` |
| Rechte | `chmod`, `chown`, `chgrp` | `chown hmeyer:alster datei.txt` |
| Benutzer | `useradd`, `passwd`, `usermod -aG`, `id`, `whoami` | `usermod -aG sudo anna` |
| Prozesse | `ps aux`, `top`, `kill`, `pkill` | `ps aux \| grep nginx` |
| Dienste | `systemctl start/stop/restart/status/enable` | `systemctl status ssh` |
| Logs | `journalctl -u <dienst>`, `dmesg` | `journalctl -u nginx --since today` |
| Speicher | `df -h`, `du -sh`, `free -h`, `lsblk` | `df -h` zeigt freien Platz je Dateisystem |
| Einhängen | `mount`, `umount`, `/etc/fstab` | `mount /dev/sdb1 /mnt/usb` |
| Netzwerk | `ip a`, `ip r`, `ping`, `ss -tulpn`, `dig`, `traceroute` | `ss -tulpn` zeigt offene Ports |
| Pakete | `apt update && apt upgrade` (Debian/Ubuntu), `dnf` (RHEL/Fedora) | `apt install openssh-server` |
| Archiv | `tar -czf`, `tar -xzf` | `tar -czf backup.tar.gz /etc` |
| Fernzugriff | `ssh`, `scp`, `rsync` | `rsync -av /daten/ backup:/daten/` |

### Pipes und Umleitungen

| Zeichen | Wirkung | Beispiel |
|---|---|---|
| `\|` | Ausgabe als Eingabe des nächsten Befehls | `ps aux \| grep java` |
| `>` | Ausgabe in Datei (überschreiben) | `df -h > platz.txt` |
| `>>` | Ausgabe an Datei anhängen | `date >> log.txt` |
| `2>` | Fehlerausgabe umleiten | `befehl 2> fehler.log` |
| `&&` | nächster Befehl nur bei Erfolg | `apt update && apt upgrade -y` |

### Signale zum Beenden

| Signal | Nummer | Wirkung |
|---|---|---|
| SIGTERM | 15 | Standard bei `kill`: Programm darf aufräumen und sauber beenden |
| SIGKILL | 9 | sofortiger Abbruch durch den Kernel, ungespeicherte Daten gehen verloren |
| SIGHUP | 1 | oft „Konfiguration neu laden“ |

Windows-Gegenstück: `tasklist` / `taskkill /PID 1234 /F` oder Task-Manager.

### Benutzer und Gruppen

- `/etc/passwd`: Benutzerkonten (ohne Passwort), `/etc/shadow`: Passwort-Hashes (nur root lesbar), `/etc/group`: Gruppen.
- Jeder Benutzer hat eine UID, jede Gruppe eine GID. root hat UID 0.

## Prüfungsfrage

**Frage:** Ein Überwachungsprogramm auf einem Linux-Server reagiert nicht mehr. Beschreiben Sie, wie Sie es im laufenden Betrieb beenden.

**Musterantwort:** Zuerst wird die Prozess-ID ermittelt, z. B. mit `ps aux | grep moncpu` oder `pgrep moncpu`. Dann wird der Prozess mit `kill <PID>` beendet. Das sendet SIGTERM, und das Programm kann sich sauber beenden. Reagiert es darauf nicht, erzwingt `kill -9 <PID>` (SIGKILL) den sofortigen Abbruch. Ungespeicherte Daten gehen dabei verloren. Läuft das Programm als Dienst, nutzt man besser `systemctl restart <dienst>`.

## Falle

- `/` (Wurzel) und `/root` (Home von root) verwechseln.
- Dauerhaft als root arbeiten. Dafür gibt es `sudo` mit Protokollierung.
- Gleich `kill -9` verwenden, ohne erst SIGTERM zu versuchen. Das riskiert Datenverlust.
- Linux ist **case-sensitive**: `Datei.txt` und `datei.txt` sind zwei Dateien.
- `rm -rf` ohne Prüfung des Pfads ausführen. Es gibt keinen Papierkorb.

## Merksatz

**„Alles ist eine Datei, alles hängt an `/`, Konfiguration liegt in `/etc`, Logs liegen in `/var/log`.“**

## Verwandt

- [chmod](../chmod.md) – Linux-Dateirechte
- [BIOS/UEFI-Kernel](bios-uefi-kernel.md) – Bootprozess, GRUB, Partitionen
- [Skripte: PowerShell, Bash, CMD](skripte-powershell-bash-cmd.md) – Automatisierung, cron
- [Dateisysteme](../../Komponenten/Storage/dateisysteme.md) – ext4, XFS
- [OS-Härtung](os-haertung.md)
- [Monitoring](monitoring.md)
- [Lektion L28 GA1 Praxisthemen](../../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md)
