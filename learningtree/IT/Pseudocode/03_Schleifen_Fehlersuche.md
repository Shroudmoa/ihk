# 03. Schleifen, Bubblesort & Fehlersuche

## 3.1 For-Schleife im Detail

```csharp
for (int i = 0; i < 5; i++) {
    Console.WriteLine(i);
}
// Ausgabe: 0,1,2,3,4
```

**Wichtige Varianten:**
- `i++` → post-increment (i = i + 1)
- `i--` → decrement
- `i += 2` → in 2er-Schritten
- Abwärts: `for (int i = 10; i >= 0; i--)`

## 3.2 Bubblesort (2022S geprüft)

Bubblesort vergleicht benachbarte Elemente und tauscht sie, wenn sie in falscher Reihenfolge sind. Größere Werte "blubbern" nach oben/Ende.

### Algorithmus

```csharp
int[] zahlen = new int[] { 5, 2, 9, 1, 5, 6 };
int temp;

// Äußere Schleife: Anzahl der Durchläufe
for (int i = 0; i < zahlen.Length - 1; i++) {
    // Innere Schleife: Vergleiche pro Durchlauf
    for (int j = 0; j < zahlen.Length - 1 - i; j++) {
        if (zahlen[j] > zahlen[j + 1]) {
            // Tausch
            temp = zahlen[j];
            zahlen[j] = zahlen[j + 1];
            zahlen[j + 1] = temp;
        }
    }
}
```

### Warum diese Bedingungen?

- **Äußere Schleife `< zahlen.Length - 1`:** Nach `n-1` Durchläufen ist Array sortiert
- **Innere Schleife `< zahlen.Length - 1 - i`:** Mit jedem Durchlauf ist das letzte Element bereits sortiert, braucht nicht mehr verglichen werden
- **Tausch mit `temp`:** Ohne temporäre Variable würden Werte überschrieben

### Ablauf nachvollziehen (kleines Beispiel)

Array: `[5, 2, 9, 1]`

**Durchlauf 1 (i=0):**
- j=0: `5 > 2` → Tausch → `[2, 5, 9, 1]`
- j=1: `5 > 9`? Nein
- j=2: `9 > 1` → Tausch → `[2, 5, 1, 9]`
Array nach Durchlauf 1: `[2, 5, 1, 9]` (9 ist korrekt am Ende)

**Durchlauf 2 (i=1):**
- j=0: `2 > 5`? Nein
- j=1: `5 > 1` → Tausch → `[2, 1, 5, 9]`

**Durchlauf 3 (i=2):**
- j=0: `2 > 1` → Tausch → `[1, 2, 5, 9]`

Fertig sortiert!

## 3.3 Fehlersuche & Schreibtischtest (Prüfungsklassiker)

In der IHK musst du oft:
1. **Den Fehler lokalisieren** (welche Zeile?)
2. **Die Fehlerursache erläutern**
3. **Den Fehler korrigieren**

### Typische Fehlerarten

| Fehlertyp | Beschreibung | Beispiel |
|---|---|---|
| **Syntaxfehler** | Verstoß gegen Sprachregeln, kompiliert nicht | Fehlendes Semikolon, Klammerfehler |
| **Semantikfehler** | Kompiliert, läuft, aber falsches Ergebnis | Logikfehler, falsche Zuweisung |
| **Laufzeitfehler** | Tritt erst zur Laufzeit auf | `IndexOutOfRangeException`, `DivideByZeroException` |

### Beispiel 1: Falsche Zuweisung (2024S)

```csharp
int max = 0;
int max2 = 0;
for (int i = 0; i < CPULoad.Length; i++) {
    if (CPULoad[i] > max) {
        max2 = max;
        max = CPULoad[i];
    } else if (CPULoad[i] > max2)
        max = CPULoad[i];   // FEHLER!
}
```

**Fehler:** Statt `max2 = CPULoad[i]` wird `max = CPULoad[i]` zugewiesen. Dadurch wird der aktuelle Höchstwert überschrieben und geht verloren.

**Korrektur:** `max2 = CPULoad[i];`

### Beispiel 2: Falsche Schleifenbedingung

```csharp
int[] arr = new int[] { 1, 2, 3, 4, 5 };
for (int i = 0; i <= arr.Length; i++) { // FALSCH!
    Console.WriteLine(arr[i]);
}
// Führt zu IndexOutOfRangeException bei i=5
```

**Fehler:** Bedingung `i <= arr.Length`. Gültige Indizes sind 0-4, bei `i=5` gibt es `arr[5]` nicht.

**Korrektur:** `i < arr.Length`

### Beispiel 3: Division durch 0

```csharp
int summe = 100;
int anzahl = 0;
double mittelwert = summe / anzahl; // DivideByZeroException
```

**Lösung:** Vor Division prüfen oder sicherstellen, dass `anzahl > 0`.

## 3.4 Structs (Strukturen)

Kommt gelegentlich vor (z.B. 2021W).

```csharp
struct Messwert {
    public DateTime Datum;
    public int Wert;
}

Messwert[] Messreihe = new Messwert[10];
Messreihe[0].Wert = 75;
Messreihe[0].Datum = DateTime.Now;
```

**Merkmale:**
- Werttyp (im Gegensatz zu Klassen)
- Fasst zusammengehörige Daten
- Zugriff über Punktnotation: `objekt.Eigenschaft`

## Übungsaufgaben

### Aufgabe 3a: Bubblesort vervollständigen
Vervollständige den fehlenden Tausch:
```csharp
if (zahlen[j] > zahlen[j + 1]) {
    // Tausch
    int temp = zahlen[j];
    zahlen[j] = __________;
    __________ = temp;
}
```

**Lösung:** `zahlen[j] = zahlen[j + 1];` und `zahlen[j + 1] = temp;`

### Aufgabe 3b: Fehlersuche
Nenne Zeile, Fehlerursache und Korrektur:
```csharp
int[] arr = new int[] { 10, 20, 30 };
int max = 0;
for (int i = 0; i < arr.Length; i++) {
    if (arr[i] > max) max = arr[i];
}
```

**Überlegung:** Wenn alle Werte negativ wären, würde `max` bei 0 bleiben (falsch). Besser `max = arr[0]` und Schleife ab Index 1 starten. Bei positiven Werten wie hier funktioniert es, aber der prinzipielle Fehler ist die Wahl des Startwerts.

### Aufgabe 3c: Schreibtischtest
Führe diesen Code gedanklich durch und trage max/max2 ein:
```csharp
int[] x = new int[] { 5, 3, 9, 7 };
int max = x[0]; // 5
int max2 = 0;
for (int i = 1; i < 4; i++) {
    if (x[i] > max) { max2 = max; max = x[i]; }
    else if (x[i] > max2) { max2 = x[i]; }
}
```

**Lösung:**
| i | x[i] | max | max2 |
|---|---|---|---|
| Start | - | 5 | 0 |
| 1 | 3 | 5 | 0 (3>5? nein, 3>0? ja) |
| 2 | 9 | 9 | 5 (9>5? ja) |
| 3 | 7 | 9 | 7 (7>9? nein, 7>5? ja) |

**Weiter:** [04_Praxistests_Musterloesungen.md](04_Praxistests_Musterloesungen.md)
