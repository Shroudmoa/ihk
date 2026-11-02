# Struktogramm und Programmablaufplan (PAP)

## In 30 Sekunden

- **Struktogramm** (Nassi-Shneiderman-Diagramm, DIN 66261): Algorithmus als ineinander geschachtelte **Blöcke**. Kein Sprung möglich, erzwingt strukturierte Programmierung.
- **Programmablaufplan (PAP, Flussdiagramm, DIN 66001):** Algorithmus als **Symbole mit Pfeilen**. Sehr anschaulich, erlaubt aber "Spaghetti-Sprünge".
- **Pseudocode:** Algorithmus als strukturierter Text, sprachunabhängig.
- Alle drei bilden die drei Grundstrukturen ab: **Sequenz**, **Verzweigung (Selektion)**, **Schleife (Iteration)**.
- Prüfungsbezug: Die IHK verwendet meist C#-ähnlichen Pseudocode; 2022S war ein **Aktivitätsdiagramm** zu vervollständigen, 2023W fragte die **Vorteile von Pseudocode**.

## Kernwissen

### Die drei Kontrollstrukturen

| Struktur | Pseudocode | Struktogramm | PAP |
|---|---|---|---|
| **Sequenz** | Anweisungen untereinander | Blöcke untereinander | Rechtecke mit Pfeilen |
| **Verzweigung** | `WENN … DANN … SONST` | Dreieck mit "ja"/"nein"-Spalten | Raute mit zwei Ausgängen |
| **Mehrfachauswahl** | `FALLS x = 1: … x = 2: …` | Dreieck mit mehreren Spalten | mehrere Rauten oder eine mit vielen Ausgängen |
| **Kopfgesteuerte Schleife** | `SOLANGE Bedingung TUE …` / `FÜR i = 0 BIS n − 1` | Bedingung oben, Block eingerückt (L-Form) | Raute vor dem Schleifenrumpf, Rückpfeil |
| **Fußgesteuerte Schleife** | `WIEDERHOLE … BIS Bedingung` | Bedingung unten (umgedrehte L-Form) | Raute nach dem Rumpf |

### PAP-Symbole (DIN 66001)

| Symbol | Bedeutung |
|---|---|
| Oval / abgerundetes Rechteck | Start, Ende |
| Rechteck | Anweisung, Verarbeitung |
| Raute | Verzweigung (Bedingung mit ja/nein) |
| Parallelogramm | Ein-/Ausgabe |
| Rechteck mit doppelten Seitenlinien | Unterprogramm-Aufruf |
| Pfeil | Ablaufrichtung |
| Kreis | Verbindungsstelle (Konnektor) |

### Beispiel: Mittelwert mit Warnung

**Pseudocode**

```
summe ← 0
FÜR i ← 0 BIS laenge(werte) − 1
    summe ← summe + werte[i]
ENDE FÜR
mittelwert ← summe / laenge(werte)
WENN mittelwert > 80 DANN
    Ausgabe "Warnung: hohe CPU-Last"
SONST
    Ausgabe "Last in Ordnung"
ENDE WENN
```

**Struktogramm (textuell gezeichnet)**

```
┌──────────────────────────────────────────────┐
│ summe ← 0                                    │
├──────────────────────────────────────────────┤
│ für i von 0 bis Länge − 1                    │
│   ┌──────────────────────────────────────────┤
│   │ summe ← summe + werte[i]                 │
├───┴──────────────────────────────────────────┤
│ mittelwert ← summe / Länge                   │
├──────────────────────────────────────────────┤
│ \            mittelwert > 80 ?             / │
│   \ ja                               nein /  │
├──────────────────────┬───────────────────────┤
│ Ausgabe "Warnung"    │ Ausgabe "in Ordnung"  │
└──────────────────────┴───────────────────────┘
```

**PAP (textuell)**

```
( Start )
    │
[ summe = 0, i = 0 ]
    │
◇ i < Länge ? ──nein──┐
    │ ja               │
[ summe += werte[i] ]  │
[ i = i + 1 ]          │
    └──► zurück zur Raute
                       │
[ mittelwert = summe / Länge ] ◄┘
    │
◇ mittelwert > 80 ? ──ja──► / Ausgabe "Warnung" / ──┐
    │ nein                                         │
/ Ausgabe "in Ordnung" /                           │
    │ ◄────────────────────────────────────────────┘
( Ende )
```

### Vergleich

| Kriterium | Pseudocode | Struktogramm | PAP |
|---|---|---|---|
| Erstellung | schnell, Text | aufwendig, Änderungen schwierig | mittel |
| Strukturzwang | mittel | hoch (keine Sprünge) | gering |
| Lesbarkeit für Laien | gut | mittel | sehr gut |
| Nähe zum Code | hoch | mittel | gering |

**Vorteile von Pseudocode (2023W):** unabhängig von einer Programmiersprache, auch für Nicht-Programmierer verständlich, Fokus auf der Logik statt auf Syntaxdetails, schnell zu schreiben und gute Vorlage für die Implementierung.

## Prüfungsfrage

**Frage:** Ein Skript soll für 24 Stundenwerte der CPU-Last zählen, wie oft der Wert 80 % überschreitet, und eine Meldung ausgeben, wenn das mehr als 18-mal der Fall war (angelehnt an 2023S). Stellen Sie den Algorithmus als Pseudocode dar und nennen Sie die verwendeten Kontrollstrukturen.

**Musterantwort:**

```
zaehler ← 0
FÜR i ← 0 BIS 23
    WENN usedCPU[i] > 80 DANN
        zaehler ← zaehler + 1
    ENDE WENN
ENDE FÜR
WENN zaehler > 18 DANN
    Ausgabe "CPU dauerhaft überlastet"
ENDE WENN
```

Verwendet werden eine **Sequenz** (Initialisierung, danach Schleife, danach Prüfung), eine **kopfgesteuerte Zählschleife** (FÜR) und zwei **einseitige Verzweigungen** (WENN ohne SONST).

## Falle

- **Raute im PAP mit nur einem Ausgang:** Jede Verzweigung braucht ja **und** nein.
- **Kopf- und fußgesteuert verwechselt:** Die fußgesteuerte Schleife läuft **mindestens einmal**.
- **Schleifengrenze im Pseudocode:** `FÜR i ← 0 BIS 23` sind 24 Durchläufe. `BIS 24` wäre ein Indexfehler.
- **Struktogramm mit Pfeilen:** Struktogramme haben keine Pfeile, die Reihenfolge ergibt sich von oben nach unten.
- **Ein-/Ausgabe als Rechteck im PAP:** Dafür ist das Parallelogramm vorgesehen.

## Merksatz

**Sequenz, Auswahl, Wiederholung** – jedes Programm besteht nur aus diesen drei Bausteinen. Struktogramm = Kästen ohne Pfeile, PAP = Symbole mit Pfeilen.

## Verwandt

- [Grundlagen & Syntax](01_Grundlagen_Syntax.md) – if/else und Schleifen in C#-Syntax
- [Schleifen & Fehlersuche](03_Schleifen_Fehlersuche.md)
- [Aktivitätsdiagramm](../UML/Aktivitaetsdiagramm.md) – UML-Variante des Ablaufplans (2022S)
- [Algorithmen: Sortieren, Suchen, Rekursion](algorithmen-sortieren-suchen-rekursion.md)
- [Lektion 4: Programmierung](../../analyse-der-exams/04_Programmierung_Grundlagen.md) – Pseudocode-Vorteile 2023W
