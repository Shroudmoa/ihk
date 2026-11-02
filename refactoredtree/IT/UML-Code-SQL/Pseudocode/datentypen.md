# Datentypen in der Programmierung

## In 30 Sekunden

- Ein **Datentyp** legt fest, welche Werte eine Variable aufnehmen kann, wie viel Speicher sie belegt und welche Operationen erlaubt sind.
- Wertebereich bei n Bit: **ohne Vorzeichen 0 bis 2ⁿ − 1**, **mit Vorzeichen −2ⁿ⁻¹ bis 2ⁿ⁻¹ − 1**.
- `int` (32 Bit): **−2.147.483.648 bis 2.147.483.647** – 2024S wörtlich abgefragt.
- `byte` (C#, 8 Bit, ohne Vorzeichen): 0 bis 255 – passt für Prozentwerte wie CPU-Last.
- Geld nie in `float`/`double`, sondern `decimal` (Programm) bzw. `DECIMAL(p,s)` (SQL).
- 2021W: Reicht `int` für 16 TiB in KiB? Nein, 17.179.869.184 > 2.147.483.647 → `long` (64 Bit).

## Kernwissen

### Ganzzahlen (C#-Bezeichnungen, wie in der Prüfungsanlage)

| Typ | Bit | Vorzeichen | Wertebereich |
|---|---:|---|---|
| `byte` | 8 | nein | 0 … 255 |
| `sbyte` | 8 | ja | −128 … 127 |
| `short` | 16 | ja | −32.768 … 32.767 |
| `ushort` | 16 | nein | 0 … 65.535 |
| `int` | 32 | ja | −2.147.483.648 … 2.147.483.647 |
| `uint` | 32 | nein | 0 … 4.294.967.295 |
| `long` | 64 | ja | ca. −9,22 · 10¹⁸ … 9,22 · 10¹⁸ |
| `ulong` | 64 | nein | 0 … ca. 1,84 · 10¹⁹ |

In Java ist `byte` dagegen **vorzeichenbehaftet** (−128 … 127). In der IHK-Anlage gilt die C#-Variante.

### Gleitkomma und Dezimal

| Typ | Bit | Genauigkeit | Einsatz |
|---|---:|---|---|
| `float` | 32 | ca. 7 Stellen | Grafik, Sensorwerte |
| `double` | 64 | ca. 15–16 Stellen | technische Berechnungen, Mittelwerte |
| `decimal` | 128 | 28–29 Stellen, dezimal exakt | **Geldbeträge** |

`0.1 + 0.2` ergibt bei `double` `0.30000000000000004`, weil 0,1 binär nicht exakt darstellbar ist (IEEE 754).

### Weitere Typen

| Typ | Inhalt | Hinweis |
|---|---|---|
| `bool` | `true` / `false` | belegt praktisch 1 Byte |
| `char` | ein Zeichen, z. B. `'A'` | C#: 16 Bit (UTF-16); `'A'` = 65 |
| `string` | Zeichenkette, z. B. `"Hallo"` | Referenztyp |
| `DateTime` | Datum und Uhrzeit | in 2021W, 2022W, 2024S verwendet |
| Array `int[]` | mehrere Werte gleichen Typs | Index ab 0, feste Länge |
| `struct` | eigener zusammengesetzter Werttyp | z. B. `diskBelegung { int wert; DateTime datum; }` |

**Werttyp vs. Referenztyp:** Werttypen (`int`, `bool`, `struct`) speichern den Wert direkt; Referenztypen (`string`, Arrays, Objekte) speichern einen Verweis auf den Speicherbereich. Eine Zuweisung kopiert beim Werttyp den Wert, beim Referenztyp nur den Verweis.

### Typumwandlung (Cast)

```csharp
int summe = 115, anzahl = 4;
double falsch  = summe / anzahl;           // 28.0  (ganzzahlige Division zuerst)
double richtig = (double)summe / anzahl;   // 28.75

int x = 300;
byte b = (byte)x;                          // 44 – Überlauf: 300 − 256
int zahl = int.Parse("42");                // Text → Zahl, FormatException bei "abc"
```

### Bitbedarf berechnen

Benötigte Bit für N verschiedene Werte: kleinstes n mit **2ⁿ ≥ N**.

- 100 Prozentwerte (0–100 = 101 Werte): 2⁷ = 128 ≥ 101 → **7 Bit**, praktisch `byte`.
- 16 TiB in KiB = 16 · 2³⁰ = 2³⁴ = 17.179.869.184 → braucht 35 Bit (ohne Vorzeichen) → `int` reicht nicht, `long` nötig.

## Prüfungsfrage

**Frage (angelehnt an 2024S):** Geben Sie den Wertebereich des Datentyps `int` an. Begründen Sie, warum für die Variablen `max` und `max2`, die CPU-Auslastungen in Prozent speichern, `byte` statt `int` verwendet wird.

**Musterantwort:** `int` ist eine 32-Bit-Ganzzahl mit Vorzeichen, Wertebereich **−2.147.483.648 bis 2.147.483.647** (2³² = 4.294.967.296 Werte). Für CPU-Auslastungen genügt `byte`: Der Wertebereich 0 bis 255 deckt 0 bis 100 % vollständig ab, `byte` belegt nur 1 statt 4 Byte (75 % weniger Speicher) und kann keine negativen Werte aufnehmen, die bei einer Auslastung fachlich unsinnig wären.

## Falle

- **byte in Java vs. C#:** In C# 0 … 255, in Java −128 … 127.
- **Summen in `byte`:** 24 Prozentwerte summiert ergeben bis 2.400 – Überlauf. Die Summe gehört in `int`.
- **Ganzzahldivision:** `int / int` schneidet Nachkommastellen ab, auch wenn das Ergebnis in `double` gespeichert wird.
- **PLZ als Zahl:** `01234` verliert die führende Null – als `string` speichern.
- **KiB vs. KB:** 1 KiB = 1.024 Byte, 1 KB = 1.000 Byte.
- **Geld als `double`:** Rundungsfehler. `decimal` verwenden.

## Merksatz

**"Zwei hoch Bits minus eins"** ist das Maximum ohne Vorzeichen; mit Vorzeichen halbiert sich der Bereich. `int` ≈ ±2,1 Milliarden.

## Verwandt

- [Grundlagen & Syntax](01_Grundlagen_Syntax.md) – Variablen, Arrays, Casts
- [Fehlerarten & Exceptions](fehlerarten-exceptions.md) – Overflow, FormatException
- [DDL, DML, DCL](../sql/ddl-dml.md) – SQL-Datentypen (2022W)
- [Bit & Byte](../../FiSi-Administrator/OSI-IP-mac-bit-byte/bit-byte.md) – Einheiten und Umrechnung
- [Lektion 4: Programmierung](../../analyse-der-exams/04_Programmierung_Grundlagen.md) – Wertebereich `int` (2024S), Bitbedarf (2021W)
