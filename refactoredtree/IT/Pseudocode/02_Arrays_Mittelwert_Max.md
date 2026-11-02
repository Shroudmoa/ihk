# 02. Arrays, Mittelwert, Max & Min

## 2.1 Mittelwertberechnung (Sehr häufig!)

Der Klassiker der IHK-Prüfung. Kommt praktisch jedes Jahr vor.

### Aufgabe
Berechne den Mittelwert eines Arrays mit Integer-Werten.

### Lösung

```csharp
int[] CPULoad = new int[] { 12, 10, 40, 73, 33, 60 };
int summe = 0;

// Summe berechnen
for (int i = 0; i < CPULoad.Length; i++) {
    summe += CPULoad[i];
}

// Mittelwert berechnen (mit Komma)
double mittelwert = (double)summe / CPULoad.Length;

Console.WriteLine("Mittelwert: " + mittelwert);
```

**Wichtige Punkte:**
- `summe` muss vor der Schleife auf `0` initialisiert werden
- Ohne `(double)`-Cast: Integer-Division!
- `CPULoad.Length` verwenden, nicht hartkodierte Zahl (flexibler)

## 2.2 Maximum finden

```csharp
int[] werte = new int[] { 12, 10, 40, 73, 33, 60 };
int max = werte[0]; // Erstes Element als Ausgangswert

for (int i = 1; i < werte.Length; i++) { // Ab Index 1 starten!
    if (werte[i] > max) {
        max = werte[i];
    }
}

Console.WriteLine("Maximum: " + max);
```

**Warum `werte[0]` als Startwert?**
Wir nehmen das erste Element als Vergleichswert, damit wir direkt vergleichen können. Falls wir `max = 0` nehmen und alle Werte negativ wären, wäre das falsch.

**Wichtig:** Bei positivem Array funktioniert auch `max = 0`, aber besser und sicherer ist `max = werte[0]`.

## 2.3 Minimum finden

```csharp
int[] werte = new int[] { 12, 10, 40, 73, 33, 60 };
int min = werte[0];

for (int i = 1; i < werte.Length; i++) {
    if (werte[i] < min) {
        min = werte[i];
    }
}
```

## 2.4 Zweithöchster Wert finden (Klassiker für Fehlersuche!)

Das ist DER häufigste Fehlersuche-Task in der IHK.

```csharp
int[] CPULoad = new int[] { 12, 10, 40, 73, 33, 60 };
int max = 0;
int max2 = 0;

for (int i = 0; i < CPULoad.Length; i++) {
    if (CPULoad[i] > max) {
        max2 = max;        // Alter Max wird zweithöchster
        max = CPULoad[i];   // Neuer Max
    } else if (CPULoad[i] > max2 && CPULoad[i] != max) {
        max2 = CPULoad[i];
    }
}

Console.WriteLine("Max: " + max + ", Zweithöchster: " + max2);
```

### Häufiger Fehler (2024S-Prüfung!)

Falscher Code:
```csharp
else if (CPULoad[i] > max2)
    max = CPULoad[i];   // FALSCH! Hier muss max2 = CPULoad[i];
```

**Richtig:** Wenn der Wert größer als `max2`, aber kleiner oder gleich `max` ist, dann wird `max2` überschrieben, **nicht** `max`!

**Merksatz:** 
- Neuer Höchstwert → alter Höchstwert wird zweithöchster
- Zwischen `max2` und `max` → wird zweithöchster
- Kleiner als `max2` → ignorieren

## 2.5 Schreibtischtest (Tabelle ausfüllen)

In der Prüfung musst du oft für jeden Schleifendurchlauf die Variablenwerte in eine Tabelle eintragen.

**Beispiel:**
```csharp
int[] arr = new int[] { 5, 12, 3, 12, 8 };
int max = arr[0]; // 5
int max2 = 0;
for (int i = 1; i < arr.Length; i++) {
    if (arr[i] > max) {
        max2 = max;
        max = arr[i];
    } else if (arr[i] > max2) {
        max2 = arr[i];
    }
}
```

**Tabelle:**

| i | arr[i] | Bedingung | max | max2 |
|---|---|---|---|---|
| - | Start | - | 5 | 0 |
| 1 | 12 | 12 > 5 → true | 12 | 5 |
| 2 | 3 | 3 > 12? nein; 3 > 5? nein | 12 | 5 |
| 3 | 12 | 12 > 12? nein; 12 > 5? ja | 12 | 12 |
| 4 | 8 | 8 > 12? nein; 8 > 12? nein | 12 | 12 |

**Tipp:** Fang immer beim Startwert an und geh Zeile für Zeile durch den Code.

## Übungsaufgaben

### Aufgabe 2a: Mittelwert
Gegeben: `{15, 25, 35, 45}`
Berechne den Mittelwert. Welche Codezeile fehlt für eine korrekte Kommazahl?
```csharp
int summe = 0;
for (int i = 0; i < werte.Length; i++) summe += werte[i];
double mw = summe / werte.Length; // Fehlerhaft?
```

**Lösung:** `double mw = (double)summe / werte.Length;`

### Aufgabe 2b: Fehlersuche
Finde den Fehler:
```csharp
int max = werte[0];
int max2 = 0;
for (int i = 0; i < werte.Length; i++) { // Start bei 0 statt 1
    if (werte[i] > max) {
        max2 = max;
        max = werte[i];
    } else if (werte[i] > max2) {
        max2 = werte[i];
    }
}
```

**Lösung:** Schleife startet bei `i = 0` statt `i = 1`. Wenn `werte[0]` bereits `max` ist, wird beim ersten Durchlauf verglichen - das kann zu unerwarteten Ergebnissen führen, aber vor allem unnötig. Besser ab Index 1 starten. Problematischer wird's wenn der alte `max2`-Wert verloren geht. Meistens wird gefordert: "Starte die Schleife ab Index 1, da max bereits werte[0] ist."

**Weiter:** [03_Schleifen_Fehlersuche.md](03_Schleifen_Fehlersuche.md)
