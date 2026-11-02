# Softwaretest: White-Box, Black-Box und Teststufen

## In 30 Sekunden

- **Black-Box-Test:** Tester kennt den Code **nicht**, prüft nur Eingabe → erwartete Ausgabe gegen die Spezifikation (Anforderungen).
- **White-Box-Test:** Tester kennt den **Quellcode** und prüft gezielt Pfade, Verzweigungen und Schleifen (Anweisungs-, Zweig-, Pfadüberdeckung).
- **Grey-Box:** Mischform mit teilweiser Kenntnis der Interna.
- **Teststufen:** Komponententest (Unit) → Integrationstest → Systemtest → Abnahmetest.
- **Schreibtischtest** = manueller White-Box-Test mit Werte-Tabelle – Standardaufgabe in jeder GA1.
- Geprüft: **White-Box- vs. Black-Box-Testverfahren** (2025W), Penetrationstest als Black-/White-Box (2025S).

## Kernwissen

### Black-Box vs. White-Box

| Kriterium | Black-Box | White-Box |
|---|---|---|
| Grundlage | Spezifikation, Lastenheft/Pflichtenheft | Quellcode, Programmstruktur |
| Wer testet? | Tester, Fachabteilung, Kunde | Entwickler |
| Ziel | erfüllt das Programm die Anforderungen? | wird jeder Codeteil korrekt durchlaufen? |
| Methoden | Äquivalenzklassen, Grenzwertanalyse, Entscheidungstabellen, Use-Case-Tests | Anweisungsüberdeckung (C0), Zweigüberdeckung (C1), Pfadüberdeckung |
| findet gut | fehlende Funktionen, falsche Ergebnisse | toten Code, Logikfehler in Verzweigungen |
| Schwäche | nicht alle Codepfade werden erreicht | fehlende Anforderungen werden nicht erkannt |

### Black-Box-Methoden am Beispiel

Funktion: CPU-Auslastung 0–100 % einlesen, ab **über 80** eine Warnung ausgeben.

**Äquivalenzklassen** (Werte, die sich gleich verhalten sollen):

| Klasse | Bereich | Repräsentant | Erwartung |
|---|---|---|---|
| ungültig, zu klein | < 0 | −5 | Fehlermeldung |
| gültig, normal | 0 … 80 | 50 | keine Warnung |
| gültig, kritisch | 81 … 100 | 90 | Warnung |
| ungültig, zu groß | > 100 | 150 | Fehlermeldung |

**Grenzwertanalyse:** Fehler entstehen an Grenzen (`>` statt `>=`). Testwerte: −1, 0, 80, 81, 100, 101.

### White-Box-Überdeckungsgrade

```csharp
if (last > 80) {          // Zweig A (true) / Zweig B (false)
    warnungen++;
}
Console.WriteLine(warnungen);
```

- **Anweisungsüberdeckung (C0):** jede Anweisung mindestens einmal ausgeführt → 1 Testfall (`last = 90`) genügt.
- **Zweigüberdeckung (C1):** jeder Zweig (true und false) mindestens einmal → 2 Testfälle (`90` und `50`). C1 schließt C0 ein.
- **Pfadüberdeckung:** alle möglichen Pfade – bei Schleifen schnell nicht mehr vollständig machbar.

### Teststufen (V-Modell)

| Stufe | prüft | Grundlage |
|---|---|---|
| **Komponenten-/Unit-Test** | einzelne Methode/Klasse | Feinentwurf |
| **Integrationstest** | Zusammenspiel der Komponenten, Schnittstellen | Grobentwurf/Architektur |
| **Systemtest** | Gesamtsystem in testnaher Umgebung | Pflichtenheft |
| **Abnahmetest** | durch den Auftraggeber, Voraussetzung für die Abnahme | Lastenheft |

### Weitere Testarten

| Testart | Zweck |
|---|---|
| **Regressionstest** | nach Änderungen prüfen, ob bisherige Funktionen noch gehen |
| **Lasttest / Performancetest** | Verhalten unter erwarteter bzw. hoher Last |
| **Stresstest** | Verhalten über der Lastgrenze |
| **Usability-Test** | Bedienbarkeit mit echten Nutzern |
| **Sicherheitstest / Penetrationstest** | Schwachstellen finden (als Black- oder White-Box) |
| **Smoke-Test** | schneller Grundfunktionstest nach dem Deployment |

### Schreibtischtest

Manueller Durchlauf des Codes mit Papier und Tabelle: pro Schleifendurchlauf werden die Werte aller Variablen eingetragen. Er ist ein **statischer White-Box-Test** und deckt Semantikfehler auf, bevor der Code ausgeführt wird.

## Prüfungsfrage

**Frage (angelehnt an 2025W):** Für ein neues Verwaltungsprogramm sollen Tests geplant werden. Erläutern Sie den Unterschied zwischen White-Box- und Black-Box-Test und nennen Sie je ein konkretes Testverfahren.

**Musterantwort:** Beim **Black-Box-Test** wird das Programm ohne Kenntnis des Quellcodes geprüft. Der Tester gibt Werte ein und vergleicht die Ausgaben mit den Anforderungen aus dem Pflichtenheft. Ein Verfahren ist die **Grenzwertanalyse**, bei der gezielt Werte an den Grenzen gültiger Bereiche getestet werden (z. B. 80 und 81 bei einer Warnschwelle von über 80). Beim **White-Box-Test** kennt der Tester den Quellcode und wählt die Testfälle so, dass bestimmte Strukturen durchlaufen werden. Ein Verfahren ist die **Zweigüberdeckung**: Jede Verzweigung wird mindestens einmal mit wahr und einmal mit falsch durchlaufen. Black-Box findet fehlende oder falsch umgesetzte Anforderungen, White-Box findet Logikfehler und nicht erreichbaren Code.

## Falle

- **Black-Box = "Test ohne Plan":** Black-Box-Tests sind systematisch (Äquivalenzklassen, Grenzwerte).
- **White-Box nur mit Debugger:** Auch der Schreibtischtest ist White-Box.
- **100 % Anweisungsüberdeckung = fehlerfrei:** Fehlende Zweige (z. B. `else`) bleiben ungetestet.
- **Abnahmetest durch den Entwickler:** Den führt der **Auftraggeber** durch.
- **Grenzwerte vergessen:** Die meisten Fehler sitzen bei `<` vs. `<=`.

## Merksatz

**Black-Box schaut auf das "Was", White-Box auf das "Wie".** Grenzwerte testen, weil Fehler an den Kanten wohnen.

## Verwandt

- [Fehlerarten & Exceptions](fehlerarten-exceptions.md) – was Tests finden sollen
- [Schleifen & Fehlersuche](03_Schleifen_Fehlersuche.md) – Schreibtischtest
- [Qualitätsmanagement](../../Management/qualitaetsmanagement.md) – Tests als Teil der Qualitätssicherung
- [Vorgehensmodelle](../../Management/vorgehensmodelle.md) – V-Modell mit Teststufen
- [Abnahme, Übergabe & Dokumentation](../../Management/abnahme-uebergabe-dokumentation.md) – Abnahmetest
- [Lektion 28: GA1-Praxisthemen](../../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md) – White-/Black-Box 2025W
