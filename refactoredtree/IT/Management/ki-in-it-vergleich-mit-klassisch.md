# KI in der IT – Vergleich mit klassischer Automatisierung

## In 30 Sekunden

- **Klassische Automatisierung** (Skripte, Regeln, Workflows) arbeitet **regelbasiert und deterministisch**: gleiche Eingabe → gleiche Ausgabe, jede Entscheidung ist nachvollziehbar.
- **KI / Machine Learning** arbeitet **datenbasiert und probabilistisch**: Das Modell lernt Muster aus Trainingsdaten und liefert Wahrscheinlichkeiten oder generierte Inhalte.
- **Generative KI / Large Language Models (LLM)** erzeugen Text, Code oder Bilder, können aber **halluzinieren** (plausible, falsche Aussagen).
- Einsatz im IT-Betrieb: Ticketklassifizierung, Anomalieerkennung im SIEM, Log-Analyse, Prognose von Kapazitäten (AIOps), Chatbots im Service Desk, Codeassistenz.
- Risiken: Halluzinationen, schlechte Datenqualität, Datenschutz (DSGVO), fehlende Erklärbarkeit, Prompt Injection, Abhängigkeit vom Anbieter.
- Rechtsrahmen: **DSGVO** und **EU-KI-Verordnung (AI Act)** mit risikobasiertem Ansatz.

## Kernwissen

### Vergleich

| Kriterium | Klassische Automatisierung | KI / Machine Learning |
|---|---|---|
| Grundlage | explizit programmierte Regeln (`if CPU > 90 then Alarm`) | aus Daten gelernte Muster |
| Ergebnis | deterministisch, reproduzierbar | probabilistisch, mit Konfidenz |
| Nachvollziehbarkeit | hoch (Regel lesbar) | gering bis mittel ("Black Box") |
| Umgang mit Neuem | nur vorhergesehene Fälle | erkennt auch unbekannte Muster |
| Aufwand | Regeln schreiben und pflegen | Daten sammeln, aufbereiten, trainieren, überwachen |
| Fehlerbild | Regel greift nicht oder falsch | falsch-positive/negative Treffer, Halluzination |
| typisches Werkzeug | PowerShell, Bash, Ansible, Cron/Task-Scheduler | ML-Modell, LLM, Anomalieerkennung |
| geeignet für | klar definierte, wiederkehrende Abläufe | unscharfe Muster, große Datenmengen, Sprache |

### Begriffe

| Begriff | Bedeutung |
|---|---|
| **Machine Learning** | Algorithmen lernen aus Beispieldaten |
| **überwachtes Lernen** | Trainingsdaten mit bekannten Ergebnissen (z. B. Tickets mit Kategorie) |
| **unüberwachtes Lernen** | Muster ohne Vorgaben finden (z. B. Cluster ungewöhnlicher Logins) |
| **Deep Learning** | ML mit vielschichtigen neuronalen Netzen |
| **LLM** | großes Sprachmodell, sagt das nächste Token voraus |
| **Halluzination** | überzeugend formulierte, aber falsche Ausgabe |
| **Prompt Injection** | manipulierte Eingabe bringt das Modell dazu, Vorgaben zu umgehen |
| **Human in the Loop** | ein Mensch prüft und gibt KI-Ergebnisse frei |
| **RAG** (Retrieval Augmented Generation) | LLM bekommt Inhalte aus eigener Wissensdatenbank, um Antworten zu belegen |

### Einsatzbeispiele im IT-Betrieb

| Aufgabe | klassisch | mit KI |
|---|---|---|
| Ticket-Routing | Schlüsselwörter im Betreff | Klassifizierung nach Inhalt, Priorisierungsvorschlag |
| Monitoring | feste Schwellenwerte | Anomalieerkennung auf Basis des Normalverhaltens |
| Security (SIEM) | Signaturen, Korrelationsregeln | Verhaltensanalyse (UEBA), Erkennung unbekannter Angriffe |
| Service Desk | FAQ-Seite | Chatbot mit Zugriff auf die Wissensdatenbank |
| Skripterstellung | manuell | Codeassistent erzeugt Entwurf, Admin prüft |

### Risiken und Maßnahmen

| Risiko | Maßnahme |
|---|---|
| Halluzination, falsche Konfiguration | Human in the Loop, Ergebnisse testen, Quellen verlangen |
| Datenschutz (personenbezogene Daten im Prompt) | keine sensiblen Daten in öffentliche Dienste, AV-Vertrag, Datenstandort EU, Pseudonymisierung |
| Geheimnisschutz | Unternehmensrichtlinie, freigegebene Werkzeuge, kein Upload von Quellcode/Passwörtern |
| Bias / Diskriminierung | Trainingsdaten prüfen, Ergebnisse stichprobenartig kontrollieren |
| fehlende Erklärbarkeit | dokumentieren, wofür die KI eingesetzt wird; Entscheidungen begründbar halten |
| Prompt Injection | Eingaben filtern, Rechte des KI-Agenten minimieren (Least Privilege) |
| Abhängigkeit vom Anbieter | Exit-Strategie, austauschbare Schnittstellen |

### EU-KI-Verordnung (AI Act) – Risikoklassen

| Klasse | Beispiel | Folge |
|---|---|---|
| unannehmbares Risiko | Social Scoring durch Behörden | verboten |
| hohes Risiko | KI zur Bewerberauswahl, kritische Infrastruktur | strenge Pflichten (Risikomanagement, Dokumentation, menschliche Aufsicht) |
| begrenztes Risiko | Chatbots, generierte Inhalte | Transparenzpflicht (Nutzer muss wissen, dass er mit KI interagiert) |
| minimales Risiko | Spamfilter | keine besonderen Pflichten |

Bei KI-Systemen zur Leistungs- oder Verhaltenskontrolle von Beschäftigten hat der Betriebsrat ein Mitbestimmungsrecht (§ 87 Abs. 1 Nr. 6 BetrVG).

## Prüfungsfrage

**Frage:** Der Service Desk möchte eingehende Tickets automatisch einer Kategorie und Priorität zuordnen. Vergleichen Sie eine regelbasierte Lösung mit einer KI-gestützten Lösung und nennen Sie zwei Risiken des KI-Einsatzes mit je einer Gegenmaßnahme.

**Musterantwort:** Eine **regelbasierte Lösung** ordnet Tickets anhand fester Regeln zu, z. B. "Betreff enthält 'Drucker' → Kategorie Hardware". Sie ist transparent, reproduzierbar und schnell umgesetzt, erkennt aber nur vorhergesehene Formulierungen und muss ständig gepflegt werden. Eine **KI-gestützte Lösung** lernt aus bereits klassifizierten Tickets und erkennt die Kategorie auch bei freien Formulierungen; sie arbeitet jedoch mit Wahrscheinlichkeiten, ihre Entscheidungen sind schwer nachvollziehbar und sie braucht gute Trainingsdaten. **Risiken:** (1) **Fehlklassifizierung** kritischer Tickets – Gegenmaßnahme: Konfidenzschwelle, unterhalb derer ein Mensch entscheidet, und Stichprobenkontrollen. (2) **Datenschutz**, weil Tickets personenbezogene Daten enthalten – Gegenmaßnahme: Verarbeitung in der EU mit Auftragsverarbeitungsvertrag oder lokal betriebenem Modell, Datenminimierung.

## Falle

- **"KI ist immer besser":** Für klar definierte Abläufe ist ein Skript billiger, schneller und nachvollziehbarer.
- **KI-Ausgabe ungeprüft übernehmen:** Generierte Befehle oder Konfigurationen können falsch oder gefährlich sein.
- **Deterministisch und probabilistisch verwechseln:** Regeln sind deterministisch, ML-Modelle probabilistisch.
- **Datenschutz vergessen:** Prompts an Cloud-KI sind eine Datenübermittlung an Dritte.
- **Halluzination als Programmfehler bezeichnen:** Sie ist eine Eigenschaft generativer Modelle, kein klassischer Bug.

## Merksatz

**"Regeln, wenn man den Fall kennt – KI, wenn man das Muster sucht – Mensch, wenn es darauf ankommt."**

## Verwandt

- [Ticketsystem, SLA & Eskalation](ticketsystem-sla-eskalation.md) – KI zur Ticketklassifizierung
- [ITIL: Incident, Problem, Change](itil-incident-problem-change.md)
- [Stakeholder- & Risikoanalyse](stakeholder-risikoanalyse.md) – KI-Risiken bewerten
- [DSGVO & TOM](DSGVO/dsgvo-tom.md) – Datenschutz beim KI-Einsatz
- [AAA & SIEM](../FiSi-Administrator/Sicherheit/aaa-siem.md) – Anomalieerkennung
- [Netzwerkanalyse & Troubleshooting](../FiSi-Administrator/Netzwerk-Administration/netzwerkanalyse-und-troubleshooting.md)
- [Betriebsrat & JAV](../../Wirtschaft/betriebsrat-jav.md) – Mitbestimmung bei technischen Überwachungseinrichtungen
