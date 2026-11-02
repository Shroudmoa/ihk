# SQL-Joins

## In 30 Sekunden

- Ein **Join** verbindet Zeilen aus mehreren Tabellen über eine Bedingung, meist **Fremdschlüssel = Primärschlüssel**.
- **INNER JOIN:** nur Zeilen mit Partner in **beiden** Tabellen.
- **LEFT JOIN:** **alle** Zeilen der linken Tabelle, rechts NULL, wenn kein Partner existiert.
- **RIGHT JOIN:** umgekehrt: alle Zeilen der rechten Tabelle.
- **FULL OUTER JOIN:** alle Zeilen beider Seiten, fehlende Partner werden mit NULL aufgefüllt.
- **CROSS JOIN:** jede Zeile mit jeder Zeile (kartesisches Produkt, m × n Zeilen).
- **SELF JOIN:** Tabelle mit sich selbst, z. B. Mitarbeiter und Vorgesetzter.
- In der Prüfung 2022W wurde ein **INNER JOIN** mit `LIKE` und Datumsfilter verlangt.

## Kernwissen

### Beispieltabellen

**Abteilung**

| AbtID | Bezeichnung |
|---:|---|
| 10 | IT |
| 20 | Vertrieb |
| 30 | Einkauf |

**Mitarbeiter**

| MaID | Name | AbtID | ChefID |
|---:|---|---:|---:|
| 1 | Klein | 10 | NULL |
| 2 | Groß | 10 | 1 |
| 3 | Lang | 20 | 1 |
| 4 | Kurz | NULL | 2 |

Einkauf (30) hat keine Mitarbeiter, Kurz (4) hat keine Abteilung.

### INNER JOIN

```sql
SELECT m.Name, a.Bezeichnung
FROM Mitarbeiter m
INNER JOIN Abteilung a ON m.AbtID = a.AbtID;
```

| Name | Bezeichnung |
|---|---|
| Klein | IT |
| Groß | IT |
| Lang | Vertrieb |

Kurz und Einkauf fallen raus, weil sie keinen Partner haben. `JOIN` ohne Zusatz bedeutet `INNER JOIN`.

**Alte Schreibweise (implizit, gleiches Ergebnis):**

```sql
SELECT m.Name, a.Bezeichnung
FROM Mitarbeiter m, Abteilung a
WHERE m.AbtID = a.AbtID;
```

### LEFT (OUTER) JOIN

```sql
SELECT m.Name, a.Bezeichnung
FROM Mitarbeiter m
LEFT JOIN Abteilung a ON m.AbtID = a.AbtID;
```

| Name | Bezeichnung |
|---|---|
| Klein | IT |
| Groß | IT |
| Lang | Vertrieb |
| Kurz | NULL |

**Typischer Einsatz:** Datensätze **ohne** Partner finden.

```sql
-- Abteilungen ohne Mitarbeiter
SELECT a.Bezeichnung
FROM Abteilung a
LEFT JOIN Mitarbeiter m ON a.AbtID = m.AbtID
WHERE m.MaID IS NULL;          -- Ergebnis: Einkauf
```

### RIGHT (OUTER) JOIN

```sql
SELECT m.Name, a.Bezeichnung
FROM Mitarbeiter m
RIGHT JOIN Abteilung a ON m.AbtID = a.AbtID;
```

| Name | Bezeichnung |
|---|---|
| Klein | IT |
| Groß | IT |
| Lang | Vertrieb |
| NULL | Einkauf |

Ein RIGHT JOIN lässt sich immer als LEFT JOIN mit vertauschten Tabellen schreiben.

### FULL OUTER JOIN

```sql
SELECT m.Name, a.Bezeichnung
FROM Mitarbeiter m
FULL OUTER JOIN Abteilung a ON m.AbtID = a.AbtID;
```

Ergebnis: Klein/IT, Groß/IT, Lang/Vertrieb, **Kurz/NULL**, **NULL/Einkauf** (5 Zeilen). MySQL kennt kein `FULL OUTER JOIN`; dort kombiniert man LEFT JOIN und RIGHT JOIN mit `UNION`.

### CROSS JOIN

```sql
SELECT m.Name, a.Bezeichnung
FROM Mitarbeiter m CROSS JOIN Abteilung a;   -- 4 × 3 = 12 Zeilen
```

Sinnvoll z. B. für alle Kombinationen aus Größen und Farben. Ein vergessenes `ON` erzeugt ungewollt dasselbe.

### SELF JOIN

```sql
SELECT m.Name AS Mitarbeiter, c.Name AS Vorgesetzter
FROM Mitarbeiter m
LEFT JOIN Mitarbeiter c ON m.ChefID = c.MaID;
```

| Mitarbeiter | Vorgesetzter |
|---|---|
| Klein | NULL |
| Groß | Klein |
| Lang | Klein |
| Kurz | Groß |

Ohne Aliase (`m`, `c`) ist ein Self Join nicht möglich.

### Join über drei Tabellen (m:n mit Zwischentabelle)

```sql
-- Tabellen: Mitarbeiter, Projekt, Mitarbeiter_Projekt(MaID, ProjektID, Stunden)
SELECT m.Name, p.Titel, mp.Stunden
FROM Mitarbeiter m
INNER JOIN Mitarbeiter_Projekt mp ON m.MaID = mp.MaID
INNER JOIN Projekt p ON mp.ProjektID = p.ProjektID
ORDER BY m.Name;
```

### Join mit Aggregation

```sql
-- Anzahl Mitarbeiter je Abteilung, auch leere Abteilungen (0)
SELECT a.Bezeichnung, COUNT(m.MaID) AS Anzahl
FROM Abteilung a
LEFT JOIN Mitarbeiter m ON a.AbtID = m.AbtID
GROUP BY a.Bezeichnung;
-- IT 2 | Vertrieb 1 | Einkauf 0
```

`COUNT(m.MaID)` statt `COUNT(*)`: sonst würde Einkauf mit 1 gezählt, weil die NULL-Zeile mitzählt.

### Übersicht

| Join | linke Zeilen ohne Partner | rechte Zeilen ohne Partner | Zeilen im Beispiel |
|---|---|---|---:|
| INNER | nein | nein | 3 |
| LEFT | ja | nein | 4 |
| RIGHT | nein | ja | 4 |
| FULL OUTER | ja | ja | 5 |
| CROSS | – (alle Kombinationen) | – | 12 |

## Prüfungsfrage

**Frage (angelehnt an 2022W):** Gegeben sind die Tabellen `Kunde(KundenNr, Name, PLZ, Ort)` und `Werbeaktion(WerbungsID, KundenNr, Werbepartner, Werbung_verschickt)`. Ermitteln Sie mit SQL, wie viele Werbungen vor dem 31.12.2021 an Kunden versendet wurden, deren Postleitzahl mit 8 beginnt. Begründen Sie die Wahl des Join-Typs.

**Musterantwort:**

```sql
SELECT COUNT(*)
FROM Werbeaktion w
INNER JOIN Kunde k ON w.KundenNr = k.KundenNr
WHERE w.Werbung_verschickt < '2021-12-31'
  AND k.PLZ LIKE '8%';
```

Ein **INNER JOIN** reicht, weil nur Werbungen gezählt werden sollen, die einem Kunden mit passender PLZ zugeordnet sind. Werbungen ohne Kunden oder Kunden ohne Werbung sind für das Ergebnis irrelevant. Die PLZ muss als Text gespeichert sein (`CHAR(5)`), sonst funktionieren `LIKE` und führende Nullen nicht.

## Falle

- **Join ohne ON/WHERE-Bedingung** → kartesisches Produkt mit m × n Zeilen.
- **WHERE hebt LEFT JOIN auf:** `LEFT JOIN Abteilung a … WHERE a.Bezeichnung = 'IT'` entfernt die NULL-Zeilen wieder, das Ergebnis entspricht einem INNER JOIN. Bedingungen für die rechte Tabelle gehören dann in die `ON`-Klausel.
- **Mehrdeutige Spaltennamen:** Steht `AbtID` in beiden Tabellen, muss man `m.AbtID` oder `a.AbtID` schreiben.
- **LEFT vs. RIGHT verwechselt:** "Alle Kunden, auch ohne Bestellung" heißt: Kunde steht **links** bei LEFT JOIN.
- **COUNT(*) nach LEFT JOIN** zählt leere Gruppen als 1. Besser `COUNT(spalte_der_rechten_tabelle)`.

## Merksatz

**INNER = Schnittmenge, LEFT = alles links, RIGHT = alles rechts, FULL = alles, CROSS = jeder mit jedem.** Wer "auch ohne" liest, braucht einen OUTER JOIN.

## Verwandt

- [SQL-Grundlagen](grundlagen.md) – SELECT, WHERE, GROUP BY, HAVING
- [DDL, DML, DCL](ddl-dml.md) – Fremdschlüssel anlegen
- [ER-Modell & Kardinalitäten](er-modell-kardinalitaeten.md) – 1:n und m:n als Grundlage für Joins
- [Normalisierung](normalisierung.md) – warum man nach dem Aufteilen wieder joinen muss
- [SQL-Übersicht](readme.md)
- [Lektion 14: Datenmodellierung, SQL & UML](../../analyse-der-exams/L14_Datenmodellierung_SQL_UML.md)
