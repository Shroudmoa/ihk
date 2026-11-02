# chmod

# Linux Dateiberechtigungen (chmod) – IHK-Prüfungsrelevant

## 1. Die drei Rechte-Typen (rwx)

Jede Datei/Ordner hat drei Basisrechte, die auf **drei Ebenen** (User, Group, Others) vergeben werden.

| Recht | Symbol | Oktalwert | Bedeutung für Dateien | Bedeutung für Verzeichnisse (Ordner) |
|---|---|---|---|---|
| **Read** | r | 4 | Datei lesen (cat, less) | Inhalt auflisten (ls) |
| **Write** | w | 2 | Datei ändern/löschen | Dateien erstellen/löschen |
| **Execute** | x | 1 | Datei ausführen (Skript, Programm) | Verzeichnis betreten (cd) |

**Merksatz:** `r=4, w=2, x=1`. Summen bilden: `rwx = 7`, `rw- = 6`, `r-x = 5`, `--x = 1`.

---

## 2. Die drei Ebenen (UGO)

| Ebene | Kürzel | Wer ist das? |
|---|---|---|
| **User** | u | Der Eigentümer der Datei |
| **Group** | g | Die zugewiesene Gruppe |
| **Others** | o | Alle anderen (Rest der Welt) |

**Beispiel: `-rwxr-x---`**
- `-` = Datei (bei `d` wäre es ein Verzeichnis)
- `rwx` = User: Lesen, Schreiben, Ausführen
- `r-x` = Group: Lesen, Ausführen (kein Schreiben)
- `---` = Others: Keine Rechte

**Oktal-Schreibweise:** `rwxr-x---` = `750` (7=rwx, 5=r-x, 0=---)

---

## 3. chmod – Befehle und Syntax

**Symbolische Schreibweise:**
```bash
chmod u+x datei.sh        # User bekommt Ausführrecht
chmod g-w datei.txt       # Group verliert Schreibrecht
chmod o=r datei.txt       # Others darf nur lesen
chmod a+r datei.txt       # Alle (a = all) bekommen Leserecht
