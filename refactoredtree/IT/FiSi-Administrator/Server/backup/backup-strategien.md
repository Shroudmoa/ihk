# Backup-Strategien

## In 30 Sekunden

- **Voll** sichert alles, **differenziell** alles seit dem letzten **Voll**-Backup, **inkrementell** alles seit dem letzten **beliebigen** Backup.
- Restore: Voll = 1 Medium · Differenziell = Voll + **letztes** Diff · Inkrementell = Voll + **alle** Inkremente lückenlos in Reihenfolge.
- **Archivbit:** Voll und Inkrementell setzen es zurück, Differenziell **nicht** – deshalb wächst das Diff jeden Tag.
- **GVS/GFS** (Großvater-Vater-Sohn) rotiert Medien: täglich (Sohn), wöchentlich (Vater), monatlich (Großvater).
- **3-2-1:** 3 Kopien, 2 Medientypen, 1 Kopie außer Haus. **D2D2T** = Disk → Disk → Tape.
- Ein Backup ohne **Restore-Test** ist nur eine Hoffnung. **RAID und Snapshots sind kein Backup.**
- Geprüft u. a. 2022S, 2022W, 2023S, 2023W, 2025S, 2025W – praktisch jedes Jahr.

## Kernwissen

### Die drei Backup-Arten

| Art | Sichert | Archivbit danach | Sicherungsdauer / Platz | Restore benötigt |
|---|---|---|---|---|
| **Vollsicherung** | alle ausgewählten Daten | zurückgesetzt | lang / hoch | nur das Vollbackup |
| **Differenziell** | alle Änderungen seit dem letzten **Vollbackup** | **bleibt gesetzt** | wächst täglich | Voll + **letztes** Diff |
| **Inkrementell** | alle Änderungen seit dem letzten **Backup jeder Art** | zurückgesetzt | kurz / gering | Voll + **alle** Inkremente seit dem Voll |
| *Image/Klon* | ganzer Datenträger 1:1 (Sektoren) | kein Archivbit-Konzept | sehr lang / sehr hoch | Image zurückspielen (Bare-Metal-Restore) |

**Archivbit-Logik (Windows-Attribut „A“):** Wird eine Datei geändert, setzt das Betriebssystem ihr Archivbit. Die Backup-Software liest das Bit, um zu entscheiden, was gesichert wird.

- Voll: sichert alles, setzt alle Bits **zurück**.
- Inkrementell: sichert nur Dateien mit gesetztem Bit, setzt sie **zurück** → nächstes Inkrement sieht nur Neues.
- Differenziell: sichert nur Dateien mit gesetztem Bit, lässt sie **gesetzt** → nächstes Diff sichert dieselben Dateien erneut plus die neuen.

### Rechenweg: Speicherbedarf einer Woche

Annahme: 400 GB Datenbestand, Vollbackup am Sonntag, an jedem Werktag (Mo–Sa) ändern sich **20 GB andere** Daten.

| Tag | Inkrementell | Differenziell | täglich Voll |
|---|---:|---:|---:|
| So (Voll) | 400 GB | 400 GB | 400 GB |
| Mo | 20 GB | 20 GB | 400 GB |
| Di | 20 GB | 40 GB | 400 GB |
| Mi | 20 GB | 60 GB | 400 GB |
| Do | 20 GB | 80 GB | 400 GB |
| Fr | 20 GB | 100 GB | 400 GB |
| Sa | 20 GB | 120 GB | 400 GB |
| **Summe** | **520 GB** | **820 GB** | **2.800 GB** |

- Inkrementell: 400 + 6 · 20 = 520 GB
- Differenziell: 400 + (20 + 40 + 60 + 80 + 100 + 120) = 400 + 420 = 820 GB
- **Kumulieren (2023S):** Sind die täglichen Inkremente gegeben (z. B. 25, 10, 10, 20, 10, 5 GB), ist das Diff des Tages die **laufende Summe**: 25 · 35 · 45 · 65 · 75 · 80 GB. Das gilt, wenn jeden Tag andere Dateien geändert werden (Worst Case). Werden dieselben Dateien mehrmals geändert, ist das Diff kleiner.

### Rechenweg: Dauer und Backup-Fenster

`Dauer = Datenmenge / Durchsatz` – vorher Einheiten angleichen.

Beispiel: 2 TiB über ein Laufwerk mit 300 MB/s.
- 2 TiB = 2 · 1024⁴ Byte = 2.199.023.255.552 Byte
- 2.199.023.255.552 B / 300.000.000 B/s ≈ 7.330 s ≈ **2 h 2 min**
- Passt das in ein Backup-Fenster von 22–4 Uhr (6 h)? Ja. Reicht das Fenster nicht mehr, wechselt man auf inkrementell, Dedup, schnellere Medien oder Snapshot-basierte Sicherung.

### Restore-Kette bestimmen (Prüfungsklassiker)

1. Zeitpunkt des Ausfalls markieren. Backups **nach** dem Ausfall (z. B. Nachtlauf um 23 Uhr bei Ausfall um 18 Uhr) sind wertlos.
2. Letztes Vollbackup **vor** dem Ausfall suchen.
3. Differenziell: nur das **letzte** Diff davor. Inkrementell: **alle** Inkremente zwischen Voll und Ausfall, in zeitlicher Reihenfolge.

**Beispiel 2025W (inkrementell):** Voll B08 (So 07.05.), Inkremente B09–B14 (Mo–Sa), Ausfall So 14.05. mittags, das Vollbackup B15 läuft erst danach. → Restore: **B08, B09, B10, B11, B12, B13, B14**.

**Gemischte Kette (Falle):** Voll am Sonntag, sonst täglich differenziell. Am Mittwoch lief versehentlich ein **inkrementelles** Backup. Ausfall Samstag 10 Uhr (Samstagsbackup lief noch nicht).
- Das Mittwochs-Inkrement hat die Archivbits zurückgesetzt. Do- und Fr-Diff enthalten deshalb nur Änderungen **seit Mittwoch**.
- Restore: Voll (So) → Diff Di (enthält Mo + Di) → Inkrement Mi → Diff Fr (enthält Do + Fr).

### Großvater-Vater-Sohn (GVS / GFS)

| Rolle | Rhythmus | Typische Medien | Aufbewahrung |
|---|---|---|---|
| **Sohn** | täglich (z. B. Mo–Do) | 4 | 1 Woche, dann überschrieben |
| **Vater** | wöchentlich (z. B. Freitag-Voll) | 4 (oder 5) | 1 Monat |
| **Großvater** | monatlich (letzter Vater des Monats) | 12 | 1 Jahr (oder länger, Archiv) |

- Medienbedarf für ein Jahr: 4 + 4 + 12 = **20 Medien** (mit 5 Vätern: 21).
- Prinzip: Ältere Generationen werden seltener überschrieben. Bei nummerierten Wochen-Vollbackups (2025W) ist das **aktuelle** Voll der Sohn, das Voll davor der Vater, das davor der Großvater: B22 = Sohn, B15 = Vater, B08 = Großvater.
- Vorteil: lange Historie mit wenigen Medien. Nachteil: Zwischenstände älterer Wochen sind nicht mehr tagesgenau vorhanden.

### 3-2-1-Regel und Erweiterungen

| Regel | Bedeutung | Beispiel |
|---|---|---|
| **3** Kopien | Original + 2 Sicherungen | Produktiv-SAN, NAS-Backup, Band |
| **2** Medientypen | unterschiedliche Technik gegen gleichartige Fehler | Disk + Tape (oder Disk + Cloud-Objektspeicher) |
| **1** Offsite | räumlich getrennt gegen Brand, Wasser, Diebstahl | Band im Bankschließfach, zweites RZ, Cloud in der EU |
| **+1** offline/immutable | gegen Ransomware, die erreichbare Backups mitverschlüsselt | Band ausgelagert (Air Gap), WORM, S3 Object Lock |
| **0** Fehler | Restore-Tests ohne Fehler | monatlicher Test-Restore |

### D2D2T – Disk-to-Disk-to-Tape

1. **Disk (Produktion) → Disk (Backup-Storage):** schnelle Sicherung im Backup-Fenster, schneller Restore einzelner Dateien (kleines RTO).
2. **Disk → Tape:** Kopie auf LTO-Band, wird ausgelagert (Offsite, offline, günstig pro TB, lange haltbar).

**Warum LTO-Band (2022W, 2025W)?** Geringe Kosten pro TB, 30 Jahre Haltbarkeit bei richtiger Lagerung, kein Strom im Lager, leicht auslagerbar (Air Gap gegen Ransomware). LTFS macht ein Band wie ein Dateisystem nutzbar (Drag & Drop, herstellerunabhängig lesbar). LTO-9: 18 TB nativ.

### Backup-Konzept – was hinein gehört

| Frage | Inhalt |
|---|---|
| Was? | Systeme, Datenbanken, Dateiablagen, Konfigurationen, Postfächer |
| Wie oft? | abgeleitet aus dem **RPO** (z. B. RPO 1 h → stündliche Sicherung oder Log-Backup) |
| Wie schnell zurück? | abgeleitet aus dem **RTO** → Disk statt Band, Snapshot, Replikation |
| Wohin? | Medien nach 3-2-1, Offsite-Ort, Verschlüsselung |
| Wie lange? | Aufbewahrung, Rotation (GVS), gesetzliche Fristen → [Archivierung](archivierung-revisionssicher.md) |
| Wer? | Verantwortliche, Vertretung, Zugriffsrechte auf Backups |
| Prüfung? | Protokolle auswerten, regelmäßige Restore-Tests, Dokumentation |

**Organisatorische Maßnahmen (2025W db):** Backup-Plan dokumentieren, Verantwortliche benennen, Restore-Tests planen und protokollieren, Medien beschriften und auslagern, Zugriff auf Backups auf wenige Personen beschränken.

## Prüfungsfrage

**Frage:** Ein Unternehmen sichert sonntags voll (300 GB) und Mo–Sa inkrementell (je ca. 15 GB). a) Welche Sicherungen braucht ein Restore nach einem Ausfall am Freitagmittag? b) Wie viel Speicher braucht die Woche? c) Nennen Sie einen Vorteil, den eine differenzielle Sicherung hätte.

**Musterantwort:** a) Das Vollbackup vom Sonntag und die Inkremente Mo, Di, Mi und Do in dieser Reihenfolge. Das Freitagsbackup läuft erst nach dem Ausfall. b) 300 GB + 6 · 15 GB = 390 GB. c) Der Restore bräuchte nur das Vollbackup und das letzte Diff (Do), also zwei Medien statt fünf. Er ginge schneller, und ein defektes Zwischenmedium würde die Kette nicht unterbrechen.

## Falle

- Differenziell und inkrementell vertauschen: **Diff = seit letztem VOLL**, **Inkr = seit letztem BACKUP**.
- Beim Inkrement nur das letzte Band nennen → es fehlen alle Zwischenstände.
- Ein Backup einplanen, das **nach** dem Ausfallzeitpunkt lief.
- Die Archivbit-Wirkung bei gemischten Ketten übersehen.
- „RAID 1 ist unser Backup“ → schützt nicht vor Löschen, Ransomware oder Brand.
- Snapshot auf demselben Storage als Backup verkaufen.
- GB und GiB mischen. Rechenweg immer mit Einheiten hinschreiben.

## Merksatz

**„Diff denkt an den Vater (Voll), Inkrement nur an den Vortag.“** Restore-Kette: Voll + letztes Diff **oder** Voll + jedes Inkrement.

## Verwandt

- [Snapshots-Backups](../virtualisierung/snapshots-backups.md) – Snapshot vs. Backup bei VMs
- [RAID](../raid.md) – Redundanz, aber kein Backup
- [Disaster Recovery & Notfallkonzept](disaster-recovery-notfallkonzept.md) – RTO/RPO, Wiederanlauf
- [Archivierung revisionssicher](archivierung-revisionssicher.md) – Backup vs. Archiv, WORM, Fristen
- [Deduplizierung & Komprimierung](../../../Komponenten/Storage/deduplizierung-komprimierung.md) – Backup-Speicher sparen
- [Kapazitätsplanung](../../../Komponenten/Storage/kapazitaetsplanung.md) – Speicherwachstum rechnen
- [DAS-NAS-SAN](../../../Komponenten/Storage/das-nas-san.md) – Backup-Ziele
- [Lektion L12 Backup-Strategien](../../../analyse-der-exams/L12_Backup_Strategien.md)
- [Lektion 02 Speicher, Backup, RAID](../../../analyse-der-exams/02_Speicher_Backup_RAID.md)
