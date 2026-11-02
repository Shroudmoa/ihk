# Qualitätsmanagement

## In 30 Sekunden

- **Qualität** = Grad, in dem Merkmale eines Produkts oder einer Dienstleistung die **Anforderungen erfüllen** (DIN EN ISO 9000).
- **Qualitätsmanagement (QM)** umfasst Qualitätsplanung, -lenkung, -sicherung und -verbesserung.
- **ISO 9001:** Norm für QM-Systeme, zertifizierbar; Grundsätze u. a. Kundenorientierung, prozessorientierter Ansatz, kontinuierliche Verbesserung, faktengestützte Entscheidungen.
- **PDCA-Zyklus (Deming-Kreis):** **Plan – Do – Check – Act** als Grundlage jeder kontinuierlichen Verbesserung (KVP).
- **Analytische QS** (prüfen: Tests, Reviews, Audits) vs. **konstruktive QS** (Fehler vermeiden: Standards, Richtlinien, Schulungen, Werkzeuge).
- In der IT eng verwandt: **ISO/IEC 20000** (IT-Service-Management), **ISO/IEC 27001** (Informationssicherheit), **ISO/IEC 25010** (Softwarequalität).

## Kernwissen

### PDCA-Zyklus

| Phase | Inhalt | Beispiel: zu viele Druckertickets |
|---|---|---|
| **Plan** | Ist-Zustand analysieren, Ziel und Maßnahme planen | 30 % aller Tickets betreffen Drucker; Ziel: halbieren; Maßnahme: Standardtreiber per GPO verteilen |
| **Do** | Maßnahme im kleinen Rahmen umsetzen (Pilot) | GPO für eine Abteilung aktivieren |
| **Check** | Ergebnis messen und mit dem Ziel vergleichen | Tickets der Pilotabteilung um 60 % gesunken |
| **Act** | bei Erfolg standardisieren, sonst anpassen und neu planen | GPO unternehmensweit ausrollen, Dokumentation aktualisieren |

Danach beginnt der Zyklus erneut – daher **kontinuierlicher Verbesserungsprozess (KVP)**, japanisch Kaizen.

### Qualitätssicherung in IT-Projekten

| konstruktiv (vorbeugend) | analytisch (prüfend) |
|---|---|
| Coding-Richtlinien, Namenskonventionen | Code-Review, Pair Programming |
| Vorlagen für Dokumentation und Konfiguration | Tests (Unit-, Integrations-, System-, Abnahmetest) |
| Schulung des Teams | Audits, Abnahmeprüfung |
| Versionsverwaltung (Git), Automatisierung (CI/CD) | Monitoring, Messung von KPIs |
| Checklisten, Vier-Augen-Prinzip | Kundenzufriedenheitsumfrage |

### Wichtige Begriffe

| Begriff | Bedeutung |
|---|---|
| **Qualitätsplanung** | Qualitätsziele und Merkmale festlegen (z. B. Verfügbarkeit, Antwortzeit) |
| **Qualitätslenkung** | Prozesse steuern, damit die Ziele erreicht werden |
| **Qualitätssicherung** | Vertrauen schaffen, dass Anforderungen erfüllt werden |
| **Audit** | systematische, unabhängige Prüfung (intern oder durch Zertifizierer) |
| **TQM** (Total Quality Management) | Qualität als Aufgabe aller Mitarbeitenden und Prozesse |
| **Verifikation** | Wurde das Produkt richtig gebaut? (gegen Spezifikation) |
| **Validierung** | Wurde das richtige Produkt gebaut? (gegen den Nutzungszweck) |
| **Lessons Learned** | Erfahrungen am Projektende sichern, um künftige Projekte zu verbessern |

### Werkzeuge

- **Ishikawa-Diagramm** (Ursache-Wirkungs-Diagramm, 6 M: Mensch, Maschine, Methode, Material, Milieu, Messung)
- **Pareto-Analyse** (80/20-Regel: wenige Ursachen erzeugen die meisten Fehler)
- **FMEA** (Fehlermöglichkeits- und Einflussanalyse, Risikoprioritätszahl)
- **Checklisten**, **Fehlersammelliste**, **Kennzahlen-Dashboards**

### Kosten der Qualität

Je später ein Fehler entdeckt wird, desto teurer seine Behebung ("Zehnerregel": Kosten steigen pro Phase etwa um den Faktor 10 – Planung → Entwicklung → Test → Betrieb). Darum lohnt sich konstruktive QS früh im Projekt.

## Prüfungsfrage

**Frage:** Nach der Einführung eines neuen Softwareverteilungssystems steigen die Supportanfragen. Erläutern Sie, wie Sie mit dem PDCA-Zyklus die Qualität verbessern, und nennen Sie je eine Maßnahme der konstruktiven und der analytischen Qualitätssicherung.

**Musterantwort:** **Plan:** Tickets auswerten (z. B. Pareto-Analyse) und Ursache ermitteln, etwa fehlerhafte Paketierung einer Anwendung; Ziel festlegen ("Tickets zur Softwareverteilung innerhalb von 4 Wochen um 50 % senken") und Maßnahme planen (Paket korrigieren, Testgruppe einführen). **Do:** korrigiertes Paket zunächst an eine Pilotgruppe verteilen. **Check:** Ticketzahlen der Pilotgruppe mit dem Ziel vergleichen. **Act:** bei Erfolg Verfahren standardisieren (jedes Paket erst an die Testgruppe), sonst Ursachenanalyse wiederholen. **Konstruktive QS:** verbindliche Paketierungs-Checkliste und Schulung der Paketierer. **Analytische QS:** jedes Paket vor der Freigabe in einer Testumgebung prüfen (Abnahmetest mit Testprotokoll).

## Falle

- **Qualität = "möglichst gut":** Qualität heißt Erfüllung der **Anforderungen** – Übererfüllung kostet Geld.
- **Check und Act vertauschen:** Erst messen (Check), dann standardisieren oder korrigieren (Act).
- **QS nur als Test am Ende:** Konstruktive Maßnahmen vermeiden Fehler billiger.
- **ISO 9001 = Produktzertifikat:** Zertifiziert wird das Managementsystem, nicht das einzelne Produkt.
- **Verifikation und Validierung vertauscht.**

## Merksatz

**"Planen, Tun, Prüfen, Handeln – und dann von vorn."** Fehler vermeiden ist billiger als Fehler finden.

## Verwandt

- [Softwaretest: White-Box & Black-Box](../UML-Code-SQL/Pseudocode/softwaretest-white-black-box.md) – analytische QS
- [Vorgehensmodelle](vorgehensmodelle.md) – V-Modell, Retrospektive in Scrum
- [ITIL: Incident, Problem, Change](itil-incident-problem-change.md) – Problem Management, kontinuierliche Verbesserung
- [Ticketsystem, SLA & Eskalation](ticketsystem-sla-eskalation.md) – KPIs
- [Abnahme, Übergabe & Dokumentation](abnahme-uebergabe-dokumentation.md) – Lessons Learned
- [Stakeholder- & Risikoanalyse](stakeholder-risikoanalyse.md) – FMEA
- [Lastenheft & Pflichtenheft](lastenheft-pflichtenheft.md) – Qualitätsanforderungen festlegen
