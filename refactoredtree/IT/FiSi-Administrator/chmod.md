# chmod – Linux-Dateirechte

## In 30 Sekunden

- Jede Datei hat einen **Besitzer (u)**, eine **Gruppe (g)** und **andere (o)**, jeweils mit den Rechten **r = 4**, **w = 2**, **x = 1**.
- Oktal: Summe pro Ebene. `rwx` = 7, `rw-` = 6, `r-x` = 5, `r--` = 4. `chmod 750` → `rwxr-x---`.
- `ls -l` zeigt die Rechte: `-rw-r--r-- 1 hmeyer alster …` = Datei, Besitzer hmeyer darf lesen und schreiben, Gruppe alster und andere dürfen nur lesen (644).
- Bei **Verzeichnissen** heißt `x` „betreten“, `r` „auflisten“, `w` „Dateien anlegen/löschen“.
- Besitz ändern mit `chown`, Gruppe mit `chgrp`. Standardrechte neuer Dateien regelt die `umask`.
- Geprüft: Ursache „Keine Berechtigung“ und Wirkung von `chmod 664` 2023W.

## Kernwissen

### Die Rechte

| Recht | Symbol | Wert | bei Dateien | bei Verzeichnissen |
|---|---|---:|---|---|
| Read | r | 4 | Inhalt lesen (`cat`, `less`) | Inhalt auflisten (`ls`) |
| Write | w | 2 | Inhalt ändern | Dateien anlegen, umbenennen, **löschen** |
| Execute | x | 1 | als Programm/Skript ausführen | betreten (`cd`), auf Dateien darin zugreifen |

Das Löschen einer Datei hängt am **w-Recht des Verzeichnisses**, nicht an dem der Datei.

### `ls -l` lesen

```
-rwxr-x--- 1 anna buchhaltung 2048 Mai 10 09:12 bericht.sh
│└┬┘└┬┘└┬┘   │    │
│ │  │  │    │    └─ Gruppe
│ │  │  │    └────── Besitzer
│ │  │  └─ others:  --- = 0
│ │  └──── group:   r-x = 5
│ └─────── user:    rwx = 7
└───────── Typ: - Datei, d Verzeichnis, l Link
```

→ Oktal **750**.

### Oktal-Umrechnung

| Oktal | Rechte | typische Verwendung |
|---:|---|---|
| 755 | `rwxr-xr-x` | Programme, Skripte, öffentliche Verzeichnisse |
| 750 | `rwxr-x---` | Skripte nur für Besitzer und Gruppe |
| 700 | `rwx------` | privates Verzeichnis (`~/.ssh`) |
| 644 | `rw-r--r--` | normale Dateien, Webseiten |
| 664 | `rw-rw-r--` | gemeinsame Gruppendateien |
| 640 | `rw-r-----` | Konfiguration mit sensiblen Daten |
| 600 | `rw-------` | private Schlüssel (`id_rsa`) |
| 777 | `rwxrwxrwx` | alle dürfen alles → **vermeiden** |

### Befehle

```bash
chmod 664 datei.txt          # oktal: rw-rw-r--
chmod u+x skript.sh          # Besitzer bekommt Ausführrecht
chmod g-w datei.txt          # Gruppe verliert Schreibrecht
chmod o=r datei.txt          # andere dürfen genau lesen
chmod a+r datei.txt          # alle (a = all) bekommen Leserecht
chmod -R 750 /projekte       # rekursiv für Verzeichnis und Inhalt
chown hmeyer:alster datei.txt   # Besitzer und Gruppe setzen
chgrp alster datei.txt          # nur die Gruppe ändern
```

### umask

Die `umask` zieht Rechte von den Standardwerten ab (Dateien 666, Verzeichnisse 777).
`umask 022` → neue Dateien 644, neue Verzeichnisse 755. `umask 007` → 660 bzw. 770.

### Sonderrechte

| Recht | Wert | Wirkung | Beispiel |
|---|---:|---|---|
| **SUID** | 4000 | Programm läuft mit den Rechten des **Besitzers** | `/usr/bin/passwd` (`-rwsr-xr-x`) |
| **SGID** | 2000 | bei Verzeichnissen: neue Dateien erben die **Gruppe** des Verzeichnisses | Projektordner einer Abteilung |
| **Sticky Bit** | 1000 | im Verzeichnis darf nur der Besitzer seine Dateien löschen | `/tmp` (`drwxrwxrwt`) |

### Rechenweg: Wer darf was?

Datei `-rw-r--r-- hmeyer alster 2023-01-10_Aenderungen.txt`, Benutzer `kschulz` ist Mitglied der Gruppe `alster` und will schreiben.
1. Ist kschulz der Besitzer? Nein.
2. Ist kschulz in der Gruppe alster? Ja → es gelten die **Gruppenrechte** `r--` → nur lesen.
3. Ergebnis: „Keine Berechtigung“. Lösung: `chmod g+w` bzw. `chmod 664`.

Linux prüft **nur die erste passende Ebene** (Besitzer → Gruppe → andere). Rechte werden nicht addiert.

## Prüfungsfrage

**Frage:** Die Datei `2023-01-10_Aenderungen.txt` hat die Rechte `-rw-r--r--`, Besitzer `hmeyer`, Gruppe `alster`. Ein Mitglied der Gruppe alster erhält beim Speichern „Keine Berechtigung“. Nennen Sie die Ursache und erläutern Sie die Auswirkung von `chmod 664`.

**Musterantwort:** Die Rechte entsprechen 644: Der Besitzer hmeyer darf lesen und schreiben, die Gruppe alster und alle anderen dürfen nur lesen. Das Gruppenmitglied hat daher kein Schreibrecht. `chmod 664` setzt die Rechte auf `rw-rw-r--`: Besitzer lesen und schreiben (4+2 = 6), Gruppe lesen und schreiben (6), andere nur lesen (4). Danach können alle Mitglieder der Gruppe alster die Datei bearbeiten, andere Benutzer weiterhin nur lesen.

## Falle

- Die Reihenfolge vertauschen: immer **u g o** (Besitzer, Gruppe, andere).
- r = 1 und x = 4 verwechseln. Richtig ist **r = 4, w = 2, x = 1**.
- Rechte der Ebenen addieren. Der Besitzer bekommt nur die Besitzerrechte, auch wenn „andere“ mehr dürften.
- `chmod 777` als Lösung vorschlagen. Das hebelt jeden Schutz aus, besser gezielt `g+w`.
- Bei Verzeichnissen das `x` vergessen: Ohne `x` kommt man nicht hinein, selbst mit `r`.
- `chmod` und `chown` verwechseln: chmod ändert Rechte, chown den Besitzer.

## Merksatz

**„Vier lesen, zwei schreiben, eins ausführen. User, Group, Others, in dieser Reihenfolge.“**

## Verwandt

- [Linux-Grundlagen](Server/linux-grundlagen.md) – Befehle, Benutzer und Gruppen
- [NTFS- und Freigabeberechtigungen](Server/ntfs-freigabeberechtigungen.md) – das Windows-Gegenstück
- [Least Privilege](Sicherheit/least-privilege.md)
- [RBAC](Netzwerk-Administration/rbac.md)
- [Dateisysteme](../Komponenten/Storage/dateisysteme.md)
- [Lektion L28 GA1 Praxisthemen](../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md)
