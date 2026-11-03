# Break-Even-Point (Gewinnschwelle) & Deckungsbeitrag

## In 30 Sekunden

- **Break-Even-Point (BEP)** = Absatzmenge, bei der **Erlöse = Gesamtkosten**: kein Gewinn, kein Verlust.
- Formel: **BEP-Menge = Fixkosten / (Preis − variable Stückkosten) = Kf / db**.
- **Deckungsbeitrag je Stück (db)** = Verkaufspreis − variable Stückkosten; er „deckt“ zuerst die Fixkosten, danach entsteht Gewinn.
- **Gewinn** = db × Menge − Fixkosten.
- Preisuntergrenze: **kurzfristig** = variable Stückkosten, **langfristig** = Selbstkosten (Stückkosten inkl. Fixkostenanteil).
- IT-Bezug: Ab wie vielen Wartungsverträgen lohnt sich ein neuer Service-Desk? Lohnt Kauf oder Miete eines Servers? (kritische Menge)

## Kernwissen

### Kostenbegriffe

| Begriff | Bedeutung | IT-Beispiel |
|---|---|---|
| **Fixkosten (Kf)** | unabhängig von der Menge, fallen auch bei 0 Stück an | Miete, Gehälter, Leasing, Lizenzflatrate |
| **variable Kosten (Kv)** | steigen mit der Menge | Hardware je verkauftem PC, Datenvolumen, Provision |
| **variable Stückkosten (kv)** | Kv / Menge | 400 € Material je PC |
| **Gesamtkosten (K)** | K = Kf + kv × x | |
| **Erlös/Umsatz (E)** | E = p × x | |
| **Stückkosten (k)** | K / x – sinken mit steigender Menge (**Fixkostendegression**) | |

### Formeln

| Größe | Formel |
|---|---|
| Deckungsbeitrag je Stück | db = p − kv |
| Gesamtdeckungsbeitrag | DB = db × x = E − Kv |
| Gewinn | G = DB − Kf = (p − kv) × x − Kf |
| **Break-Even-Menge** | x_BEP = Kf / db |
| Break-Even-Umsatz | E_BEP = x_BEP × p |
| Menge für Zielgewinn | x = (Kf + Zielgewinn) / db |
| Sicherheitsabstand | (geplante Menge − x_BEP) / geplante Menge × 100 |
| **Kritische Menge** (Verfahrensvergleich) | x_krit = (Kf₂ − Kf₁) / (kv₁ − kv₂) |

### Grafik (Prinzip)

- **Fixkostenlinie:** waagerecht in Höhe Kf.
- **Gesamtkostenlinie:** beginnt bei Kf auf der y-Achse und steigt mit der Steigung kv.
- **Erlöslinie:** beginnt im Ursprung und steigt steiler (Steigung p).
- **Schnittpunkt** von Erlös- und Gesamtkostenlinie = BEP; links davon Verlustzone, rechts Gewinnzone.

Wertetabelle zum Beispiel unten (Kf = 50.000 €, kv = 15 €, p = 25 €):

| Menge x | Erlöse | Gesamtkosten | Ergebnis |
|---:|---:|---:|---:|
| 0 | 0 € | 50.000 € | −50.000 € |
| 3.000 | 75.000 € | 95.000 € | −20.000 € |
| **5.000** | **125.000 €** | **125.000 €** | **0 € (BEP)** |
| 7.000 | 175.000 € | 155.000 € | +20.000 € |

### Rechenbeispiel

Nordlicht bietet Managed-Workplace-Pakete an (Station 7): Preis 25 € pro Paket und Monat, variable Kosten 15 € (Lizenzen, Support-Anteil), Fixkosten 50.000 € pro Jahr.

- db = 25 − 15 = **10 €**
- x_BEP = 50.000 / 10 = **5.000 Pakete** (Paketmonate) pro Jahr
- Bei 6.500 Paketen: G = 10 × 6.500 − 50.000 = **15.000 € Gewinn**
- Für 20.000 € Zielgewinn: (50.000 + 20.000) / 10 = **7.000 Pakete**

### Kritische Menge: Kauf oder Miete?

Server mieten: 0 € Fixkosten, 2,00 € pro Nutzerstunde. Server kaufen: 6.000 € Fixkosten (Abschreibung) pro Jahr, 0,50 € pro Nutzerstunde.
x_krit = (6.000 − 0) / (2,00 − 0,50) = **4.000 Stunden**. Unterhalb lohnt Miete, oberhalb Kauf.

### Vorteile / Grenzen

- **Vorteil:** zeigt die Mindestmenge zur Kostendeckung, Grundlage für Preisentscheidungen und Make-or-Buy.
- **Grenzen:** unterstellt konstante Preise und lineare Kosten, nur ein Produkt; sprungfixe Kosten (zusätzlicher Mitarbeiter ab x Tickets) und Preisnachlässe werden nicht abgebildet.

## Prüfungsfrage

**Frage (Rechenaufgabe):** Ein IT-Dienstleister verkauft Backup-Appliances für 1.200 € je Stück. Die variablen Kosten betragen 750 € je Stück, die Fixkosten 81.000 € pro Jahr.
a) Berechnen Sie den Deckungsbeitrag je Stück.
b) Berechnen Sie die Break-Even-Menge.
c) Wie hoch ist der Gewinn bei einem Absatz von 240 Stück?

**Musterantwort:**
a) db = 1.200 − 750 = **450 €**
b) x_BEP = 81.000 / 450 = **180 Stück**
c) G = 450 × 240 − 81.000 = 108.000 − 81.000 = **27.000 €**

## Falle

- Fixkosten **nicht** in den Deckungsbeitrag je Stück einrechnen – db = p − kv.
- BEP ist eine **Menge** (Stück); in € ist es der Break-Even-**Umsatz**.
- Bei Nachkommastellen **aufrunden**: Bei 180,3 Stück ist der BEP erst mit 181 Stück erreicht.
- Kurzfristige Preisuntergrenze ist kv, nicht der Selbstkostenpreis – kurzfristig darf man unter den Vollkosten verkaufen, solange ein positiver db bleibt.
- Kritische Menge: Differenz der **Fixkosten** durch Differenz der **variablen** Kosten, nicht umgekehrt.

## Merksatz

**„Der Deckungsbeitrag zahlt erst die Fixkosten ab – jedes Stück danach ist Gewinn.“**

## Verwandt

- [Ökonomisches Prinzip & Kennzahlen](oekonomisches-prinzip-kennzahlen.md) – Wirtschaftlichkeit, Rentabilität
- [Unternehmensgründung & Businessplan](unternehmensgruendung-businessplan.md) – Rentabilitätsvorschau
- [Unternehmensziele & Zielkonflikte](unternehmensziele-zielkonflikte.md) – Gewinn als ökonomisches Ziel
- [Make or Buy, Kauf, Leasing, Miete (IT)](../../IT/Management/make-or-buy-kauf-leasing-miete.md) – kritische Menge in der Praxis
- [Kostenvergleichsrechnung (IT)](../../IT/Management/kostenvergleichsrechnung.md)
- [Lektion 22 – Kostenrechnung](../../IT/analyse-der-exams/L22_Kostenrechnung.md) – Kostenarten, Amortisation, Angebotsvergleich
- Prüfungsbezug: in der WiSo bisher nicht gerechnet, aber Grundlage für GA1-Kalkulationen
