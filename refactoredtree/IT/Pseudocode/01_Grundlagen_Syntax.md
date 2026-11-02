# 01. Grundlagen, Syntax & Datentypen

## 1.1 Datentypen und Wertebereiche

Für die IHK musst du die wichtigsten Datentypen und ihre Wertebereiche kennen.

| Typ | Bits | Wertebereich | Bemerkung |
|---|---|---|---|
| `byte` | 8 | 0 ... 255 | Nur positive Ganzzahlen, sehr speichersparend |
| `sbyte` | 8 | -128 ... 127 | Signed Byte |
| `short` | 16 | -32.768 ... 32.767 | |
| `ushort` | 16 | 0 ... 65.535 | |
| `int` | 32 | -2.147.483.648 ... 2.147.483.647 | Standard für Ganzzahlen |
| `uint` | 32 | 0 ... 4.294.967.295 | |
| `long` | 64 | -9.223.372.036.854.775.808 ... 9.223.372.036.854.775.807 | Für sehr große Zahlen |
| `string` | - | Zeichenkette | Text |
| `bool` | - | `true` / `false` | Wahrheitswert |
| `DateTime` | - | Datum und Uhrzeit | Kommt seltener, aber wiederholt vor |

**Merksatz:** Wenn ein Wert 0-100 sein soll (z.B. CPU-Auslastung), nimm `byte` statt `int` – passt in Wertebereich, weniger Speicher, verhindert versehentliche negative Werte.

## 1.2 Variablen & Zuweisung

```csharp
int summe = 0;        // Deklaration und Initialisierung
int wert = 42;
summe = summe + wert; // Zuweisung
```

- `int x = 5;` → Variable `x` vom Typ `int` mit Wert `5`
- `=` ist eine **Zuweisung**, nicht ein Vergleich!
- `==` ist ein **Vergleich**

## 1.3 Arrays

Arrays sind der häufigste Datentyp in Prüfungsaufgaben.

```csharp
int[] werte = new int[] { 12, 10, 40, 73, 33, 60 };
// Indizes:                0   1   2   3   4   5
```

- **Index beginnt bei 0:** Erstes Element ist `[0]`
- **`.Length`:** Gibt die Anzahl der Elemente zurück (`6` im Beispiel)
- **Zugriff:** `werte[0]` liefert `12`, `werte[3]` liefert `73`

## 1.4 Verzweigungen (if/else)

```csharp
if (wert > 50) {
    Console.WriteLine("Über 50");
} else if (wert == 50) {
    Console.WriteLine("Genau 50");
} else {
    Console.WriteLine("Unter 50");
}
```

**Wichtige Operatoren:**
| Operator | Bedeutung |
|---|---|
| `>` | größer als |
| `<` | kleiner als |
| `>=` | größer oder gleich |
| `<=` | kleiner oder gleich |
| `==` | gleich (Vergleich!) |
| `!=` | ungleich |
| `&&` | logisches UND |
| `\|\|` | logisches ODER |

## 1.5 Schleifen

### for-Schleife (sehr häufig)

```csharp
for (int i = 0; i < werte.Length; i++) {
    Console.WriteLine(werte[i]);
}
```

Aufbau: `for (Initialisierung; Bedingung; Inkrement) { ... }`

- `int i = 0` → Startwert
- `i < werte.Length` → Solange wahr, wird Schleife ausgeführt
- `i++` → Nach jedem Durchlauf wird `i` um 1 erhöht

### while-Schleife

```csharp
int i = 0;
while (i < 5) {
    // ...
    i++;
}
```

### do-while-Schleife

```csharp
int i = 0;
do {
    // Wird mindestens einmal ausgeführt
    i++;
} while (i < 5);
```

## 1.6 Typumwandlung

```csharp
int summe = 150;
int anzahl = 4;
double mittelwert = (double)summe / anzahl; // Ergibt 37,5
```

Ohne `(double)` würde eine **ganzzahlige Division** erfolgen: `150 / 4 = 37` (Kommaanteil wird abgeschnitten)!

**Faustregel:** Willst du einen echten Durchschnitt mit Nachkommastellen, musst du mindestens einen Operanden in `double` casten.

## Übungsaufgabe 1

Gegeben:
```csharp
byte auslastung = 75;
int[] werte = new int[] { 10, 20, 30, 40 };
```

Fragen:
1. Wie viele Elemente hat `werte`?
2. Welchen Wert hat `werte[2]`?
3. Warum wäre `byte` für `auslastung` hier sinnvoll?

**Lösung:**
1. `werte.Length = 4` (Indizes 0-3)
2. `werte[2] = 30`
3. Wertebereich 0-100 passt in byte (0-255), speichersparender und verhindert negative Werte

**Weiter:** [02_Arrays_Mittelwert_Max.md](02_Arrays_Mittelwert_Max.md)
