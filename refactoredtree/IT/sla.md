# SLA

# SLA – Service Level Agreement

**SLA (Service Level Agreement)** = vertragliche Vereinbarung über die zugesicherte Servicequalität zwischen Dienstleister und Kunde.

Ein SLA legt beispielsweise Verfügbarkeit, Reaktionszeiten, Entstörungszeiten, Wartungsfenster, Messmethoden und mögliche Konsequenzen bei Nichterfüllung fest.

## Verfügbarkeit: die „Nines“

Die Verfügbarkeit wird häufig in Prozent angegeben. Je mehr Neunen, desto weniger Ausfallzeit ist erlaubt.

**Formel:**

`Ausfallzeit = Bezugszeitraum × (1 − Verfügbarkeit)`

Bei einem Jahr mit 365 Tagen gilt: `365 × 24 = 8.760 Stunden`.

| Verfügbarkeit | Ausfallzeit pro Jahr | Ausfallzeit pro 30-Tage-Monat |
|---:|---:|---:|
| 90 % | 876 Stunden | 72 Stunden |
| 99 % – 2 Nines | 87,6 Stunden | 7,2 Stunden |
| 99,9 % – 3 Nines | 8,76 Stunden | 43,2 Minuten |
| 99,99 % – 4 Nines | 52,56 Minuten | 4,32 Minuten |
| 99,999 % – 5 Nines | 5,256 Minuten | 25,92 Sekunden |

**Wichtig:** 99,9 % Verfügbarkeit bedeutet maximal 8,76 Stunden Ausfall pro Jahr, wenn rund um die Uhr gemessen wird und geplante Wartung nicht herausgerechnet wird.

```markdown
## 99,5 % und 5 Nines im Vergleich

| Verfügbarkeit | Bezeichnung | Max. Ausfallzeit/Jahr |
|---:|---|---:|
| 99 % | 2 Nines | 87,6 Stunden |
| 99,5 % | 99,5 % Verfügbarkeit | 43,8 Stunden |
| 99,9 % | 3 Nines | 8,76 Stunden |
| 99,99 % | 4 Nines | 52,56 Minuten |
| 99,999 % | 5 Nines | 5,26 Minuten |

„Nines“ bezeichnet die Anzahl der Neunen nach dem Komma bei der Verfügbarkeit, z. B. 99,999 % = 5 Nines. 99,5 % wird normalerweise nicht als 2 Nines bezeichnet.

Die Werte beziehen sich auf 365 Tage und darauf, dass sämtliche Ausfallzeiten in die Messung eingehen. Ein SLA kann stattdessen monatlich messen oder bestimmte Zeiten ausschließen.

## Was gilt als Ausfall?

Ein SLA muss festlegen, wann ein Dienst als nicht verfügbar gilt. Beispiele:

- **Komplettausfall:** Der Dienst ist für Nutzer nicht erreichbar.
- **Teilausfall:** Eine zugesicherte Kernfunktion funktioniert nicht.
- **Leistungsausfall:** Der Dienst unterschreitet vereinbarte Grenzwerte, z. B. bei Antwortzeiten.
- **Geplante Wartung:** Kann vom Ausfallwert ausgenommen sein, wenn das SLA dies ausdrücklich vorsieht.
- **Kundenseitige Störung:** Kann ausgeschlossen sein, wenn beispielsweise das Kundennetzwerk die Ursache ist.

Entscheidend sind die Vertragsdefinition, die Messmethode und der Messzeitraum. Manche SLAs werten beispielsweise nur Ausfälle ab einer bestimmten Mindestdauer.

## Geplante Wartung und höhere Gewalt

| Begriff | Bedeutung | Mögliche Auswirkung auf das SLA |
|---|---|---|
| Geplante Wartung | Vorher angekündigte Updates, Upgrades oder Hardwarearbeiten | Kann aus der Verfügbarkeitsberechnung ausgeschlossen sein |
| Notfallwartung | Ungeplante Arbeiten zur Behebung oder Vermeidung schwerwiegender Probleme | Behandlung hängt vom Vertrag ab |
| Höhere Gewalt (Force Majeure) | Außergewöhnliches Ereignis außerhalb der zumutbaren Kontrolle einer Vertragspartei, z. B. bestimmte Naturkatastrophen | Kann Haftung oder SLA-Pflichten nach den Vertragsbedingungen beeinflussen |
| Drittanbieter-Ausfall | Störung bei einem externen Anbieter, z. B. Telekommunikation oder Hosting | Kann ausgeschlossen sein, muss aber vertraglich geprüft werden |

**Wichtig:** Wartung oder höhere Gewalt sind nicht automatisch immer vom SLA ausgeschlossen. Das hängt von der konkreten Vereinbarung und gegebenenfalls dem anwendbaren Recht ab. Auch bei höherer Gewalt können vertragliche Informations- oder Notfallpflichten bestehen.

Beispiele für echte Vertragsklauseln mit Wartungs- und Force-Majeure-Ausnahmen:
- [Beispiel-SLA mit Verfügbarkeitsdefinition und Ausnahmen](https://assets.applytosupply.digitalmarketplace.service.gov.uk/g-cloud-14/documents/720323/427161438010014-terms-and-conditions-2024-05-04-1031.pdf)
- [Beispiel-SLA mit Wartungsfenstern und Service Credits](https://assets.applytosupply.digitalmarketplace.service.gov.uk/g-cloud-14/documents/715960/793015773126709-terms-and-conditions-2024-04-29-2035.pdf)

## Konsequenzen bei SLA-Verletzung

Wenn der Anbieter die vereinbarte Leistung nicht einhält, können je nach Vertrag folgende Konsequenzen gelten:

- **Service Credits:** Gutschrift oder Rabatt auf zukünftige Rechnungen.
- **Vertragsstrafen:** Nur, wenn wirksam vereinbart und rechtlich anwendbar.
- **Schadensersatz:** Kann bei entsprechenden rechtlichen und vertraglichen Voraussetzungen infrage kommen.
- **Sonderkündigungsrecht:** Kann bei bestimmten oder wiederholten schwerwiegenden Verstößen vorgesehen sein.
- **Eskalation:** Vorgesehener Ablauf über Support, Service Manager oder Management.

Ein SLA-Verstoß führt nicht automatisch zu einer Geldstrafe oder zu Schadensersatz. Der Vertrag regelt, welche Ansprüche bestehen und wie der Kunde diese geltend machen muss.

## IHK-Merksatz

Ein SLA definiert messbare Serviceziele, insbesondere Verfügbarkeit, Reaktionszeiten und Entstörungszeiten. Der Vertrag muss festlegen, wie Ausfälle gemessen werden, welche Wartungszeiten oder außergewöhnlichen Ereignisse ausgeschlossen sind und welche Konsequenzen bei Nichterfüllung gelten. Service Credits, Schadensersatz und Kündigungsrechte sind voneinander zu unterscheiden.
