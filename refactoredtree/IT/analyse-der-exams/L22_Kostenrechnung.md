# Lektion 22 – Kosten- & Wirtschaftlichkeitsrechnung

**Bereich:** Wirtschafts- und Sozialkunde (WiSo), Rechnungswesen  
**Relevanz:** Sehr hoch – in AP2 regelmäßig als Rechenaufgabe (z. B. Angebot vergleichen, Break-even, Amortisation).  
**Lernzeit:** ca. 2h

## 1. Warum das wichtig ist

In der AP2 wird oft verlangt, ob ein Projekt wirtschaftlich ist, welches Angebot
das günstigste ist oder nach wie vielen Jahren sich eine Investition rechnet.
Ohne diese Formeln verliert man schnell Punkte – sie sind aber mit wenig Aufwand
sicher beherrschbar.

## 2. Grundlagen: Kostenarten

| Kostenart | Abhängigkeit | Beispiele |
|-----------|--------------|-----------|
| **Fixkosten** | Unabhängig von der Menge | Miete, Gehälter (fest), Versicherungen, Abschreibungen |
| **Variable Kosten** | Abhängig von der Stückzahl | Material, Lizenz pro User, Fracht |

Gesamtkosten = Fixkosten + (variable Stückkosten × Menge)

## 3. Wirtschaftlichkeitsvergleich

- **Deckungsbeitrag pro Stück:** `DB = Preis − variable Stückkosten`
- **Deckungsbeitrag gesamt:** `DB_ges = Gesamtumsatz − variable Gesamtkosten`
- Der DB deckt die Fixkosten; was darüber bleibt, ist Gewinn.

**Break-even (Gewinnschwelle):** Menge, bei der `DB_ges = Fixkosten`.

```
Break-even-Menge = Fixkosten / (Preis − variable Stückkosten)
```

## 4. Wirtschaftlichkeit & Amortisation

- **Wirtschaftlichkeit:** Verhältnis `Ertrag / Aufwand`. Je größer, desto besser.
- **Amortisation (Rückgewinnungszeit):**

```
Amortisationsdauer = Anschaffung / jährliche Einsparung   (in Jahren)
```

- **Eigenkapitalrentabilität:**

```
Eigenkapitalrentabilität = (Reingewinn / Eigenkapital) × 100   (in %)
```

## 5. Rechenbeispiel: Zwei Angebote vergleichen

Ein Server soll beschafft werden. Die laufende Einsparung gegenüber Altbetrieb
beträgt 4.000 €/Jahr.

| Angebot | Anschaffung | Fixkosten/Jahr | Variable Kosten/Jahr |
|---------|-------------|----------------|----------------------|
| A | 20.000 € | 2.000 € | 1.000 € |
| B | 28.000 € | 1.000 € | 1.500 € |

Gesamtkosten/Jahr bei 10 Servern (Annahme):

- A: 2.000 + 10 × 1.000 = 12.000 €
- B: 1.000 + 10 × 1.500 = 16.000 €

→ A ist im laufenden Betrieb günstiger.

**Amortisation der Mehrinvestition** von B gegenüber A (8.000 € teurer):
B spart pro Jahr 4.000 € (12.000 − 16.000 = −4.000, also B kostet 4.000 mehr
→ hier lohnt sich B **nicht**). Bei umgekehrten Werten würde man
`8.000 / 4.000 = 2 Jahre` rechnen.

## 6. Übungsaufgabe (im Prüfungsstil, selbst erstellt)

> Eine neue Firewall kostet 15.000 € (Fixkosten unverändert 3.000 €/Jahr,
> variable Kosten 0). Sie spart jährlich 5.000 € Wartungsaufwand.
> Ein Cloud-Dienst kostet keine Anschaffung, aber 6.000 €/Jahr dauerhaft.
> Welche Lösung ist nach 4 Jahren wirtschaftlicher?

<details><summary>Lösung (aufklappen)</summary>

**Firewall (Kauf):**
- Anschaffung: 15.000 €
- Laufend 4 Jahre: 4 × 3.000 € = 12.000 €
- Gesamt 4 J.: 27.000 €

**Cloud:**
- Laufend 4 Jahre: 4 × 6.000 € = 24.000 €
- Gesamt 4 J.: 24.000 €

→ Die **Cloud-Lösung** ist nach 4 Jahren mit 24.000 € günstiger als die
Firewall mit 27.000 €.

**Amortisation der Firewall** gegenüber Cloud-Mehrkosten pro Jahr
(6.000 − 3.000 = 3.000 € gespart):
`15.000 / 3.000 = 5 Jahre`. Erst nach 5 Jahren wäre die Firewall günstiger.

</details>

## Selbstcheck – kannst du das?

- [ ] Ich unterscheide Fix- und variable Kosten an Beispielen.
- [ ] Ich kann einen Wirtschaftlichkeitsvergleich zweier Angebote durchführen.
- [ ] Ich berechne die Eigenkapitalrentabilität in % und wende echte Prüfungszahlen an.
- [ ] Ich berechne die Amortisationsdauer einer Mehrinvestition.
- [ ] Ich vergleiche zwei Angebote anhand Gesamtkosten.
- [ ] Ich kenne die typischen Fallstricke: Deckungsbeitragsrechnung und Break-even kommen in den echten Prüfungen 2021–2026 nicht vor – verschwende keine Lernzeit daran.

## Weiterführende Ressourcen

- IHK-Lösungs-PDFs im Ordner `*/WiSo/*` und `*/Loesungen/WISO_*`
- Lektion 11 – WiSo Grundlagen (Kostenbegriffe)
- Klassisches WiSo-Lehrbuch: Kapitel Rechnungswesen / Investitionsrechnung
