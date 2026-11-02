# Aktivitätsdiagramm

> Dynamisches UML-Diagramm zur Modellierung von Abläufen, Geschäftsprozessen und Arbeitsflüssen

## In 30 Sekunden

- Das **Aktivitätsdiagramm** ist ein **dynamisches** UML-Diagramm und zeigt den **Ablauf** von Aktionen – ähnlich einem Flussdiagramm.
- Elemente: **Startknoten** (●), **Aktion** (abgerundetes Rechteck), **Entscheidung/Zusammenführung** (◇ mit `[Bedingung]`), **Gabelung/Vereinigung** (Balken für Parallelität), **Endknoten** (◉).
- **Swimlanes** (Partitionen) zeigen, wer für welche Aktion zuständig ist.
- Geprüft: **2022S** – Aktivitätsdiagramm zur regelbasierten Verschiebung von Dateien von NAS-1 nach NAS-2 vervollständigen (14 Punkte).

## Kernwissen

### Zweck

Das **Aktivitätsdiagramm** zeigt den **Ablauf** von Aktivitäten. Es ist besonders gut für Geschäftsprozesse, Algorithmen oder Use-Cases geeignet. In der IHK wird es **sehr häufig als Zeichenaufgabe** verlangt!

### Grundelemente

| Symbol | Name | Beschreibung |
|---|---|---|
| ⚫ (gefüllter Kreis) | **Startknoten** | Beginn des Ablaufs |
| ⬛ mit abgerundeten Ecken | **Aktivität** | Auszuführende Aktion/Tätigkeit (Verb + Objekt empfohlen) |
| ◇ (Raute) | **Entscheidungsknoten** | Verzweigung nach Bedingung (Ja/Nein, true/false) |
| ───► | **Fluss/Übergang** | Richtung des Ablaufs |
| ◉ (Kreis mit Punkt) | **Endknoten** | Ende des Ablaufs |

### Swimlanes (Verantwortlichkeitsbereiche)

**Swimlanes** trennen den Ablauf nach Zuständigkeiten (Personen, Rollen, Abteilungen oder Systeme).

```
┌─────────────┬─────────────┬─────────────┐
│    Kunde     │   Vertrieb   │   Lager      │
├─────────────┼─────────────┼─────────────┤
│  Bestellung │              │              │
│  aufgeben    │              │              │
│      │       │              │              │
│      ▼       │  Prüfen      │              │
│              │  Verfügbar?  │              │
│              │      │       │              │
│              │     Ja/Nein  │              │
└─────────────┴─────────────┴─────────────┘
```

**Nutzen:** Sofort ersichtlich, wer was wann macht.

### Fork & Join (parallele Abläufe)

- **Fork (Gabelung):** Ein Fluss wird in mehrere parallele Flüsse aufgeteilt
- **Join (Zusammenführung):** Parallele Flüsse werden wieder synchronisiert

Dargestellt als **dicker Balken**.

### Beispiel: "Urlaub beantragen" (mit Swimlanes)

```
┌─────────────┬─────────────┬─────────────┐
│    Mitarbeiter│   Vorgesetzter│   HR-System   │
├─────────────┼─────────────┼─────────────┤
│      ●       │              │              │
│      │       │              │              │
│      ▼       │              │              │
│  Urlaubsantrag│              │              │
│   erstellen  │              │              │
│      │       │              │              │
│      └───►   │  Antrag      │              │
│              │  prüfen      │              │
│              │      │       │              │
│              │   Genehmigt? │              │
│              │    / \       │              │
│             Nein Ja         │              │
│              │    │         │              │
│      ◄───────┘    └───►     │  Genehmigung │
│                          │  erfassen    │
│                          │      │       │
│                          │      ▼       │
│                          │      ◉       │
└─────────────┴─────────────┴─────────────┘
```

### Entscheidungsregeln

- **Entscheidungsknoten:** Hat immer **genau eine eingehende** und **mindestens zwei ausgehende** Kanten
- **Bedingungen in eckigen Klammern:** `[Genehmigt]`, `[Verfügbar == ja]`
- **Gegensätzliche Bedingungen:** Verwende `[Ja]` / `[Nein]` oder `[Wahr]` / `[Falsch]`

### Unterschiede zu anderen Diagrammen

| Diagramm | Fokus |
|---|---|
| **Aktivitätsdiagramm** | **Ablauf/Prozess** - "Was passiert wann?" |
| **Sequenzdiagramm** | **Interaktion/Kommunikation** - "Wer sendet welche Nachricht wann?" |
| **Use-Case-Diagramm** | **Funktionalität** - "Was kann der Nutzer tun?" |

### Typische Prüfungsaufgaben

1. **Vervollständigen:** Fehlende Symbole/Bedingungen ergänzen
2. **Zeichnen:** Ablauf aus Textbeschreibung erstellen
3. **Fehlersuche:** Fehler im Aktivitätsdiagramm finden
4. **Swimlanes zuordnen:** Zuständigkeiten korrekt einteilen

## Prüfungsfrage

> Erstellen Sie ein Aktivitätsdiagramm für den Vorgang "Artikel bestellen": 
> Kunde legt Artikel in Warenkorb → Prüfe ob Warenkorb leer → Falls leer Abbruch, sonst zur Kasse → Zahlung durchführen → Bestellung bestätigen.

**Musterantwort:**

1. Start ●
2. Artikel in Warenkorb legen
3. Warenkorb prüfen
4. Entscheidung: [Warenkorb leer?]
   - Ja → Abbruch → Ende ◉
   - Nein → Zur Kasse gehen
5. Zahlung durchführen
6. Bestellung bestätigen
7. Ende ◉

### Checkliste für Aktivitätsdiagramme

- [ ] **Startknoten** vorhanden
- [ ] **Aktivitäten** kurz und prägnant formuliert (Substantiv+Verb oder Infinitiv)
- [ ] **Entscheidungen** mit `[Bedingung]` beschriftet
- [ ] **Jeder Pfad** endet bei Endknoten
- [ ] **Swimlanes** falls Zuständigkeiten gefragt sind
- [ ] **Logische Reihenfolge** korrekt

## Falle

- **Bedingungen ohne eckige Klammern** oder nur ein Ausgang an der Raute: Jede Entscheidung braucht beschriftete, sich ausschließende Ausgänge (`[ja]`/`[nein]`).
- **Überlappende Bedingungen** wie `> 40 %` und `> 60 %` ohne Obergrenze – in 2022S waren die Bereiche 40–60, 60–80 und über 80 % sauber zu trennen.
- **Kein Endknoten** auf einem Pfad oder mehrere Startknoten.
- **Gabelung (Parallelität) mit Entscheidung verwechseln:** Der Balken startet alle Pfade gleichzeitig, die Raute wählt einen.
- **Aktivitätsdiagramm mit Zustandsdiagramm verwechseln:** Aktionen sind Tätigkeiten, keine Zustände.

## Merksatz

- **Aktivitätsdiagramm = Flussdiagramm light** - Ähnelt Ablaufdiagrammen, aber UML-konform
- **Entscheidung = Ja/Nein-Frage** - Immer zwei oder mehr Ausgänge
- **Swimlane = Wer ist dran?** - Sehr häufig in IHK-Aufgaben

## Verwandt

- [Klassendiagramm](Klassendiagramm.md)
- [Anwendungsfalldiagramm-Use-Case-Diagramm](Anwendungsfalldiagramm-Use-Case-Diagramm.md)
- [Sequenzdiagramm](Sequenzdiagramm.md)
- [Zustandsdiagramm](zustandsdiagramm.md)
- [UML-Übersicht](readme.md)
- [Lektion 14: Datenmodellierung, SQL & UML](../../analyse-der-exams/L14_Datenmodellierung_SQL_UML.md)
- [Struktogramm & PAP](../Pseudocode/struktogramm-pap.md) – Abläufe ohne UML
