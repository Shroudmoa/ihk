# Pseudocode-Vorbereitung für die IHK AP2 (FiSi)

> **Ziel:** Du lernst, Pseudocode in der IHK-Prüfung zu lesen, zu verstehen, Fehler zu finden und kleine Aufgaben selbst zu lösen.

## Warum Pseudocode in der AP2?

- **Jährlich geprüft:** In fast jedem GA1-Prüfungsteil 2021-2026 tauchte eine Programmieraufgabe auf (meist Aufgabe 3 oder 4).
- **C#-ähnlich:** Die IHK verwendet eine Syntax, die stark an C# angelehnt ist, aber du brauchst keine tiefgreifenden C#-Kenntnisse.
- **Keine fertige IDE nötig:** Du löst auf Papier - Verstehen der Logik reicht!
- **Punkteträchtig:** Oft nur wenige Zeilen Code, aber leicht Punkte zu holen wenn man weiß worauf man achten muss.

## Typische Aufgabentypen

| Typ | Häufigkeit | Was wird verlangt? |
|---|---|---|
| **Mittelwertberechnung** | ★★★★★ (sehr häufig) | Summe über Array bilden, durch Anzahl teilen |
| **Max/Min/zweithöchster Wert** | ★★★★★ | Vergleichslogik verstehen und Fehler finden |
| **Arrays durchlaufen** | ★★★★★ | `for`-Schleife mit Index `i` |
| **Bubblesort** | ★★★★☆ | Algorithmus vervollständigen oder Ablauf nachvollziehen |
| **Fehlersuche/Schreibtischtest** | ★★★★★ | Fehler finden, benennen, korrigieren |
| **Datentypen/Wertebereiche** | ★★★★☆ | `int`, `byte`, `long` etc. richtig zuordnen |
| **if/else-Verzweigungen** | ★★★★★ | Logik nachverfolgen |
| **Exceptions** | ★★★★☆ | `IndexOutOfRangeException`, `DivideByZeroException` erkennen |

## Prüfungsumfeld

- **Syntax-Anlage:** Meist liegt eine kurze Syntax-Anlage bei (ähnlich zu den in `analyse-der-exams/04_Programmierung_Grundlagen.md` gezeigten Beispielen)
- **C#-ähnliche Syntax:** `Console.WriteLine`, Methodenaufrufe, Arrays, Schleifen
- **Pseudocode-Charakter:** Nicht perfekt C#, sondern prüfungsgerecht vereinfacht
- **Fokus liegt auf Logik**, nicht auf Perfektion

## Wie du am effektivsten lernst

1. **Lesen lernen:** Lies Code Zeile für Zeile, verfolge Variablenwerte
2. **Schreibtischtest:** Trage Werte für jede Iteration in eine Tabelle ein
3. **Fehler erkennen:** Achte auf Indizes (0-basiert), Grenzwerte, Zuweisungen
4. **Üben, üben, üben:** Nimm dir die Beispiele aus den Analyse-Dokumenten vor
5. **Logik verstehen:** Frag dich "Was will dieser Code erreichen?"

## Wichtige Grundregeln

- **Arrays starten bei Index 0:** `array[0]` ist das erste Element
- **Array.Length:** Gibt Anzahl der Elemente zurück
- **Zuweisung `=` vs. Vergleich `==`:** Sehr häufiger Stolperstein
- **Typumwandlung:** `(double)` bei Division für Kommazahlen
- **Schleifenbedingungen:** `< array.Length` nicht `<= array.Length` (verhindert IndexOutOfRange)
- **Variable vor Verwendung initialisieren:** Häufiger Fehler

**Weiter:** [01_Grundlagen_Syntax.md](01_Grundlagen_Syntax.md) → Einstieg in Syntax & Datentypen
