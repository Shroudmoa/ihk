# AOB – Anordnungsbeziehungen

## In 30 Sekunden

- **Anordnungsbeziehungen (AOB)** beschreiben die **logisch-zeitliche Abhängigkeit zwischen zwei Vorgängen** in Netzplan und Gantt-Diagramm (DIN 69900).
- Vier Arten, benannt nach dem Zeitpunkt des **Vorgängers** und dann des **Nachfolgers**:
  **EA** Ende–Anfang (Normalfolge), **AA** Anfang–Anfang, **EE** Ende–Ende, **AE** Anfang–Ende (Sprungfolge).
- **EA** ist mit Abstand am häufigsten und Standard in Prüfungsnetzplänen.
- Ein **Zeitabstand** (Lag) verschiebt die Beziehung: positiv = Wartezeit, negativ = Überlappung (Lead).
- AOB ist **nicht** "aufwandsorientierte Budgetierung" – die Abkürzung steht in der Projektplanung für Anordnungsbeziehung.

## Kernwissen

### Die vier Anordnungsbeziehungen

| Kürzel | Name | Regel | IT-Beispiel |
|---|---|---|---|
| **EA** | Normalfolge | B kann **anfangen**, wenn A **beendet** ist | Server erst installieren, wenn die Hardware geliefert ist |
| **AA** | Anfangsfolge | B kann **anfangen**, wenn A **angefangen** hat | Dokumentation beginnt zusammen mit der Konfiguration |
| **EE** | Endfolge | B kann erst **enden**, wenn A **beendet** ist | Testen endet erst, wenn die Programmierung beendet ist |
| **AE** | Sprungfolge | B kann erst **enden**, wenn A **angefangen** hat | Altsystem läuft weiter, bis das neue System startet (Parallelbetrieb) |

### Zeitabstände

| Schreibweise | Bedeutung |
|---|---|
| EA + 2 | B beginnt frühestens **2 Tage nach dem Ende** von A (z. B. Lieferzeit, Aushärten, Wartefrist) |
| EA − 1 | B beginnt bereits **1 Tag vor dem Ende** von A (Überlappung) |
| AA + 3 | B beginnt frühestens **3 Tage nach dem Beginn** von A |
| EE + 1 | B endet frühestens **1 Tag nach dem Ende** von A |

### Rechenbeispiel (Zählweise: Projektstart = Zeitpunkt 0)

| Vorgang | Dauer | Vorgänger, AOB |
|---|---:|---|
| A Hardware bestellen und liefern | 5 Tage | – |
| B Server installieren | 3 Tage | A, EA |
| C Dokumentation | 4 Tage | B, AA + 1 |
| D Abnahmetest | 2 Tage | B, EA + 2 (Wartefrist für Monitoring) |

Vorwärtsrechnung (FAZ = frühester Anfang, FEZ = frühestes Ende):

| Vorgang | FAZ | FEZ | Rechnung |
|---|---:|---:|---|
| A | 0 | 5 | Start |
| B | 5 | 8 | EA: FAZ(B) = FEZ(A) = 5 |
| C | 6 | 10 | AA + 1: FAZ(C) = FAZ(B) + 1 = 6 |
| D | 10 | 12 | EA + 2: FAZ(D) = FEZ(B) + 2 = 10 |

Projektende: **12 Tage** (max. FEZ von C = 10 und D = 12). Hätte C eine EA-Beziehung zu B, wäre C erst bei 8 gestartet und bei 12 fertig – die AA-Beziehung spart hier Zeit.

### Darstellung

- **Vorgangsknotennetzplan:** Pfeil zwischen den Knoten, beschriftet mit AOB und Zeitabstand (fehlt die Angabe, gilt EA ohne Abstand).
- **Gantt-Diagramm:** Pfeil vom Balkenende (E) bzw. Balkenanfang (A) des Vorgängers zum Anfang bzw. Ende des Nachfolgers.

## Prüfungsfrage

**Frage:** Erläutern Sie die Anordnungsbeziehungen "Normalfolge" und "Anfangsfolge" an einem Beispiel aus einem Rollout-Projekt und berechnen Sie den frühesten Anfang von Vorgang B, wenn A (Dauer 4 Tage) bei Zeitpunkt 2 beginnt und B mit "EA + 1" an A gekoppelt ist.

**Musterantwort:** Bei der **Normalfolge (EA)** darf der Nachfolger erst beginnen, wenn der Vorgänger abgeschlossen ist, z. B. dürfen die Clients erst verteilt werden, wenn das Image fertig getestet ist. Bei der **Anfangsfolge (AA)** darf der Nachfolger beginnen, sobald der Vorgänger begonnen hat, z. B. kann die Schulung der Anwender beginnen, sobald der Rollout startet. **Rechnung:** FEZ(A) = 2 + 4 = 6; FAZ(B) = FEZ(A) + 1 = **7**.

## Falle

- **Reihenfolge der Buchstaben:** Der **erste** Buchstabe gehört zum **Vorgänger**, der zweite zum **Nachfolger**. EA = "Ende des Vorgängers → Anfang des Nachfolgers".
- **Zeitabstand vergessen:** EA + 2 heißt, der Nachfolger wartet 2 Zeiteinheiten.
- **AE mit EA verwechseln:** Die Sprungfolge ist selten und betrifft das **Ende** des Nachfolgers.
- **Zählweise mischen:** Entweder ab Zeitpunkt 0 (FEZ = FAZ + Dauer) oder ab Tag 1 (FEZ = FAZ + Dauer − 1) rechnen – nicht beides.
- **AOB als Budgetbegriff:** In der Projektplanung ist AOB immer die Anordnungsbeziehung.

## Merksatz

**"Erst der Vorgänger, dann der Nachfolger – EA ist die Normalfolge."** Plus heißt warten, Minus heißt überlappen.

## Verwandt

- [Netzplan](netzplan.md) – Vorwärts- und Rückwärtsrechnung, kritischer Pfad
- [Gantt-Diagramm](gantt-diagramm.md) – AOB als Pfeile zwischen Balken
- [Projektstrukturplan (PSP)](projektstrukturplan-psp.md) – liefert die Arbeitspakete
- [PERT](pert.md) – Dauern unter Unsicherheit
- [Ressourcenplan](ressourcenplan.md)
