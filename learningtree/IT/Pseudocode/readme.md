# Pseudocode - Vorbereitung für die IHK AP2

Diese Sammlung ist speziell für FiSi-Azubis konzipiert, um sich gezielt auf den Pseudocode-Teil der IHK-Abschlussprüfung vorzubereiten.

## Lernreihenfolge

| Datei | Thema | Priorität |
|---|---|---|
| [00_Einleitung.md](00_Einleitung.md) | Überblick, Aufgabentypen, Prüfungsumfeld | ★★★★☆ |
| [01_Grundlagen_Syntax.md](01_Grundlagen_Syntax.md) | Datentypen, Arrays, Schleifen, if/else | ★★★★★ |
| [02_Arrays_Mittelwert_Max.md](02_Arrays_Mittelwert_Max.md) | Mittelwert, Max, Min, Zweithöchster (Klassiker!) | ★★★★★ |
| [03_Schleifen_Fehlersuche.md](03_Schleifen_Fehlersuche.md) | Bubblesort, Schreibtischtest, Fehlersuche | ★★★★★ |
| [04_Praxistests_Musterloesungen.md](04_Praxistests_Musterloesungen.md) | Originalnahe Übungstests mit Lösungen | ★★★★★ |

## Top 5 für die Prüfung

1. **Mittelwert berechnen** - Kommt in fast jedem Jahrgang vor
2. **Maximum/zweithöchster Wert** - Sehr häufig, oft als Fehlersuche
3. **Schreibtischtest durchführen** - Werte in Tabelle eintragen
4. **Fehler finden und korrigieren** - Zeile + Ursache + Korrektur
5. **Bubblesort verstehen/vervollständigen** - Kommt regelmäßig

## Schnellzugriff: Häufigste Fehler

| Fehler | Korrektur |
|---|---|
| `for (int i = 0; i <= arr.Length; i++)` | `i < arr.Length` (IndexOutOfRange verhindern) |
| `double x = sum / count;` ohne Cast | `(double)sum / count` für Kommazahl |
| `max = arr[i];` im else-if für zweithöchsten Wert | `max2 = arr[i];` |
| Schleife startet bei 0 wenn `max = arr[0]` | Ab Index 1 starten oder Logik anpassen |
| `=` statt `==` in if-Bedingung | `==` für Vergleich |

## Verknüpfung zu anderen Lerninhalten

- [Analyse der Prüfungen - Programmierung](../analyse-der-exams/04_Programmierung_Grundlagen.md) - Echte Prüfungsfälle aus 2021-2026
- [Lernpfad-Verbindungen](../../Lernpfad-Verbindungen.md) - Gesamtüberblick

## Lernplan (für 2-3 Tage vor Prüfung)

**Tag 1:** 00-01 durcharbeiten + Übungsaufgaben lösen
**Tag 2:** 02-03 durcharbeiten, Schreibtischtests üben
**Tag 3:** 04 Praxistests unter Zeitdruck (10-15 Min pro Test)

**Tipp:** Nimm dir Stift und Papier - genau so wie in der echten Prüfung!
