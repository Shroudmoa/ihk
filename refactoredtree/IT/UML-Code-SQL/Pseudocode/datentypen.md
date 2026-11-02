# Datentypen in der Programmierung

Datentypen legen fest, welche Art von Daten eine Variable speichern kann (z.B. Zahlen, Text) und wie viel Speicherplatz dafür reserviert wird.

## 1. Ganzzahlen (Integer)
*   **Eigenschaft:** Speichern positive und negative ganze Zahlen (keine Kommastellen).
*   **Berechnung (IHK-Wissen):**
    *   **Unsigned (ohne Vorzeichen):** \(0\) bis \((2^n - 1)\).
    *   **Signed (mit Vorzeichen):** \(-2^{(n-1)}\) bis \((2^{(n-1)} - 1)\). Ein Bit wird für das Vorzeichen (0 = positiv, 1 = negativ) geopfert.

| Datentyp | Größe | Wertebereich (mit Vorzeichen) | Wertebereich (ohne Vorzeichen) |
| :--- | :--- | :--- | :--- |
| **byte** | 8 Bit | -128 bis 127 | 0 bis 255 |
| **short** | 16 Bit | -32.768 bis 32.767 | 0 bis 65.535 |
| **int** | 32 Bit | -2.147.483.648 bis 2.147.483.647 | 0 bis 4.294.967.295 |
| **long** | 64 Bit | ca. ±9,2 Trillionen | 0 bis ca. 18,4 Trillionen |

## 2. Gleitkommazahlen (Floating Point)
*   **Eigenschaft:** Speichern Zahlen mit Nachkommastellen (z.B. 3,14).
*   **Prüfungsfalle:** Gleitkommazahlen sind **nicht exakt** (Rundungsfehler durch IEEE-754). Für Geldbeträge niemals `float` oder `double` nutzen!

| Datentyp | Größe | Präzision | Einsatzgebiet |
| :--- | :--- | :--- | :--- |
| **float** | 32 Bit | ca. 7 Dezimalstellen | Grafik, Physik (ungenau) |
| **double** | 64 Bit | ca. 15-17 Dezimalstellen | Wissenschaftliche Berechnungen |
| **decimal** | 128 Bit | 28-29 Dezimalstellen | **Finanzen / Geld (exakt)** |

## 3. Wahrheitswerte (Boolean)
*   **Datentyp:** `bool`
*   **Eigenschaft:** Nur zwei Zustände: `true` (wahr / 1) oder `false` (falsch / 0).
*   **Größe:** Theoretisch 1 Bit, in der Praxis (C#/Java) meist 1 Byte (8 Bit).

## 4. Zeichen (Character)
*   **Datentyp:** `char`
*   **Eigenschaft:** Ein einzelnes Zeichen (Buchstabe, Ziffer, Symbol).
*   **Größe:** 16 Bit (UTF-16 in Java/C#) oder 8 Bit (ASCII in C/C++).
*   **Hintergrund:** Wird intern als Zahl gespeichert (z.B. 'A' = 65 im ASCII).

## 5. Komplexe Datentypen (Referenzdatentypen)
*   **Eigenschaft:** Speichern nicht den Wert selbst, sondern eine **Speicheradresse (Referenz)** auf ein Objekt.
*   **Beispiele:**
    *   `String`: Zeichenkette (z.B. "Hallo").
    *   `Array`: Sammlung von Elementen desselben Typs (z.B. `int[] zahlen = {1, 2, 3};`).
    *   `Objekt`: Instanz einer Klasse (z.B. `Kunde` mit Name und Adresse).

## 6. IHK-Prüfungsfallen & Merksätze
*   **`int`-Wertebereich auswendig können:** -2.147.483.648 bis +2.147.483.647.
*   **„byte statt int“:** Immer mit **Speicheroptimierung** argumentieren (75% Ersparnis). Zudem kann `byte` (unsigned) keine negativen Werte speichern, was bei Prozentwerten (CPU-Last, Fortschritt) ideal ist.
*   **Geld nie mit `float` / `double`:** Immer `decimal` nutzen.
*   **Datentyp-Größe:** 8 Bit = 1 Byte. 1024 Byte = 1 KiB (nicht 1000!).




# IHK-Aufgabe: Datentypen (int & byte)

## ba) Wertebereich für den Datentyp „int“
*   **Wertebereich:** **−2.147.483.648 bis +2.147.483.647**
*   **Begründung:** Ein `int` ist eine 32-Bit-Ganzzahl mit Vorzeichen. \(2^{32} = 4.294.967.296\) Zustände. Aufgeteilt auf positive und negative Zahlen (inkl. Null) ergibt das die obigen Grenzen.

## bb) Grund für den Datentyp „byte“ statt „int“ für max und max2
*   **Speicheroptimierung:** Ein `byte` belegt nur **8 Bit** (1 Byte) im Arbeitsspeicher, ein `int` hingegen **32 Bit** (4 Byte). Durch die Wahl von `byte` wird der Speicherverbrauch um 75 % reduziert.
*   *Hinweis für die Prüfung:* Ein `byte` kann Werte von 0 bis 255 speichern. Da eine CPU-Auslastung (in Prozent) niemals über 100 liegen kann, ist ein `byte` vollkommen ausreichend und verhindert zudem negative Werte (da ohne Vorzeichen).
