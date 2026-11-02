# 04. Praxistests & Musterlösungen

## Test 1: Mittelwertberechnung (Klassiker)

**Aufgabe:**
Gegeben sei folgendes Array mit CPU-Auslastungswerten in Prozent:
```csharp
int[] CPULoad = new int[] { 45, 62, 38, 71, 55 };
```
Berechne den arithmetischen Mittelwert. Gib den vollständigen, korrekten Code an.

**Musterlösung:**
```csharp
int[] CPULoad = new int[] { 45, 62, 38, 71, 55 };
int summe = 0;

for (int i = 0; i < CPULoad.Length; i++) {
    summe += CPULoad[i];
}

double mittelwert = (double)summe / CPULoad.Length;
Console.WriteLine("Durchschnittliche CPU-Auslastung: " + mittelwert + " %");
```

**Punktevergabe:** Summe bilden (2), Schleife korrekt (2), Division mit Cast (3), Ausgabe (1)

## Test 2: Maximum und Zweithöchster Wert

**Aufgabe:**
Finde das Maximum und den zweithöchsten Wert aus folgendem Array:
```csharp
int[] werte = new int[] { 12, 87, 45, 87, 23 };
```
Hinweis: Der Wert 87 kommt doppelt vor.

**Musterlösung:**
```csharp
int[] werte = new int[] { 12, 87, 45, 87, 23 };
int max = werte[0];
int max2 = 0;

for (int i = 1; i < werte.Length; i++) {
    if (werte[i] > max) {
        max2 = max;
        max = werte[i];
    } else if (werte[i] > max2 && werte[i] != max) {
        max2 = werte[i];
    }
}

// Ausgabe: Max: 87, Zweithöchster: 45
```

**Erklärung:** Beim zweiten Vorkommen von 87 ist `werte[i] > max` false und `werte[i] != max` false, daher wird `max2` nicht überschrieben. Korrekt ist 45.

## Test 3: Fehlersuche (2024S-Stil)

**Aufgabe:**
Der folgende Code soll das Maximum und den zweithöchsten Wert ermitteln. Er enthält einen Fehler. 
Lokalisieren Sie die fehlerhafte Stelle, erläutern Sie die Fehlerursache und korrigieren Sie den Code.

```csharp
int[] CPULoad = new int[] { 12, 10, 40, 73, 33, 60 };
int max = 0;
int max2 = 0;

for (int i = 0; i < CPULoad.Length; i++) {
    if (CPULoad[i] > max) {
        max2 = max;
        max = CPULoad[i];
    } else if (CPULoad[i] > max2)
        max = CPULoad[i];
}
```

**Musterlösung:**

- **Fehlerhafte Stelle:** Zeile mit `max = CPULoad[i];` im `else if`-Zweig
- **Fehlerursache:** An dieser Stelle wird nicht der zweithöchste Wert (`max2`) überschrieben, sondern der Höchstwert (`max`). Dadurch geht der bisherige Höchstwert verloren.
- **Korrektur:**
```csharp
else if (CPULoad[i] > max2)
    max2 = CPULoad[i];  // Statt max
```

## Test 4: Schreibtischtest

**Aufgabe:**
Führen Sie den folgenden Code manuell durch und tragen Sie die Werte von `i`, `arr[i]`, `max` und `max2` für jeden Schleifendurchlauf in die Tabelle ein.

```csharp
int[] arr = new int[] { 5, 12, 3, 12, 8 };
int max = arr[0];
int max2 = 0;
for (int i = 1; i < arr.Length; i++) {
    if (arr[i] > max) {
        max2 = max;
        max = arr[i];
    } else if (arr[i > max2] == false) // Trickfrage? Oder normal
        // besser normal
    }
}
```

Vereinfacht korrekter Code für Test:
```csharp
int[] arr = new int[] { 5, 12, 3, 12, 8 };
int max = 5;
int max2 = 0;
for (int i = 1; i < 5; i++) {
    if (arr[i] > max) {
        max2 = max;
        max = arr[i];
    } else if (arr[i] > max2) {
        max2 = arr[i];
    }
}
```

**Lösungstabelle:**

| i | arr[i] | Bedingung(en) | max | max2 |
|---|---|---|---|---|
| (Start) | - | - | 5 | 0 |
| 1 | 12 | `12 > 5` → true | 12 | 5 |
| 2 | 3 | `3 > 12` false, `3 > 5` false | 12 | 5 |
| 3 | 12 | `12 > 12` false, `12 > 5` true | 12 | 12 |
| 4 | 8 | `8 > 12` false, `8 > 12` false | 12 | 12 |

**Ergebnis:** max = 12, max2 = 12

## Test 5: Bubblesort verstehen

**Aufgabe:**
Was macht folgender Ausschnitt? Welche Elemente sind nach dem ersten Durchlauf richtig sortiert?

```csharp
for (int i = 0; i < zahlen.Length - 1; i++) {
    for (int j = 0; j < zahlen.Length - tag 1 - i; j++) {
        if (zahlen[j] > zahlen[j + 1]) {
            int temp = zahlen[j];
            zahlen[j] = zahlen[j + 1];
            zahlen[j + 1] = temp;
        }
    }
}
```

**Musterlösung:**
- **Funktion:** Bubblesort zum Aufsteigend sortieren (kleinste zuerst)
- **Nach 1. Durchlauf:** Das größte Element ist an der richtigen Position am Ende des Arrays
- **Grund:** Bei jedem Tausch wandert das größere Element um einen Platz nach rechts, beim Durchlauf wandert das größte Element schließlich ganz ans Ende

## Test 6: Datentypen

**Auffrage:**
Für eine CPU-Auslastung in Prozent (0-100) soll ein Datentyp gewählt werden.
a) Warum ist `byte` besser als `int`?
b) Reicht `int` für den Wertbereich auch?
c) Welches Problem könnte bei der Wahl von `byte` entstehen?

**Musterlösung:**
a) `byte` ist speichersparender (1 Byte statt 4 Bytes) und deckt den benötigten Wertebereich (0-255) vollständig ab.
b) Ja, `int` reicht auch, ist aber unnötig groß für diesen Zweck.
c) Falls versehentlich negative Werte eingegeben werden, kann `byte` diese nicht darstellen. Außerdem ist der Wertebereich zwar ausreichend, aber man muss auf keine negativen Werte bedacht sein. Sinnvoller ist `byte` hier aber trotzdem, da keine negativen Werte auftreten sollen.

## Prüftipps für Pseudocode

### Allgemein
- **Zeile für Zeile lesen:** Nicht überfliegen, sondern Variablen verfolgen
- **Schreibtischtest immer machen:** Kostet wenig Zeit, spart Punkte
- **Indizes checken:** 0-basiert, `< Length` nicht `<= Length`
- **Zuweisung vs Vergleich:** `=` vs `==`

### Bei Fehlersuche
- **Genau benennen:** "Zeile X: Statt `max2` wird `max` zugewingwiesen"
- **Ursache erklären:** Kurz und knackig, warum das falsch ist
- **Korrektur angeben:** Die korrekte Zeile schreiben

### Bei Schleifen
- **Startwert checken:** Wenn `max = werte[0]`, dann Schleife ab 1 starten
- **Grenzwert checken:** Letzter Durchlauf kritisch betrachten
- **Tausch immer mit temp:** Wenn Tausch verlangt wird, `temp` nicht vergessen

## Schnelle Checkliste vor Abgabe

- [ ] Habe ich den Code vollständig durchgelesen?
- [ ] Habe ich eine Tabelle für den Schreibtischtest angelegt?
- [ ] Sind alle Indizes korrekt (`< Length`)?
- [ ] Habe ich bei Division an Typumwandlung gedacht?
- [ ] Ist `max`-Initialisierung sinnvoll (`werte[0]` statt `0`)?
- [ ] Habe ich Zuweisung `=` vom Vergleich `==` unterschieden?
- [ ] Habe ich Fehler klar lokalisiert und begründet?

**Zurück zur Übersicht:** [00_Einleitung.md](00_Einleitung.md)
