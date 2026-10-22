# Cloud_computing

## Cloud-Computing-Servicemodelle (IHK)

| Modell | Bedeutung | Anbieter verwaltet | Kunde verwaltet | Beispiel |
|---|---|---|---|---|
| IaaS | Infrastructure as a Service | Hardware, Netzwerk, Virtualisierung | Betriebssystem, Anwendungen, Daten | AWS EC2, Azure VMs |
| PaaS | Platform as a Service | Infrastruktur, Betriebssystem, Laufzeitumgebung | Anwendungen, Daten | Google App Engine |
| SaaS | Software as a Service | Gesamte Anwendung und Infrastruktur | Nutzung, Einstellungen, Daten | Microsoft 365 |
| FaaS | Function as a Service | Infrastruktur, Laufzeit, Skalierung | Funktionen, Code | AWS Lambda |
| CaaS | Container as a Service | Container-Plattform, je nach Anbieter Infrastruktur | Container-Images, Anwendungen | Amazon ECS |
| DBaaS | Database as a Service | Datenbankbetrieb, Patches, Infrastruktur | Daten, Abfragen, Zugriffsrechte | Amazon RDS |
| STaaS | Storage as a Service | Speicherinfrastruktur und Verfügbarkeit | Daten, Zugriffsrechte, Aufbewahrung | Amazon S3 |
| NaaS | Network as a Service | Netzwerkdienste und Netzwerkressourcen | Konfiguration, Richtlinien, Nutzung | Cloud-VPN, SD-WAN-Dienste |

### IHK-Prüfungswissen

**Grundprinzip:** Je höher die Abstraktion, desto weniger Infrastruktur muss der Kunde selbst verwalten.

- **IaaS:** Maximale Kontrolle, aber hoher Administrationsaufwand.
- **PaaS:** Fokus auf Entwicklung statt Serververwaltung.
- **SaaS:** Fertige Software über Browser oder Client nutzen.
- **FaaS:** Einzelne Funktionen ereignisgesteuert ausführen; häufig nutzungsabhängige Abrechnung.
- **CaaS:** Container bereitstellen und verwalten; nicht mit Contact Center as a Service verwechseln.
- **DBaaS:** Datenbankadministration teilweise an den Anbieter auslagern.
- **STaaS:** Speicher flexibel bereitstellen; z. B. Objektspeicher oder Dateispeicher.
- **NaaS:** Netzwerkfunktionen als Dienst beziehen.

### Vor- und Nachteile

| Vorteile | Nachteile |
|---|---|
| Skalierbarkeit | Anbieterabhängigkeit (Vendor Lock-in) |
| Geringere Anfangsinvestitionen | Laufende Betriebskosten |
| Schnelle Bereitstellung | Abhängigkeit von Internet und Anbieter |
| Weniger eigener Wartungsaufwand | Datenschutz- und Compliance-Anforderungen |

### DSGVO und IT-Sicherheit

- **DSGVO Art. 28:** Auftragsverarbeitung prüfen, wenn der Cloud-Anbieter personenbezogene Daten im Auftrag verarbeitet.
- **DSGVO Art. 32:** Geeignete technische und organisatorische Maßnahmen (TOM).
- **Shared Responsibility Model:** Anbieter und Kunde teilen sich die Sicherheitsverantwortung.
- **IHK-Merksatz:** Cloud-Nutzung überträgt Verwaltungsaufgaben, aber nicht automatisch die rechtliche Verantwortung.

### Merkhilfe

IaaS = Infrastruktur  
PaaS = Entwicklungsplattform  
SaaS = fertige Software  
FaaS = einzelne Funktionen  
CaaS = Container  
DBaaS = Datenbank  
STaaS = Speicher  
NaaS = Netzwerk


### Vor- und Nachteile der Cloud-Servicemodelle (IHK)

| Modell | Vorteile | Nachteile |
|---|---|---|
| IaaS | Hohe Kontrolle, flexibel skalierbar | Hoher Administrationsaufwand, Betriebssystem muss gewartet werden |
| PaaS | Schnelle Entwicklung, keine eigene Serververwaltung | Eingeschränkte Systemkontrolle, Anbieterabhängigkeit |
| SaaS | Sofort nutzbar, keine lokale Installation/Wartung erforderlich | Wenig Anpassungsmöglichkeiten, Abhängigkeit vom Anbieter |
| FaaS | Automatische Skalierung, Abrechnung nach Ausführung | Cold Starts, Laufzeit- und Ausführungsbeschränkungen |
| CaaS | Portabilität, einfache Skalierung von Containern | Komplexe Orchestrierung, Sicherheitskonfiguration erforderlich |
| DBaaS | Weniger Wartungsaufwand, automatisierte Backups möglich | Laufende Kosten, eingeschränkte Kontrolle über Datenbankkonfiguration |
| STaaS | Flexibler Speicher, hohe Skalierbarkeit | Zugriffskosten, Datenschutz- und Abhängigkeitsrisiken |
| NaaS | Zentrale Verwaltung, flexible Netzwerkressourcen | Anbieterabhängigkeit, Internet-/Providerabhängigkeit |

**IHK-Merksatz:** Cloud-Dienste reduzieren den eigenen Betriebsaufwand, erhöhen aber häufig die Abhängigkeit vom Anbieter und erfordern weiterhin Sicherheits- und Datenschutzmaßnahmen.

## Verwandt

- [VM-Container-Server](vm-container-server.md) – Abstraktionsebenen IaaS bis SaaS
- [Hypervisor-Typen](hypervisor-typen.md) – Basis der Virtualisierung
- [Live-Migration-VMs](live-migration-vms.md) – Migration im Rechenzentrum
- [Snapshots-Backups](snapshots-backups.md) – Datensicherung in der Cloud

