# Cloud Computing

## In 30 Sekunden

- Cloud = IT-Ressourcen **on demand über das Netz**, **selbst bereitgestellt**, **elastisch** und **nach Verbrauch abgerechnet** (NIST-Merkmale: On-Demand Self-Service, Broad Network Access, Resource Pooling, Rapid Elasticity, Measured Service).
- Servicemodelle: **IaaS** (Infrastruktur, Kunde verwaltet ab dem OS), **PaaS** (Plattform, Kunde bringt nur App und Daten), **SaaS** (fertige Software).
- Bereitstellung: **Public**, **Private**, **Hybrid**, **Community**.
- Typische Argumente: **CAPEX → OPEX**, Skalierbarkeit, schnelle Bereitstellung, weniger Betriebsaufwand. Dagegen: Datenschutz/DSGVO (AVV, Drittland), Vendor Lock-in, Internet- und Anbieterabhängigkeit, laufende Kosten.
- Geprüft in 5 von 10 Jahrgängen (2021W, 2022W, 2023W, 2024S, 2024W, 2026S).

## Kernwissen

### Servicemodelle (IaaS, PaaS, SaaS)

| Modell | Bedeutung | Anbieter verwaltet | Kunde verwaltet | Beispiele |
|---|---|---|---|---|
| **IaaS** | virtuelle Infrastruktur mieten | Hardware, Netz, Storage, Virtualisierung | Betriebssystem, Middleware, Anwendungen, Daten | AWS EC2, Azure VMs, Google Compute Engine |
| **PaaS** | Entwicklungs-/Laufzeitplattform mieten | zusätzlich OS, Runtime, Middleware, Datenbank-Engine | Anwendung und Daten | Azure App Service, Google App Engine, Heroku, Azure SQL |
| **SaaS** | fertige Software nutzen | alles inkl. Anwendung und Updates | Benutzer, Einstellungen, Daten | Microsoft 365, Google Workspace, Salesforce |

**Je höher die Abstraktion, desto weniger verwaltet der Kunde selbst, und desto weniger kann er anpassen.**

**Zuordnung (2021W):** E-Mail → **SaaS** (Standardanwendung, kein Betrieb). Datenbank → **PaaS** (Anbieter übernimmt Patches, Backup, HA, Kunde verwaltet Schema und Zugriffe). IaaS nur, wenn volle Kontrolle über das OS nötig ist.

### Weitere „as a Service“-Modelle

| Modell | Bedeutung | Beispiel |
|---|---|---|
| **FaaS** (Function) | Code läuft ereignisgesteuert, nur bei Aufruf abgerechnet (Serverless) | AWS Lambda, Azure Functions |
| **CaaS** (Container) | verwaltete Container-Plattform | Amazon ECS, Google Cloud Run, Kubernetes-Dienste |
| **DBaaS** | verwaltete Datenbank | Amazon RDS, Azure SQL Database |
| **STaaS** (Storage) | Objektspeicher | Amazon S3, Azure Blob Storage |
| **BaaS** (Backup) | Datensicherung als Dienst, automatisch offsite | Veeam Cloud Connect, Azure Backup |
| **DRaaS** | Disaster Recovery als Dienst | Replikation in die Cloud |

### Bereitstellungsmodelle

| Modell | Beschreibung | Vorteile | Nachteile |
|---|---|---|---|
| **Public Cloud** | geteilte Infrastruktur für viele Kunden (Multi-Tenancy) | hoch skalierbar, keine Investition, global verfügbar | weniger Kontrolle, Datenschutz, Abhängigkeit |
| **Private Cloud** | exklusiv für eine Organisation (Single-Tenancy), on-premises oder gehostet | volle Kontrolle, Datenhoheit, Compliance | teuer, eigener Betrieb, begrenzte Elastizität |
| **Hybrid Cloud** | Kombination Private + Public | sensible Daten privat, Lastspitzen in die Public Cloud (Cloud Bursting) | komplexer (Netz, Identitäten, Sicherheit) |
| **Community Cloud** | gemeinsam für Organisationen mit gleichen Anforderungen | geteilte Kosten, gemeinsame Compliance | selten, Abstimmungsaufwand |

### Cloud vs. On-Premises (2022W)

| Kriterium | On-Premises | Public Cloud |
|---|---|---|
| Investition | hoch (**CAPEX**) | gering, laufende Kosten (**OPEX**), Pay-per-Use |
| Kontrolle/Datenhoheit | vollständig, keine AVV nötig | beim Anbieter, Standort und DSGVO prüfen |
| Latenz | gering im LAN | abhängig von Internetanbindung und Entfernung |
| Skalierbarkeit | begrenzt, Beschaffung dauert Wochen | in Minuten, elastisch |
| Verfügbarkeit | Eigenverantwortung | georedundante Rechenzentren, SLA |
| Wartung | eigenes Personal | Anbieter (je nach Modell) |
| Abhängigkeit | Hardwarehersteller | Vendor Lock-in, Internetverbindung |

### Datenschutz und Sicherheit

- **Art. 28 DSGVO:** Auftragsverarbeitungsvertrag (AVV), wenn der Anbieter personenbezogene Daten im Auftrag verarbeitet.
- **Drittlandtransfer:** Serverstandort (EU) und Rechtsgrundlage prüfen (Angemessenheitsbeschluss, Standardvertragsklauseln).
- **Art. 32 DSGVO:** technische und organisatorische Maßnahmen, z. B. Verschlüsselung at rest und in transit, MFA.
- **Shared Responsibility Model:** Der Anbieter sichert die Infrastruktur, der Kunde seine Konten, Daten und Konfiguration (bei IaaS auch das OS).
- **Anforderungen an den Provider (2024S):** SLA mit Verfügbarkeit und RTO/RPO, AVV und EU-Standort, Zertifizierungen (ISO 27001, BSI C5), Exit-Strategie und Datenexport, Support-Zeiten.

### Measured Service (2021W)

Abrechnung nach tatsächlicher Nutzung: VM-Laufzeit pro Stunde/Sekunde, belegter Speicher pro GB und Monat, Datenverkehr pro GB, Anzahl Funktionsaufrufe. Es entstehen keine Fixkosten für ungenutzte Kapazität.

## Prüfungsfrage

**Frage:** Ein Unternehmen will seinen Mailserver und seine Datenbank in die Cloud verlagern. Ordnen Sie jeweils ein Servicemodell zu und nennen Sie zwei Aspekte, die vor der Auslagerung zu prüfen sind.

**Musterantwort:** Für E-Mail eignet sich **SaaS** (z. B. Microsoft 365), da es sich um eine standardisierte Anwendung handelt und der Anbieter Betrieb, Updates und Verfügbarkeit übernimmt. Für die Datenbank eignet sich **PaaS** (z. B. Azure SQL), weil der Anbieter Betriebssystem, Datenbank-Engine, Patches und Backups verwaltet und das Unternehmen nur Schema, Daten und Zugriffe pflegt. Vorher zu prüfen sind erstens Datenschutz und Compliance: Ein Auftragsverarbeitungsvertrag nach Art. 28 DSGVO ist nötig, und der Speicherort sollte in der EU liegen. Zweitens die Verfügbarkeit und die Anbindung: Das SLA mit garantierter Verfügbarkeit sowie eine ausreichend schnelle und redundante Internetverbindung sind zu prüfen, weil ohne Internet kein Zugriff möglich ist.

## Falle

- IaaS und PaaS vertauschen: Bei IaaS patcht der **Kunde** das Betriebssystem.
- „Cloud = automatisch günstiger“. Bei dauerhaft hoher Last kann On-Premises günstiger sein.
- „Daten in der Cloud = Verantwortung beim Anbieter“. Die rechtliche Verantwortung bleibt beim Unternehmen.
- Private Cloud mit „eigenes Rechenzentrum ohne Virtualisierung“ verwechseln. Auch die Private Cloud bietet Self-Service und Elastizität.
- CaaS mit „Contact Center as a Service“ verwechseln.

## Merksatz

**„IaaS = Rohbau, PaaS = möbliertes Haus, SaaS = Hotelzimmer.“**

## Verwandt

- [SLA](sla.md) – vertragliche Verfügbarkeit
- [Virtualisierung](../virtualisierung/virtualisierung.md) – technische Basis der Cloud
- [Hypervisor-Typen](../virtualisierung/hypervisor-typen.md)
- [VM-Container-Server](../virtualisierung/vm-container-server.md)
- [Container & Docker](../virtualisierung/container-docker.md) – CaaS
- [Skalierbarkeit-Elastizität](../virtualisierung/skalierbarkeit-elastizitaet.md)
- [Backup-Strategien](../backup/backup-strategien.md) – BaaS, Cloud als Offsite-Kopie
- [Lizenzierung](../lizenzierung.md) – Subscription
- [HA-Hochverfügbarkeit](../ha-hochverfuegbarkeit.md)
- [CAPEX-OPEX](../../../Management/capex-opex.md)
- [TCO](../../../Management/tco.md)
- [DSGVO-TOM](../../../Management/DSGVO/dsgvo-tom.md)
- [Lektion 01 Cloud Computing](../../../analyse-der-exams/01_Cloud_Computing.md)
