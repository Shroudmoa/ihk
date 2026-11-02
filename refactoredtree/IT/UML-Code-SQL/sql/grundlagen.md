# SQL-Grundlagen: SELECT, WHERE, GROUP BY, HAVING

## In 30 Sekunden

- SQL fragt relationale Datenbanken ab. Abfragen beginnen mit `SELECT … FROM …`, alles andere ist optional.
- **Schreibreihenfolge:** `SELECT → FROM → WHERE → GROUP BY → HAVING → ORDER BY → LIMIT`.
- **Ausführungsreihenfolge (logisch):** `FROM → WHERE → GROUP BY → HAVING → SELECT → ORDER BY → LIMIT`. Darum darf in `WHERE` noch keine Aggregatfunktion stehen.
- `WHERE` filtert **Zeilen vor** der Gruppierung, `HAVING` filtert **Gruppen nach** der Gruppierung.
- Aggregatfunktionen: `COUNT`, `SUM`, `AVG`, `MIN`, `MAX`. Jede Spalte im `SELECT`, die nicht aggregiert wird, muss im `GROUP BY` stehen.
- `NULL` prüft man mit `IS NULL` / `IS NOT NULL`, **nie** mit `= NULL`.
- In der AP2 kam SQL in 2021W, 2022S und 2022W vor (COUNT mit WHERE, Top 3 mit ORDER BY, INNER JOIN mit LIKE). Seit 2023 wird eher Datenmodellierung geprüft, SQL-Grundlagen bleiben aber Pflicht.

## Kernwissen

### Beispieltabellen (für alle Abfragen unten)

**Kunde**

| KundenNr | Name | Ort | PLZ | Aktiv |
|---:|---|---|---|---|
| 1 | Müller GmbH | Augsburg | 86150 | true |
| 2 | Schmidt KG | Hamburg | 20095 | true |
| 3 | Meyer AG | Augsburg | 86159 | false |
| 4 | Becker OHG | München | 80331 | true |
| 5 | Wagner e.K. | Berlin | 10115 | true |

**Bestellung**

| BestellNr | KundenNr | Datum | Betrag |
|---:|---:|---|---:|
| 100 | 1 | 2025-01-10 | 1200.00 |
| 101 | 1 | 2025-02-03 | 800.00 |
| 102 | 2 | 2025-02-15 | 3500.00 |
| 103 | 4 | 2025-03-01 | 450.00 |
| 104 | 2 | 2025-03-20 | 1500.00 |
| 105 | 1 | 2025-04-02 | 2000.00 |

### 1. SELECT – Spalten auswählen

```sql
SELECT * FROM Kunde;                          -- alle Spalten, alle Zeilen
SELECT Name, Ort FROM Kunde;                  -- nur bestimmte Spalten
SELECT DISTINCT Ort FROM Kunde;               -- doppelte Orte nur einmal
SELECT Name AS Firma, PLZ AS Postleitzahl     -- Spalten-Alias
FROM Kunde;
SELECT BestellNr, Betrag, Betrag * 1.19 AS Brutto  -- berechnete Spalte
FROM Bestellung;
```

`SELECT DISTINCT Ort` liefert: Augsburg, Hamburg, München, Berlin (4 Zeilen statt 5).

### 2. WHERE – Zeilen filtern

| Operator | Beispiel | Bedeutung |
|---|---|---|
| `=`, `<>` bzw. `!=` | `Ort = 'Augsburg'` | gleich / ungleich |
| `<`, `>`, `<=`, `>=` | `Betrag >= 1000` | Vergleich |
| `AND`, `OR`, `NOT` | `Ort = 'Augsburg' AND Aktiv = true` | logische Verknüpfung |
| `BETWEEN a AND b` | `Betrag BETWEEN 500 AND 1500` | Bereich **inklusive** Grenzen |
| `IN (…)` | `Ort IN ('Berlin', 'Hamburg')` | Wert aus Liste |
| `LIKE` | `PLZ LIKE '8%'` | Muster: `%` = beliebig viele Zeichen, `_` = genau ein Zeichen |
| `IS NULL` | `Telefon IS NULL` | kein Wert vorhanden |

```sql
-- Prüfungsklassiker 2022W: aktive Kunden in Augsburg zählen
SELECT COUNT(*) FROM Kunde
WHERE Ort = 'Augsburg' AND Aktiv = true;           -- Ergebnis: 1

-- Alle Kunden im PLZ-Gebiet 8
SELECT Name, PLZ FROM Kunde WHERE PLZ LIKE '8%';   -- Müller, Meyer, Becker

-- Bestellungen im 1. Quartal 2025
SELECT * FROM Bestellung
WHERE Datum BETWEEN '2025-01-01' AND '2025-03-31';  -- 100 bis 104

-- Kunden aus Berlin oder Hamburg
SELECT Name FROM Kunde WHERE Ort IN ('Berlin', 'Hamburg');

-- Klammern bei AND/OR setzen! AND bindet stärker als OR
SELECT * FROM Kunde
WHERE (Ort = 'Augsburg' OR Ort = 'München') AND Aktiv = true;  -- Müller, Becker
```

### 3. ORDER BY und LIMIT – sortieren und begrenzen

```sql
SELECT Name, Ort FROM Kunde ORDER BY Ort ASC, Name ASC;  -- ASC ist Standard
SELECT * FROM Bestellung ORDER BY Betrag DESC;           -- größter Betrag zuerst

-- Top 3 Bestellungen (Prüfungsklassiker 2022S "Top 3 Windräder")
SELECT BestellNr, Betrag FROM Bestellung
ORDER BY Betrag DESC
LIMIT 3;                       -- MySQL, PostgreSQL, SQLite

SELECT TOP 3 BestellNr, Betrag -- MS SQL Server
FROM Bestellung
ORDER BY Betrag DESC;
```

Ergebnis Top 3: 102 (3500), 105 (2000), 104 (1500).

### 4. Aggregatfunktionen

| Funktion | Liefert | NULL-Verhalten |
|---|---|---|
| `COUNT(*)` | Anzahl Zeilen | zählt alle Zeilen |
| `COUNT(spalte)` | Anzahl Werte | ignoriert NULL |
| `SUM(spalte)` | Summe | ignoriert NULL |
| `AVG(spalte)` | Durchschnitt | ignoriert NULL (teilt nur durch Anzahl Nicht-NULL) |
| `MIN(spalte)` / `MAX(spalte)` | kleinster / größter Wert | ignoriert NULL |

```sql
SELECT COUNT(*) AS Anzahl, SUM(Betrag) AS Umsatz,
       AVG(Betrag) AS Schnitt, MIN(Betrag) AS Kleinste, MAX(Betrag) AS Groesste
FROM Bestellung;
-- Anzahl 6 | Umsatz 9450.00 | Schnitt 1575.00 | Kleinste 450.00 | Groesste 3500.00

SELECT COUNT(DISTINCT KundenNr) FROM Bestellung;  -- 3 verschiedene Kunden haben bestellt
```

### 5. GROUP BY – Gruppen bilden

```sql
-- Umsatz und Anzahl Bestellungen pro Kunde
SELECT KundenNr, COUNT(*) AS Anzahl, SUM(Betrag) AS Umsatz
FROM Bestellung
GROUP BY KundenNr
ORDER BY Umsatz DESC;
```

| KundenNr | Anzahl | Umsatz |
|---:|---:|---:|
| 2 | 2 | 5000.00 |
| 1 | 3 | 4000.00 |
| 4 | 1 | 450.00 |

```sql
-- Kunden pro Ort
SELECT Ort, COUNT(*) AS Kunden FROM Kunde GROUP BY Ort;
-- Augsburg 2 | Hamburg 1 | München 1 | Berlin 1
```

### 6. HAVING – Gruppen filtern

```sql
-- Nur Kunden mit mehr als 4500 € Umsatz
SELECT KundenNr, SUM(Betrag) AS Umsatz
FROM Bestellung
GROUP BY KundenNr
HAVING SUM(Betrag) > 4500;           -- nur KundenNr 2 (5000)

-- WHERE und HAVING kombiniert: nur Bestellungen ab 2025-02-01 zählen,
-- danach nur Kunden mit mindestens 2 solcher Bestellungen
SELECT KundenNr, COUNT(*) AS Anzahl
FROM Bestellung
WHERE Datum >= '2025-02-01'
GROUP BY KundenNr
HAVING COUNT(*) >= 2;                -- KundenNr 1 (101, 105) und 2 (102, 104)
```

### 7. Unterabfragen (Subqueries)

```sql
-- Bestellungen über dem Durchschnitt (1575)
SELECT BestellNr, Betrag FROM Bestellung
WHERE Betrag > (SELECT AVG(Betrag) FROM Bestellung);  -- 102, 105

-- Kunden, die nie bestellt haben (IN / NOT IN)
SELECT Name FROM Kunde
WHERE KundenNr NOT IN (SELECT KundenNr FROM Bestellung);  -- Meyer AG, Wagner e.K.

-- dasselbe mit NOT EXISTS (robust, auch wenn die Unterabfrage NULL liefert)
SELECT k.Name FROM Kunde k
WHERE NOT EXISTS (SELECT 1 FROM Bestellung b WHERE b.KundenNr = k.KundenNr);

-- Unterabfrage im FROM (abgeleitete Tabelle)
SELECT AVG(Umsatz) AS DurchschnittProKunde
FROM (SELECT KundenNr, SUM(Betrag) AS Umsatz
      FROM Bestellung GROUP BY KundenNr) AS u;          -- (4000+5000+450)/3 = 3150
```

### 8. Nützliche Funktionen

```sql
SELECT UPPER(Name), LOWER(Ort), LENGTH(Name) FROM Kunde;
SELECT CONCAT(Name, ' (', Ort, ')') AS Anzeige FROM Kunde;   -- "Müller GmbH (Augsburg)"
SELECT ROUND(AVG(Betrag), 2) FROM Bestellung;
SELECT COALESCE(Telefon, 'unbekannt') FROM Kunde;          -- Ersatzwert für NULL
SELECT YEAR(Datum) AS Jahr, MONTH(Datum) AS Monat, SUM(Betrag)
FROM Bestellung GROUP BY YEAR(Datum), MONTH(Datum);       -- Umsatz pro Monat (MySQL/MS SQL)
```

Datumswerte schreibt man im ISO-Format `'JJJJ-MM-TT'` in einfachen Hochkommas.

## Prüfungsfrage

**Frage:** Gegeben ist die Tabelle `Betriebsdaten(ID, IDWindrad, Datum, TagesErzeugung)`. Formulieren Sie eine SQL-Abfrage, die die IDs der drei Windräder mit der höchsten Tageserzeugung am 03.05.2022 ausgibt. Formulieren Sie außerdem eine Abfrage, die die durchschnittliche Tageserzeugung je Windrad im Mai 2022 liefert, aber nur für Windräder mit mehr als 500 kWh Durchschnitt.

**Musterantwort:**

```sql
SELECT IDWindrad
FROM Betriebsdaten
WHERE Datum = '2022-05-03'
ORDER BY TagesErzeugung DESC
LIMIT 3;                       -- MS SQL: SELECT TOP 3 IDWindrad ...

SELECT IDWindrad, AVG(TagesErzeugung) AS Schnitt
FROM Betriebsdaten
WHERE Datum BETWEEN '2022-05-01' AND '2022-05-31'
GROUP BY IDWindrad
HAVING AVG(TagesErzeugung) > 500;
```

`WHERE` schränkt zuerst auf den Zeitraum ein, `GROUP BY` bildet eine Gruppe pro Windrad, `HAVING` filtert die Gruppen nach dem Durchschnitt.

## Falle

- **Aggregat in WHERE:** `WHERE SUM(Betrag) > 1000` ist ein Fehler. Gruppenbedingungen gehören in `HAVING`.
- **Spalte fehlt im GROUP BY:** `SELECT Ort, Name, COUNT(*) FROM Kunde GROUP BY Ort` ist in Standard-SQL ungültig, weil `Name` weder gruppiert noch aggregiert ist.
- **`= NULL`:** liefert nie ein Ergebnis. Richtig: `IS NULL`.
- **`NOT IN` mit NULL:** Enthält die Unterabfrage einen NULL-Wert, liefert `NOT IN` keine Zeile. `NOT EXISTS` ist sicherer.
- **Strings ohne Hochkommas:** `WHERE Ort = Augsburg` sucht eine Spalte namens Augsburg. Text und Datum immer in `'…'`.
- **LIMIT ohne ORDER BY:** "Top 3" ohne Sortierung liefert beliebige drei Zeilen.
- **`COUNT(spalte)` vs. `COUNT(*)`:** `COUNT(Telefon)` zählt nur Zeilen mit Telefonnummer.
- **AND vor OR:** Ohne Klammern wird `A OR B AND C` als `A OR (B AND C)` ausgewertet.

## Merksatz

**"Sie Fragen Wie Gute Hunde Ordnen"** – SELECT, FROM, WHERE, GROUP BY, HAVING, ORDER BY. **WHERE filtert Zeilen, HAVING filtert Gruppen.**

## Verwandt

- [SQL-Übersicht](readme.md) – alle SQL-Blätter und Kurzreferenz
- [SQL-Joins](joins.md) – mehrere Tabellen verknüpfen
- [DDL, DML, DCL](ddl-dml.md) – Tabellen anlegen, Daten ändern, Rechte vergeben
- [ER-Modell & Kardinalitäten](er-modell-kardinalitaeten.md) – woher die Tabellen kommen
- [Normalisierung](normalisierung.md) – warum Daten auf mehrere Tabellen verteilt sind
- [Index, Transaktionen & Locking](index-transaktionen-locking.md) – Performance und paralleler Zugriff
- [Lektion 14: Datenmodellierung, SQL & UML](../../analyse-der-exams/L14_Datenmodellierung_SQL_UML.md) – echte Prüfungsaufgaben
