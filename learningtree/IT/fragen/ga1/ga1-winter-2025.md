## 1. Aufgabe (Server-Hardware & Bottleneck)

### aa) Systemauswahl – Datenbank- vs. Virtualisierungsserver
- **Datenbankserver:** **System 1**.
  - *Begründung:* Datenbanken (OLTP) profitieren von hoher Single-Thread-Performance (3,4 GHz) und schnellem Storage-I/O. 12 TB NVMe lokal bietet maximale IOPS ohne SAN-Latenz. 2 TB RAM sind ausreichend. 25 GbE reicht für DB-Traffic. System 2 ist überdimensioniert und teuer.
- **Virtualisierungsserver:** **System 2**.
  - *Begründung:* Viele VMs benötigen maximalen RAM (4 TB), viele Kerne (256) und hohen Netzwerkdurchsatz (100 GbE) für Live-Migration und Storage-Traffic (SAN/NAS). GPU-Support und vCenter-Integration sind für Host-Management essenziell.

### ab) Mix-and-Match-Prinzip
- **Prinzip:** Festplatten unterschiedlicher Hersteller, Modelle und Chargen werden in einem RAID-Verbund gemischt. Ziel: Vermeidung systematischer Ausfälle (Bad Batches), bei denen Platten derselben Produktionscharge zeitgleich ausfallen (Korrelation der Fehler).

### ac) Badewannenkurve – 3 Zeiträume
- **Zeitraum I (Frühausfälle):** Hohe Ausfallrate durch Fertigungsfehler. Maßnahme: Burn-in-Test vor Produktivbetrieb; Platten nach wenigen Wochen prüfen.
- **Zeitraum II (Nutzungsphase):** Konstante, niedrige Ausfallrate (Zufallsausfälle). Maßnahme: RAID + Hot-Spare; Monitoring (SMART).
- **Zeitraum III (Verschleiß):** Steigende Ausfallrate durch Abnutzung. Maßnahme: Rechtzeitiger Austausch (Preventive Maintenance) vor Erreichen der Grenze.

### c) Bottleneck & Maßnahmen
- **Bottleneck:** **Arbeitsspeicher** – 91% physischer Speicher belegt, 0 harte Fehler. Prozesse `MdMng.exe` (149 MB) und `ServerManager.exe` (62 MB) verbrauchen viel. Auslagerung (Pagefile) würde Performance brechen.
- **Maßnahme 1:** RAM aufrüsten (mehr Module, größere Kapazität) oder unnötige Prozesse beenden.
- **Maßnahme 2:** Speicherfresser analysieren und optimieren (z.B. `MdMng.exe` durch leichteren Agent ersetzen); Memory-Leak prüfen.

---

## 2. Aufgabe (Datenbank-Design & Administration)

### a) Leistungsmerkmale
- **Indexierung:** Beschleunigt SELECT/WHERE/JOIN durch B-Tree/Hash-Strukturen. Reduziert Full-Table-Scans. Nachteil: Mehr Speicher, langsamere INSERT/UPDATE (Index muss gepflegt werden).
- **Locking-Mechanismen:** Steuern parallelen Zugriff. Row-Level-Locking erlaubt hohe Parallelität; Table-Locks verhindern Konflikte, reduzieren aber Durchsatz. MVCC (Multi-Version Concurrency Control) vermeidet Reads-Blocking.

### b) ERM – Primärschlüssel & Kardinalitäten
- **PKs:** `Kategorie.KategorieID`, `Lieferant.LieferantID`, `Teil.TeilID`, `Bestellung.BestellungID`, `Bestellposition.BestellpositionID`.
- **Kardinalitäten:**
  - Kategorie (1) → Teil (N): Eine Kategorie hat viele Teile.
  - Lieferant (1) → Bestellung (N): Ein Lieferant hat viele Bestellungen.
  - Bestellung (1) → Bestellposition (N): Eine Bestellung hat viele Positionen.
  - Teil (1) → Bestellposition (N): Ein Teil kann in vielen Positionen vorkommen.

### c) Sicherheitsmaßnahmen
| Anforderung | Maßnahme |
|---|---|
| Gesetze zum Schutz von Daten | Datenschutzbeauftragter bestellen (bereits gegeben) |
| Unberechtigten Zugriff verhindern | **RBAC (Role-Based Access Control), starke Passwörter, MFA** |
| Änderungen nachvollziehbar | **Audit-Logging / Trigger / Datenbank-Audit** |
| Hohe Verfügbarkeit | **Cluster (Failover), Replikation, RAID, Backup** |

### d) Verzögertes Reaktionsverhalten – 2 Gründe
- **Fehlende/veraltete Indizes:** Full-Table-Scans bei großen Tabellen; Query-Optimizer wählt schlechten Plan.
- **Locking/Blocking:** Lange Transaktionen halten Locks; andere Queries warten (Lock Waits).
- **Weitere:** RAM-Engpass (Buffer Pool zu klein), langsame I/O (Disk-Bottleneck), fehlende Statistiken.

---

## 3. Aufgabe (Programmierung – Service-IDs)

### aa) Analyse – Ist-Werte im Array
- **Gegeben:** `PerfCPU = {40,42,47,52,50,60,76,77,81}`.
- **Schleife:** `i = 1; i < 6; i += 3` → i = 1, 4.
- **i=1:** temp = (40+42+47)/3 = 43. `PerfCPU[1]=0; PerfCPU[0]=43; PerfCPU[2]=0;` → **43, 0, 0, 52, 50, 60, 76, 77, 81**.
- **i=4:** temp = (52+50+60)/3 = 54. `PerfCPU[4]=0; PerfCPU[3]=54; PerfCPU[5]=0;` → **43, 0, 0, 54, 0, 0, 76, 77, 81**.

### ab) Zwei korrekte Anweisungen
- **Fehler 1 – Schleifengrenze:** `i < 6` verarbeitet das letzte Triple (Index 6,7,8) nicht. Korrekt: `i < PerfCPU.Length - 1` oder `i <= 7`.
- **Fehler 2 – Falsche Mittelwertposition:** `PerfCPU[i-1] = temp;` setzt den Mittelwert an die falsche Stelle. Korrekt: `PerfCPU[i] = temp;` (Mittelwert in die mittlere Position).

### b) CSV – Argumente
- **ba) Vorteile (2):**
  - **Universell lesbar:** Reiner Text; von Excel, LibreOffice, Python, Shell lesbar; kein proprietäres Format.
  - **Geringe Dateigröße/Einfachheit:** Kein Overhead durch Metadaten; einfache Struktur, leicht zu parsen.
- **bb) Nachteil:** Keine Standardisierung für Sonderzeichen (Kommas, Anführungszeichen, Zeilenumbrüche); Escaping muss implementiert werden. Keine Typinformationen (alles Text).

---

## 4. Aufgabe (MTBF/MTTF & RAID/Backup)

### a) MTBF vs. MTTF
- **MTBF (Mean Time Between Failures):** Durchschnittliche Zeit zwischen zwei Ausfällen bei **reparierbaren** Systemen. Relevant für Backup-System, das nach Ausfall repariert wird. Hoher MTBF = hohe Zuverlässigkeit.
- **MTTF (Mean Time To Failure):** Erwartete Zeit bis zum **ersten** Ausfall bei **nicht reparierbaren** Systemen. Relevant für Festplatten (Verschleißteile). Hoher MTTF = lange Lebensdauer.

### ba) RAID 6 vs. RAID 5 – Vorteil
- **RAID 6:** Übersteht den gleichzeitigen Ausfall von **2 Platten** (Dual-Parity). RAID 5 übersteht nur 1 Plattenausfall. Bei großen Verbänden (Rebuild-Zeit) ist RAID 6 sicherer.

### bb) RAID 6 – Nettokapazität (20 × 8 TiB)
- **Formel:** Netto = (N − 2) × 8 TiB = (20 − 2) × 8 = 18 × 8 = **144 TiB**.

### bc) LTO-Bandlaufwerk – 3 Gründe
- **Kosten:** Pro TiB deutlich günstiger als HDD/SSD; ideal für Langzeitarchivierung.
- **Lebensdauer:** Bänder halten 30+ Jahre bei korrekter Lagerung; kein Stromverbrauch im Offline-Lager.
- **Offsite/Portabilität:** Physische Bänder können einfach ausgelagert werden (Air Gap); Schutz vor Ransomware/Cyberangriffen.

### c) Restore-Bänder (Störung 14. Mai mittags)
- **Letztes Vollbackup vor 14. Mai:** **B08** (So 07.05.).
- **Inkremente seit B08:** B09 (Mo 08.), B10 (Di 09.), B11 (Mi 10.), B12 (Do 11.), B13 (Fr 12.), B14 (Sa 13.).
- **Störung:** 14.05. mittags → B15 (So 14.05., voll) ist **nach** der Störung, unbrauchbar.
- **Reihenfolge:** **B08, B09, B10, B11, B12, B13, B14**.

### b) Großvater-Vater-Sohn – B22 als Sohn
- **Sohn:** B22 (Voll So 21.05.).
- **Vater:** B15 (Voll So 14.05.) – letztes Vollbackup davor.
- **Großvater:** B08 (Voll So 07.05.) – letztes Vollbackup davor.

### da) Datensicherung vs. Datenarchivierung
- **Datensicherung (Backup):** Kurzfristige Kopie zur Wiederherstellung nach Ausfall/Störung. Ziel: RTO/RPO. Daten werden regelmäßig überschrieben/rotieren.
- **Datenarchivierung:** Langfristige, unveränderbare Aufbewahrung (Compliance, gesetzliche Fristen). Ziel: Revisionssicherheit, Nachvollziehbarkeit. Daten bleiben Jahrzehnte erhalten.

### db) Organisatorische Maßnahmen (3)
- **Backup-Plan dokumentieren:** Wer, wann, was, wohin; Verantwortlichkeiten festlegen.
- **Regelmäßige Restore-Tests:** Sicherstellen, dass Backups wiederherstellbar sind (sonst kein Backup).
- **Offsite-Lagerung:** Physische Bänder/Datenträger an einem anderen Standort lagern (Schutz vor Feuer/Diebstahl).
- **Zugriffskontrolle:** Nur autorisierte Personen dürfen Backups erstellen/lesen/löschen.

---

**Merksatz:** IHK-Aufgaben sind **Kausalitätsketten** – wer RAID-6-Netto, Backup-Ketten (Voll→Inkr) und MTBF/MTTF nicht als Hard-Facts sitzen hat, verliert Punkte an der Oberfläche. Lerne Standards als Vokabeln, nicht als Prosa.
