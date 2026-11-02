# Fehlerarten und Exceptions

## In 30 Sekunden

- **Syntaxfehler:** Verstoß gegen die Regeln der Sprache, der Compiler meldet ihn, das Programm startet nicht.
- **Semantikfehler (Logikfehler):** Programm läuft, liefert aber ein fachlich falsches Ergebnis. Nur durch Tests oder Schreibtischtest zu finden.
- **Laufzeitfehler:** tritt erst bei der Ausführung auf und löst eine **Exception** aus, z. B. `IndexOutOfRangeException` oder `DivideByZeroException`.
- Exceptions behandelt man mit **`try` / `catch` / `finally`**.
- Geprüft: **Syntax- vs. Semantikfehler** (2025S), **zwei Umstände für Exceptions** (2023W), `IndexOutOfRangeException` im Code (2025W), Fehlersuche mit Zeile + Ursache + Korrektur (2024S–2026S).

## Kernwissen

### Fehlerarten im Vergleich

| Fehlerart | Wann bemerkt? | Beispiel | Wie finden? |
|---|---|---|---|
| **Syntaxfehler** | beim Kompilieren | fehlendes `;`, `int x = "Hallo";`, Klammer fehlt | Compiler-Meldung |
| **Laufzeitfehler** | während der Ausführung | Division durch 0, Index außerhalb des Arrays | Exception, Absturz, Log |
| **Semantik-/Logikfehler** | gar nicht automatisch | `max = wert` statt `max2 = wert`; `<=` statt `<`; Mittelwert ohne Cast | Schreibtischtest, Unit-Test |

### Wichtige Exceptions (C#)

| Exception | Ursache | Beispiel |
|---|---|---|
| `IndexOutOfRangeException` | Arrayindex < 0 oder ≥ `Length` | `werte[5]` bei 5 Elementen |
| `DivideByZeroException` | ganzzahlige Division durch 0 | Mittelwert bei leerem Array |
| `NullReferenceException` | Zugriff auf ein Objekt, das `null` ist | `kunde.Name` ohne `new Kunde()` |
| `FormatException` | Text lässt sich nicht umwandeln | `int.Parse("abc")` |
| `OverflowException` | Wert passt nicht in den Typ (im `checked`-Kontext) | `checked((byte)300)` |
| `FileNotFoundException` | Datei existiert nicht | Logdatei auf falschem Pfad |
| `StackOverflowException` | zu tiefe oder endlose Rekursion | Rekursion ohne Basisfall |
| `InvalidCastException` | unzulässige Typumwandlung | `(string)objZahl` |

Hinweis: Bei `double` gibt es keine `DivideByZeroException` – `10.0 / 0` ergibt `Infinity`.

### try / catch / finally

```csharp
int[] last = { 40, 42, 47 };
try {
    int summe = 0;
    for (int i = 0; i <= last.Length; i++) {    // Fehler: <= statt <
        summe += last[i];
    }
    Console.WriteLine(summe / last.Length);
}
catch (IndexOutOfRangeException ex) {
    Console.WriteLine("Indexfehler: " + ex.Message);
}
catch (DivideByZeroException) {
    Console.WriteLine("Keine Messwerte vorhanden.");
}
finally {
    Console.WriteLine("Auswertung beendet.");  // läuft immer, z. B. Datei schließen
}
```

- `try`: Code, der eine Exception auslösen kann.
- `catch`: Behandlung – **spezifische** Exceptions vor allgemeinem `Exception`.
- `finally`: wird immer ausgeführt (Ressourcen freigeben).
- `throw new ArgumentException("…")`: eigene Exception auslösen.

**Besser als fangen ist vermeiden:** Schleife mit `i < last.Length`, vor der Division `if (last.Length > 0)` prüfen, `int.TryParse` statt `int.Parse`.

### Vorgehen bei Fehlersuche-Aufgaben

1. **Zeile nennen** (Nummer oder Code zitieren).
2. **Fehlerursache** in einem Satz erklären (nicht nur "falsch").
3. **Korrektur** als richtige Codezeile angeben.
4. Bei Bedarf **Fehlerart** benennen (Syntax, Laufzeit, Semantik).

## Prüfungsfrage

**Frage (angelehnt an 2025S/2023W):** Unterscheiden Sie Syntax- und Semantikfehler an je einem Beispiel. Nennen Sie außerdem zwei Umstände, unter denen in einem Programm zur Mittelwertberechnung über ein Array eine Exception ausgelöst wird.

**Musterantwort:** Ein **Syntaxfehler** verletzt die Grammatik der Programmiersprache, z. B. ein fehlendes Semikolon in `int summe = 0`; der Compiler bricht ab, das Programm kann nicht ausgeführt werden. Ein **Semantikfehler** ist syntaktisch korrekt, liefert aber ein falsches Ergebnis, z. B. `RAM2[i*2+1] = i;` statt `i + 1` oder `double mw = summe / anzahl;` ohne Cast – das Programm läuft, die Werte stimmen nicht. **Exceptions:** (1) Die Schleife greift mit `i <= werte.Length` auf einen Index außerhalb des Arrays zu → `IndexOutOfRangeException`. (2) Das Array ist leer, bei `summe / werte.Length` wird ganzzahlig durch 0 geteilt → `DivideByZeroException`.

## Falle

- **Laufzeitfehler als Syntaxfehler bezeichnen:** `werte[10]` kompiliert fehlerfrei und scheitert erst beim Ausführen.
- **Semantikfehler "findet der Compiler":** Nein, nur Tests finden ihn.
- **Leeres `catch {}`:** verschluckt Fehler, die Ursache bleibt unentdeckt.
- **`catch (Exception)` zuerst:** spezifischere `catch`-Blöcke dahinter werden nie erreicht (Compilerfehler in C#).
- **Nur "Zeile 7 ist falsch" schreiben:** Ursache und Korrektur bringen die Punkte.

## Merksatz

**Syntax = "falsch geschrieben", Semantik = "falsch gedacht", Laufzeit = "falsch gelaufen".** Exceptions fängt man mit `try`/`catch`, besser vermeidet man sie durch Prüfungen.

## Verwandt

- [Schleifen & Fehlersuche](03_Schleifen_Fehlersuche.md) – Fehlertypen in Beispielen
- [Praxistests & Musterlösungen](04_Praxistests_Musterloesungen.md)
- [Softwaretest: White-Box & Black-Box](softwaretest-white-black-box.md) – wie man Semantikfehler findet
- [Datentypen](datentypen.md) – Überlauf und Casts
- [Lektion 4: Programmierung](../../analyse-der-exams/04_Programmierung_Grundlagen.md) – Exceptions 2023W/2025W, Syntax vs. Semantik 2025S
