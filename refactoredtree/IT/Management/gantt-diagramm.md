# Gantt-Diagramm (Balkenplan)

## In 30 Sekunden

- Das **Gantt-Diagramm** stellt Vorgänge als **horizontale Balken auf einer Zeitachse** dar: Balkenlänge = Dauer, Balkenposition = Start und Ende.
- Zusätzlich möglich: **Meilensteine** (Raute, Dauer 0), **Abhängigkeiten** (Pfeile, meist EA), **Fortschritt** (gefüllter Balkenanteil), **Puffer** und **kritischer Pfad** (farbig).
- Stärke: Termine und Parallelität auf einen Blick, gut für Kommunikation mit Auftraggeber und Team.
- Schwäche: Abhängigkeiten und Puffer bei großen Projekten schlecht erkennbar – dafür gibt es den **Netzplan**.
- Werkzeuge: MS Project, GanttProject, OpenProject, Jira-Timeline, Excel.

## Kernwissen

### Bestandteile

| Element | Darstellung | Bedeutung |
|---|---|---|
| Vorgang | Balken | Arbeitspaket mit Dauer |
| Sammelvorgang | schwarzer Klammerbalken | Phase, fasst Vorgänge zusammen (aus dem PSP) |
| Meilenstein | ◆ | Ereignis ohne Dauer, z. B. "Abnahme erteilt" |
| Abhängigkeit | Pfeil | Anordnungsbeziehung (EA, AA, EE, AE) |
| Fortschritt | gefüllter Teil des Balkens | Fertigstellungsgrad |
| Heute-Linie | senkrechte Linie | Stichtag für Soll-Ist-Vergleich |
| Puffer | gestrichelte Verlängerung | Zeit, um die ein Vorgang sich verschieben darf |

### Rechenbeispiel: vom Vorgangsplan zum Gantt-Diagramm

Projekt "Neuer Fileserver" (Angaben in Arbeitstagen, Start = Zeitpunkt 0, alle AOB = EA):

| Nr. | Vorgang | Dauer | Vorgänger |
|---|---|---:|---|
| A | Anforderungsanalyse | 2 | – |
| B | Hardware bestellen und liefern | 5 | A |
| C | Server einrichten | 3 | B |
| D | Datenmigration vorbereiten | 4 | A |
| E | Migration und Test | 2 | C, D |
| M | Meilenstein "Go-Live" | 0 | E |

Vorwärtsrechnung:

| Vorgang | FAZ | FEZ | Gesamtpuffer |
|---|---:|---:|---:|
| A | 0 | 2 | 0 |
| B | 2 | 7 | 0 |
| C | 7 | 10 | 0 |
| D | 2 | 6 | **4** (spätestens bei 6 beginnen, da E bei 10 startet) |
| E | 10 | 12 | 0 |
| M | 12 | 12 | 0 |

```
Tag (Beginn)  0         1
              0123456789012
A Analyse     ██
B Hardware      █████
C Server             ███
D Migr.vorb.    ████░░░░
E Migration             ██
M Go-Live                 ◆

█ = Vorgang (ein Zeichen = ein Tag), ░ = Puffer von D, ◆ = Meilenstein
Kritischer Pfad: A – B – C – E
```

**Kritischer Pfad:** A → B → C → E = 2 + 5 + 3 + 2 = **12 Tage**. D darf sich um bis zu 4 Tage verschieben, ohne das Projektende zu gefährden.

### Gantt vs. Netzplan

| Kriterium | Gantt-Diagramm | Netzplan |
|---|---|---|
| Fokus | **Wann** läuft was? | **Was hängt wovon ab?** |
| Zeitachse | ja, maßstäblich | nein, Zeiten stehen in den Knoten |
| kritischer Pfad | nur wenn markiert | direkt aus der Rechnung (GP = 0) |
| Puffer | optional dargestellt | wird berechnet (GP, FP) |
| Verständlichkeit | sehr hoch, auch für Laien | erfordert Methodenwissen |
| große Projekte | schnell unübersichtlich | übersichtlicher für Abhängigkeiten |

In der Praxis ergänzen sich beide: Der Netzplan liefert die Zeiten, das Gantt-Diagramm zeigt sie an.

## Prüfungsfrage

**Frage:** Erstellen Sie aus der obigen Vorgangsliste ein Gantt-Diagramm, ermitteln Sie die Projektdauer und erläutern Sie, welche Auswirkungen eine Verzögerung der Datenmigrationsvorbereitung (D) um 3 Tage hat. Nennen Sie einen Vorteil des Gantt-Diagramms gegenüber dem Netzplan.

**Musterantwort:** Die Balken werden ab den frühesten Anfangszeitpunkten eingetragen: A 0–2, B 2–7, C 7–10, D 2–6, E 10–12, Meilenstein Go-Live bei 12. Die Projektdauer beträgt **12 Tage**, der kritische Pfad ist A–B–C–E. D hat einen Gesamtpuffer von 4 Tagen (E beginnt erst bei Tag 10, D endet bereits bei Tag 6). Eine Verzögerung von D um 3 Tage verschiebt das Ende von D auf Tag 9; das liegt noch innerhalb des Puffers, das **Projektende bleibt bei 12 Tagen**. Vorteil des Gantt-Diagramms: Es zeigt die zeitliche Lage und Parallelität der Vorgänge maßstabsgetreu und ist für Auftraggeber ohne Methodenwissen leicht verständlich.

## Falle

- **Balken nach Ende statt Dauer zeichnen:** Länge = Dauer, Startpunkt = FAZ.
- **Meilenstein als Balken:** Meilensteine haben keine Dauer und werden als Raute gezeichnet.
- **Parallelität = Ressourcen vorhanden:** Laufen B und D parallel, braucht man genug Personal – mit dem [Ressourcenplan](ressourcenplan.md) prüfen.
- **Kritischer Pfad = längster Balken:** Er ist die längste **Kette** abhängiger Vorgänge.
- **Kalendertage und Arbeitstage mischen:** Wochenenden verlängern die Kalenderdauer.

## Merksatz

**"Gantt zeigt das Wann, der Netzplan das Warum."** Balkenlänge = Dauer, Raute = Meilenstein.

## Verwandt

- [Netzplan](netzplan.md) – Vorwärts-/Rückwärtsrechnung, Puffer
- [AOB – Anordnungsbeziehungen](aob.md) – Pfeile zwischen Balken
- [Projektstrukturplan (PSP)](projektstrukturplan-psp.md) – Vorgänge und Sammelvorgänge
- [Ressourcenplan](ressourcenplan.md) – Auslastung bei parallelen Balken
- [Soll-Ist-Vergleich](soll-ist-vergleich.md) – Fortschritt gegen Plan
- [Projektmanagement-Grundlagen](projektmanagement-grundlagen.md) – Phasen und Meilensteine
