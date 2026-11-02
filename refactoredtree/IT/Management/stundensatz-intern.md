# Interner Stundensatz

## In 30 Sekunden

- Der **interne Stundensatz** gibt an, was eine **produktive** Arbeitsstunde eines Mitarbeitenden das Unternehmen kostet.
- Formel: `Stundensatz = Gesamtkosten des Arbeitsplatzes pro Jahr ÷ produktive Stunden pro Jahr`.
- Gesamtkosten = Bruttogehalt + Arbeitgeberanteile zur Sozialversicherung + Sonderzahlungen + Arbeitsplatz- und Gemeinkosten.
- Produktive Stunden = Arbeitstage nach Abzug von Wochenenden, Feiertagen, Urlaub, Krankheit **und** nicht verrechenbarer Zeit (Meetings, Weiterbildung).
- Anwendung: Projektkalkulation (`Aufwand in h × Stundensatz`), interne Leistungsverrechnung, Make-or-Buy-Vergleich mit externen Dienstleistern.
- Externer **Verrechnungssatz** = interner Satz + Gewinnzuschlag (+ ggf. Wagnis).

## Kernwissen

### Schritt 1: produktive Stunden ermitteln

| Position | Tage |
|---|---:|
| Kalendertage | 365 |
| − Wochenenden (52 × 2) | − 104 |
| − Feiertage (bundeslandabhängig) | − 10 |
| − Urlaub | − 30 |
| − Krankheit (Erfahrungswert) | − 10 |
| **= Anwesenheitstage** | **211** |
| × 8 Stunden | **1.688 h** |
| − 15 % nicht produktiv (Meetings, Weiterbildung, Verwaltung) | − 253,2 h |
| **= produktive Stunden** | **≈ 1.435 h** |

### Schritt 2: Jahreskosten ermitteln

| Kostenart | Betrag |
|---|---:|
| Bruttogehalt (12 × 3.700 € + 3.600 € Weihnachts-/Urlaubsgeld) | 48.000 € |
| Arbeitgeberanteil Sozialversicherung (Annahme ca. 21 %) | 10.080 € |
| Arbeitsplatzkosten (Büro, Hardware, Software, Strom) | 6.000 € |
| anteilige Gemeinkosten (Verwaltung, Personalabteilung) | 8.000 € |
| **Gesamtkosten pro Jahr** | **72.080 €** |

Der Arbeitgeberanteil setzt sich aus den Anteilen zur Renten-, Kranken-, Pflege- und Arbeitslosenversicherung sowie der vollständig vom Arbeitgeber getragenen Unfallversicherung zusammen; die genauen Sätze stehen in der Aufgabe oder werden pauschal vorgegeben.

### Schritt 3: Stundensatz

`Interner Stundensatz = 72.080 € ÷ 1.435 h = 50,23 €/h`

### Schritt 4: Anwendung in der Projektkalkulation

| Arbeitspaket | Aufwand | Kosten (× 50,23 €) |
|---|---:|---:|
| Anforderungsanalyse | 16 h | 803,68 € |
| Installation und Konfiguration | 64 h | 3.214,72 € |
| Test und Dokumentation | 40 h | 2.009,20 € |
| **Summe Personalkosten** | **120 h** | **6.027,60 €** |

Für ein Angebot an einen externen Kunden mit 20 % Gewinnzuschlag: 50,23 € × 1,20 = **60,28 €/h**.

### Vergleich mit externem Dienstleister

Ein Dienstleister bietet die Installation für 85 €/h an. Intern kostet die Stunde 50,23 €. Bei 64 h: intern 3.214,72 €, extern 5.440 €. Rein kostenmäßig ist die Eigenleistung günstiger – sofern Kapazität und Know-how vorhanden sind (siehe [Make-or-Buy](make-or-buy-kauf-leasing-miete.md)).

## Prüfungsfrage

**Frage:** Ein Systemadministrator verursacht jährliche Personalkosten (inkl. Arbeitgeberanteile) von 62.400 €. Hinzu kommen 7.600 € Arbeitsplatz- und Gemeinkosten. Er arbeitet an 210 Tagen zu je 8 Stunden, davon sind 80 % produktiv. Berechnen Sie den internen Stundensatz und die Kosten eines Arbeitspakets mit 3,5 Personentagen.

**Musterantwort:**
- Gesamtkosten: 62.400 € + 7.600 € = 70.000 €
- produktive Stunden: 210 × 8 h × 0,8 = 1.344 h
- Stundensatz: 70.000 € ÷ 1.344 h = **52,08 €/h**
- Arbeitspaket: 3,5 PT × 8 h = 28 h; 28 h × 52,08 € = **1.458,24 €**

## Falle

- **Nur das Bruttogehalt ansetzen:** Arbeitgeberanteile, Arbeitsplatz- und Gemeinkosten fehlen dann.
- **52 × 40 = 2.080 h als Basis:** Urlaub, Feiertage und Krankheit werden ignoriert, der Satz wird zu niedrig.
- **Anwesenheit = Produktivität:** Nicht verrechenbare Zeiten mindern die Basis.
- **Personentage und Stunden verwechseln:** 1 PT = 8 h (sofern nicht anders angegeben).
- **Rundung zu früh:** Erst am Ende auf zwei Nachkommastellen runden.

## Merksatz

**"Alle Kosten durch produktive Stunden."** Je weniger produktive Stunden, desto teurer jede einzelne.

## Verwandt

- [Aufwandsschätzung](aufwandsschaetzung.md) – woher die Stunden kommen
- [Ressourcenplan](ressourcenplan.md) – wer wann verfügbar ist
- [Make-or-Buy, Kauf, Leasing, Miete](make-or-buy-kauf-leasing-miete.md) – intern vs. extern
- [Soll-Ist-Vergleich](soll-ist-vergleich.md) – geplante vs. tatsächliche Stunden
- [TCO](tco.md) – Personalkosten im Lebenszyklus
- [Sozialversicherung](../../Wirtschaft/sozialversicherung.md) – Arbeitgeberanteile
- [Lektion 22: Kostenrechnung](../analyse-der-exams/L22_Kostenrechnung.md)
