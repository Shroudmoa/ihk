# Algorithmen: Sortieren, Suchen, Rekursion

## In 30 Sekunden

- **Bubblesort:** benachbarte Elemente vergleichen und tauschen, das größte wandert pro Durchlauf ans Ende. Aufwand **O(n²)**. 2022S als Lückencode geprüft.
- Weitere einfache Sortierverfahren: **Selectionsort** (kleinstes suchen und nach vorn tauschen) und **Insertionsort** (wie Spielkarten einsortieren), beide O(n²). Schnell: **Quicksort**, **Mergesort** mit O(n · log n).
- **Lineare Suche:** jedes Element prüfen, O(n), funktioniert immer.
- **Binäre Suche:** nur auf **sortierten** Daten, halbiert den Suchbereich jedes Mal, O(log n).
- **Rekursion:** Funktion ruft sich selbst auf, braucht eine **Abbruchbedingung**, sonst `StackOverflowException`.
- Prüfungsnah sind außerdem Varianten der Min/Max-Suche mit Index (2026S: Task mit höchster Priorität = kleinster Wert).

## Kernwissen

### Bubblesort

```csharp
int[] werte = { 30, 24, 12, 50, 11 };
int temp;
for (int p = 0; p < werte.Length - 1; p++) {          // Durchläufe
    for (int i = 0; i < werte.Length - 1 - p; i++) {  // Vergleiche
        if (werte[i] > werte[i + 1]) {                // > aufsteigend, < absteigend
            temp = werte[i];
            werte[i] = werte[i + 1];
            werte[i + 1] = temp;
        }
    }
}
```

| Nach Durchlauf | Array | fest am Ende |
|---|---|---|
| Start | 30, 24, 12, 50, 11 | – |
| p = 0 | 24, 12, 30, 11, **50** | 50 |
| p = 1 | 12, 24, 11, **30, 50** | 30 |
| p = 2 | 12, 11, **24, 30, 50** | 24 |
| p = 3 | **11, 12, 24, 30, 50** | alles |

**Optimierung:** Ein Flag `getauscht` – wird in einem Durchlauf nichts getauscht, ist das Array sortiert und die äußere Schleife kann abbrechen.

### Selectionsort

```csharp
for (int i = 0; i < a.Length - 1; i++) {
    int minIndex = i;
    for (int j = i + 1; j < a.Length; j++) {
        if (a[j] < a[minIndex]) minIndex = j;
    }
    int temp = a[i]; a[i] = a[minIndex]; a[minIndex] = temp;
}
```

### Vergleich der Sortierverfahren

| Verfahren | Prinzip | Aufwand (Durchschnitt) | stabil |
|---|---|---|---|
| Bubblesort | Nachbarn tauschen | O(n²) | ja |
| Selectionsort | Minimum suchen und nach vorn | O(n²) | nein |
| Insertionsort | in sortierten Teil einfügen | O(n²), bei fast sortierten Daten schnell | ja |
| Quicksort | Pivot wählen, aufteilen, rekursiv | O(n · log n), schlechtester Fall O(n²) | nein |
| Mergesort | halbieren, rekursiv sortieren, mischen | O(n · log n) | ja |

**Stabil** heißt: Gleiche Werte behalten ihre ursprüngliche Reihenfolge.

### Lineare Suche

```csharp
int Suche(int[] a, int gesucht) {
    for (int i = 0; i < a.Length; i++) {
        if (a[i] == gesucht) return i;   // Index des ersten Treffers
    }
    return -1;                           // nicht gefunden
}
```

### Binäre Suche (Array muss sortiert sein)

```csharp
int BinaerSuche(int[] a, int gesucht) {
    int links = 0, rechts = a.Length - 1;
    while (links <= rechts) {
        int mitte = (links + rechts) / 2;
        if (a[mitte] == gesucht) return mitte;
        if (a[mitte] < gesucht) links = mitte + 1;
        else rechts = mitte - 1;
    }
    return -1;
}
```

Beispiel: Suche 30 in `{11, 12, 24, 30, 50}` → mitte = 2 (24 < 30) → links = 3 → mitte = 3 (30) → gefunden nach 2 Vergleichen. Bei 1.000.000 Elementen reichen höchstens 20 Vergleiche (2²⁰ ≈ 1,05 Mio.), linear bis zu 1.000.000.

### Min/Max mit Index (Prüfungsmuster 2026S)

```csharp
int[] prio = { 5, 7, 4, 8, 3, 7, 3 };   // kleinere Zahl = wichtiger
int indexMax = 0;
int maxPrio = 10;
for (int i = 0; i < prio.Length; i++) {
    if (prio[i] < maxPrio) {             // "<" behält bei Gleichstand den ersten Index
        maxPrio = prio[i];
        indexMax = i;
    }
}
// Ergebnis: maxPrio = 3, indexMax = 4
```

### Rekursion

```csharp
int Fakultaet(int n) {
    if (n <= 1) return 1;            // Abbruchbedingung (Basisfall)
    return n * Fakultaet(n - 1);     // rekursiver Aufruf
}
// Fakultaet(4) = 4 * 3 * 2 * 1 = 24
```

Aufrufkette: `F(4) → 4·F(3) → 4·3·F(2) → 4·3·2·F(1) → 4·3·2·1 = 24`. Jeder Aufruf legt einen Eintrag auf dem **Stack** ab.

| Rekursiv | Iterativ |
|---|---|
| oft kürzer und eleganter (Bäume, Verzeichnisse durchlaufen) | meist schneller, weniger Speicher |
| Gefahr Stack Overflow bei großer Tiefe | keine Stackgrenze |
| braucht Basisfall | braucht Schleifenbedingung |

**IT-Beispiel:** Alle Dateien eines Verzeichnisbaums auflisten – Funktion listet Dateien und ruft sich für jedes Unterverzeichnis selbst auf.

## Prüfungsfrage

**Frage (angelehnt an 2022S):** Ergänzen Sie den Bubblesort-Algorithmus für das Array `{30, 24, 12, 50, 11, 49, 11}` so, dass aufsteigend sortiert wird, und erläutern Sie, warum die innere Schleife bei jedem Durchlauf kürzer werden darf.

**Musterantwort:**

```csharp
int[] werte = { 30, 24, 12, 50, 11, 49, 11 };
int temp;
int len = werte.Length;
for (int p = 0; p <= len - 2; p++) {
    for (int i = 0; i < len - 1 - p; i++) {
        if (werte[i] > werte[i + 1]) {
            temp = werte[i];
            werte[i] = werte[i + 1];
            werte[i + 1] = temp;
        }
    }
}
```

Nach jedem Durchlauf der äußeren Schleife steht das größte noch unsortierte Element an seiner endgültigen Position am Ende. Nach Durchlauf p sind die letzten p + 1 Elemente sortiert, deshalb muss die innere Schleife nur bis `len − 1 − p` vergleichen. Die Grenze `len − 1` verhindert außerdem den Zugriff auf `werte[i + 1]` außerhalb des Arrays.

## Falle

- **Tausch ohne `temp`:** `a[i] = a[i+1]; a[i+1] = a[i];` – beide Felder haben danach denselben Wert.
- **Innere Schleife bis `< Length`:** `a[i + 1]` läuft im letzten Durchlauf über das Ende → `IndexOutOfRangeException`.
- **Binäre Suche auf unsortierten Daten:** liefert falsche Ergebnisse.
- **Rekursion ohne Basisfall:** endlose Aufrufe → `StackOverflowException`.
- **`<` und `<=` bei Min/Max:** entscheidet, ob bei Gleichstand der erste oder letzte Index gewinnt.

## Merksatz

**Bubblesort: "Die Großen blubbern nach oben."** Binäre Suche: "Halbieren statt durchprobieren – aber nur, wenn sortiert." Rekursion: "Ohne Ausgang kein Ende."

## Verwandt

- [Schleifen & Fehlersuche](03_Schleifen_Fehlersuche.md) – Bubblesort Schritt für Schritt
- [Arrays, Mittelwert, Max](02_Arrays_Mittelwert_Max.md)
- [Fehlerarten & Exceptions](fehlerarten-exceptions.md) – IndexOutOfRange, StackOverflow
- [Struktogramm & PAP](struktogramm-pap.md)
- [Index, Transaktionen & Locking](../sql/index-transaktionen-locking.md) – B-Baum als Suchstruktur in Datenbanken
- [Lektion 4: Programmierung](../../analyse-der-exams/04_Programmierung_Grundlagen.md) – Bubblesort 2022S
