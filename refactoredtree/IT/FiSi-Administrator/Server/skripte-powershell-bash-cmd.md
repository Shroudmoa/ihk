# Skripte und Kommandozeile (CMD, PowerShell, Bash)

## In 30 Sekunden

- Admin-Aufgaben werden per Skript **automatisiert** und per **Aufgabenplanung** zeitgesteuert: Windows `SCHTASKS`, Linux `cron`.
- **Platzhalter:** `*` steht für beliebig viele Zeichen, `?` für genau ein Zeichen.
- **PATH:** Die Shell findet Programme nur im aktuellen Verzeichnis (CMD) bzw. in den Ordnern der `PATH`-Variable. Sonst den absoluten Pfad angeben.
- **PowerShell** reicht **Objekte** durch die Pipeline (`Get-Service | Where-Object Status -eq Running`), CMD und Bash reichen **Text** weiter.
- Geprüft: `copy` mit Wildcards und PATH-Fehler 2022S, `SCHTASKS` 2023S, Prozess beenden 2026S. Programmieraufgaben (Arrays, Schleifen) kommen in jeder GA1 vor.

## Kernwissen

### Wichtige Befehle im Vergleich

| Aufgabe | CMD (Windows) | PowerShell | Bash (Linux) |
|---|---|---|---|
| Verzeichnis anzeigen | `dir` | `Get-ChildItem` (`ls`) | `ls -la` |
| Kopieren | `copy`, `xcopy`, `robocopy` | `Copy-Item` | `cp`, `rsync` |
| Verschieben | `move` | `Move-Item` | `mv` |
| Löschen | `del`, `rmdir /s` | `Remove-Item` | `rm`, `rm -r` |
| Prozesse | `tasklist`, `taskkill /PID n /F` | `Get-Process`, `Stop-Process` | `ps aux`, `kill` |
| Dienste | `sc query`, `net start` | `Get-Service`, `Restart-Service` | `systemctl` |
| Netzwerk | `ipconfig /all`, `ping`, `tracert`, `nslookup` | `Get-NetIPAddress`, `Test-NetConnection` | `ip a`, `ping`, `traceroute`, `dig` |
| Zeitsteuerung | `schtasks` | `Register-ScheduledTask` | `crontab -e` |
| Hilfe | `befehl /?` | `Get-Help befehl` | `man befehl`, `befehl --help` |

### Platzhalter (2022S)

Aufgabe: Alle Logdateien aus `C:\system\log` mit einem Datum vom 01.–09.04.2022 im Namen nach `X:\debug` kopieren.

```bat
copy C:\system\log\*2022040?.log X:\debug
```

- `*` vor dem Datum: beliebiger Namensanfang.
- `2022040?`: genau ein beliebiges Zeichen für den Tag 1–9.
- `.log`: nur Logdateien.

### PATH-Problem (2022S)

Der Aufruf `qbackup` schlägt fehl, obwohl `C:\Backup\qbackup.exe` existiert. Die Shell sucht nur im aktuellen Verzeichnis und in den PATH-Ordnern.
Lösung: absoluter Pfad `C:\Backup\qbackup.exe`, vorher `cd C:\Backup` oder `C:\Backup` zur PATH-Variable hinzufügen.

### Aufgabenplanung

**Windows (2023S):**

```bat
SCHTASKS /Create /TN CPUMON /TR "D:\util\MONCPU.EXE" /SC DAILY /ST 16:10
```

`/TN` = Name der Aufgabe, `/TR` = auszuführendes Programm, `/SC` = Rhythmus (MINUTE, HOURLY, DAILY, WEEKLY), `/ST` = Startzeit.

**Linux (cron):** `Minute Stunde Tag Monat Wochentag Befehl`

```
30 22 * * 1-5  /usr/local/bin/backup.sh      # Mo–Fr um 22:30
0  *  * * *    /usr/local/bin/check_disk.sh  # jede volle Stunde
```

### Beispielskript Bash: Plattenplatz prüfen

```bash
#!/bin/bash
# Warnt, wenn die Root-Partition zu mehr als 80 % belegt ist
GRENZE=80
BELEGT=$(df / --output=pcent | tail -1 | tr -dc '0-9')
if [ "$BELEGT" -gt "$GRENZE" ]; then
    echo "$(date): Warnung – / ist zu ${BELEGT}% belegt" >> /var/log/plattencheck.log
fi
```

### Beispielskript PowerShell: gestoppte automatische Dienste

```powershell
# Listet alle Dienste mit Starttyp Automatisch, die nicht laufen, und startet sie neu
Get-Service |
    Where-Object { $_.StartType -eq 'Automatic' -and $_.Status -ne 'Running' } |
    ForEach-Object {
        Write-Output "Starte $($_.Name)"
        Start-Service -Name $_.Name
    }
```

Ausführungsrichtlinie: `Set-ExecutionPolicy RemoteSigned` erlaubt lokale Skripte und verlangt für heruntergeladene Skripte eine Signatur.

## Prüfungsfrage

**Frage:** Das Programm `D:\util\MONCPU.EXE` soll täglich um 16:10 Uhr als Aufgabe „CPUMON“ gestartet werden. Geben Sie den Befehl an und erläutern Sie die Parameter.

**Musterantwort:** `SCHTASKS /Create /TN CPUMON /TR "D:\util\MONCPU.EXE" /SC DAILY /ST 16:10`. `/Create` legt eine neue Aufgabe an. `/TN` gibt den Aufgabennamen CPUMON an. `/TR` nennt das auszuführende Programm mit vollständigem Pfad. `/SC DAILY` legt die tägliche Ausführung fest, `/ST 16:10` die Startzeit im 24-Stunden-Format.

## Falle

- `*` und `?` verwechseln. `*2022040*` würde auch `20220401x` oder `202204011` treffen.
- Programmpfad ohne Anführungszeichen, wenn er Leerzeichen enthält (`"C:\Program Files\..."`).
- Linux-Pfade mit Backslash schreiben oder Groß-/Kleinschreibung ignorieren.
- In cron den Wochentag falsch zählen: 0 und 7 = Sonntag, 1 = Montag.
- Skripte mit Klartext-Passwörtern. Besser Dienstkonten mit minimalen Rechten, Zugangsdaten aus einem Tresor oder Credential Store.

## Merksatz

**„Stern = beliebig viel, Fragezeichen = genau eins. Nicht im PATH = voller Pfad.“**

## Verwandt

- [Linux-Grundlagen](linux-grundlagen.md) – Befehle, Pipes, Signale
- [Monitoring](monitoring.md) – Überwachung, die Skripte oft übernehmen
- [Backup-Strategien](backup/backup-strategien.md) – Sicherungsjobs zeitgesteuert
- [chmod](../chmod.md) – Skript ausführbar machen (`chmod +x`)
- [Pseudocode Grundlagen](../../UML-Code-SQL/Pseudocode/01_Grundlagen_Syntax.md) – Programmieraufgaben der GA1
- [Lektion L28 GA1 Praxisthemen](../../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md)
