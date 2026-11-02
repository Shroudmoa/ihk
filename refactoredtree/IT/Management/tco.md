# TCO – Total Cost of Ownership

## In 30 Sekunden

- **TCO** sind die **Gesamtkosten eines IT-Systems über den ganzen Lebenszyklus**: Anschaffung, Einführung, Betrieb, Support und Ausmusterung.
- Ziel: Alternativen fair vergleichen, z. B. eigener Server vs. Cloud, Kauf vs. Leasing, Produkt A vs. B.
- Faustformel: `TCO = einmalige Kosten + Σ laufende Kosten über die Nutzungsdauer + Kosten am Lebensende`.
- **Direkte Kosten** (Budget, sichtbar) und **indirekte Kosten** (versteckt: Ausfallzeiten, Selbsthilfe der Anwender, Schulung).
- Der Kaufpreis macht bei IT oft nur einen kleinen Teil der TCO aus; Personal und Betrieb dominieren.
- Prüfungsbezug: Stromkosten eines Server-Clusters (2023W), LTO-Band mit niedrigerer TCO als All-Disk (2022W), Kosten von Cloud vs. eigener Lösung.

## Kernwissen

### Kostenbestandteile

| Phase | Beispiele | direkt / indirekt |
|---|---|---|
| **Anschaffung** | Hardware, Lizenzen, Zubehör, Lieferung | direkt |
| **Einführung** | Installation, Migration, Konfiguration, Schulung | direkt |
| **Betrieb** | Strom, Kühlung, Stellfläche, Wartungsverträge, Software-Subscriptions, Cloud-Gebühren | direkt |
| **Administration und Support** | Personalkosten Admin, Helpdesk, Patchmanagement, Backup | direkt |
| **Ausfälle und Produktivität** | Ausfallzeiten, Wartezeit der Anwender, gegenseitige Hilfe ("Kollege, kannst du mal?") | indirekt |
| **Lebensende** | Datenlöschung (zertifiziert), Entsorgung (ElektroG), Migration zum Nachfolger | direkt |

### Stromkosten berechnen

`Kosten = Leistung (kW) × Stunden × Preis pro kWh`

Ein Server mit 400 W im Dauerbetrieb: 0,4 kW × 8.760 h = 3.504 kWh pro Jahr; bei 0,30 €/kWh = **1.051,20 € pro Jahr**. Für Kühlung wird oft ein Zuschlag über den **PUE-Wert** gerechnet (PUE 1,5 → × 1,5 = 1.576,80 €).

### Rechenbeispiel: eigener Server vs. Cloud über 5 Jahre

Annahmen: interner Stundensatz Administration 60 €/h, Strompreis 0,30 €/kWh.

**Eigener Server (On-Premises)**

| Position | Rechnung | Betrag |
|---|---|---:|
| Hardware | | 12.000,00 € |
| Lizenzen | | 4.000,00 € |
| Einrichtung | 20 h × 60 € | 1.200,00 € |
| Strom | 1.051,20 €/Jahr × 5 | 5.256,00 € |
| Wartungsvertrag | 1.000 €/Jahr × 5 | 5.000,00 € |
| Administration | 4 h/Monat × 12 × 60 € = 2.880 €/Jahr × 5 | 14.400,00 € |
| Entsorgung und Datenlöschung | | 300,00 € |
| **TCO 5 Jahre** | | **42.156,00 €** |

**Cloud (IaaS)**

| Position | Rechnung | Betrag |
|---|---|---:|
| Migration | | 2.000,00 € |
| Abonnement | 600 €/Monat × 60 Monate | 36.000,00 € |
| Administration | 1 h/Monat × 60 × 60 € | 3.600,00 € |
| **TCO 5 Jahre** | | **41.600,00 €** |

**Ergebnis:** Die Cloud ist über 5 Jahre um 556 € günstiger, also fast gleichauf. Die Entscheidung fällt deshalb über qualitative Kriterien (Datenschutz, Skalierbarkeit, Abhängigkeit vom Anbieter) – typischer Fall für eine [Nutzwertanalyse](nutzwertanalyse.md). Würde der eigene Server 7 statt 5 Jahre genutzt, verschiebt sich das Ergebnis zugunsten On-Premises.

### TCO pro Arbeitsplatz

Werden die TCO durch die Anzahl der Arbeitsplätze und Jahre geteilt, entsteht eine Kennzahl für Benchmarks: 42.156 € ÷ 5 Jahre ÷ 25 Nutzer = **337,25 € pro Nutzer und Jahr**.

### Maßnahmen zur TCO-Senkung

- Standardisierung der Hardware und Software (weniger Varianten, einfacher Support)
- zentrale Verwaltung (Softwareverteilung, Gruppenrichtlinien, MDM)
- Virtualisierung und Konsolidierung
- energieeffiziente Geräte (80-PLUS-Netzteile, Energiesparfunktionen)
- Schulung der Anwender (weniger Tickets)
- Self-Service-Portal und Wissensdatenbank

## Prüfungsfrage

**Frage:** Die Geschäftsführung möchte das günstigste Angebot für 20 neue Notebooks wählen. Erläutern Sie, warum ein Vergleich der Anschaffungspreise nicht ausreicht, und nennen Sie vier Kostenarten, die in eine TCO-Betrachtung einfließen.

**Musterantwort:** Der Anschaffungspreis bildet nur einen Teil der Kosten ab, die über die Nutzungsdauer entstehen. Ein günstigeres Gerät kann durch höhere Ausfallraten, kürzere Garantie oder höheren Supportaufwand insgesamt teurer werden. Die TCO berücksichtigen deshalb auch (1) **Einführungskosten** wie Image-Erstellung und Rollout, (2) **Betriebskosten** wie Strom und Garantieverlängerung bzw. Wartung, (3) **Support- und Administrationskosten** für Helpdesk und Updates sowie (4) **Kosten am Lebensende** für zertifizierte Datenlöschung und Entsorgung. Zusätzlich fließen indirekte Kosten wie Ausfallzeiten der Mitarbeitenden ein.

## Falle

- **Nur Anschaffung vergleichen:** Gerade bei Cloud vs. On-Premises entscheidet der Betrieb.
- **Personalkosten vergessen:** Administrationsstunden sind oft der größte Posten.
- **Unterschiedliche Zeiträume:** Alternativen nur über dieselbe Laufzeit vergleichen.
- **Watt und kWh verwechselt:** Leistung (W) × Zeit (h) = Energie (Wh). 400 W sind nicht 400 kWh.
- **Indirekte Kosten als "nicht messbar" weglassen:** Sie lassen sich schätzen (Ausfallstunden × Stundensatz).

## Merksatz

**"Der Kaufpreis ist die Spitze des Eisbergs."** TCO = kaufen + einführen + betreiben + betreuen + entsorgen.

## Verwandt

- [CAPEX-OPEX](capex-opex.md) – Investition vs. laufende Kosten
- [Amortisation](amortisation.md) – wann sich die Investition rechnet
- [Kostenvergleichsrechnung](kostenvergleichsrechnung.md) – Alternativen pro Jahr vergleichen
- [Make-or-Buy, Kauf, Leasing, Miete](make-or-buy-kauf-leasing-miete.md)
- [Nutzwertanalyse](nutzwertanalyse.md) – wenn die Kosten fast gleich sind
- [Stundensatz intern](stundensatz-intern.md) – Personalkosten bewerten
- [Cloud-Computing](../FiSi-Administrator/Server/Cloud/cloud-computing.md)
- [Lektion 28: GA1-Praxisthemen](../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md) – Stromkostenberechnung 2023W
- [Lektion 22: Kostenrechnung](../analyse-der-exams/L22_Kostenrechnung.md)
