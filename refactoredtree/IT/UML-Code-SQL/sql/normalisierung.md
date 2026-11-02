# Normalisierung (1. bis 3. Normalform)

## In 30 Sekunden

- **Normalisierung** zerlegt Tabellen schrittweise, um **Redundanz** und **Anomalien** zu vermeiden.
- **1. NF:** Alle Werte sind **atomar**, keine Wiederholungsgruppen, es gibt einen Primärschlüssel.
- **2. NF:** 1. NF und jedes Nicht-Schlüsselattribut hängt vom **ganzen** Primärschlüssel ab (keine **partielle** Abhängigkeit). Nur relevant bei **zusammengesetztem** Schlüssel.
- **3. NF:** 2. NF und kein Nicht-Schlüsselattribut hängt von einem anderen Nicht-Schlüsselattribut ab (keine **transitive** Abhängigkeit).
- Anomalien: **Einfüge-, Änderungs- und Löschanomalie**.
- 2025W fragte explizit die **3. Normalform** ab (zusammen mit PK und Kardinalitäten).

## Kernwissen

### Anomalien in einer nicht normalisierten Tabelle

| Anomalie | Beispiel |
|---|---|
| **Einfügeanomalie** | Ein neuer Kunde kann erst gespeichert werden, wenn er etwas bestellt. |
| **Änderungsanomalie** | Kunde zieht um: Der Ort muss in allen seinen Bestellzeilen geändert werden; wird eine vergessen, ist der Bestand widersprüchlich. |
| **Löschanomalie** | Löscht man die einzige Bestellung eines Kunden, sind auch seine Kundendaten weg. |

### Abhängigkeiten

- **Funktional abhängig:** B hängt von A ab (A → B), wenn zu jedem A-Wert genau ein B-Wert gehört. `KundenNr → Kundenname`.
- **Voll funktional abhängig:** B hängt vom **ganzen** zusammengesetzten Schlüssel ab, nicht schon von einem Teil.
- **Transitiv abhängig:** A → B → C, wobei B kein Schlüssel ist. `BestellNr → KundenNr → Kundenname`.

### Durchgerechnetes Beispiel

**Ausgangstabelle (nicht normalisiert)**

| BestellNr | Datum | KundenNr | Kunde | PLZ | Ort | Artikel |
|---:|---|---:|---|---|---|---|
| 1001 | 2025-03-01 | 7 | Müller GmbH | 86150 | Augsburg | 2× Maus (A1, 15 €), 1× Switch (A9, 120 €) |
| 1002 | 2025-03-02 | 8 | Schmidt KG | 20095 | Hamburg | 5× Maus (A1, 15 €) |

Problem: Die Spalte `Artikel` enthält mehrere Werte (nicht atomar).

**1. NF – atomare Werte, eine Zeile pro Position**

| BestellNr | ArtNr | Datum | KundenNr | Kunde | PLZ | Ort | Bezeichnung | Preis | Menge |
|---:|---|---|---:|---|---|---|---|---:|---:|
| 1001 | A1 | 2025-03-01 | 7 | Müller GmbH | 86150 | Augsburg | Maus | 15 | 2 |
| 1001 | A9 | 2025-03-01 | 7 | Müller GmbH | 86150 | Augsburg | Switch | 120 | 1 |
| 1002 | A1 | 2025-03-02 | 8 | Schmidt KG | 20095 | Hamburg | Maus | 15 | 5 |

Primärschlüssel: **(BestellNr, ArtNr)**. Redundanz: Kundendaten und Artikeldaten stehen mehrfach.

**2. NF – partielle Abhängigkeiten auflösen**

- `Datum, KundenNr, Kunde, PLZ, Ort` hängen nur von `BestellNr` ab → Tabelle **Bestellung**.
- `Bezeichnung, Preis` hängen nur von `ArtNr` ab → Tabelle **Artikel**.
- `Menge` hängt vom ganzen Schlüssel ab → bleibt in **Bestellposition**.

| Bestellposition | Bestellung | Artikel |
|---|---|---|
| **BestellNr, ArtNr**, Menge | **BestellNr**, Datum, KundenNr, Kunde, PLZ, Ort | **ArtNr**, Bezeichnung, Preis |

**3. NF – transitive Abhängigkeiten auflösen**

In `Bestellung` gilt `BestellNr → KundenNr → Kunde, PLZ, Ort`. Die Kundendaten wandern in eine eigene Tabelle. Streng genommen gilt zusätzlich `PLZ → Ort`; ob man auch das auslagert, ist eine Abwägung (in der Praxis oft nicht, weil eine PLZ mehrere Orte haben kann).

| Tabelle | Spalten (PK fett) | FK |
|---|---|---|
| Kunde | **KundenNr**, Kunde, PLZ, Ort | – |
| Bestellung | **BestellNr**, Datum, KundenNr | KundenNr → Kunde |
| Artikel | **ArtNr**, Bezeichnung, Preis | – |
| Bestellposition | **BestellNr, ArtNr**, Menge | BestellNr → Bestellung, ArtNr → Artikel |

Jetzt steht jede Information **genau einmal** in der Datenbank. Umzug eines Kunden = eine Zeile ändern.

### Prüfschema für die Prüfung

1. Gibt es mehrere Werte in einem Feld oder Spalten wie `Telefon1, Telefon2, Telefon3`? → nicht 1. NF.
2. Ist der PK zusammengesetzt und hängt ein Attribut nur von einem Teil ab? → nicht 2. NF.
3. Hängt ein Nicht-Schlüsselattribut von einem anderen Nicht-Schlüsselattribut ab? → nicht 3. NF.

Eine Tabelle mit **einspaltigem** Primärschlüssel, die in der 1. NF ist, ist automatisch in der 2. NF.

### Denormalisierung

Bewusstes Zurücknehmen der Normalisierung, z. B. für Auswertungen (Data Warehouse) oder um teure Joins zu sparen. Der Preis: Redundanz und Pflegeaufwand.

## Prüfungsfrage

**Frage:** Die Tabelle `Geraet(InventarNr, Bezeichnung, MitarbeiterNr, MitarbeiterName, AbteilungNr, Abteilungsname)` ist nicht in der 3. Normalform. Begründen Sie das und überführen Sie die Tabelle in die 3. NF.

**Musterantwort:** Der Primärschlüssel ist `InventarNr`. Die Tabelle ist in der 2. NF, weil der Schlüssel einspaltig ist. Sie verletzt die 3. NF, weil transitive Abhängigkeiten bestehen: `InventarNr → MitarbeiterNr → MitarbeiterName, AbteilungNr` und `MitarbeiterNr → AbteilungNr → Abteilungsname`. Lösung:

- `Abteilung(`**AbteilungNr**`, Abteilungsname)`
- `Mitarbeiter(`**MitarbeiterNr**`, MitarbeiterName, AbteilungNr → Abteilung)`
- `Geraet(`**InventarNr**`, Bezeichnung, MitarbeiterNr → Mitarbeiter)`

Damit wird ein Abteilungsname nur noch einmal gespeichert, Änderungsanomalien entfallen.

## Falle

- **2. NF bei einspaltigem Schlüssel prüfen:** Partielle Abhängigkeit setzt einen zusammengesetzten Schlüssel voraus.
- **2. und 3. NF verwechselt:** 2. NF = Abhängigkeit von einem **Teil des Schlüssels**, 3. NF = Abhängigkeit von einem **Nicht-Schlüssel**.
- **Fremdschlüssel vergessen:** Beim Zerlegen muss die Verbindung erhalten bleiben, sonst gehen Informationen verloren.
- **Berechnete Werte speichern:** `Gesamtpreis = Menge × Preis` als Spalte ist redundant.
- **"Atomar" falsch verstanden:** Eine Adresse `"Hauptstr. 5, 86150 Augsburg"` in einem Feld ist nicht atomar, wenn nach Ort gesucht werden soll.

## Merksatz

**"Der Schlüssel, der ganze Schlüssel und nichts als der Schlüssel."** 1. NF: es gibt einen Schlüssel und atomare Werte. 2. NF: abhängig vom ganzen Schlüssel. 3. NF: von nichts anderem als dem Schlüssel.

## Verwandt

- [ER-Modell & Kardinalitäten](er-modell-kardinalitaeten.md) – Entitäten, PK, FK
- [DDL, DML, DCL](ddl-dml.md) – die normalisierten Tabellen anlegen
- [SQL-Joins](joins.md) – normalisierte Tabellen wieder zusammenführen
- [Index, Transaktionen & Locking](index-transaktionen-locking.md)
- [Lektion 14: Datenmodellierung, SQL & UML](../../analyse-der-exams/L14_Datenmodellierung_SQL_UML.md)
