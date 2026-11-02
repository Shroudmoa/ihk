# Lektion 14 – Datenmodellierung, SQL & UML

**Bereich:** GA1 (Konzeption und Administration von IT-Systemen) / GA2 (Entwicklung)
**Relevanz:** hoch – ER-Modell, SQL-Abfragen und UML-Diagramme sind feste Bestandteile vieler AP2-Aufgaben
**Lernzeit:** ca. 2h

---

## 1. Warum das wichtig ist

Datenbanken und Modellierungssprachen sind das Rückgrat fast jeder Fachanwendung. In der AP2 musst du Entity-Relationship-Modelle lesen/zeichnen, SQL-Abfragen formulieren und UML-Diagramme (oft als Zeichenaufgabe) erstellen können.

> ⚠️ **Wichtige Korrektur (Verifiziert am echten Prüfungskorpus, 10 Jahrgänge
> 2021–2026):** SQL kam **NUR in 3 von 10 Jahrgängen** vor – **2021W, 2022S,
> 2022W**. **Seit 2023 gibt es KEIN SQL mehr in GA1!** Statt SQL wird seit
> 2023 Datenbankmodellierung geprüft: ER-Diagramme, Normalisierung,
> Indexierung und Locking, z. B. **2025W: 3. Normalform, Primärschlüssel und
> Kardinalitäten** (siehe Abschnitt 3b). UML wurde nur in **2021W und 2022S**
> explizit unter diesem Namen abgefragt; verwandte Themen (Aggregation/
> Komposition) kamen 2023S vor.

## 1b. Die drei echten SQL-Original-Beispiele (2021W/2022S/2022W)

**2021W:**
```sql
SELECT COUNT(*) FROM tblDiskM WHERE Datum = '2021-11-21';
```

**2022S:**
```sql
SELECT count(*) FROM Windrad WHERE IDWindpark = 102;

SELECT top 3(LeistungAktuell) FROM Betriebsdaten
WHERE IDWindpark = 102 AND Datum = "2020.05.03";
```

**2022W** (umfangreichste SQL-Aufgabe, mit Referenzblatt):
```sql
-- Tabelle anlegen mit PK/FK
CREATE TABLE Werbeaktion (
    WerbungsID int NOT NULL,
    KundenNr int,
    -- ... weitere Spalten
    PRIMARY KEY(WerbungsID),
    FOREIGN KEY(KundenNr) REFERENCES Kunde(KundenNr)
);

-- Zählabfrage mit mehreren Bedingungen
SELECT COUNT(*) FROM Kunde
WHERE Ort = 'Augsburg' AND Kunde_Aktiv = 'true';

-- INNER JOIN mit Filterung
SELECT ...
FROM Kunde k
INNER JOIN Werbeaktion w ON k.KundenNr = w.KundenNr
WHERE PLZ LIKE '8%' AND Werbung_verschickt < '2021-12-31';
```
Zusätzlich mussten 2022W **SQL-Datentypen zugeordnet** werden
(`int`, `varchar(n)`, `date`, `char(n)`, `boolean`) sowie
**nicht-relationale Datenbanktypen** genannt werden: **Dokumenten-DB**,
**Key-Value-DB**, **Graphen-DB**.

## 2. Grundlagen – ER-Modell

| Begriff | Bedeutung | Beispiel |
|---|---|---|
| **Entität** | ein Objekt der realen Welt (wird zu einer Tabelle) | `Kunde`, `Rechnung` |
| **Attribut** | Eigenschaft einer Entität (Spalte) | `Kundennr`, `Name` |
| **Beziehung** | Verbindung zwischen Entitäten | Kunde *stellt aus* Rechnung |
| **Kardinalität** | Anzahl der beteiligten Objekte | 1:1, 1:n, m:n |
| **Fremdschlüssel (FK)** | verweist auf Primärschlüssel einer anderen Tabelle → **referentielle Integrität** | `Rechnung.Kundennr` → `Kunde.Kundennr` |

⚠️ **m:n-Beziehungen** können in relationalen DBs nicht direkt abgebildet werden → sie müssen in eine **Zwischentabelle** (mit zwei FKs) aufgelöst werden.

## 3. SQL – die wichtigsten Befehle (Übungsstoff, real geprüft nur 2021W–2022W)

`SELECT, WHERE, ORDER BY, COUNT, GROUP BY, INNER JOIN, INSERT, UPDATE, DELETE`

Beispieltabelle `tblKunde`:

| Kundennr | Name | Ort | Umsatz |
|---|---|---|---|
| 1 | Müller | Berlin | 5000 |
| 2 | Schmidt | Hamburg | 2000 |
| 3 | Meyer | Berlin | 8000 |

```sql
-- 1) Alle Kunden aus Berlin, sortiert nach Name
SELECT * FROM tblKunde WHERE Ort = 'Berlin' ORDER BY Name;

-- 2) Anzahl Kunden pro Ort (Gruppierung + Zählung)
SELECT Ort, COUNT(*) AS Anzahl FROM tblKunde GROUP BY Ort;

-- 3) Inner Join: Rechnungen mit Kundennamen (tblRechnung hat FK Kundennr)
SELECT k.Name, r.Betrag
FROM tblKunde k
INNER JOIN tblRechnung r ON k.Kundennr = r.Kundennr;

-- 4) Datensatz ändern / löschen
UPDATE tblKunde SET Ort = 'München' WHERE Kundennr = 2;
DELETE FROM tblKunde WHERE Kundennr = 3;
```

## 3b. Datenbankmodellierung/Normalisierung als SQL-Nachfolgethema (2025W)

Seit SQL 2023 aus GA1 verschwunden ist, wurde stattdessen **Datenbank-
modellierung** geprüft. **2025W** konkret: **3. Normalform**,
**Primärschlüssel** und **Kardinalitäten** in einem ER-Diagramm.

- **1. Normalform (1NF):** nur atomare (nicht weiter teilbare) Werte je Feld,
  keine Wiederholungsgruppen.
- **2. Normalform (2NF):** 1NF **und** jedes Nicht-Schlüsselattribut hängt
  **voll funktional** vom (ggf. zusammengesetzten) Primärschlüssel ab
  (keine Abhängigkeit von nur einem Teil des Schlüssels).
- **3. Normalform (3NF):** 2NF **und** keine **transitiven Abhängigkeiten**
  zwischen Nicht-Schlüsselattributen (ein Nicht-Schlüsselattribut darf nicht
  von einem anderen Nicht-Schlüsselattribut abhängen).

Ziel der Normalisierung: Redundanzen vermeiden, Anomalien (Einfüge-, Änderungs-,
Löschanomalien) verhindern.

## 4. Grundlagen – UML

Unterscheidung **statische Sicht** vs. **dynamische Sicht**:

| Sicht | Diagrammtyp | Inhalt |
|---|---|---|
| Statisch | **Klassendiagramm** | Klassen mit Attributen + Methoden, Assoziation, Vererbung |
| Statisch | Komponentendiagramm | Bausteine/Module des Systems |
| Dynamisch | **Aktivitätsdiagramm** | Ablauf: Aktivität, Entscheidung/Verzweigung, Swimlane (Zuständigkeitsbereiche) |
| Dynamisch | Sequenzdiagramm | Nachrichtenaustausch zwischen Objekten über die Zeit |

⚠️ **Aktivitätsdiagramme** werden in der AP2 oft als **Zeichenaufgabe** verlangt – Symbole beherrschen: Startknoten, Aktivität (Rundrechteck), Entscheidung (Raute mit [Bedingung]), Swimlane (horizontale/vertikale Trennung nach Rolle).

**Klassendiagramm:** Klasse = Rechteck mit 3 Bereichen (Klassenname / Attribute / Methoden). Vererbung = gepunktete Linie mit leerer Dreiecksspitze, Assoziation = durchgezogene Linie.

## 5. Übungsaufgabe (im Prüfungsstil, selbst erstellt)

> Gegeben ist `tblKunde` (Kundennr, Name, Ort, Umsatz) mit den drei Beispieldatensätzen aus §3.
>
> a) Schreiben Sie eine SQL-Abfrage, die den Gesamtumsatz pro Ort ermittelt (nur Orte mit Umsatz > 5000).
> b) Zeichnen Sie (stichwortartig beschreiben) ein Aktivitätsdiagramm für "Bestellung aufgeben": Start → Warenkorb prüfen → [leer?] Ja: Abbrechen / Nein: Zahlung → Ende.

<details>
<summary>Lösung (aufklappen)</summary>

a)
```sql
SELECT Ort, SUM(Umsatz) AS Gesamt
FROM tblKunde
GROUP BY Ort
HAVING SUM(Umsatz) > 5000;
```
Ergebnis: Berlin = 13000 (5000+8000) → erfüllt die Bedingung; Hamburg = 2000 → fällt raus.

b) Aktivitätsdiagramm:
- **Startknoten**
- Aktivität **Warenkorb prüfen**
- **Entscheidung** [leer?]
  - Ja → Aktivität **Abbrechen** → Ende
  - Nein → Aktivität **Zahlung** → **Ende**
- Optional in **Swimlane** "Kunde" / "System" aufteilen.

</details>

## 6. Selbstcheck – kannst du das?

- [ ] Ich kann Entität, Attribut, Beziehung und Kardinalität erklären
- [ ] Ich weiß, wie m:n in einer relationalen DB aufgelöst wird
- [ ] Ich kann SELECT/WHERE/ORDER BY/GROUP BY/INNER JOIN sicher formulieren
- [ ] Ich kann referentielle Integrität über Fremdschlüssel erklären
- [ ] Ich unterscheide statische von dynamischen UML-Diagrammen
- [ ] Ich kann ein Aktivitätsdiagramm mit Entscheidung/Swimlane skizzieren

## 7. Weiterführende Ressourcen

- **w3schools SQL Tutorial** (sql.w3schools.com) – interaktiv SELECT/JOIN/UPDATE üben
- **Wikipedia: Entity-Relationship-Modell** – Notationen (Chen, Krähenfuß) im Überblick
- **UML筋 (uml-diagrams.org)** – Aktivitäts- und Klassendiagramm-Symbole
- Altklausuren in deinem Repo (GA2-Aufgaben zu Datenmodellierung und SQL)

---
*Ende der Lektionen-Reihe (12–14).*
