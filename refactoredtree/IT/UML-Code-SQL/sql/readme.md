# SQL und Datenbanken – Übersicht

> Lernblätter zu relationalen Datenbanken, SQL, Datenmodellierung und Datenformaten für die AP2 (FiSi, AO 2020).

## Prüfungsrelevanz

- **SQL-Abfragen** wurden in **2021W, 2022S und 2022W** verlangt: `COUNT` mit `WHERE`, Top 3 mit `ORDER BY … DESC`, `CREATE TABLE` mit PK/FK, `INNER JOIN` mit `LIKE` und Datumsfilter, SQL-Datentypen zuordnen.
- Seit 2023 kommt eher **Datenmodellierung**: ER-Modell, Primärschlüssel, Kardinalitäten, 3. Normalform, Indexierung und Locking (2025W).
- Weitere Datenthemen: **nicht-relationale DB-Typen** (2022W), **CSV-Vor- und Nachteile** (2024W, 2025W).

## Lernblätter

| Blatt | Inhalt | Priorität |
|---|---|---|
| [SQL-Grundlagen](grundlagen.md) | SELECT, WHERE, ORDER BY, LIMIT, Aggregatfunktionen, GROUP BY, HAVING, Unterabfragen | ★★★★★ |
| [SQL-Joins](joins.md) | INNER, LEFT, RIGHT, FULL OUTER, CROSS, SELF JOIN mit Ergebnistabellen | ★★★★★ |
| [DDL, DML, DCL, TCL](ddl-dml.md) | CREATE, ALTER, DROP, Constraints, INSERT, UPDATE, DELETE, GRANT, REVOKE, Transaktionen, Datentypen | ★★★★★ |
| [ER-Modell & Kardinalitäten](er-modell-kardinalitaeten.md) | Entität, Attribut, PK/FK, 1:1, 1:n, m:n, referentielle Integrität | ★★★★★ |
| [Normalisierung](normalisierung.md) | Anomalien, 1. bis 3. Normalform mit durchgerechnetem Beispiel | ★★★★★ |
| [Index, Transaktionen & Locking](index-transaktionen-locking.md) | B-Baum-Index, ACID, Sperren, Deadlock, langsame Datenbanken | ★★★★☆ |
| [NoSQL-Datenbanktypen](nosql-datenbanktypen.md) | Dokument, Key-Value, Wide Column, Graph, Zeitreihe, BASE vs. ACID | ★★★☆☆ |
| [Datenformate CSV, JSON, XML](datenformate-json-xml-csv.md) | Aufbau, Vergleich, CSV-Vor- und Nachteile | ★★★★☆ |

## Lernreihenfolge

1. ER-Modell → Normalisierung (Wie entstehen die Tabellen?)
2. DDL (Tabellen anlegen) → DML (Daten pflegen)
3. SELECT-Grundlagen → Joins
4. Index, Transaktionen, Locking → NoSQL → Datenformate

## Kurzreferenz

### Begriffe

| Begriff | Bedeutung |
|---|---|
| DB / DBMS / RDBMS | Datenbestand / Verwaltungssoftware / relationales DBMS (MySQL, PostgreSQL, MS SQL Server, Oracle) |
| Relation, Tupel, Attribut | Tabelle, Zeile (Datensatz), Spalte |
| Primärschlüssel (PK) | eindeutig, nicht NULL |
| Fremdschlüssel (FK) | verweist auf einen PK, sichert referentielle Integrität |
| NULL | kein Wert – nicht 0 und nicht leerer Text |

### SQL-Kategorien

| Kategorie | Befehle |
|---|---|
| DDL – Definition | `CREATE`, `ALTER`, `DROP`, `TRUNCATE` |
| DML – Manipulation | `INSERT`, `UPDATE`, `DELETE` |
| DQL – Abfrage | `SELECT` |
| DCL – Rechte | `GRANT`, `REVOKE` |
| TCL – Transaktionen | `COMMIT`, `ROLLBACK`, `SAVEPOINT` |

### Abfrage-Gerüst

```sql
SELECT   spalten, AGGREGAT(spalte) AS alias
FROM     tabelle1 t1
JOIN     tabelle2 t2 ON t1.fk = t2.pk
WHERE    zeilenbedingung
GROUP BY spalten
HAVING   gruppenbedingung
ORDER BY spalte DESC
LIMIT    n;              -- MS SQL: SELECT TOP n ...
```

### Die häufigsten Fehler

- `UPDATE`/`DELETE` ohne `WHERE` → alle Zeilen betroffen.
- `= NULL` statt `IS NULL`.
- Aggregatfunktion in `WHERE` statt `HAVING`.
- Nicht aggregierte Spalte fehlt im `GROUP BY`.
- Join ohne `ON` → kartesisches Produkt.
- PLZ/Telefon als `INT` statt `CHAR`/`VARCHAR`, Geld als `FLOAT` statt `DECIMAL`.

## Verwandt

- [Lektion 14: Datenmodellierung, SQL & UML](../../analyse-der-exams/L14_Datenmodellierung_SQL_UML.md) – echte Prüfungsaufgaben
- [Lektion 4: Programmierung](../../analyse-der-exams/04_Programmierung_Grundlagen.md)
- [UML-Übersicht](../UML/readme.md) – Klassendiagramm als Gegenstück zum ERM
- [Pseudocode-Übersicht](../Pseudocode/readme.md)
