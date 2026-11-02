# Lektion 18 – Verfügbarkeit & Redundanz berechnen

**Bereich:** Systemzuverlässigkeit / IT-Betrieb
**Relevanz:** Mittel–Hoch – Rechenaufgaben zu Verfügbarkeit sind typisch in AP2 (WiSo/Technik)
**Lernzeit:** ca. 2h

## 1. Warum das wichtig ist
- Unternehmen definieren Verfügbarkeit als SLA (z. B. 99,9 %).
- Prüfung verlangt Umrechnung von Prozent in Ausfallzeit und umgekehrt.
- Redundanz-Konzepte (parallel/seriell) müssen bewertet werden können.

## 2. Grundlagen – Verfügbarkeit in %
Jahresbasis (8760 h):

| Verfügbarkeit | Ausfall pro Jahr |
|---------------|------------------|
| 99 % | ≈ 3,65 Tage (87,6 h) |
| 99,9 % | ≈ 8,76 h |
| 99,99 % | ≈ 52,6 min |
| 99,999 % | ≈ 5,26 min |

Merkhilfe: pro zusätzliche 9 etwa Faktor 10 weniger Ausfallzeit.

## 3. Formel MTBF / MTTR
- **MTBF** = Mean Time Between Failures (mittlere Betriebsdauer bis Fehler)
- **MTTR** = Mean Time To Repair (mittlere Reparaturdauer)

```
Verfügbarkeit = MTBF / (MTBF + MTTR)
```

Beispiel: MTBF = 8760 h, MTTR = 8,76 h → V = 8760 / 8768,76 ≈ 0,999 = 99,9 %

## 4. Verfügbarkeitsberechnung in % (SLA-relevant)

Prüfungsrelevant ist die Umrechnung von Prozenten in Ausfallzeiten und umgekehrt (SLA-Bezug). Der Begriff **MTBF/MTTR** findet sich in den echten AP2-Prüfungen **nicht** – stattdessen werden prozentuale Verfügbarkeiten und daraus resultierende Ausfallzeiten abgefragt.

Jahresbasis (8760 h):

| Verfügbarkeit | Ausfall pro Jahr |
|---------------|------------------|
| 99 % | ≈ 3,65 Tage (87,6 h) |
| 99,9 % | ≈ 8,76 h |
| 99,99 % | ≈ 52,6 min |
| 99,999 % | ≈ 5,26 min |

Merkhilfe: pro zusätzliche 9 etwa Faktor 10 weniger Ausfallzeit.

**Kombinierte Verfügbarkeit:**

- **Parallele Redundanz** (z. B. zwei 99%--Leitungen): `1 − (0,01 × 0,01) = 1 − 0,0001 = 0,9999 = 99,99 %`
- **Serielle Kette** (nacheinander hängende Dienste): Die Verfügbarkeiten werden multipliziert, das Ergebnis sinkt. Beispiel: 99,9 % × 99,9 % = 0,998001 ≈ 99,80 %.

## 5. Übungsaufgabe (im Prüfungsstil, selbst erstellt)

Ein WLAN-Router hat eine Verfügbarkeit von **99,9 %** gemäß SLA. Wie hoch ist die gesamte Ausfallzeit in einem Kalenderjahr (8760 Stunden)?

<details>
<summary>Lösung (aufklappen)</summary>

Ausfallzeit = (1 − 0,999) × 8760 h = 0,001 × 8760 = **8,76 h/Jahr** (entspricht ca. 2,2 Stunden pro Woche).

</details>

## Selbstcheck – kannst du das?
- [ ] Ich kann 99 % / 99,9 % / 99,99 % in Ausfallzeit umrechnen.
- [ ] Ich wende die Formel V = 1 − (1−p1)×(1−p2) … für parallele Verfügbarkeiten an.
- [ ] Ich unterscheide parallele und serielle Schaltung von Diensten/Leitungen.
- [ ] Ich berechne die Ausfallzeit bei 99,9 %-SLA für ein Jahr.
- [ ] Ich erkenne, dass Redundanz die Verfügbarkeit erhöht.

## Weiterführende Ressourcen
- IHK-Prüfungsaufgaben AP2 (Verfügbarkeitsberechnung)
- SLA-Definitionen und -Rechenbeispiele
- BSI-Grundschutz: Ausfallsicherheit und Redundanz
