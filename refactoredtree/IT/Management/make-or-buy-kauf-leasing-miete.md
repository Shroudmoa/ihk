# Make-or-Buy sowie Kauf, Leasing und Miete

## In 30 Sekunden

- **Make-or-Buy:** Leistung **selbst erstellen** (Eigenentwicklung, eigener Betrieb) oder **fremd beziehen** (Standardsoftware, Dienstleister, Outsourcing, Cloud).
- Entscheidung nach **Kosten** (interner Stundensatz vs. Angebotspreis), **Know-how**, **Kapazität**, **Kernkompetenz**, **Abhängigkeit** und **Datenschutz**.
- **Kauf:** Eigentum, hohe Anfangsinvestition (CAPEX), Abschreibung über die Nutzungsdauer, volle Verfügungsgewalt.
- **Leasing:** Nutzungsüberlassung gegen monatliche Raten, oft mit Service, Gerätetausch nach Laufzeit, Raten als Betriebsausgabe (OPEX), Leasinggeber bleibt Eigentümer.
- **Miete:** kurzfristige, flexible Nutzungsüberlassung, meist kündbar, Vermieter trägt Instandhaltung; teuer bei langer Nutzung.
- Gesamtkosten immer über **dieselbe Laufzeit** vergleichen (TCO).

## Kernwissen

### Make-or-Buy-Kriterien

| Für **Make** (Eigenleistung) | Für **Buy** (Fremdbezug) |
|---|---|
| Know-how und freie Kapazität vorhanden | fehlendes Spezialwissen, Personalengpass |
| Kernkompetenz, Wettbewerbsvorteil | Standardleistung ohne strategischen Wert |
| hohe Anforderungen an Datenschutz/Geheimhaltung | Anbieter mit Zertifizierung (ISO 27001) und SLA |
| interne Kosten niedriger | Skaleneffekte des Anbieters, Fixkosten werden variabel |
| Unabhängigkeit von Lieferanten | schnelle Verfügbarkeit, kein eigener Aufbau |

**Rechenbeispiel:** Eine interne Webanwendung erfordert 300 h Entwicklung zu 55 €/h intern = 16.500 € plus 2.000 € pro Jahr Pflege. Eine Standardsoftware kostet 6.000 € Lizenz + 15 % Wartung (900 €/Jahr) + 40 h Anpassung à 55 € = 2.200 €.
Über 4 Jahre: Make = 16.500 + 4 × 2.000 = **24.500 €**, Buy = 6.000 + 2.200 + 4 × 900 = **11.800 €**. Buy ist deutlich günstiger, sofern die Standardsoftware die Muss-Anforderungen erfüllt.

### Kauf, Leasing, Miete im Vergleich

| Kriterium | Kauf | Leasing | Miete |
|---|---|---|---|
| Eigentum | Käufer | Leasinggeber | Vermieter |
| Laufzeit | unbegrenzt | fest (z. B. 36 Monate) | kurz, kündbar |
| Kapitalbedarf zu Beginn | hoch | gering (ggf. Sonderzahlung) | gering |
| Bilanz/Kostenart | Anlagevermögen, Abschreibung (CAPEX) | Raten als Aufwand (OPEX), bilanzneutral beim Leasingnehmer (HGB) | Mietaufwand (OPEX) |
| Wartung, Risiko des Untergangs | beim Käufer | meist beim Leasingnehmer (Versicherungspflicht), Service optional | beim Vermieter |
| Flexibilität | gering (Wiederverkauf) | mittel (Tausch zum Laufzeitende) | hoch |
| Gesamtkosten bei langer Nutzung | meist am niedrigsten | höher (Zins, Marge) | am höchsten |
| typischer Einsatz | langlebige Infrastruktur, eigene Server | Notebook-/Drucker-Flotten mit festem Erneuerungszyklus | Messe, Schulung, Projektspitzen |

### Rechenbeispiel: 30 Notebooks für 3 Jahre

| Variante | Rechnung | Kosten 3 Jahre |
|---|---|---:|
| **Kauf** | 30 × 1.100 € + Garantieverlängerung 30 × 120 € − Restwert 30 × 150 € | 32.100 € |
| **Leasing** | 30 × 36 € × 36 Monate (inkl. Vor-Ort-Service) | 38.880 € |
| **Miete** | 30 × 55 € × 36 Monate | 59.400 € |

Rechnung Kauf: 33.000 + 3.600 − 4.500 = 32.100 €.

**Bewertung:** Kauf ist am günstigsten, bindet aber 36.600 € Kapital sofort. Leasing kostet 6.780 € mehr, schont die Liquidität und enthält Service sowie den Gerätetausch nach 3 Jahren. Miete lohnt sich nur für kurze Zeiträume: Für 2 Monate Schulungsbetrieb kostet sie 30 × 55 € × 2 = 3.300 €, ein Kauf wäre dafür unwirtschaftlich.

### Weitere Beschaffungsmodelle in der IT

- **Software as a Service / Subscription:** Nutzung gegen laufende Gebühr, Updates inklusive.
- **Managed Services:** Dienstleister betreibt Systeme nach SLA.
- **Device as a Service:** Geräte, Rollout, Support und Rücknahme als Paket pro Monat.
- **Outsourcing / Outtasking:** komplette Bereiche bzw. einzelne Aufgaben werden ausgelagert.

## Prüfungsfrage

**Frage:** Ihr Unternehmen benötigt für ein 18-monatiges Projekt 10 zusätzliche Workstations. Vergleichen Sie Kauf, Leasing und Miete anhand von drei Kriterien und geben Sie eine begründete Empfehlung.

**Musterantwort:** **Kapitalbedarf:** Der Kauf erfordert die volle Anschaffung sofort, Leasing und Miete verteilen die Kosten auf monatliche Raten. **Flexibilität:** Mietverträge sind kurzfristig kündbar, Leasingverträge haben feste Laufzeiten (häufig 24–36 Monate), gekaufte Geräte müssen nach dem Projekt weiterverwendet oder verkauft werden. **Gesamtkosten:** Bei langer Nutzung ist der Kauf meist am günstigsten, bei kurzer Nutzung die Miete, weil kein Restwertrisiko besteht. **Empfehlung:** Da die Geräte nur 18 Monate benötigt werden und danach keine Verwendung absehbar ist, ist **Miete** (oder ein Leasingvertrag mit passender Laufzeit) zu empfehlen. Ein Kauf wäre nur sinnvoll, wenn die Workstations nach dem Projekt andere Altgeräte ersetzen.

## Falle

- **Leasing = Miete:** Leasing ist langfristig, mit fester Laufzeit; Risiken (Beschädigung, Versicherung) trägt meist der Leasingnehmer.
- **Restwert beim Kauf vergessen:** Der Verkaufserlös am Ende senkt die Kosten.
- **Unterschiedliche Leistungsumfänge vergleichen:** Leasing mit Service vs. Kauf ohne Garantieverlängerung ist kein fairer Vergleich.
- **Make ohne Folgekosten:** Eigenentwicklungen brauchen Pflege, Dokumentation und Know-how-Sicherung.
- **Buy ohne Exit-Strategie:** Datenexport und Vertragslaufzeit gegen Vendor Lock-in prüfen.

## Merksatz

**"Kaufen für lange, leasen für planbar, mieten für kurz."** Make, wenn es Kernkompetenz ist – Buy, wenn es Standard ist.

## Verwandt

- [CAPEX-OPEX](capex-opex.md) – Investition vs. laufende Kosten
- [TCO](tco.md) – Gesamtkosten über die Laufzeit
- [Kostenvergleichsrechnung](kostenvergleichsrechnung.md) – kritische Menge
- [Amortisation](amortisation.md)
- [Stundensatz intern](stundensatz-intern.md) – Eigenleistung bewerten
- [Nutzwertanalyse](nutzwertanalyse.md) – qualitative Kriterien
- [Cloud-Computing](../FiSi-Administrator/Server/Cloud/cloud-computing.md) – Buy als Service
- [Lektion 10: Lizenzierung](../analyse-der-exams/10_Lizenzierung.md) – Lizenzmodelle beim Fremdbezug
- [Lektion 22: Kostenrechnung](../analyse-der-exams/L22_Kostenrechnung.md)
