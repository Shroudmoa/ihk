# 4. Programmierung (Grundlagen)

*Quelle: GA1 Aufgabe 3c (2021), GA1 Aufgabe 3 (2024)*

> ✅ **Verifiziert am echten Prüfungskorpus (10 Jahrgänge 2021–2026):**
> Programmierung kommt in **JEDEM** der 10 Jahrgänge vor (meist 3./4. Aufgabe) –
> durchgehend C#-artige Pseudo-Syntax mit beigelegter Syntax-Anlage
> (`Console.WriteLine`, `MessageBox.Show`, `for`/`while`/`do`-Schleifen, `if`/`else`).
> Konkrete Belege: Array-Mittelwertberechnung (2022W, 2023S, 2023W, 2024S, 2024W,
> 2025S, 2025W, 2026S), Bubblesort (2022S), Fehlersuche/Schreibtischtest
> (2024S, 2024W, 2025S, 2025W, 2026S), Exceptions (2023W, 2025W),
> OOP-Begriffe (2022W, 2023S), `int`-Wertebereich (2024S),
> Syntax- vs. Semantikfehler (2025S), Pseudocode-Vorteile (2023W).

Die Prüfung testet **grundlegendes Verständnis** (kein großes Programmieren),
meist in C#-ähnlicher Syntax. Schwerpunkte:

## 4.1 Datentypen & Wertebereiche

| Typ | Bits | Bereich |
|-----|------|---------|
| `byte` | 8 | 0 … 255 |
| `sbyte` | 8 | -128 … 127 |
| `short` | 16 | -32.768 … 32.767 |
| `ushort` | 16 | 0 … 65.535 |
| `int` | 32 | -2.147.483.648 … 2.147.483.647 |
| `uint` | 32 | 0 … 4.294.967.295 |
| `string` | – | Zeichenkette |
| `bool` | – | `true` / `false` |
| `DateTime` | – | Datum/Uhrzeit (belegt in 2021W, 2022W, 2024S) |

**Prüfung (2024S):** Wertebereich von `int` angeben →
**-2.147.483.648 bis 2.147.483.647**.

**Warum `byte` statt `int`?** (CPU-Auslastung in %):
- Wertebereich (0–100) passt voll in `byte`
- Keine negativen Werte möglich → Fehlerquelle vermieden
- Speicherplatz sparsamer

## 4.2 Arrays

```csharp
int[] CPULoad = new int[] { 12, 10, 40, 73, 33, 60 };
for (int i = 0; i < CPULoad.Length; i++) { /* ... */ }
```
- `CPULoad.Length` liefert die Anzahl der Felder.
- Index beginnt bei **0**.

**Mittelwertberechnung über ein Array** ist der mit Abstand häufigste
Array-Aufgabentyp (CPU-/RAM-Auslastung als eindimensionales `int`-Array):
belegt in **2022W, 2023S, 2023W, 2024S, 2024W, 2025S, 2025W, 2026S** –
also in fast jedem neueren Jahrgang.
```csharp
int summe = 0;
for (int i = 0; i < CPULoad.Length; i++) {
    summe += CPULoad[i];
}
double mittelwert = (double)summe / CPULoad.Length;
```

**Häufige Fehleranalyse-Aufgabe (Schreibtischtest):** Eine Schleife soll max
und zweithöchsten Wert finden. Klassiker-Fehler (dieses konkrete Beispiel als
`MaxMax2()`-Methode war 2024S dran, Fehler im `else if`-Zweig):
```csharp
else if (CPULoad[i] > max2)
    max = CPULoad[i];   // FALSCH: max statt max2!
```
Richtig: `max2 = CPULoad[i];`

Weitere Jahrgänge mit Fehlersuche/Schreibtischtest-Tabelle zum Vervollständigen:
**2024W, 2025S, 2025W, 2026S**.

**Tipp:** Trage bei Tabellenaufgaben Schritt für Schritt die Werte ein
(ähnlich wie im Aufgabenblatt vorgegeben) – Folgefehler werden bei der
Korrektur oft noch toleriert.

## 4.2b Bubblesort (2022S)

2022S verlangte einen **vollständigen Bubblesort-Algorithmus** zum
Vervollständigen (äußere + innere Schleife, Tausch über eine `temp`-Variable):

```csharp
for (int i = 0; i < zahlen.Length - 1; i++) {
    for (int j = 0; j < zahlen.Length - 1 - i; j++) {
        if (zahlen[j] > zahlen[j + 1]) {
            int temp = zahlen[j];
            zahlen[j] = zahlen[j + 1];
            zahlen[j + 1] = temp;
        }
    }
}
```
Merksatz: äußere Schleife = Anzahl Durchläufe, innere Schleife = Vergleich
benachbarter Elemente, größere „blubbern" bei jedem Durchlauf ans Ende.

## 4.3 Strukturen (struct)

```csharp
struct diskBelegung {
    public int wert;        // aktuelle Belegung
    public DateTime datum;  // Zeitpunkt
}
diskBelegung[] Disk_2 = new diskBelegung[...];
// Zugriff auf erstes Element:
int x = Disk_2[0].wert;
```

## 4.3b Exceptions

Exceptions wurden konkret geprüft in **2023W** (Array-Index außerhalb des
gültigen Bereichs, Division durch 0) und **2025W** (dort explizit als
`IndexOutOfRangeException` im Code benannt):

```csharp
int[] werte = new int[5];
int x = werte[5];   // IndexOutOfRangeException (gültig: Index 0..4)

int y = 10 / 0;      // DivideByZeroException
```

## 4.3c OOP-Begriffe

- **Datenkapselung** und **Vererbung** wurden 2022W abgefragt (private Felder
  + öffentliche Zugriffsmethoden bzw. Basisklasse → abgeleitete Klasse).
- **Aggregation vs. Komposition** (2023S) am Beispiel Gebäude:
  - *Aggregation* (schwache Bindung): **Gebäude – Mieter** → der Mieter
    existiert auch ohne das Gebäude weiter (kann umziehen).
  - *Komposition* (starke Bindung, Existenzabhängigkeit): **Gebäude – Raum**
    → ein Raum kann ohne das Gebäude nicht existieren, wird mit dem Gebäude
    gemeinsam zerstört.

## 4.3d Fehlertypen & Pseudocode

- **Syntaxfehler vs. Semantikfehler** wurden 2025S explizit unterschieden:
  Syntaxfehler = Verstoß gegen die Sprachregeln (Code kompiliert nicht),
  Semantikfehler = Code läuft, liefert aber ein fachlich falsches Ergebnis.
- **Vorteile von Pseudocode** (2023W): sprach-/plattformunabhängig,
  leichter verständlich (auch für Nicht-Programmierer), Fokus auf Logik statt
  Syntax, gute Grundlage für die spätere Implementierung in beliebiger Sprache.

## 4.4 Bit-Bedarf berechnen

Aufgabe: Speicherbelegung bis 16 TiB in KiB in einer `int`-Variablen.

```
16 TiB = 16 * 1024 GiB = 16384 GiB
16384 GiB * 1024 MiB = 16.777.216 MiB
* 1024 = 17.179.869.184 KiB
```
`int` max = 2.147.483.647 → reicht **nicht**! → mind. **64 Bit**
(`long`) nötig. Rechnung: 2^63-1 ≥ Wert.

## 4.5 SQL (Grundlagen)

> ⚠️ **Korrektur:** SQL kam **NUR in 3 von 10 Jahrgängen** vor
> (**2021W, 2022S, 2022W**) – **nicht** jedes Jahr! Seit **2023 taucht kein
> SQL mehr in GA1** auf. Details und die drei echten Original-Beispiele
> siehe `L14_Datenmodellierung_SQL_UML.md`. Statt SQL kommt seit 2023
> häufiger Datenbankmodellierung (ER-Diagramm, Normalform, Indexierung)
> dran, z. B. 2025W: 3. Normalform, Primärschlüssel/Kardinalitäten.

```sql
-- 2021W: Anzahl Einträge vom 21.11.2021 in Tabelle tblDiskM
SELECT COUNT(*) FROM tblDiskM WHERE Datum = '2021-11-21';
```
Weitere Prüfungskandidaten (falls SQL wieder drankommt): `SELECT`, `INSERT`,
`UPDATE`, `DELETE`, `WHERE`, `JOIN` (einfach).

## 4.6 UML-Diagramme

> ⚠️ **Korrektur:** UML wurde nur in **2/10 Jahrgängen** (2021W, 2022S)
> explizit unter diesem Namen abgefragt; verwandte Diagrammtypen (Aggregation/
> Komposition, ER-Diagramme) tauchten dann in 2023S und 2025W auf.

| Sicht | Diagramme (je 2 nennen) |
|-------|--------------------------|
| **Statisch (logischer Aufbau)** | Klassendiagramm, Objektdiagramm, Paketdiagramm, Kollaborationsdiagramm |
| **Dynamisch (Ablauf/Interaktion)** | Aktivitätsdiagramm, Sequenzdiagramm, Zustandsdiagramm |

**2021W** fragte genau diese Zuordnung ab: statische Sicht →
Klassen-/Objekt-/Paket-/Kollaborationsdiagramm; dynamische Sicht →
Aktivitäts-/Sequenz-/Zustandsdiagramm.

**2022S** verlangte ein **konkretes Aktivitätsdiagramm zum Vervollständigen**
(Speicherbelegung nach NAS-Regeln, 14 Punkte, mit beigelegter Symbol-Übersicht).

## Typische Prüfungsaufgabe

> „Lokalisieren Sie die fehlerhafte Stelle im Quellcode und erläutern Sie die
> Fehlerursache."

→ Zeile nennen + **einen Satz zur Ursache** (nicht nur „falsch").
