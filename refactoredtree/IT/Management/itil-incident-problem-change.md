# ITIL: Incident, Problem, Change

## In 30 Sekunden

- **ITIL** (IT Infrastructure Library, aktuell ITIL 4) ist ein Best-Practice-Rahmenwerk für **IT-Service-Management (ITSM)**. Die Norm dazu ist **ISO/IEC 20000**.
- **Incident Management:** gestörten Service **so schnell wie möglich wiederherstellen** (Workaround erlaubt).
- **Problem Management:** die **Ursache** wiederkehrender oder schwerer Störungen finden und dauerhaft beseitigen (Known Error, Root Cause Analysis).
- **Change Management (Change Enablement):** Änderungen an der IT **kontrolliert** bewerten, genehmigen, durchführen und dokumentieren (Standard-, Normal-, Notfall-Change; CAB).
- **Service Request:** keine Störung, sondern eine Standardanforderung (Passwort-Reset, neuer Zugang, Software).
- **Service Desk** = zentrale Anlaufstelle (Single Point of Contact, SPOC) für Anwender.

## Kernwissen

### Abgrenzung

| Begriff | Definition | Beispiel | Ziel |
|---|---|---|---|
| **Incident** | ungeplante Unterbrechung oder Qualitätsminderung eines Service | Drucker druckt nicht, VPN bricht ab | schnelle Wiederherstellung |
| **Service Request** | Standardanfrage eines Anwenders | neues Postfach, Softwareinstallation | Erfüllung nach Katalog |
| **Problem** | (unbekannte) Ursache eines oder mehrerer Incidents | VPN bricht täglich um 12 Uhr ab, weil die Lease-Zeit falsch ist | Ursache beseitigen |
| **Known Error** | Problem mit bekannter Ursache und dokumentiertem Workaround | "Treiber X verursacht Absturz, Version Y verwenden" | Wissen bereitstellen |
| **Change** | Hinzufügen, Ändern oder Entfernen von etwas mit Auswirkung auf Services | Firmware-Update, neue Firewall-Regel | Risiko kontrollieren |

### Incident-Management-Prozess

1. **Erfassen** (Ticket über Telefon, Mail, Portal, Monitoring)
2. **Kategorisieren** (Hardware, Netzwerk, Software …)
3. **Priorisieren** – **Priorität = Auswirkung (Impact) × Dringlichkeit (Urgency)**
4. **Erstdiagnose** im 1st Level, Lösung aus der Wissensdatenbank
5. **Eskalieren** (funktional an 2nd/3rd Level, hierarchisch an Führungskräfte)
6. **Lösen und Wiederherstellen**
7. **Schließen** nach Bestätigung durch den Anwender, Dokumentation

### Prioritätsmatrix

| Dringlichkeit ↓ / Auswirkung → | hoch (ganzes Unternehmen) | mittel (Abteilung) | gering (ein Nutzer) |
|---|---|---|---|
| **hoch** (Arbeit unmöglich) | P1 kritisch | P2 hoch | P3 mittel |
| **mittel** (eingeschränkt) | P2 hoch | P3 mittel | P4 niedrig |
| **gering** (Workaround vorhanden) | P3 mittel | P4 niedrig | P5 Planung |

### Problem Management

- **reaktiv:** nach wiederholten oder schweren Incidents
- **proaktiv:** Trends im Monitoring und in Tickets auswerten, bevor Störungen auftreten
- Methoden der Ursachenanalyse: **5-Why-Methode**, **Ishikawa-Diagramm** (Fischgräten: Mensch, Maschine, Methode, Material, Milieu, Messung), Kepner-Tregoe
- Ergebnis: Known Error in der **Known Error Database (KEDB)**, dauerhafte Lösung über einen **Change**

### Change Management

| Change-Typ | Merkmal | Genehmigung |
|---|---|---|
| **Standard-Change** | häufig, risikoarm, vorab genehmigt | keine Einzelgenehmigung (z. B. neuer Benutzer, RAM-Erweiterung nach Vorlage) |
| **Normal-Change** | geplant, Risiko zu bewerten | Change Manager bzw. **CAB** (Change Advisory Board) |
| **Emergency-Change** | dringend, z. B. kritischer Sicherheitspatch | **ECAB** (Emergency CAB), Dokumentation ggf. nachträglich |

Ablauf eines Normal-Change: **Request for Change (RfC)** → Bewertung (Nutzen, Risiko, Ressourcen, Rückfallplan) → Genehmigung → Planung im **Wartungsfenster** → Test (Testumgebung) → Umsetzung → **Post Implementation Review** → Abschluss. Ein **Rollback-Plan** ist Pflicht.

### Weitere ITIL-Praktiken (Auswahl)

Service Level Management (SLA), Configuration Management (**CMDB** mit Configuration Items), Release/Deployment Management, Knowledge Management, Availability und Capacity Management, Information Security Management.

## Prüfungsfrage

**Frage:** Seit drei Wochen melden Anwender jeden Montagmorgen, dass das ERP-System nicht erreichbar ist. Der Service Desk startet den Applikationsserver jeweils neu, danach funktioniert alles. Ordnen Sie die Vorgänge den ITIL-Prozessen zu und beschreiben Sie das weitere Vorgehen.

**Musterantwort:** Jede Montagsmeldung ist ein **Incident**; der Neustart ist ein **Workaround**, der den Service schnell wiederherstellt (Ziel des Incident Managements). Weil die Störung wiederholt auftritt, wird ein **Problem** eröffnet. Im **Problem Management** wird die Ursache analysiert, z. B. mit Logauswertung und 5-Why-Methode (etwa: Wochenend-Backup sperrt die Datenbank zu lange, Speicher läuft voll). Bis zur Lösung wird ein **Known Error** mit Workaround dokumentiert. Die dauerhafte Lösung (z. B. Backup-Zeitfenster verschieben, Speicher erweitern) wird als **Change** beantragt, bewertet, genehmigt, im Wartungsfenster umgesetzt und überprüft.

## Falle

- **Incident und Problem verwechseln:** Incident = Symptom beheben, Problem = Ursache beseitigen.
- **Service Request als Incident:** "Ich brauche ein neues Headset" ist keine Störung.
- **Priorität nur nach Lautstärke des Anrufers:** Priorität ergibt sich aus Auswirkung und Dringlichkeit.
- **Änderung ohne Change:** Auch eine "kleine" Firewall-Regel kann Services lahmlegen – ohne Rollback-Plan riskant.
- **Emergency-Change ohne Dokumentation:** Die Dokumentation darf später erfolgen, aber nicht entfallen.

## Merksatz

**"Incident löscht das Feuer, Problem sucht den Brandstifter, Change baut den Rauchmelder ein."**

## Verwandt

- [Ticketsystem, SLA & Eskalation](ticketsystem-sla-eskalation.md) – Werkzeug und Kennzahlen
- [Qualitätsmanagement](qualitaetsmanagement.md) – PDCA und kontinuierliche Verbesserung
- [Stakeholder- & Risikoanalyse](stakeholder-risikoanalyse.md) – Risikobewertung im Change
- [Patchmanagement TU/SU/PU](../FiSi-Administrator/Netzwerk-Administration/patchmanagement-tu-su-pu.md) – Changes über Test- und Staging-Umgebung
- [SLA](../FiSi-Administrator/Server/Cloud/sla.md) – Service Level Management
- [Netzwerkanalyse & Troubleshooting](../FiSi-Administrator/Netzwerk-Administration/netzwerkanalyse-und-troubleshooting.md) – Diagnose im 2nd Level
- [BPMN](../UML-Code-SQL/UML/BPMN.md) – ITSM-Prozesse modellieren
