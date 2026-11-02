# Lastenheft und Pflichtenheft

## In 30 Sekunden

- **Lastenheft** = **WAS** und **WOFÜR**: Anforderungen des **Auftraggebers** an das Ergebnis, aus Anwendersicht.
- **Pflichtenheft** = **WIE** und **WOMIT**: Umsetzungskonzept des **Auftragnehmers**, beschreibt, wie die Anforderungen erfüllt werden.
- Reihenfolge: Lastenheft → Angebot(e) → Pflichtenheft → Vertrag → Umsetzung → **Abnahme gegen das Pflichtenheft**.
- **Funktionale Anforderungen** (was das System tut) und **nicht-funktionale Anforderungen** (wie gut: Leistung, Sicherheit, Verfügbarkeit, Ergonomie).
- Anforderungen müssen **eindeutig, vollständig, prüfbar und widerspruchsfrei** sein.
- Normbezug: DIN 69901-5, VDI 3694; Qualitätsmerkmale für Software nach ISO/IEC 25010.

## Kernwissen

### Gegenüberstellung

| Kriterium | Lastenheft | Pflichtenheft |
|---|---|---|
| erstellt von | Auftraggeber (Kunde, Fachabteilung) | Auftragnehmer (IT-Dienstleister, IT-Abteilung) |
| Leitfrage | Was soll erreicht werden? | Wie wird es umgesetzt? |
| Sicht | Anwender, fachlich | Technik, lösungsorientiert |
| Zeitpunkt | vor der Ausschreibung | nach Auftragserteilung bzw. als Teil des Angebots |
| Detailgrad | grob bis mittel | detailliert, technisch |
| Zweck | Grundlage für Angebote und Vergleich | Grundlage für Umsetzung, Vertrag und Abnahme |

### Typische Gliederung

| Lastenheft | Pflichtenheft |
|---|---|
| 1. Ausgangssituation (Ist-Zustand) | 1. Zielbestimmung (Muss-, Wunsch-, Abgrenzungskriterien) |
| 2. Zielsetzung | 2. Produkteinsatz (Anwendungsbereich, Zielgruppen) |
| 3. funktionale Anforderungen | 3. Produktumgebung (Hardware, Software, Schnittstellen) |
| 4. nicht-funktionale Anforderungen | 4. Funktionen im Detail (z. B. Use Cases) |
| 5. Rahmenbedingungen (Budget, Termine, Normen, Datenschutz) | 5. Daten und Leistungsdaten |
| 6. Abnahmekriterien | 6. Qualitätsanforderungen, Testfälle |
| 7. Mengengerüst (Nutzer, Standorte, Datenvolumen) | 7. Projektplan, Lieferumfang, Abnahmeverfahren |

### Funktionale vs. nicht-funktionale Anforderungen

| Funktional (Was?) | Nicht-funktional (Wie gut?) |
|---|---|
| Benutzer können Tickets per Webportal anlegen | Antwortzeit des Portals < 2 Sekunden |
| Das System sendet bei Statusänderung eine E-Mail | Verfügbarkeit 99,5 % während der Geschäftszeiten |
| Administratoren können Kategorien pflegen | Anmeldung per Single Sign-on über Active Directory |
| Auswertung offener Tickets pro Woche | Datenhaltung in einem Rechenzentrum in der EU (DSGVO) |
| Export der Tickets als CSV | barrierefreie Bedienung nach BITV 2.0 |

### Qualitätsmerkmale nach ISO/IEC 25010 (Auswahl)

Funktionale Eignung, Leistungseffizienz, Kompatibilität, Benutzbarkeit, Zuverlässigkeit, Sicherheit, Wartbarkeit, Übertragbarkeit.

### Gute und schlechte Anforderungen

| schlecht | besser |
|---|---|
| "Das WLAN soll schnell sein." | "In allen Büros werden mindestens 100 Mbit/s pro Client bei 30 gleichzeitigen Clients erreicht." |
| "Das System soll sicher sein." | "Zugriff nur mit MFA; Passwörter mindestens 12 Zeichen; Protokollierung aller Admin-Zugriffe für 90 Tage." |
| "Möglichst bald fertig." | "Go-Live am 30.06., Abnahme bis 07.07." |

### Änderungen (Change Requests)

Ändert der Auftraggeber nach Freigabe die Anforderungen, wird ein **Change Request** gestellt: Auswirkungen auf Kosten, Zeit und Qualität bewerten, freigeben oder ablehnen, Pflichtenheft und Plan anpassen. So bleibt das magische Dreieck unter Kontrolle ("Scope Creep" vermeiden).

## Prüfungsfrage

**Frage:** Ein Kunde beauftragt Ihr Unternehmen mit der Einrichtung eines WLANs für sein Hotel. Unterscheiden Sie Lastenheft und Pflichtenheft und ordnen Sie folgende Aussagen zu: (a) "Gäste sollen in allen Zimmern kostenloses Internet nutzen können." (b) "Es werden 24 Access Points vom Typ X mit WPA3 und einem eigenen Gast-VLAN 40 installiert."

**Musterantwort:** Das **Lastenheft** wird vom Auftraggeber (Hotel) erstellt und beschreibt aus seiner Sicht, **was** die Lösung leisten soll. Das **Pflichtenheft** erstellt der Auftragnehmer (IT-Dienstleister); es beschreibt, **wie** und **womit** die Anforderungen umgesetzt werden, und ist Grundlage für die Abnahme. Aussage (a) ist eine fachliche Anforderung und gehört ins **Lastenheft**. Aussage (b) beschreibt die technische Umsetzung und gehört ins **Pflichtenheft**.

## Falle

- **Ersteller vertauscht:** Lasten**heft** = Auftraggeber ("Ich habe die Last"), Pflichten**heft** = Auftragnehmer ("Ich habe die Pflicht").
- **Technik im Lastenheft:** Produktnamen und Konfigurationen schränken den Angebotsvergleich unnötig ein.
- **Nicht messbare Anforderungen:** Ohne Messgröße ist keine Abnahme möglich.
- **Abnahme gegen das Lastenheft:** Vertraglich maßgeblich ist das (freigegebene) Pflichtenheft.
- **Nicht-funktionale Anforderungen vergessen:** Datenschutz, Verfügbarkeit und Performance sind oft der Streitpunkt bei der Abnahme.

## Merksatz

**"Lastenheft = WAS (Kunde), Pflichtenheft = WIE (Dienstleister)."** Anforderungen nur so schreiben, dass man sie prüfen kann.

## Verwandt

- [Projektmanagement-Grundlagen](projektmanagement-grundlagen.md) – SMART-Ziele, Phasen
- [Nutzwertanalyse](nutzwertanalyse.md) – Angebote anhand der Anforderungen vergleichen
- [Abnahme, Übergabe & Dokumentation](abnahme-uebergabe-dokumentation.md) – Abnahme gegen das Pflichtenheft
- [Vorgehensmodelle](vorgehensmodelle.md) – Anforderungen im Wasserfall vs. Product Backlog in Scrum
- [Anwendungsfalldiagramm](../UML-Code-SQL/UML/Anwendungsfalldiagramm-Use-Case-Diagramm.md) – Funktionen grafisch erfassen
- [Softwaretest: White-Box & Black-Box](../UML-Code-SQL/Pseudocode/softwaretest-white-black-box.md) – Abnahmetest
- [SLA](../FiSi-Administrator/Server/Cloud/sla.md) – nicht-funktionale Anforderungen im Betrieb
