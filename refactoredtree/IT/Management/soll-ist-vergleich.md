# Soll-Ist-Vergleich und Projektcontrolling

## In 30 Sekunden

- Der **Soll-Ist-Vergleich** stellt geplante Werte (Soll) den tatsächlichen Werten (Ist) gegenüber – für **Kosten, Termine und Leistung** (die Ecken des magischen Dreiecks).
- **Absolute Abweichung** = Ist − Soll, **relative Abweichung** = (Ist − Soll) ÷ Soll × 100 %.
- Ablauf: **messen → vergleichen → Ursachen analysieren → Maßnahmen einleiten → Plan anpassen** (Regelkreis).
- **Earned Value Analysis (Fertigstellungswert):** verbindet Kosten und Fortschritt; Kennzahlen CPI und SPI.
- **Meilensteintrendanalyse (MTA):** zeigt, wie sich prognostizierte Meilensteintermine über die Berichtszeitpunkte verschieben.
- Ein reiner Kostenvergleich ohne Fortschritt ist irreführend: "50 % des Budgets verbraucht" sagt nichts, wenn man nicht weiß, wie viel fertig ist.

## Kernwissen

### Einfacher Soll-Ist-Vergleich je Arbeitspaket

| Arbeitspaket | Soll (h) | Ist (h) | Abweichung (h) | Abweichung (%) |
|---|---:|---:|---:|---:|
| Netzwerkplanung | 24 | 20 | − 4 | − 16,7 % |
| Switches konfigurieren | 40 | 52 | + 12 | + 30,0 % |
| Clients umstellen | 60 | 66 | + 6 | + 10,0 % |
| Dokumentation | 16 | 16 | 0 | 0,0 % |
| **Summe** | **140** | **154** | **+ 14** | **+ 10,0 %** |

Bei 50,23 €/h ([interner Stundensatz](stundensatz-intern.md)) entspricht die Überschreitung 14 × 50,23 € = 703,22 €. Größter Treiber ist "Switches konfigurieren" – dort beginnt die **Abweichungsanalyse** (z. B. unerwartete Firmware-Updates, fehlende VLAN-Dokumentation).

### Earned Value Analysis

| Kürzel | Bedeutung | Frage |
|---|---|---|
| **BAC** (Budget at Completion) | Gesamtbudget | Was darf das Projekt kosten? |
| **PV** (Planned Value) | geplante Kosten bis zum Stichtag | Was hätten wir bis heute ausgeben sollen? |
| **EV** (Earned Value) | Fertigstellungsgrad × BAC | Was ist das bisher Geleistete wert? |
| **AC** (Actual Cost) | tatsächliche Kosten bis zum Stichtag | Was haben wir ausgegeben? |

| Kennzahl | Formel | Interpretation |
|---|---|---|
| Kostenabweichung **CV** | EV − AC | negativ = teurer als geplant |
| Planabweichung **SV** | EV − PV | negativ = hinter dem Plan |
| Kostenentwicklungsindex **CPI** | EV ÷ AC | < 1 = Kostenüberschreitung |
| Terminentwicklungsindex **SPI** | EV ÷ PV | < 1 = Verzug |
| Prognose Gesamtkosten **EAC** | BAC ÷ CPI | erwartete Kosten bei gleicher Effizienz |

**Rechenbeispiel:** Budget BAC = 40.000 €. Zum Stichtag sollten laut Plan 50 % erledigt sein (PV = 20.000 €). Tatsächlich sind 40 % fertig (EV = 0,40 × 40.000 € = 16.000 €), ausgegeben wurden 18.000 € (AC).

- CV = 16.000 − 18.000 = **− 2.000 €** (zu teuer)
- SV = 16.000 − 20.000 = **− 4.000 €** (im Verzug)
- CPI = 16.000 ÷ 18.000 = **0,89** → pro ausgegebenem Euro nur 89 Cent Leistung
- SPI = 16.000 ÷ 20.000 = **0,80** → 80 % des geplanten Fortschritts
- EAC = 40.000 ÷ 0,89 ≈ **45.000 €** (exakt 40.000 × 18.000 ÷ 16.000 = 45.000 €)

Ohne Fortschrittsbetrachtung hätte man gesagt: "18.000 € statt 20.000 € ausgegeben, wir liegen unter Budget" – falsch.

### Meilensteintrendanalyse (MTA)

In einem Diagramm stehen auf der x-Achse die Berichtszeitpunkte, auf der y-Achse die prognostizierten Meilensteintermine.

| Verlauf der Linie | Bedeutung |
|---|---|
| waagerecht | Termin stabil |
| steigend | Meilenstein verschiebt sich nach hinten (Verzug) |
| fallend | Meilenstein wird früher erreicht |
| Zickzack | unzuverlässige Planung |

### Typische Gegenmaßnahmen

- zusätzliche Ressourcen (Crashing, kostet Geld)
- Vorgänge parallelisieren (Fast Tracking, erhöht Risiko)
- Leistungsumfang reduzieren (mit dem Auftraggeber über Change Request)
- Termin oder Budget anpassen (Eskalation an Lenkungsausschuss)
- Ursachen beseitigen (Schulung, bessere Werkzeuge, klarere Anforderungen)

## Prüfungsfrage

**Frage:** In einem Migrationsprojekt (Budget 30.000 €) sind zum Stichtag laut Plan Arbeiten im Wert von 15.000 € vorgesehen. Tatsächlich sind 60 % der Arbeit erledigt, die Ist-Kosten betragen 21.000 €. Beurteilen Sie Kosten- und Terminsituation.

**Musterantwort:** EV = 0,60 × 30.000 € = 18.000 €. **Kosten:** CV = 18.000 − 21.000 = − 3.000 €, CPI = 18.000 ÷ 21.000 ≈ 0,86 – das Projekt ist teurer als geplant. Prognose: 30.000 ÷ 0,857 ≈ 35.000 € Gesamtkosten. **Termin:** SV = 18.000 − 15.000 = + 3.000 €, SPI = 1,2 – das Projekt ist dem Zeitplan voraus. Wahrscheinlich wurde mit mehr Personal gearbeitet als geplant. Maßnahme: Ursachen klären, Ressourceneinsatz anpassen und die Budgetüberschreitung frühzeitig an den Auftraggeber kommunizieren.

## Falle

- **Nur Kosten vergleichen:** Ohne Fertigstellungsgrad keine Aussage über Effizienz.
- **Vorzeichen:** Bei Kosten ist Ist > Soll schlecht, bei EV-Kennzahlen bedeutet ein negativer CV/SV "schlecht".
- **CPI und SPI vertauscht:** CPI = Kosten (EV ÷ **AC**), SPI = Zeit (EV ÷ **PV**).
- **Relative Abweichung auf Ist beziehen:** Basis ist immer der **Soll**-Wert.
- **Abweichung feststellen, aber keine Maßnahme nennen:** Controlling heißt steuern, nicht nur messen.

## Merksatz

**"Soll – Ist – Warum – Was tun?"** CPI für Kosten, SPI für Termine; unter 1 heißt Handlungsbedarf.

## Verwandt

- [Projektmanagement-Grundlagen](projektmanagement-grundlagen.md) – magisches Dreieck, Meilensteine
- [Netzplan](netzplan.md) – kritischer Pfad und Puffer
- [Gantt-Diagramm](gantt-diagramm.md) – Fortschritt grafisch
- [Ressourcenplan](ressourcenplan.md)
- [Stundensatz intern](stundensatz-intern.md)
- [Stakeholder- & Risikoanalyse](stakeholder-risikoanalyse.md) – Abweichungen früh erkennen
- [Lektion 22: Kostenrechnung](../analyse-der-exams/L22_Kostenrechnung.md)
