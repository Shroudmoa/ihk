# Kostenvergleichsrechnung

## In 30 Sekunden

- Die **Kostenvergleichsrechnung** ist ein **statisches Investitionsrechenverfahren**: Alternativen werden nach ihren **durchschnittlichen Kosten pro Periode** (meist pro Jahr) verglichen. Die günstigste gewinnt.
- Gesamtkosten pro Jahr = **fixe Kosten** (Abschreibung, kalkulatorische Zinsen, Wartungsvertrag, Lizenzen) + **variable Kosten** (abhängig von der Menge: Strom, Toner, Klicks, Nutzer).
- **Abschreibung (linear):** `(Anschaffungskosten − Restwert) ÷ Nutzungsdauer`.
- **Kalkulatorische Zinsen:** `(Anschaffungskosten + Restwert) ÷ 2 × Zinssatz` (durchschnittlich gebundenes Kapital).
- **Kritische Menge:** die Menge, bei der zwei Alternativen gleich teuer sind: `(Fixkosten B − Fixkosten A) ÷ (var. Stückkosten A − var. Stückkosten B)`.
- Schwäche: Erlöse, Zeitwert des Geldes und Qualität bleiben unberücksichtigt.

## Kernwissen

### Formeln

| Größe | Formel |
|---|---|
| Abschreibung pro Jahr | (AK − RW) ÷ ND |
| kalkulatorische Zinsen pro Jahr | (AK + RW) ÷ 2 × i |
| Fixkosten pro Jahr | Abschreibung + Zinsen + sonstige fixe Kosten |
| variable Kosten pro Jahr | variable Stückkosten × Menge |
| Gesamtkosten pro Jahr | Fixkosten + variable Kosten |
| Stückkosten | Gesamtkosten ÷ Menge |
| kritische Menge | Δ Fixkosten ÷ Δ variable Stückkosten |

AK = Anschaffungskosten, RW = Restwert, ND = Nutzungsdauer, i = Zinssatz

### Rechenbeispiel: zwei Multifunktionsdrucker

Annahmen: 5 Jahre Nutzung, kein Restwert, kalkulatorischer Zinssatz 6 %, 60.000 Seiten pro Jahr.

| | Gerät A (günstig) | Gerät B (Abteilungsgerät) |
|---|---:|---:|
| Anschaffung | 2.000 € | 8.000 € |
| Wartungsvertrag pro Jahr | 150 € | 400 € |
| variable Kosten pro Seite (Toner, Strom, Papierausfall) | 0,040 € | 0,015 € |

| Kosten pro Jahr | Gerät A | Gerät B |
|---|---:|---:|
| Abschreibung (AK ÷ 5) | 400,00 € | 1.600,00 € |
| kalk. Zinsen (AK ÷ 2 × 6 %) | 60,00 € | 240,00 € |
| Wartung | 150,00 € | 400,00 € |
| **Fixkosten** | **610,00 €** | **2.240,00 €** |
| variable Kosten (60.000 × Stückkosten) | 2.400,00 € | 900,00 € |
| **Gesamtkosten** | **3.010,00 €** | **3.140,00 €** |
| Kosten pro Seite | 0,0502 € | 0,0523 € |

Bei 60.000 Seiten ist **Gerät A** um 130 € pro Jahr günstiger.

**Kritische Menge:**
`(2.240 − 610) ÷ (0,040 − 0,015) = 1.630 ÷ 0,025 = 65.200 Seiten pro Jahr`

Unter 65.200 Seiten ist A günstiger, darüber B. Druckt die Abteilung künftig 80.000 Seiten, kostet A 610 + 3.200 = 3.810 €, B 2.240 + 1.200 = 3.440 € – dann lohnt sich B.

| Seiten pro Jahr | Gerät A (610 € + 0,040 €/Seite) | Gerät B (2.240 € + 0,015 €/Seite) | günstiger |
|---:|---:|---:|---|
| 20.000 | 1.410 € | 2.540 € | A |
| 60.000 | 3.010 € | 3.140 € | A |
| 65.200 | 3.218 € | 3.218 € | gleich (kritische Menge) |
| 80.000 | 3.810 € | 3.440 € | B |

Grafisch: Gerät A startet niedriger (geringe Fixkosten), steigt aber steiler (hohe variable Kosten). Die beiden Kostengeraden schneiden sich bei der kritischen Menge.

### Einordnung

| Statische Verfahren | Dynamische Verfahren |
|---|---|
| Kostenvergleichsrechnung | Kapitalwertmethode |
| Gewinnvergleichsrechnung | Methode des internen Zinsfußes |
| Rentabilitätsrechnung (ROI) | Annuitätenmethode |
| Amortisationsrechnung | dynamische Amortisation |

Statisch = rechnet mit Durchschnittswerten einer Periode, ohne Abzinsung.

## Prüfungsfrage

**Frage:** Für die Zeiterfassung stehen eine Kauflösung (Anschaffung 12.000 €, Nutzungsdauer 4 Jahre, kein Restwert, Wartung 800 €/Jahr) und eine Cloud-Lösung (3,50 € pro Mitarbeiter und Monat, keine Anschaffung) zur Wahl. Vernachlässigen Sie Zinsen. Ermitteln Sie die jährlichen Kosten bei 50 Mitarbeitenden und die kritische Mitarbeiterzahl.

**Musterantwort:**
- Kauf: Abschreibung 12.000 € ÷ 4 = 3.000 € + Wartung 800 € = **3.800 € pro Jahr** (unabhängig von der Mitarbeiterzahl).
- Cloud: 3,50 € × 12 = 42 € pro Mitarbeiter und Jahr; 50 × 42 € = **2.100 € pro Jahr**.
- Kritische Menge: 3.800 € ÷ 42 € = **90,5** → ab **91 Mitarbeitenden** ist die Kauflösung günstiger.

Bei 50 Mitarbeitenden ist die Cloud-Lösung um 1.700 € pro Jahr günstiger. Plant das Unternehmen starkes Wachstum, sollte die Entscheidung mit der erwarteten Mitarbeiterzahl am Ende der Nutzungsdauer überprüft werden.

## Falle

- **Abschreibung und Anschaffung doppelt zählen:** In der Kostenvergleichsrechnung pro Jahr steht nur die Abschreibung, nicht der volle Kaufpreis.
- **Zinsen auf den vollen Anschaffungswert:** Gebunden ist durchschnittlich nur die Hälfte (bei Restwert 0).
- **Kritische Menge nicht runden:** Bei Stückzahlen auf die nächste ganze Einheit aufrunden und angeben, ab wann die Alternative günstiger ist.
- **Unterschiedliche Leistung ignorieren:** Der Vergleich gilt nur bei gleicher Leistung/Qualität – sonst Nutzwertanalyse ergänzen.
- **Monat und Jahr mischen:** Cloud-Preise pro Monat immer auf das Jahr umrechnen.

## Merksatz

**"Fix plus variabel pro Jahr – wer weniger kostet, gewinnt."** Die kritische Menge sagt, ab wann sich das Blatt wendet.

## Verwandt

- [Amortisation](amortisation.md) – wann sich die Mehrinvestition auszahlt
- [TCO](tco.md) – Gesamtkosten über den Lebenszyklus
- [Make-or-Buy, Kauf, Leasing, Miete](make-or-buy-kauf-leasing-miete.md)
- [CAPEX-OPEX](capex-opex.md)
- [Angebotsvergleich & Bezugskalkulation](angebotsvergleich-bezugskalkulation.md)
- [Break-Even-Point](../../Wirtschaft/bep.md) – gleiche Logik mit Erlösen
- [Lektion 22: Kostenrechnung](../analyse-der-exams/L22_Kostenrechnung.md) – Fixkosten, variable Kosten, Angebotsvergleich
