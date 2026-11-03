# CAPEX und OPEX

## In 30 Sekunden

- **CAPEX** (Capital Expenditure) = **Investitionsausgaben** für Anlagegüter, die über Jahre genutzt werden (Server, Switches, eigene Rechenzentrumstechnik). Sie werden **aktiviert und abgeschrieben**.
- **OPEX** (Operational Expenditure) = **laufende Betriebsausgaben** (Cloud-Abo, Miete, Leasingraten, Strom, Wartung, Gehälter). Sie sind sofort **Aufwand** der Periode.
- Cloud-Computing verschiebt Kosten typischerweise **von CAPEX zu OPEX** ("pay as you go").
- CAPEX bindet Kapital und belastet die Liquidität zu Beginn, OPEX ist flexibel, kann über lange Laufzeit aber teurer werden.
- Vergleich immer über die Gesamtkosten der Laufzeit ([TCO](tco.md)).

## Kernwissen

| Begriff | Vollständiger Name | Erklärung |
|---|---|---|
| **CAPEX** | Capital Expenditure | Investitionsausgaben, z. B. Server, Switches oder eigene Rechenzentrums-Hardware. |
| **OPEX** | Operational Expenditure | Laufende Betriebsausgaben, z. B. Cloud-Abos, Strom, Wartung und Support. |
| **TCO** | Total Cost of Ownership | Gesamtkosten über den Lebenszyklus, inklusive Anschaffung, Betrieb, Wartung und Entsorgung. |
| **ROI** | Return on Investment | Bewertet das Verhältnis zwischen Investition und wirtschaftlichem Nutzen. |
| **Vendor Lock-in** | Herstellerbindung | Ein Wechsel wird durch proprietäre Technologien, Datenformate, Schnittstellen oder hohe Migrationskosten erschwert. |
| **Exit Strategy** | Ausstiegsstrategie | Plant, wie ein Anbieterwechsel technisch, organisatorisch und finanziell umgesetzt werden kann. |
| **SLA** | Service Level Agreement | Vereinbart messbare Serviceleistungen, z. B. Verfügbarkeit und Reaktionszeiten. |
| **Skalierbarkeit** | Scalability | Fähigkeit, Ressourcen an steigenden oder sinkenden Bedarf anzupassen. |
| **Opportunitätskosten** | Opportunity Cost | Nutzen der besten Alternative, auf die man durch eine Entscheidung verzichtet. |
| **Abschreibung** | Depreciation | Verteilt die Anschaffungs- oder Herstellungskosten eines Vermögensgegenstands buchhalterisch über seine Nutzungsdauer. |
| **Break-even** | Gewinnschwelle | Punkt, an dem Erlöse und Kosten gleich hoch sind. |
| **Make or Buy** | Eigenentwicklung oder Fremdbezug | Entscheidung, ob eine Leistung selbst erstellt oder extern eingekauft wird. |
| **OPEX-Modell** | Laufendes Kostenmodell | Kosten fallen regelmäßig an, beispielsweise als monatliche Cloud-Gebühr. |
| **CAPEX-Modell** | Investitionsmodell | Höhere Anfangsinvestition, beispielsweise für eigene Serverhardware. |

### Beispiel: Eigener Server vs. Cloud

| Kriterium | Eigener Server | Cloud-Dienst |
|---|---|---|
| Kostenmodell | Häufig hoher CAPEX | Häufig laufender OPEX |
| Wartung | Eigenverantwortung oder Dienstleister | Je nach Service-Modell teilweise beim Anbieter |
| Skalierung | Hardware muss erweitert werden | Ressourcen häufig kurzfristig anpassbar |
| Kontrolle | Mehr Kontrolle über Hardware und Umgebung | Abhängig vom Anbieter und Service-Modell |
| Vendor Lock-in | Möglich, z. B. durch proprietäre Hardware | Möglich, z. B. durch proprietäre APIs und Dienste |
| Ausstieg | Hardwarewechsel und Datenmigration | Datenexport, Migration und Vertragsbedingungen prüfen |

### IHK-Fakten

- **CAPEX vs. OPEX:** Anschaffung bzw. Investition gegenüber laufenden Betriebsausgaben.
- **TCO statt nur Kaufpreis:** Auch Strom, Personal, Lizenzen, Wartung und Migration berücksichtigen.
- **Vendor Lock-in reduzieren:** Offene Standards, portable Datenformate, dokumentierte Schnittstellen und getestete Datenexporte nutzen.
- **Cloud ist nicht automatisch günstiger:** Laufzeit, Datenübertragung, Support, Skalierung und Ressourcenverbrauch beeinflussen die Gesamtkosten.
- **DSGVO:** Bei Cloud-Diensten Auftragsverarbeitung, Datenstandorte, Zugriffsschutz und gegebenenfalls Drittlandtransfers prüfen.

### Rechenbeispiel: Wirkung auf das Jahresergebnis

Ein Server kostet 15.000 € (Nutzungsdauer 5 Jahre, lineare Abschreibung). Alternativ kostet eine vergleichbare Cloud-Instanz 300 € pro Monat.

| | Kauf (CAPEX) | Cloud (OPEX) |
|---|---:|---:|
| Auszahlung im Jahr 1 | 15.000 € | 3.600 € |
| Aufwand im Jahr 1 (Gewinn- und Verlustrechnung) | 3.000 € Abschreibung | 3.600 € |
| Summe über 5 Jahre (ohne Strom, Wartung, Personal) | 15.000 € | 18.000 € |

Der Kauf bindet sofort 15.000 € Liquidität, belastet das Ergebnis aber nur mit 3.000 € pro Jahr. Die Cloud schont die Liquidität und ist flexibel kündbar, kostet hier über 5 Jahre jedoch 3.000 € mehr – bevor Strom, Wartung und Administration des eigenen Servers eingerechnet sind (siehe [TCO](tco.md)).

## Prüfungsfrage

**Frage:** Die Geschäftsführung überlegt, die eigene Serverlandschaft durch IaaS zu ersetzen. Erläutern Sie die Begriffe CAPEX und OPEX und nennen Sie je einen Vor- und Nachteil der Verlagerung zu OPEX.

**Musterantwort:** **CAPEX** sind Investitionsausgaben für langlebige Wirtschaftsgüter wie eigene Server; sie werden aktiviert und über die Nutzungsdauer abgeschrieben. **OPEX** sind laufende Betriebsausgaben wie monatliche Cloud-Gebühren, die direkt als Aufwand verbucht werden. **Vorteil** der Verlagerung zu OPEX: keine hohe Anfangsinvestition, Kosten passen sich dem tatsächlichen Verbrauch an (Skalierbarkeit), Liquidität bleibt erhalten. **Nachteil:** Über lange Laufzeiten können die Gesamtkosten höher sein, und es entsteht eine Abhängigkeit vom Anbieter (Vendor Lock-in, Preisänderungen, Datenmigration beim Wechsel).

## Falle

- **CAPEX = teuer, OPEX = billig:** Entscheidend sind die Gesamtkosten über die Laufzeit.
- **Abschreibung als Auszahlung:** Die Auszahlung erfolgt beim Kauf, die Abschreibung verteilt nur den Aufwand.
- **Leasing als CAPEX:** Leasingraten sind in der Regel OPEX (beim Leasingnehmer nach HGB).
- **Personalkosten vergessen:** Gehälter sind OPEX und entfallen bei der Cloud nicht vollständig.
- **Datenschutz ausblenden:** Die Cloud-Entscheidung ist nie nur eine Kostenfrage.

## Merksatz

 CAPEX beschreibt Investitionsausgaben, OPEX laufende Betriebsausgaben und TCO die Gesamtkosten über den Lebenszyklus. Vendor Lock-in bezeichnet die erschwerte Wechselmöglichkeit zu einem anderen Anbieter.

## Verwandt

- [TCO](tco.md) – Gesamtkosten über den Lebenszyklus
- [Make-or-Buy, Kauf, Leasing, Miete](make-or-buy-kauf-leasing-miete.md)
- [Kostenvergleichsrechnung](kostenvergleichsrechnung.md)
- [Amortisation](amortisation.md)
- [Cloud-Computing](../FiSi-Administrator/Server/Cloud/cloud-computing.md)
- [SLA](../FiSi-Administrator/Server/Cloud/sla.md)
- [Break-Even-Point](../../Wirtschaft/Unternehmen/bep.md)
- [Lektion 1: Cloud Computing](../analyse-der-exams/01_Cloud_Computing.md)
