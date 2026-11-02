# Cloud Computing

## Cloud-Servicemodelle (IaaS, PaaS, SaaS)

| Modell | Bedeutung | Anbieter verwaltet | Kunde verwaltet | Beispiele |
|---|---|---|---|---|
| **IaaS (Infrastructure as a Service)** | Virtuelle Infrastruktur mieten | Hardware, Netz, Storage, Virtualisierung | Betriebssystem, Middleware, Anwendungen, Daten | AWS EC2, Microsoft Azure VMs, Google Compute Engine |
| **PaaS (Platform as a Service)** | Entwicklungsplattform mieten | Infrastruktur, OS, Runtime, Middleware | Anwendungen & Daten | Heroku, Google App Engine, Microsoft Azure App Service |
| **SaaS (Software as a Service)** | Fertige Software mieten/nutzen | Komplett inkl. Anwendung & Updates | Nur Nutzung, User & Einstellungen | Microsoft 365, Google Workspace, Salesforce, Dropbox |

**Merksatz:** Je höher die Abstraktion, desto weniger verwaltet der Kunde selbst.

## Weitere Cloud-Servicemodelle

| Modell | Bedeutung | Erklärung | Beispiel |
|---|---|---|---|
| **FaaS (Function as a Service)** | Funktionsbasierte Ausführung | Ereignisgesteuert, Code wird nur bei Aufruf ausgeführt, nutzungsabhängig | AWS Lambda, Azure Functions |
| **CaaS (Container as a Service)** | Container-Plattform als Dienst | Verwaltung & Orchestrierung von Containern, Kunde deployt Container-Images | Amazon ECS, Google Cloud Run |
| **DBaaS (Database as a Service)** | Datenbank als Dienst | Anbieter kümmert sich um Patches, Backup, Hochverfügbarkeit | Amazon RDS, Azure SQL Database |
| **STaaS (Storage as a Service)** | Speicher als Dienst | Flexibler Speicher über das Netz (meist Objektspeicher) | Amazon S3, Azure Blob Storage |
| **NaaS (Network as a Service)** | Netzwerkinfrastruktur als Dienst | Virtuelle Netzwerke, Firewalls, VPN etc. als Dienst bezogen | Cloud-VPN, SD-WAN-Dienste |

## Bereitstellungsmodelle

| Modell | Beschreibung | Vor-/Nachteile |
|---|---|---|
| **Public Cloud** | Geteilte Infrastruktur für viele Kunden (Multi-Tenancy) | + Skalierbar, kostengünstig, elastisch / – Weniger Kontrolle, Datenschutz |
| **Private Cloud** | Exklusiv für ein Unternehmen (Single-Tenancy) | + Höchste Kontrolle & Sicherheit / – Teurer, Eigenbetrieb |
| **Hybrid Cloud** | Kombination Public + Private Cloud | + Flexibel, sensible Daten privat, Peaks in Public / – Komplexer |
| **Community Cloud** | Gemeinsames Nutzung für Organisationen mit gleichen Anforderungen | + Geteilte Kosten, gemeinsames Ziel / – Weniger verbreitet |

## Cloud vs. On-Premises

| Kriterium | **On-Premises** | **Cloud** |
|---|---|---|
| **Investition** | Hoch (**CAPEX**) | Vorwiegend (**OPEX**), Pay-per-Use |
| **Verantwortung** | Vollständig | Geteilt nach Servicemodell |
| **Skalierbarkeit/Elastizität** | Eingeschränkt | Sehr hoch & automatisch |
| **Verfügbarkeit** | Eigenverantwortung | Hoch durch Anbieter-Redundanz |
| **Datensouveränität** | Vollständig | Standortabhängig (DSGVO beachten) |

## IHK-Merksätze

- **IaaS = Infrastruktur** | **PaaS = Plattform** | **SaaS = Software**
- **CAPEX → OPEX** = typisches Cloud-Argument
- **FaaS = ereignisgesteuert & verbrauchsabhängig**
- **CaaS ≠ Contact Center as a Service** (häufige Verwechslungsfalle)
- **Multi-Tenancy (Public) vs. Single-Tenancy (Private)**

## Verwandt

- [Virtualisierung](virtualisierung.md)
- [Hypervisor-Typen](hypervisor-typen.md)
- [VM-Container-Server](vm-container-server.md)
- [Skalierbarkeit-Elastizität](skalierbarkeit-elastizitaet.md)
- [CAPEX-OPEX](capex-opex.md)
- [TCO](tco.md)
- [HA-Hochverfügbarkeit](ha-hochverfuegbarkeit.md)


### Vor- und Nachteile Cloud

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

