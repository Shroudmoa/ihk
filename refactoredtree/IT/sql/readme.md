```markdown
# SQL – Strukturierte Zusammenfassung

## 1. Grundlagen

### Datenbank vs. DBMS
*   **DB (Datenbank):** Sammlung zusammengehöriger Daten (Tabellen).
*   **DBMS (Datenbankmanagementsystem):** Software zur Verwaltung (z.B. MySQL, PostgreSQL, Oracle, MS SQL Server).
*   **RDBMS (Relationales DBMS):** Speichert Daten in Tabellen (Relationen) mit Beziehungen (Fremdschlüssel).

### Wichtige Begriffe
*   **Relation:** Tabelle.
*   **Tupel:** Datensatz (Zeile).
*   **Attribut:** Spalte (Feld).
*   **Primärschlüssel (PK):** Eindeutige Identifizierung eines Datensatzes. Darf nicht NULL sein.
*   **Fremdschlüssel (FK):** Verweist auf Primärschlüssel einer anderen Tabelle. Sorgt für referentielle Integrität.
*   **NULL:** Kein Wert (unbekannt/nicht vorhanden). Nicht verwechseln mit 0 oder Leerstring.

### SQL-Kategorien
*   **DDL (Data Definition Language):** Struktur definieren (`CREATE`, `ALTER`, `DROP`, `TRUNCATE`).
*   **DML (Data Manipulation Language):** Daten ändern (`INSERT`, `UPDATE`, `DELETE`).
*   **DQL (Data Query Language):** Daten abfragen (`SELECT`).
*   **DCL (Data Control Language):** Rechte (`GRANT`, `REVOKE`).
*   **TCL (Transaction Control Language):** Transaktionen (`COMMIT`, `ROLLBACK`).

---

## 2. DDL – Data Definition Language

### Datenbank/Tabelle erstellen
```sql
CREATE DATABASE FirmenDB;

CREATE TABLE Mitarbeiter (
    ID INT PRIMARY KEY,
    Name VARCHAR(100) NOT NULL,
    Gehalt DECIMAL(10,2),
    Eintrittsdatum DATE,
    Abteilung_ID INT,
    FOREIGN KEY (Abteilung_ID) REFERENCES Abteilung(ID)
);
```

### Tabelle ändern
```sql
ALTER TABLE Mitarbeiter ADD Email VARCHAR(255);
ALTER TABLE Mitarbeiter DROP COLUMN Email;
ALTER TABLE Mitarbeiter MODIFY Gehalt DECIMAL(12,2); -- Spaltentyp ändern
```

### Tabelle/Datenbank löschen
```sql
DROP TABLE Mitarbeiter;       -- Tabelle + Struktur löschen
DROP DATABASE FirmenDB;       -- Datenbank löschen
TRUNCATE TABLE Mitarbeiter;   -- Alle Daten löschen, Struktur bleibt
```

### Datentypen (wichtig für IHK)
*   `INT` / `BIGINT` – Ganzzahlen.
*   `DECIMAL(p,s)` / `NUMERIC` – Dezimalzahlen (p = Gesamtstellen, s = Nachkommastellen). Für Geld.
*   `VARCHAR(n)` – Zeichenkette variabler Länge.
*   `CHAR(n)` – Feste Länge.
*   `DATE` – Datum (YYYY-MM-DD).
*   `DATETIME` / `TIMESTAMP` – Datum + Uhrzeit.
*   `BOOLEAN` – Wahr/Falsch.

---

## 3. DML – Data Manipulation Language

### Daten einfügen
```sql
INSERT INTO Mitarbeiter (ID, Name, Gehalt, Eintrittsdatum, Abteilung_ID)
VALUES (1, 'Max Mustermann', 4500.00, '2023-01-15', 10);
```

### Daten ändern
```sql
UPDATE Mitarbeiter
SET Gehalt = 5000.00, Name = 'Max Muster'
WHERE ID = 1;
```
*   **Warnung:** Ohne `WHERE` werden alle Datensätze geändert.

### Daten löschen
```sql
DELETE FROM Mitarbeiter
WHERE ID = 1;
```
*   **Warnung:** Ohne `WHERE` werden alle Datensätze gelöscht.

### Transaktionen (IHK-relevant)
```sql
BEGIN TRANSACTION;
UPDATE Konto SET Saldo = Saldo - 100 WHERE ID = 1;
UPDATE Konto SET Saldo = Saldo + 100 WHERE ID = 2;
COMMIT;   -- Bestätigen
-- ROLLBACK; -- Bei Fehler rückgängig
```
*   **ACID-Prinzip:** Atomicity, Consistency, Isolation, Durability.

---

## 4. DQL – Data Query Language (SELECT)

### Grundstruktur
```sql
SELECT Spalte1, Spalte2
FROM Tabelle
WHERE Bedingung
GROUP BY Spalte
HAVING Bedingung
ORDER BY Spalte ASC|DESC;
```

### Alle Spalten
```sql
SELECT * FROM Mitarbeiter;
```

### DISTINCT (Duplikate entfernen)
```sql
SELECT DISTINCT Abteilung_ID FROM Mitarbeiter;
```

### WHERE – Filtern
```sql
SELECT * FROM Mitarbeiter WHERE Gehalt > 4000;
SELECT * FROM Mitarbeiter WHERE Name LIKE 'Max%';   -- beginnt mit Max
SELECT * FROM Mitarbeiter WHERE Name LIKE '%mann';  -- endet mit mann
SELECT * FROM Mitarbeiter WHERE Gehalt BETWEEN 3000 AND 5000;
SELECT * FROM Mitarbeiter WHERE Abteilung_ID IN (10, 20, 30);
SELECT * FROM Mitarbeiter WHERE Email IS NULL;
SELECT * FROM Mitarbeiter WHERE Email IS NOT NULL;
```

### Logische Operatoren
```sql
WHERE Gehalt > 3000 AND Abteilung_ID = 10
WHERE Gehalt < 2000 OR Abteilung_ID = 20
WHERE NOT Abteilung_ID = 10
```

### ORDER BY – Sortieren
```sql
SELECT * FROM Mitarbeiter ORDER BY Gehalt DESC;
SELECT * FROM Mitarbeiter ORDER BY Name ASC, Gehalt DESC;
```

### LIMIT / TOP – Begrenzen
```sql
SELECT * FROM Mitarbeiter LIMIT 10;        -- MySQL/PostgreSQL
SELECT TOP 10 * FROM Mitarbeiter;          -- MS SQL Server
```

### Aggregatfunktionen
| Funktion | Beschreibung |
| :--- | :--- |
| `COUNT(*)` | Anzahl Datensätze |
| `SUM(Spalte)` | Summe |
| `AVG(Spalte)` | Durchschnitt |
| `MIN(Spalte)` | Minimum |
| `MAX(Spalte)` | Maximum |

```sql
SELECT COUNT(*) FROM Mitarbeiter;
SELECT AVG(Gehalt) FROM Mitarbeiter;
SELECT Abteilung_ID, COUNT(*) AS Anzahl
FROM Mitarbeiter
GROUP BY Abteilung_ID;
```

### GROUP BY & HAVING
*   `GROUP BY`: Gruppiert Datensätze für Aggregate.
*   `HAVING`: Filtert **nach** der Gruppierung (wie WHERE, aber für Aggregate).

```sql
SELECT Abteilung_ID, AVG(Gehalt) AS Durchschnitt
FROM Mitarbeiter
GROUP BY Abteilung_ID
HAVING AVG(Gehalt) > 4000;
```

### Aliase (AS)
```sql
SELECT Name AS Mitarbeitername, Gehalt * 12 AS Jahresgehalt
FROM Mitarbeiter;
```

### String-Funktionen (Beispiele)
```sql
SELECT UPPER(Name) FROM Mitarbeiter;        -- Großbuchstaben
SELECT LOWER(Name) FROM Mitarbeiter;        -- Kleinbuchstaben
SELECT CONCAT(Vorname, ' ', Nachname) FROM Mitarbeiter;
SELECT LENGTH(Name) FROM Mitarbeiter;       -- Länge
```

### Datumsfunktionen (Beispiele)
```sql
SELECT YEAR(Eintrittsdatum) FROM Mitarbeiter;
SELECT MONTH(Eintrittsdatum) FROM Mitarbeiter;
SELECT DATEDIFF(YEAR, Eintrittsdatum, GETDATE()) AS JahreImBetrieb FROM Mitarbeiter; -- MS SQL
```

---

## 5. JOINs – Tabellen verknüpfen

### INNER JOIN
*   Gibt nur Datensätze zurück, die in **beiden** Tabellen einen Treffer haben.

```sql
SELECT m.Name, a.Abteilungsname
FROM Mitarbeiter m
INNER JOIN Abteilung a ON m.Abteilung_ID = a.ID;
```

### LEFT JOIN (LEFT OUTER JOIN)
*   Alle Datensätze der **linken** Tabelle + passende der rechten. Keine Übereinstimmung → NULL.

```sql
SELECT m.Name, a.Abteilungsname
FROM Mitarbeiter m
LEFT JOIN Abteilung a ON m.Abteilung_ID = a.ID;
```
*   *Anwendungsfall:* Alle Mitarbeiter anzeigen, auch die ohne Abteilung.

### RIGHT JOIN (RIGHT OUTER JOIN)
*   Alle Datensätze der **rechten** Tabelle + passende der linken.

```sql
SELECT m.Name, a.Abteilungsname
FROM Mitarbeiter m
RIGHT JOIN Abteilung a ON m.Abteilung_ID = a.ID;
```
*   *Anwendungsfall:* Alle Abteilungen anzeigen, auch die ohne Mitarbeiter.

### FULL OUTER JOIN
*   Alle Datensätze aus **beiden** Tabellen. Keine Übereinstimmung → NULL auf beiden Seiten.

```sql
SELECT m.Name, a.Abteilungsname
FROM Mitarbeiter m
FULL OUTER JOIN Abteilung a ON m.Abteilung_ID = a.ID;
```

### CROSS JOIN
*   Kartesisches Produkt: Jede Zeile der linken Tabelle mit jeder Zeile der rechten Tabelle kombinieren.

```sql
SELECT m.Name, a.Abteilungsname
FROM Mitarbeiter m
CROSS JOIN Abteilung a;
```

### SELF JOIN
*   Tabelle mit sich selbst verknüpfen (z.B. Mitarbeiter und dessen Vorgesetzter).

```sql
SELECT m.Name AS Mitarbeiter, v.Name AS Vorgesetzter
FROM Mitarbeiter m
LEFT JOIN Mitarbeiter v ON m.Vorgesetzter_ID = v.ID;
```

### JOIN-Übersicht (IHK-Merksatz)
| JOIN-Typ | Ergebnis |
| :--- | :--- |
| INNER JOIN | Nur Übereinstimmungen |
| LEFT JOIN | Alles links + Übereinstimmungen rechts |
| RIGHT JOIN | Alles rechts + Übereinstimmungen links |
| FULL OUTER JOIN | Alles aus beiden Tabellen |
| CROSS JOIN | Kartesisches Produkt |
| SELF JOIN | Tabelle mit sich selbst |

---

## 6. Unterabfragen (Subqueries)

### In WHERE
```sql
SELECT Name FROM Mitarbeiter
WHERE Gehalt > (SELECT AVG(Gehalt) FROM Mitarbeiter);
```

### Mit IN
```sql
SELECT Name FROM Mitarbeiter
WHERE Abteilung_ID IN (SELECT ID FROM Abteilung WHERE Ort = 'Berlin');
```

### Mit EXISTS
```sql
SELECT Name FROM Mitarbeiter m
WHERE EXISTS (SELECT 1 FROM Abteilung a WHERE a.ID = m.Abteilung_ID AND a.Ort = 'Berlin');
```

---

## 7. Views (Sichten)

*   Virtuelle Tabellen (gespeicherte Abfrage). Keine echten Daten.
*   **Vorteil:** Zugriffskontrolle, Vereinfachung komplexer Abfragen, Datenunabhängigkeit.

```sql
CREATE VIEW vw_Mitarbeiter_Berlin AS
SELECT m.Name, m.Gehalt, a.Abteilungsname
FROM Mitarbeiter m
INNER JOIN Abteilung a ON m.Abteilung_ID = a.ID
WHERE a.Ort = 'Berlin';

SELECT * FROM vw_Mitarbeiter_Berlin;

DROP VIEW vw_Mitarbeiter_Berlin;
```

---

## 8. Indizes

*   Beschleunigen Abfragen (wie ein Buchindex).
*   **Nachteil:** Verlangsamen INSERT/UPDATE/DELETE (Index muss aktualisiert werden).

```sql
CREATE INDEX idx_name ON Mitarbeiter(Name);
DROP INDEX idx_name ON Mitarbeiter;
```

---

## 9. IHK-Prüfungsrelevante Fehlerquellen

*   **Ohne WHERE bei UPDATE/DELETE:** Alle Datensätze betroffen.
*   **NULL-Vergleiche:** `WHERE Spalte = NULL` funktioniert nicht. Richtig: `WHERE Spalte IS NULL`.
*   **GROUP BY vergessen:** Bei Aggregatfunktionen + normalen Spalten muss `GROUP BY` verwendet werden.
*   **HAVING statt WHERE:** `WHERE` filtert vor der Gruppierung, `HAVING` danach.
*   **JOIN ohne ON:** Führt zu CROSS JOIN (kartesisches Produkt) – meist unerwünscht.
*   **Primärschlüssel NULL:** Nicht erlaubt.
*   **Fremdschlüssel-Verletzung:** Einfügen eines Werts, der im referenzierten PK nicht existiert (referentielle Integrität verletzt).
```
