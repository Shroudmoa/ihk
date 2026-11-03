# Amortisationsrechnung

## In 30 Sekunden

- Die **Amortisationsrechnung** (Pay-back-Methode) ermittelt, nach welcher Zeit die Investition durch Rückflüsse (Einsparungen oder Mehrerlöse) wieder hereingeholt ist.
- **Durchschnittsmethode:** `Amortisationsdauer = Anschaffungskosten / durchschnittlicher jährlicher Rückfluss`.
- **Kumulationsmethode:** Rückflüsse Jahr für Jahr aufaddieren, bis sie die Investition erreichen – nötig bei **ungleichen** Rückflüssen.
- **Rückfluss** = Einsparung bzw. Mehrerlös **minus** neue laufende Kosten. Bei Gewinnangabe gilt: Rückfluss = Gewinn + Abschreibung.
- Entscheidungsregel: Amortisationsdauer < Nutzungsdauer (bzw. < vorgegebene Soll-Dauer); kürzer ist besser (geringeres Risiko).
- Statisches Verfahren: Zinsen und Zahlungen nach der Amortisation werden nicht berücksichtigt.

## Kernwissen

### Formeln

| Größe | Formel |
|---|---|
| Amortisationsdauer (Durchschnitt) | Anschaffungskosten ÷ jährlicher Rückfluss |
| jährlicher Rückfluss (Kostenvergleich) | alte laufende Kosten − neue laufende Kosten |
| jährlicher Rückfluss (Gewinnvergleich) | Gewinn pro Jahr + Abschreibung pro Jahr |
| Abschreibung (linear) | (Anschaffungskosten − Restwert) ÷ Nutzungsdauer |
| Amortisation einer Mehrinvestition | Mehrkosten der Anschaffung ÷ jährliche Mehreinsparung |
| ROI (Return on Investment, einfach) | Gewinn ÷ investiertes Kapital × 100 % |

### Rechenbeispiel 1: Durchschnittsmethode

Ein Unternehmen virtualisiert 8 alte Server auf 2 neue Hosts.

| Position | Betrag |
|---|---:|
| Anschaffung (Hardware, Lizenzen, Einrichtung) | 48.000 € |
| Einsparung Strom, Wartung, Stellfläche pro Jahr | 18.500 € |
| neue laufende Kosten (Support-Vertrag) pro Jahr | 2.500 € |
| **jährlicher Rückfluss** | **16.000 €** |

`Amortisationsdauer = 48.000 € ÷ 16.000 €/Jahr = 3 Jahre`

Bei einer Nutzungsdauer von 5 Jahren lohnt sich die Investition: Nach 3 Jahren ist das Kapital zurück, die Jahre 4 und 5 bringen je 16.000 € Vorteil.

### Rechenbeispiel 2: Kumulationsmethode (ungleiche Rückflüsse)

Investition in ein Ticketsystem: 30.000 €. Die Einsparungen steigen, weil sich die Mitarbeitenden einarbeiten.

| Jahr | Rückfluss | kumuliert | offen |
|---:|---:|---:|---:|
| 1 | 9.000 € | 9.000 € | 21.000 € |
| 2 | 11.000 € | 20.000 € | 10.000 € |
| 3 | 12.000 € | 32.000 € | – |

Im 3. Jahr fehlen noch 10.000 € von 12.000 € Rückfluss:
`Amortisationsdauer = 2 + 10.000 ÷ 12.000 = 2 + 0,83 = 2,83 Jahre ≈ 2 Jahre und 10 Monate` (0,83 × 12 = 10 Monate).

### Rechenbeispiel 3: Rückfluss aus Gewinn und Abschreibung

Ein Druckzentrum kostet 20.000 €, Nutzungsdauer 5 Jahre, kein Restwert. Es erzeugt einen zusätzlichen Gewinn von 2.000 € pro Jahr (nach Abschreibung).

- Abschreibung: 20.000 € ÷ 5 = 4.000 €/Jahr
- Rückfluss: 2.000 € + 4.000 € = 6.000 €/Jahr
- Amortisation: 20.000 € ÷ 6.000 € = **3,33 Jahre** (3 Jahre und 4 Monate)

Die Abschreibung wird addiert, weil sie den Gewinn mindert, aber kein Geld abfließt.

### Rechenbeispiel 4: Mehrinvestition

| | Switch A | Switch B (energieeffizient) |
|---|---:|---:|
| Anschaffung | 6.000 € | 8.400 € |
| Strom und Wartung pro Jahr | 1.900 € | 1.300 € |

Mehrinvestition B: 2.400 €, Mehreinsparung B: 600 €/Jahr → Amortisation der Mehrinvestition **4 Jahre**. Bei 5 Jahren Nutzung lohnt sich B knapp (5 × 600 € = 3.000 € > 2.400 €).

### Vor- und Nachteile

| Vorteile | Nachteile |
|---|---|
| einfach, schnell, gut erklärbar | statisch: keine Zinsen, kein Zeitwert des Geldes |
| Maß für Risiko und Liquidität | Rückflüsse nach dem Amortisationszeitpunkt werden ignoriert |
| gut für schnelllebige IT-Investitionen | sagt nichts über die Rentabilität aus |

Dynamische Alternative: **Kapitalwertmethode** (abgezinste Rückflüsse).

## Prüfungsfrage

**Frage:** Eine neue Backup-Appliance kostet 24.000 €. Sie ersetzt einen Dienstleister, der bisher 9.000 € pro Jahr gekostet hat. Für Wartung und Strom der Appliance fallen jährlich 1.500 € an. Berechnen Sie die Amortisationsdauer und beurteilen Sie die Investition bei einer Nutzungsdauer von 4 Jahren.

**Musterantwort:** Jährlicher Rückfluss = 9.000 € − 1.500 € = 7.500 €. Amortisationsdauer = 24.000 € ÷ 7.500 €/Jahr = **3,2 Jahre** (3 Jahre und ca. 2,4 Monate). Da die Nutzungsdauer 4 Jahre beträgt, amortisiert sich die Appliance innerhalb der Nutzungsdauer. Über 4 Jahre ergibt sich ein Vorteil von 4 × 7.500 € − 24.000 € = 6.000 €. Die Investition ist wirtschaftlich; das Risiko ist aber relativ hoch, weil der Puffer bis zum Ende der Nutzung nur 0,8 Jahre beträgt.

## Falle

- **Brutto-Einsparung statt Netto-Rückfluss:** Neue laufende Kosten müssen abgezogen werden.
- **Abschreibung vergessen:** Wird der Gewinn angegeben, ist der Rückfluss Gewinn **plus** Abschreibung.
- **Dezimaljahre falsch umrechnen:** 3,2 Jahre sind nicht 3 Jahre und 2 Monate, sondern 3 Jahre und 0,2 × 12 = 2,4 Monate.
- **Durchschnittsmethode bei ungleichen Rückflüssen:** Bei schwankenden Werten die Kumulationsmethode verwenden.
- **Kürzeste Amortisation = beste Investition:** Nicht unbedingt – eine Alternative mit längerer Amortisation kann über die Nutzungsdauer mehr einbringen.

## Merksatz

**"Investition durch Rückfluss – und der Rückfluss ist netto."** Amortisation misst das Risiko, nicht den Gewinn.

## Verwandt

- [TCO](tco.md) – Gesamtkosten über den Lebenszyklus
- [Kostenvergleichsrechnung](kostenvergleichsrechnung.md) – Alternativen nach Kosten vergleichen
- [Make-or-Buy, Kauf, Leasing, Miete](make-or-buy-kauf-leasing-miete.md)
- [CAPEX-OPEX](capex-opex.md)
- [Break-Even-Point](../../Wirtschaft/Unternehmen/bep.md) – Gewinnschwelle über die Menge
- [Ökonomisches Prinzip & Kennzahlen](../../Wirtschaft/Unternehmen/oekonomisches-prinzip-kennzahlen.md) – Wirtschaftlichkeit, Rentabilität
- [Lektion 22: Kostenrechnung](../analyse-der-exams/L22_Kostenrechnung.md) – Prüfungsaufgaben zur Amortisation
