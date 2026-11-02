# Index, Transaktionen und Locking

## In 30 Sekunden

- Ein **Index** ist eine sortierte Hilfsstruktur (meist **B-Baum**), die Suchen, Joins und Sortierungen beschleunigt. Nachteil: mehr Speicher, langsamere Schreibvorgänge.
- Eine **Transaktion** fasst mehrere Anweisungen zu einer Einheit zusammen: alles oder nichts (`COMMIT` / `ROLLBACK`).
- **ACID:** Atomicity, Consistency, Isolation, Durability.
- **Locking** regelt parallelen Zugriff: Lesesperre (shared) und Schreibsperre (exclusive), auf Zeilen- oder Tabellenebene.
- Ein **Deadlock** entsteht, wenn zwei Transaktionen gegenseitig auf die Sperre der anderen warten.
- 2025W fragte **Indexierung** und **Locking** als Leistungsmerkmale eines DBMS sowie **Gründe für langsame Antwortzeiten** einer Datenbank ab.

## Kernwissen

### Index

```sql
CREATE INDEX idx_kunde_ort ON Kunde(Ort);
CREATE UNIQUE INDEX idx_kunde_email ON Kunde(Email);    -- erzwingt Eindeutigkeit
CREATE INDEX idx_best_kd_dat ON Bestellung(KundenNr, Datum);  -- zusammengesetzt
DROP INDEX idx_kunde_ort ON Kunde;                     -- MySQL / MS SQL
```

| Vorteile | Nachteile |
|---|---|
| schnelle Suche mit `WHERE`, `JOIN`, `ORDER BY` | zusätzlicher Speicherplatz |
| vermeidet Full Table Scan | `INSERT`/`UPDATE`/`DELETE` langsamer, weil der Index mitgepflegt wird |
| `UNIQUE`-Index sichert Eindeutigkeit | bei wenigen unterschiedlichen Werten (z. B. Geschlecht, Boolean) kaum Nutzen |

**Wann indexieren?** Spalten, nach denen oft gesucht, gefiltert, gejoint oder sortiert wird: Primärschlüssel (automatisch), Fremdschlüssel, Suchfelder. **Nicht** für Spalten, die ständig geändert und selten gesucht werden.

**Größenordnung:** Ohne Index muss bei 1.000.000 Zeilen jede Zeile gelesen werden. Ein B-Baum findet den Eintrag mit wenigen Seitenzugriffen (Höhe ~ log n, typischerweise 3 bis 4 Ebenen).

### ACID

| Eigenschaft | Bedeutung | Beispiel Überweisung |
|---|---|---|
| **Atomicity** (Atomarität) | alles oder nichts | Abbuchung und Gutschrift passieren beide oder keine |
| **Consistency** (Konsistenz) | nach der Transaktion gelten alle Regeln und Constraints | Saldo darf nicht gegen `CHECK` verstoßen |
| **Isolation** | parallele Transaktionen beeinflussen sich nicht sichtbar | zweite Buchung sieht keinen halben Zwischenstand |
| **Durability** (Dauerhaftigkeit) | nach `COMMIT` bleiben Daten erhalten, auch bei Stromausfall | Transaktionslog (Write-Ahead-Log) |

```sql
START TRANSACTION;
UPDATE Lager SET Bestand = Bestand - 5 WHERE ArtNr = 'A1';
INSERT INTO Bestellposition (BestellNr, ArtNr, Menge) VALUES (1003, 'A1', 5);
COMMIT;      -- bei Fehler: ROLLBACK;
```

### Locking

| Sperre | Erlaubt anderen | Zweck |
|---|---|---|
| **Shared Lock** (Lesesperre, S) | weiteres Lesen, kein Schreiben | konsistentes Lesen |
| **Exclusive Lock** (Schreibsperre, X) | weder Lesen (je nach Isolationsstufe) noch Schreiben | Änderung schützen |

| Granularität | Vorteil | Nachteil |
|---|---|---|
| **Zeilensperre (Row Lock)** | hohe Parallelität | mehr Verwaltungsaufwand |
| **Seitensperre** | Mittelweg | sperrt Nachbarzeilen mit |
| **Tabellensperre** | einfach, wenig Overhead | blockiert alle anderen Zugriffe |

**Optimistisches Sperren:** keine Sperre beim Lesen, beim Speichern wird geprüft, ob jemand anderes inzwischen geändert hat (Versionsnummer). **Pessimistisches Sperren:** sofort sperren (`SELECT … FOR UPDATE`).

**MVCC (Multi-Version Concurrency Control):** Leser sehen einen Schnappschuss, Schreiber blockieren Leser nicht (PostgreSQL, InnoDB, Oracle).

### Probleme bei fehlender Isolation

| Problem | Beschreibung |
|---|---|
| **Lost Update** | zwei Transaktionen ändern denselben Wert, eine Änderung geht verloren |
| **Dirty Read** | Lesen nicht festgeschriebener Daten, die später per Rollback verschwinden |
| **Non-Repeatable Read** | derselbe Datensatz liefert beim zweiten Lesen einen anderen Wert |
| **Phantom Read** | eine wiederholte Abfrage liefert zusätzliche Zeilen |

Isolationsstufen von schwach nach stark: `READ UNCOMMITTED` → `READ COMMITTED` → `REPEATABLE READ` → `SERIALIZABLE`.

### Deadlock

T1 sperrt Zeile A und wartet auf B, T2 sperrt B und wartet auf A. Das DBMS erkennt den Zyklus und bricht eine Transaktion ab (Rollback). Vermeidung: Ressourcen immer in **gleicher Reihenfolge** sperren, Transaktionen kurz halten.

### Gründe für langsame Datenbankantworten (2025W)

1. **Fehlende oder veraltete Indizes** → Full Table Scans.
2. **Sperren/Blocking** durch lange Transaktionen.
3. **Zu wenig RAM** → Buffer Pool/Cache zu klein, viele Plattenzugriffe.
4. **Langsamer Speicher** (HDD statt SSD/NVMe, I/O-Engpass).
5. **Ineffiziente Abfragen** (`SELECT *`, fehlende `WHERE`-Bedingung, Funktionen auf indexierten Spalten).
6. **Netzwerklatenz** zwischen Anwendung und Datenbank (z. B. Cloud-DB).

## Prüfungsfrage

**Frage:** Erläutern Sie die Leistungsmerkmale "Indexierung" und "Locking-Mechanismen" eines Datenbanksystems und nennen Sie je einen Nachteil.

**Musterantwort:** **Indexierung** legt für ausgewählte Spalten eine sortierte Zugriffsstruktur (z. B. B-Baum) an. Abfragen mit `WHERE`, `JOIN` oder `ORDER BY` auf diese Spalten müssen nicht mehr die ganze Tabelle lesen und werden deutlich schneller. Nachteil: Der Index benötigt Speicher und muss bei jedem `INSERT`, `UPDATE` und `DELETE` aktualisiert werden, Schreibvorgänge werden langsamer. **Locking-Mechanismen** sperren Datensätze oder Tabellen während einer Transaktion, damit parallele Zugriffe keine inkonsistenten Daten erzeugen (z. B. Lost Update). Nachteil: Sperren führen zu Wartezeiten und im Extremfall zu Deadlocks; grobe Tabellensperren senken die Parallelität stark.

## Falle

- **"Mehr Indizes = schneller":** Nur Lesezugriffe profitieren, Schreibzugriffe werden langsamer.
- **ACID-Buchstaben vertauscht:** C = **Consistency**, nicht "Commit".
- **Durability mit Backup verwechselt:** Durability schützt festgeschriebene Transaktionen bei Absturz (Log), ersetzt aber kein Backup.
- **Tabellensperre als Standard empfehlen:** In Mehrbenutzersystemen sind Zeilensperren üblich.
- **Index auf jede Spalte:** verschwendet Speicher und bremst Schreiblast.

## Merksatz

**Index = Inhaltsverzeichnis, Transaktion = alles oder nichts, Lock = "Bitte nicht stören"-Schild.**

## Verwandt

- [DDL, DML, DCL](ddl-dml.md) – `CREATE INDEX`, `COMMIT`, `ROLLBACK`
- [SQL-Grundlagen](grundlagen.md)
- [Normalisierung](normalisierung.md)
- [NoSQL-Datenbanktypen](nosql-datenbanktypen.md) – BASE statt ACID
- [Cluster](../../FiSi-Administrator/Server/cluster.md) – Hochverfügbarkeit für Datenbanken
- [Lektion 14: Datenmodellierung, SQL & UML](../../analyse-der-exams/L14_Datenmodellierung_SQL_UML.md)
- [Lektion 28: GA1-Praxisthemen](../../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md) – Latenz bei Cloud-Datenbanken
