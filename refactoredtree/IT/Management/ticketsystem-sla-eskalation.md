# Ticketsystem, SLA und Eskalation

## In 30 Sekunden

- Ein **Ticketsystem** (Trouble-Ticket-System, z. B. Jira Service Management, OTRS/Znuny, ServiceNow, GLPI) erfasst jede Anfrage als **Ticket** mit eindeutiger Nummer, Status, Priorität und Verlauf.
- **Support-Level:** **1st Level** (Service Desk, Annahme, Standardlösungen), **2nd Level** (Fachspezialisten), **3rd Level** (Hersteller, Entwickler, externe Dienstleister).
- **Eskalation:** **funktional** (an höheres Support-Level mit mehr Fachwissen) oder **hierarchisch** (an Vorgesetzte, wenn SLA, Kosten oder Kundenbeziehung gefährdet sind).
- **SLA-Zeiten:** **Reaktionszeit** (bis zur qualifizierten Rückmeldung) und **Lösungs-/Wiederherstellungszeit** (bis der Service wieder läuft) – meist je Priorität und innerhalb der **Servicezeit**.
- Kennzahlen (KPI): First Call Resolution Rate, mittlere Lösungszeit (MTTR), SLA-Erfüllungsquote, Ticketvolumen, Kundenzufriedenheit.
- Prüfungsbezug: SLA-Verfügbarkeit berechnen (2025W: 99 % → 87,6 h Ausfall pro Jahr), Netzwerk-Support-Szenarien in GA2.

## Kernwissen

### Inhalt eines Tickets

| Feld | Beispiel |
|---|---|
| Ticketnummer | #2025-04711 |
| Melder, Kontakt, Standort | M. Groß, Vertrieb, Raum 2.14 |
| Kategorie | Netzwerk → WLAN |
| Beschreibung, Fehlermeldung, Screenshot | "kein WLAN seit 9 Uhr, SSID nicht sichtbar" |
| Auswirkung / Dringlichkeit / Priorität | Abteilung / hoch / P2 |
| Status | Neu → In Bearbeitung → Wartet → Gelöst → Geschlossen |
| Bearbeiter, Gruppe | 2nd Level Netzwerk |
| Zeitstempel, SLA-Fristen | Eingang 09:12, Reaktion bis 09:42, Lösung bis 13:12 |
| Lösung, Verknüpfungen | AP neu gestartet; verknüpft mit Problem #P-112 |

### Vorteile eines Ticketsystems

- nichts geht verloren, alle Anfragen sind nachvollziehbar (Revisionssicherheit)
- Priorisierung und automatische Zuweisung an Gruppen
- Überwachung der SLA-Fristen mit **automatischer Eskalation**
- Wissensdatenbank aus gelösten Tickets, Self-Service-Portal
- Auswertungen (Häufungen erkennen → Problem Management)
- Entlastung durch Standardantworten und Workflows

### SLA-Matrix (Beispiel, Servicezeit Mo–Fr 7–18 Uhr)

| Priorität | Beispiel | Reaktionszeit | Lösungszeit |
|---|---|---:|---:|
| P1 kritisch | Server/ERP für alle ausgefallen | 15 min | 4 h |
| P2 hoch | Abteilung ohne WLAN | 30 min | 8 h |
| P3 mittel | einzelner Arbeitsplatz gestört, Workaround vorhanden | 2 h | 2 Arbeitstage |
| P4 niedrig | Schönheitsfehler, Wunsch | 1 Arbeitstag | 5 Arbeitstage |

**Rechenbeispiel Servicezeit:** Ein P3-Ticket geht Freitag um 16:00 ein, Lösungszeit 2 Arbeitstage à 11 Stunden = 22 Servicestunden. Freitag verbleiben 2 h (bis 18 Uhr), Montag 11 h, Dienstag 9 h → Frist **Dienstag 16:00**. Wochenenden zählen nicht, weil sie außerhalb der Servicezeit liegen.

### Verfügbarkeit im SLA

`zulässige Ausfallzeit = Bezugszeitraum × (1 − Verfügbarkeit)`

- 99 % pro Jahr: 8.760 h × 0,01 = **87,6 h** (Prüfung 2025W)
- 99,9 % pro Jahr: 8,76 h
- 99,9 % pro 30-Tage-Monat: 720 h × 0,001 = 0,72 h = **43,2 min**

### Eskalationsstufen

| Art | Auslöser | Beispiel |
|---|---|---|
| **funktional** | Wissen oder Berechtigung im aktuellen Level reicht nicht | 1st Level übergibt Ticket an 2nd Level Datenbank |
| **hierarchisch** | SLA droht zu reißen, hoher Schaden, Kundenbeschwerde, Entscheidung nötig | Teamleitung wird bei 80 % verbrauchter Lösungszeit informiert, IT-Leitung bei Überschreitung |
| **automatisch** | Zeitschwelle im Ticketsystem erreicht | E-Mail an Gruppe bei 50 %, SMS an Bereitschaft bei P1 |

### Kennzahlen

| KPI | Formel / Bedeutung |
|---|---|
| **First Call Resolution (FCR)** | im Erstkontakt gelöste Tickets ÷ alle Tickets |
| **SLA-Erfüllungsquote** | Tickets innerhalb der Frist ÷ alle Tickets |
| **MTTR** (Mean Time to Repair/Restore) | durchschnittliche Zeit bis zur Wiederherstellung |
| **Backlog** | Anzahl offener Tickets |
| **CSAT** | Kundenzufriedenheit aus Umfragen |

Beispiel: 1.200 Tickets im Monat, 780 im Erstkontakt gelöst → FCR = 65 %. 1.140 innerhalb der SLA → Erfüllungsquote 95 %.

## Prüfungsfrage

**Frage:** Ein Kunde hat mit Ihrem Unternehmen ein SLA mit einer Verfügbarkeit von 99,5 % pro Monat (30 Tage, 24/7) vereinbart. Im Juni war der Dienst 3 Stunden und 50 Minuten nicht verfügbar. Prüfen Sie, ob das SLA eingehalten wurde, und erläutern Sie den Unterschied zwischen funktionaler und hierarchischer Eskalation.

**Musterantwort:** Bezugszeitraum 30 × 24 h = 720 h; zulässige Ausfallzeit 720 h × 0,005 = **3,6 h = 3 h 36 min**. Tatsächlicher Ausfall 3 h 50 min > 3 h 36 min → das **SLA wurde nicht eingehalten** (Verfügbarkeit = (720 − 3,83) ÷ 720 ≈ 99,47 %); es greifen die vereinbarten Konsequenzen (z. B. Gutschrift). **Funktionale Eskalation** bedeutet, ein Ticket an eine Stelle mit höherer Fachkompetenz weiterzugeben (1st → 2nd → 3rd Level). **Hierarchische Eskalation** bedeutet, Vorgesetzte bzw. die Leitungsebene einzubinden, weil Fristen, Kosten oder die Kundenbeziehung gefährdet sind und Entscheidungen oder zusätzliche Ressourcen nötig werden.

## Falle

- **Reaktionszeit = Lösungszeit:** Reaktion ist die qualifizierte Rückmeldung, nicht die Behebung.
- **Servicezeit ignorieren:** Fristen laufen nur innerhalb der vereinbarten Servicezeiten.
- **Funktional und hierarchisch vertauscht:** funktional = Fachwissen, hierarchisch = Führungsebene.
- **Ticket schließen ohne Bestätigung:** Erst "gelöst", dann nach Rückmeldung oder Frist "geschlossen".
- **Monat und Jahr bei der Verfügbarkeit mischen:** Bezugszeitraum aus dem SLA nehmen.

## Merksatz

**"Erst erfassen, dann priorisieren, dann eskalieren – und immer gegen die SLA-Uhr."** Funktional = nach oben im Wissen, hierarchisch = nach oben in der Verantwortung.

## Verwandt

- [ITIL: Incident, Problem, Change](itil-incident-problem-change.md) – Prozesse hinter dem Ticket
- [SLA](../FiSi-Administrator/Server/Cloud/sla.md) – Verfügbarkeitsstufen und Vertragsinhalte
- [Zustandsdiagramm](../UML-Code-SQL/UML/zustandsdiagramm.md) – Ticketstatus als Zustandsautomat
- [KI in der IT](ki-in-it-vergleich-mit-klassisch.md) – automatische Ticketklassifizierung
- [Qualitätsmanagement](qualitaetsmanagement.md) – KPIs und kontinuierliche Verbesserung
- [HA – Hochverfügbarkeit](../FiSi-Administrator/Server/ha-hochverfuegbarkeit.md)
- [Lektion 18: Netzwerk-Berechnungen](../analyse-der-exams/L18_Netzwerk_Berechnungen.md) – Verfügbarkeit berechnen
- [Lektion 3: Verfügbarkeit & Ausfallsicherheit](../analyse-der-exams/03_Verfuegbarkeit_Ausfallsicherheit.md)
