# ER-Modell und Kardinalitäten

## In 30 Sekunden

- Das **Entity-Relationship-Modell (ERM)** beschreibt Daten **fachlich**, bevor Tabellen angelegt werden.
- **Entität** = Objekt (wird zur Tabelle), **Attribut** = Eigenschaft (wird zur Spalte), **Beziehung** = Verbindung (wird zu Fremdschlüssel oder Zwischentabelle).
- **Kardinalität** = wie viele Objekte an einer Beziehung beteiligt sind: **1:1, 1:n, m:n**.
- **1:n** → Fremdschlüssel auf der **n-Seite**. **m:n** → eigene **Zwischentabelle** mit zwei Fremdschlüsseln.
- Jede Tabelle braucht einen **Primärschlüssel** (eindeutig, nicht NULL).
- Geprüft: 2022S (Kardinalitäten Windpark – Windrad – Betriebsdaten, referentielle Integrität), 2025W (Primärschlüssel und Kardinalitäten in einem ERM mit Kategorie, Teil, Lieferant, Bestellung, Bestellposition).

## Kernwissen

### Bausteine

| Begriff | Bedeutung | Chen-Notation | Beispiel |
|---|---|---|---|
| Entität / Entitätstyp | Objekt bzw. Klasse gleichartiger Objekte | Rechteck | `Kunde` |
| Attribut | Eigenschaft | Ellipse | `Name`, `Ort` |
| Schlüsselattribut | identifiziert eine Entität eindeutig | unterstrichene Ellipse | `KundenNr` |
| Beziehung | Verbindung zwischen Entitäten | Raute | Kunde *erteilt* Auftrag |
| Kardinalität | Anzahl beteiligter Objekte | `1`, `n`, `m` an den Linien | 1 : n |

### Schlüsselarten

| Schlüssel | Bedeutung |
|---|---|
| **Primärschlüssel (PK)** | ausgewählter Schlüssel, eindeutig, nie NULL, unveränderlich |
| **Fremdschlüssel (FK)** | Spalte, die auf den PK einer anderen Tabelle verweist |
| **Zusammengesetzter Schlüssel** | PK aus mehreren Spalten, z. B. `(BestellNr, PosNr)` |
| **Kandidatenschlüssel** | jede minimale eindeutige Spaltenkombination, aus der der PK gewählt wird |
| **Surrogatschlüssel** | künstliche ID ohne fachliche Bedeutung (`AUTO_INCREMENT`) |
| **Natürlicher Schlüssel** | fachlicher Wert, z. B. ISBN, Kfz-Kennzeichen |

### Kardinalitäten und ihre Umsetzung

| Kardinalität | Beispiel | Umsetzung in Tabellen |
|---|---|---|
| **1:1** | Mitarbeiter – Firmenparkplatz | FK auf einer Seite mit `UNIQUE`, oder Tabellen zusammenlegen |
| **1:n** | Abteilung – Mitarbeiter | FK `AbtID` in **Mitarbeiter** (n-Seite) |
| **m:n** | Student – Kurs | Zwischentabelle `Belegung(StudentID, KursID)`; PK = beide FKs |

**Leserichtung:** "Eine Abteilung hat **n** Mitarbeiter, ein Mitarbeiter gehört zu **1** Abteilung." Man liest immer von einer Entität aus und fragt: "Mit wie vielen der anderen ist **ein** Exemplar verbunden?"

### Min-Max- bzw. Krähenfuß-Notation

| Krähenfuß-Symbol | Bedeutung | (min,max) |
|---|---|---|
| `──┼┼──` | genau eins | (1,1) |
| `──o┼──` | null oder eins | (0,1) |
| `──┼<──` | eins oder viele | (1,n) |
| `──o<──` | null oder viele | (0,n) |

Die (min,max)-Notation sagt auch, ob die Beziehung **Pflicht** (min = 1) oder **optional** (min = 0) ist.

### Vom ERM zur Tabelle – Beispiel 2025W (Lager/Einkauf)

```
Kategorie 1 ──── n Teil 1 ──── n Bestellposition n ──── 1 Bestellung n ──── 1 Lieferant
```

| Tabelle | Primärschlüssel | Fremdschlüssel |
|---|---|---|
| Kategorie | KategorieID | – |
| Teil | TeilID | KategorieID → Kategorie |
| Lieferant | LieferantID | – |
| Bestellung | BestellungID | LieferantID → Lieferant |
| Bestellposition | BestellpositionID (oder `BestellungID + PosNr`) | BestellungID → Bestellung, TeilID → Teil |

Die **Bestellposition** ist die aufgelöste m:n-Beziehung zwischen Bestellung und Teil: Eine Bestellung enthält viele Teile, ein Teil kommt in vielen Bestellungen vor. Die Position trägt zusätzlich die Attribute der Beziehung (`Menge`, `Einzelpreis`).

### Referentielle Integrität (2022S)

- Der FK-Wert muss im referenzierten PK existieren oder NULL sein.
- **INSERT** eines Betriebsdatensatzes mit nicht existierender `IDWindrad` wird abgelehnt.
- **DELETE** eines Windrads, zu dem Betriebsdaten existieren, wird abgelehnt (`RESTRICT`) oder kaskadiert (`CASCADE`).
- Ziel: keine **verwaisten Datensätze** (Orphans).

### Schwache Entität

Eine schwache Entität ist ohne übergeordnete Entität nicht identifizierbar, z. B. `Bestellposition` ohne `Bestellung`. Ihr Schlüssel enthält den Schlüssel der starken Entität (`BestellungID + PosNr`). Im UML-Klassendiagramm entspricht das einer **Komposition**.

## Prüfungsfrage

**Frage:** Ein Windparkbetreiber speichert Windparks, Windräder und tägliche Betriebsdaten. Jedes Windrad steht in genau einem Windpark, pro Windrad werden täglich Betriebsdaten erfasst. Zusätzlich sollen Techniker erfasst werden, die mehrere Windräder warten; ein Windrad wird von mehreren Technikern gewartet. Geben Sie die Kardinalitäten an und beschreiben Sie die Umsetzung in Tabellen.

**Musterantwort:**
- **Windpark – Windrad: 1:n.** Ein Windpark hat viele Windräder, ein Windrad gehört zu genau einem Windpark. Umsetzung: FK `IDWindpark` in `Windrad`.
- **Windrad – Betriebsdaten: 1:n.** FK `IDWindrad` in `Betriebsdaten`.
- **Techniker – Windrad: m:n.** Umsetzung über die Zwischentabelle `Wartung(IDTechniker, IDWindrad, Datum)` mit zwei Fremdschlüsseln; PK z. B. `(IDTechniker, IDWindrad, Datum)`.
- Mit aktivierter **referentieller Integrität** lassen sich keine Betriebsdaten zu einem nicht existierenden Windrad speichern.

## Falle

- **FK auf der falschen Seite:** Bei 1:n gehört der FK **immer auf die n-Seite**. Eine Spalte `MitarbeiterIDs` in `Abteilung` verletzt die 1. Normalform.
- **m:n direkt abbilden:** In relationalen Datenbanken unmöglich, es braucht eine Zwischentabelle.
- **Kardinalität verkehrt herum gelesen:** Immer "ein X hat wie viele Y?" für beide Richtungen beantworten.
- **Fachliche Attribute als PK:** Name oder E-Mail können sich ändern und sind nicht immer eindeutig. Lieber Surrogatschlüssel.
- **Beziehungsattribute vergessen:** Menge und Preis gehören in die Zwischentabelle, nicht in `Teil` oder `Bestellung`.

## Merksatz

**"Der Fremdschlüssel wandert zum n."** Und m:n braucht immer einen Vermittler: die Zwischentabelle.

## Verwandt

- [Normalisierung](normalisierung.md) – 1. bis 3. Normalform
- [DDL, DML, DCL](ddl-dml.md) – PK und FK mit `CREATE TABLE` anlegen
- [SQL-Joins](joins.md) – Tabellen über FK wieder verbinden
- [Klassendiagramm](../UML/Klassendiagramm.md) – Multiplizitäten und Komposition in UML
- [NoSQL-Datenbanktypen](nosql-datenbanktypen.md) – wenn kein Tabellenmodell passt
- [Lektion 14: Datenmodellierung, SQL & UML](../../analyse-der-exams/L14_Datenmodellierung_SQL_UML.md)
