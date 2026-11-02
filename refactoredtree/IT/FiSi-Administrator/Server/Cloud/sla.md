# SLA – Service Level Agreement

## In 30 Sekunden

- Ein **SLA** ist die vertragliche Vereinbarung über **messbare Servicequalität** zwischen Anbieter und Kunde: Verfügbarkeit, Reaktions- und Entstörungszeiten, Wartungsfenster, Messmethode, Konsequenzen.
- **Ausfallzeit = Bezugszeitraum · (1 − Verfügbarkeit)**. 99,9 % im Jahr = 8,76 h, 99 % = 87,6 h.
- Wichtig ist, **was** als Ausfall zählt und **wie** gemessen wird (Monat oder Jahr, mit oder ohne geplante Wartung).
- Bei Verletzung gibt es meist **Service Credits** (Gutschriften), Schadensersatz oder Kündigung nur, wenn der Vertrag es vorsieht.
- Geprüft: SLA-Verfügbarkeit und Kostenverlust 2026S, SLA als Provider-Anforderung 2024S, SLA 99 % in GA2 2025W.

## Kernwissen

**SLA (Service Level Agreement)** = vertragliche Vereinbarung über die zugesicherte Servicequalität zwischen Dienstleister und Kunde.

Ein SLA legt beispielsweise Verfügbarkeit, Reaktionszeiten, Entstörungszeiten, Wartungsfenster, Messmethoden und mögliche Konsequenzen bei Nichterfüllung fest.

### Verfügbarkeit: die „Nines“

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

### 99,5 % im Vergleich

| Verfügbarkeit | Bezeichnung | Max. Ausfallzeit/Jahr |
|---:|---|---:|
| 99 % | 2 Nines | 87,6 Stunden |
| 99,5 % | 99,5 % Verfügbarkeit | 43,8 Stunden |
| 99,9 % | 3 Nines | 8,76 Stunden |
| 99,99 % | 4 Nines | 52,56 Minuten |
| 99,999 % | 5 Nines | 5,26 Minuten |

„Nines“ bezeichnet die Anzahl der Neunen nach dem Komma bei der Verfügbarkeit, z. B. 99,999 % = 5 Nines. 99,5 % wird normalerweise nicht als 2 Nines bezeichnet.

Die Werte beziehen sich auf 365 Tage und darauf, dass sämtliche Ausfallzeiten in die Messung eingehen. Ein SLA kann stattdessen monatlich messen oder bestimmte Zeiten ausschließen.

### Was gilt als Ausfall?

Ein SLA muss festlegen, wann ein Dienst als nicht verfügbar gilt. Beispiele:

- **Komplettausfall:** Der Dienst ist für Nutzer nicht erreichbar.
- **Teilausfall:** Eine zugesicherte Kernfunktion funktioniert nicht.
- **Leistungsausfall:** Der Dienst unterschreitet vereinbarte Grenzwerte, z. B. bei Antwortzeiten.
- **Geplante Wartung:** Kann vom Ausfallwert ausgenommen sein, wenn das SLA dies ausdrücklich vorsieht.
- **Kundenseitige Störung:** Kann ausgeschlossen sein, wenn beispielsweise das Kundennetzwerk die Ursache ist.

Entscheidend sind die Vertragsdefinition, die Messmethode und der Messzeitraum. Manche SLAs werten beispielsweise nur Ausfälle ab einer bestimmten Mindestdauer.

### Geplante Wartung und höhere Gewalt

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

### Konsequenzen bei SLA-Verletzung

Wenn der Anbieter die vereinbarte Leistung nicht einhält, können je nach Vertrag folgende Konsequenzen gelten:

- **Service Credits:** Gutschrift oder Rabatt auf zukünftige Rechnungen.
- **Vertragsstrafen:** Nur, wenn wirksam vereinbart und rechtlich anwendbar.
- **Schadensersatz:** Kann bei entsprechenden rechtlichen und vertraglichen Voraussetzungen infrage kommen.
- **Sonderkündigungsrecht:** Kann bei bestimmten oder wiederholten schwerwiegenden Verstößen vorgesehen sein.
- **Eskalation:** Vorgesehener Ablauf über Support, Service Manager oder Management.

Ein SLA-Verstoß führt nicht automatisch zu einer Geldstrafe oder zu Schadensersatz. Der Vertrag regelt, welche Ansprüche bestehen und wie der Kunde diese geltend machen muss.

### Rechenweg: Ausfallzeit und Kosten (Muster 2026S)

1. Ausfallanteil: 100 % − 99,9 % = 0,1 % = 0,001
2. Ausfallzeit: 8.760 h · 0,001 = **8,76 h pro Jahr**
3. Kosten bei 500 €/h: 8,76 h · 500 €/h = **4.380 € pro Jahr**, wenn die Ausfallzeit voll ausgeschöpft wird

### Typische SLA-Kennzahlen

| Kennzahl | Bedeutung | Beispiel |
|---|---|---|
| Verfügbarkeit | Anteil der Betriebszeit im Messzeitraum | 99,9 % pro Monat |
| Reaktionszeit | bis sich ein Techniker meldet | 30 min bei Priorität 1 |
| Entstörungszeit (Wiederherstellungszeit) | bis der Dienst wieder läuft | 4 h |
| Servicezeit | wann der Support erreichbar ist | 24/7 oder Mo–Fr 8–18 Uhr |
| RTO/RPO | Wiederanlauf und maximaler Datenverlust | RTO 4 h, RPO 1 h |

### Zusammenfassung

Ein SLA definiert messbare Serviceziele, insbesondere Verfügbarkeit, Reaktionszeiten und Entstörungszeiten. Der Vertrag muss festlegen, wie Ausfälle gemessen werden, welche Wartungszeiten oder außergewöhnlichen Ereignisse ausgeschlossen sind und welche Konsequenzen bei Nichterfüllung gelten. Service Credits, Schadensersatz und Kündigungsrechte sind voneinander zu unterscheiden.

## Prüfungsfrage

**Frage:** Ein Cloud-Anbieter garantiert 99,9 % Verfügbarkeit pro Jahr. Ein Ausfall kostet das Unternehmen 500 € pro Stunde. Berechnen Sie die maximale Ausfallzeit und den möglichen Verlust pro Jahr und nennen Sie zwei weitere Inhalte, die im SLA geregelt sein sollten.

**Musterantwort:** Die zulässige Ausfallzeit beträgt 8.760 h · (1 − 0,999) = 8,76 h pro Jahr. Wird sie ausgeschöpft, entsteht ein Verlust von 8,76 h · 500 €/h = 4.380 € pro Jahr. Weitere Inhalte im SLA sind die Reaktions- und Entstörungszeiten im Störungsfall sowie die Messmethode mit Ausnahmen, etwa ob geplante Wartungsfenster aus der Verfügbarkeit herausgerechnet werden. Außerdem sollte geregelt sein, welche Gutschriften (Service Credits) bei Unterschreitung gelten.

## Falle

- 99,9 % als „fast immer da“ abtun, statt die Ausfallzeit konkret auszurechnen.
- Die Stunden des Jahres falsch ansetzen: 365 · 24 = **8.760 h**.
- Einen Monatswert mit einem Jahreswert vergleichen.
- Glauben, das SLA garantiere Schadensersatz in voller Höhe. Meist gibt es nur gedeckelte Gutschriften.
- Geplante Wartung übersehen, die laut Vertrag nicht als Ausfall zählt.

## Merksatz

**„Jede zusätzliche Neun teilt die Ausfallzeit durch zehn. 99,9 % = 8,76 Stunden im Jahr.“**

## Verwandt

- [Verfügbarkeit & Kennzahlen](../verfuegbarkeit-kennzahlen.md) – MTBF, MTTR, Reihe/Parallel
- [HA-Hochverfügbarkeit](../ha-hochverfuegbarkeit.md)
- [Disaster Recovery & Notfallkonzept](../backup/disaster-recovery-notfallkonzept.md) – RTO/RPO
- [Cloud Computing](cloud-computing.md)
- [Lektion 03 Verfügbarkeit](../../../analyse-der-exams/03_Verfuegbarkeit_Ausfallsicherheit.md)
- [Lektion L18 Verfügbarkeit berechnen](../../../analyse-der-exams/L18_Netzwerk_Berechnungen.md)
