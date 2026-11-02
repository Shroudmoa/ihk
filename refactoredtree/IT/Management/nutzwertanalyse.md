# Nutzwertanalyse

## In 30 Sekunden

- Die **Nutzwertanalyse (NWA)** vergleicht Alternativen anhand **mehrerer, auch nicht in Geld messbarer Kriterien** (z. B. Bedienbarkeit, Support, Datenschutz).
- Ablauf: **Kriterien festlegen → gewichten (Summe 100 %) → Alternativen bewerten (Punkte) → Punkte × Gewicht → Summe = Nutzwert → höchster Nutzwert gewinnt.**
- **K.-o.-Kriterien** (Muss-Kriterien) werden vorher geprüft: Wer sie nicht erfüllt, scheidet aus.
- Ergänzt den rein monetären Vergleich (Kostenvergleich, TCO). Ein Kosten-Nutzen-Verhältnis ergibt sich mit `Kosten ÷ Nutzwert`.
- Prüfungsbezug: Anbieter- und Gerätevergleiche (MDM-Anbieter 2025S, Serverauswahl 2024W/2025W, Webcam-Auswahl 2022S) werden oft mit gewichteten Kriterien begründet.

## Kernwissen

### Ablauf

1. **Ziel und Alternativen** festlegen.
2. **K.-o.-Kriterien** prüfen (z. B. "Rechenzentrum in der EU", "AD-Anbindung").
3. **Bewertungskriterien** festlegen, möglichst unabhängig voneinander.
4. **Gewichtung** je Kriterium in Prozent (Summe = 100 %), ggf. per paarweisem Vergleich.
5. **Bewertungsskala** definieren, z. B. 1 (schlecht) bis 10 (sehr gut).
6. Jede Alternative je Kriterium **bewerten**.
7. **Teilnutzwert** = Gewicht × Punkte; **Nutzwert** = Summe der Teilnutzwerte.
8. **Rangfolge** bilden, Ergebnis begründen und **Sensitivität** prüfen (ändert sich das Ergebnis bei leicht anderer Gewichtung?).

### Rechenbeispiel: Auswahl eines Ticketsystems

K.-o.-Kriterium "Hosting in der EU": Alle drei Anbieter erfüllen es.

| Kriterium | Gewicht | A Punkte | A gewichtet | B Punkte | B gewichtet | C Punkte | C gewichtet |
|---|---:|---:|---:|---:|---:|---:|---:|
| Funktionsumfang | 30 % | 8 | 2,40 | 9 | 2,70 | 6 | 1,80 |
| Kosten (5 Jahre) | 25 % | 6 | 1,50 | 4 | 1,00 | 9 | 2,25 |
| Bedienbarkeit | 20 % | 9 | 1,80 | 7 | 1,40 | 7 | 1,40 |
| Schnittstellen (AD, Mail) | 15 % | 7 | 1,05 | 9 | 1,35 | 5 | 0,75 |
| Support/SLA | 10 % | 8 | 0,80 | 6 | 0,60 | 5 | 0,50 |
| **Nutzwert** | **100 %** | | **7,55** | | **7,05** | | **6,70** |

**Ergebnis:** Anbieter **A** hat den höchsten Nutzwert (7,55) und wird empfohlen, obwohl C am günstigsten ist.

**Sensitivität:** Würde man Kosten mit 40 % und Funktionsumfang mit 15 % gewichten, ergäbe sich A = 0,15·8 + 0,40·6 + 0,20·9 + 0,15·7 + 0,10·8 = 1,20 + 2,40 + 1,80 + 1,05 + 0,80 = 7,25 und C = 0,90 + 3,60 + 1,40 + 0,75 + 0,50 = 7,15 – A bleibt vorn, das Ergebnis ist robust.

### Paarweiser Vergleich zur Gewichtung

Jedes Kriterium wird gegen jedes andere verglichen: wichtiger = 2, gleich = 1, unwichtiger = 0. Die Zeilensummen ergeben, ins Verhältnis gesetzt, die Gewichte. Das macht die Gewichtung nachvollziehbarer als "Bauchgefühl".

### Kosten-Nutzen-Verhältnis

Kosten A: 30.000 €, C: 20.000 €. `Kosten pro Nutzwertpunkt`: A = 30.000 ÷ 7,55 = 3.973,51 €, C = 20.000 ÷ 6,70 = 2.985,07 €. Je nach Unternehmensziel (bester Nutzen vs. bestes Preis-Leistungs-Verhältnis) kann also auch C begründet werden – deshalb Kosten entweder **als Kriterium** in der NWA **oder separat** betrachten, nicht doppelt.

### Vor- und Nachteile

| Vorteile | Nachteile |
|---|---|
| bezieht qualitative Kriterien ein | Gewichtung und Punkte sind subjektiv |
| transparent und nachvollziehbar dokumentiert | Scheingenauigkeit durch Dezimalzahlen |
| strukturiert die Entscheidung im Team | abhängige Kriterien werden doppelt gezählt |
| leicht erweiterbar | aufwendig bei vielen Alternativen |

## Prüfungsfrage

**Frage:** Für die Beschaffung eines MDM-Systems liegen zwei Angebote vor. Kriterien und Gewichtung: Sicherheitsfunktionen 40 %, Preis 30 %, Bedienbarkeit 20 %, Support 10 %. Anbieter X erhält 9 / 5 / 7 / 8 Punkte, Anbieter Y 6 / 9 / 8 / 6 Punkte (Skala 1–10). Ermitteln Sie die Nutzwerte und geben Sie eine Empfehlung.

**Musterantwort:**
- X: 0,4·9 + 0,3·5 + 0,2·7 + 0,1·8 = 3,6 + 1,5 + 1,4 + 0,8 = **7,3**
- Y: 0,4·6 + 0,3·9 + 0,2·8 + 0,1·6 = 2,4 + 2,7 + 1,6 + 0,6 = **7,3**

Beide Anbieter erreichen denselben Nutzwert. Da Sicherheitsfunktionen das am höchsten gewichtete Kriterium sind und X dort deutlich besser abschneidet (9 statt 6), wird **X** empfohlen – vorausgesetzt, das Budget lässt den höheren Preis zu. Alternativ kann ein zusätzliches Kriterium oder eine Sensitivitätsanalyse die Entscheidung absichern.

## Falle

- **Gewichte summieren sich nicht auf 100 %** → Ergebnis verzerrt.
- **Skala umgedreht:** Beim Kriterium "Preis" bekommt das **günstigste** Angebot die **meisten** Punkte.
- **K.-o.-Kriterien gewichten:** Ein Muss-Kriterium gehört nicht in die Gewichtung, sondern davor.
- **Nur rechnen, nicht begründen:** Die Empfehlung muss mit Bezug auf die wichtigsten Kriterien begründet werden.
- **Kosten doppelt berücksichtigen** (als Kriterium und zusätzlich im Kostenvergleich).

## Merksatz

**"Gewicht mal Punkte, Summe bilden, Sieger begründen."** K.-o. zuerst, Preis-Skala umdrehen, Gewichte = 100 %.

## Verwandt

- [Angebotsvergleich & Bezugskalkulation](angebotsvergleich-bezugskalkulation.md) – Preise vergleichbar machen
- [Kostenvergleichsrechnung](kostenvergleichsrechnung.md) – monetärer Vergleich
- [TCO](tco.md) – Kostenkriterium über die Nutzungsdauer
- [Make-or-Buy, Kauf, Leasing, Miete](make-or-buy-kauf-leasing-miete.md)
- [Lastenheft & Pflichtenheft](lastenheft-pflichtenheft.md) – woher die Kriterien kommen
- [MDM & BYOD](../FiSi-Administrator/Netzwerk-Administration/mdm-byod.md) – Anbietervergleich 2025S
- [Lektion 28: GA1-Praxisthemen](../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md) – Serverauswahl anhand von Spezifikationen
