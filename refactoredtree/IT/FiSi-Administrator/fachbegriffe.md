# Fachbegriffe

# IT-Slang und Fachbegriffe – IHK AP2

Begriffe, mit denen du technische Probleme, Softwarearchitektur und IT-Prozesse präzise beschreiben kannst. Die Definitionen orientieren sich an etablierten technischen Quellen; einige Begriffe werden in Teams unterschiedlich verwendet.

## Softwareentwicklung

| Begriff | Bedeutung | Kurz erklärt |
|---|---|---|
| **Technical Debt** | Technische Schulden | Kurzfristige technische Kompromisse erhöhen den späteren Wartungs- und Änderungsaufwand. |
| **Refactoring** | Code umstrukturieren | Verbessert die interne Codequalität, ohne das beobachtbare Verhalten zu verändern. |
| **Code Smell** | Hinweis auf ein Codeproblem | Auffälligkeit im Code, die auf ein tieferes Designproblem hindeuten kann. Nicht jeder Code Smell ist automatisch ein Fehler. |
| **Legacy System** | Bestandssystem | Bestehendes System, das weiter genutzt wird, obwohl Technologie oder Wartbarkeit problematisch sein können. |
| **Workaround** | Behelfslösung | Umgeht ein Problem, ohne zwingend dessen Ursache zu beheben. |
| **Hotfix** | Dringende Fehlerkorrektur | Schnelle Korrektur eines kritischen Fehlers, häufig direkt für ein Produktivsystem. |
| **Patch** | Softwarekorrektur | Änderung, die beispielsweise Fehler oder Sicherheitslücken behebt. |
| **Regression** | Rückschritt durch Änderung | Eine Änderung verursacht einen Fehler in bereits funktionierender Funktionalität. |
| **Breaking Change** | Inkompatible Änderung | Eine Änderung bricht bestehende Schnittstellen oder die Kompatibilität mit bisherigen Nutzern. |
| **Backward Compatibility** | Rückwärtskompatibilität | Eine neue Version unterstützt weiterhin relevante ältere Schnittstellen oder Formate. |
| **Refactoring vs. Rewrite** | Umstrukturierung vs. Neuentwicklung | Refactoring verbessert bestehenden Code schrittweise; ein Rewrite ersetzt ihn weitgehend neu. |
| **Code Duplication** | Doppelte Codeabschnitte | Gleiche Logik an mehreren Stellen erhöht Änderungsaufwand und Inkonsistenzrisiko. |
| **Hardcoding** | Fest codierte Werte | Werte wie IP-Adressen oder Zugangsdaten stehen direkt im Code statt in geeigneter Konfiguration. |
| **Dependency** | Abhängigkeit | Eine Komponente benötigt eine andere Bibliothek, einen Dienst oder ein System. |
| **Dependency Hell** | Abhängigkeitskonflikte | Bibliotheken haben unvereinbare Versionen oder widersprüchliche Anforderungen. |
| **Vendor Lock-in** | Herstellerbindung | Ein Wechsel wird durch proprietäre Technologien, Datenformate oder hohe Migrationskosten erschwert. |

Quellen: [Martin Fowler – Technical Debt](https://martinfowler.com/bliki/TechnicalDebt.html), [Martin Fowler – Code Smell](https://martinfowler.com/bliki/CodeSmell.html).

## Architektur und Systemdesign

| Begriff | Bedeutung | Kurz erklärt |
|---|---|---|
| **Single Point of Failure (SPOF)** | Einzelne Ausfallstelle | Der Ausfall einer einzelnen Komponente kann den gesamten Dienst beeinträchtigen. |
| **Bottleneck** | Engpass | Eine begrenzte Ressource limitiert die Gesamtleistung, z. B. CPU, RAM, Storage oder Netzwerk. |
| **Scalability** | Skalierbarkeit | Fähigkeit, steigende Last durch zusätzliche Ressourcen oder bessere Ressourcennutzung zu bewältigen. |
| **Vertical Scaling (Scale up)** | Vertikale Skalierung | Einen Server leistungsfähiger machen, z. B. mehr RAM oder CPU-Kerne. |
| **Horizontal Scaling (Scale out)** | Horizontale Skalierung | Weitere Server oder Instanzen hinzufügen und die Last verteilen. |
| **High Availability (HA)** | Hohe Verfügbarkeit | Durch Redundanz und Failover werden Ausfallzeiten reduziert. |
| **Fault Tolerance** | Fehlertoleranz | Das System kann trotz bestimmter Fehler weiterarbeiten. |
| **Redundancy** | Redundanz | Kritische Komponenten sind mehrfach vorhanden, damit ein Ausfall abgefangen werden kann. |
| **Failover** | Umschaltung bei Ausfall | Ein Ersatzsystem übernimmt nach einem Fehler, automatisch oder manuell. |
| **Graceful Degradation** | Kontrollierter Funktionsverlust | Bei Teilausfällen bleiben Kernfunktionen verfügbar, während optionale Funktionen eingeschränkt werden. |
| **Loose Coupling** | Lose Kopplung | Komponenten hängen möglichst wenig von internen Details anderer Komponenten ab. |
| **Tight Coupling** | Enge Kopplung | Änderungen oder Ausfälle einer Komponente wirken sich stark auf andere Komponenten aus. |
| **Single Responsibility** | Ein Verantwortungsbereich | Eine Komponente soll einen klar abgegrenzten Verantwortungsbereich besitzen. |
| **Separation of Concerns** | Trennung von Zuständigkeiten | Unterschiedliche Aufgaben werden in getrennten Komponenten oder Schichten umgesetzt. |
| **Monolith** | Monolithische Anwendung | Eine Anwendung wird als zusammenhängende Einheit entwickelt und bereitgestellt. |
| **Microservices** | Kleine, eigenständig betreibbare Dienste | Fachliche Funktionen werden auf getrennte Dienste verteilt, die über definierte Schnittstellen kommunizieren. |
| **Distributed System** | Verteiltes System | Mehrere vernetzte Rechner oder Dienste arbeiten zusammen; Netzwerkfehler und Latenzen müssen berücksichtigt werden. |
| **Blast Radius** | Auswirkungsbereich eines Fehlers | Beschreibt, wie viele Systeme oder Nutzer von einem Fehler betroffen sein können. |
| **Single Tenant** | Einzelmandantenbetrieb | Eine Instanz oder Umgebung wird exklusiv für einen Mandanten betrieben. |
| **Multi-Tenant** | Mehrmandantenbetrieb | Mehrere Mandanten nutzen dieselbe Anwendung oder Infrastruktur mit logischer Trennung ihrer Daten und Rechte. |

Quelle: [AWS Well-Architected – Ausfälle in verteilten Systemen begrenzen](https://wa.aws.amazon.com/wellarchitected/2020-07-02T19-33-23/wat.question.REL_5.en.html).

```markdown
## Betrieb und DevOps

| Begriff | Bedeutung | Kurz erklärt |
|---|---|---|
| **DevOps** | Development + Operations | Entwicklung und IT-Betrieb arbeiten eng zusammen und automatisieren Abläufe. |
| **SRE** | Site Reliability Engineering | Wendet Software-Engineering auf Betrieb, Verfügbarkeit und Zuverlässigkeit an. |
| **Toil** | Wiederkehrende Betriebsarbeit | Manuelle, repetitive und automatisierbare Aufgaben, die mit dem System mitwachsen. |
| **Observability** | Beobachtbarkeit | Aus Logs, Metriken und Traces ableiten, was in einem System passiert. |
| **Monitoring** | Systemüberwachung | Erfasst Zustände und Messwerte; Alarme melden relevante Probleme. |
| **Telemetry** | Telemetriedaten | Automatisch erfasste Daten über Zustand und Verhalten eines Systems. |
| **Alert Fatigue** | Alarmmüdigkeit | Zu viele irrelevante Alarme führen dazu, dass wichtige Warnungen übersehen werden. |
| **Incident** | Betriebsstörung | Ereignis, das einen Dienst oder dessen Qualität beeinträchtigt. |
| **Root Cause** | Grundursache | Ursache, deren Behebung das zugrunde liegende Problem adressiert. |
| **Postmortem** | Nachbesprechung eines Vorfalls | Dokumentiert Auswirkungen, Ursachen und Maßnahmen, damit ähnliche Vorfälle seltener auftreten. |
| **Runbook** | Betriebsanleitung | Dokumentiert konkrete Schritte für wiederkehrende Betriebsaufgaben oder Störungen. |
| **MTTR** | Mean Time to Repair/Recover/Restore | Zeitkennzahl für Reparatur oder Wiederherstellung; die genaue Bedeutung hängt von der Definition ab. |
| **MTBF** | Mean Time Between Failures | Durchschnittliche Betriebszeit zwischen Ausfällen. |
| **Error Budget** | Fehlerbudget | Bei einem definierten SLO der tolerierte Anteil an Fehlern oder Nichtverfügbarkeit. |
| **SLO** | Service Level Objective | Internes oder vereinbartes Ziel für eine messbare Serviceeigenschaft. |
| **SLI** | Service Level Indicator | Messgröße für die Servicequalität, z. B. Antwortzeit oder Erfolgsquote. |
| **SLA** | Service Level Agreement | Vertragliche Vereinbarung über Serviceleistungen und gegebenenfalls Konsequenzen bei Nichteinhaltung. |

Quellen: [Google SRE – Toil](https://sre.google/sre-book/eliminating-toil/), [Google SRE – Monitoring](https://sre.google/sre-book/monitoring-distributed-systems/), [Google SRE – Postmortems](https://sre.google/sre-book/postmortem-culture/).

## Fehlerbehandlung und verteilte Systeme

| Begriff | Bedeutung | Kurz erklärt |
|---|---|---|
| **Idempotency** | Idempotenz | Wiederholte identische Operationen haben hinsichtlich des definierten Ergebnisses dieselbe Wirkung wie eine einmalige Ausführung. |
| **Retry** | Wiederholungsversuch | Eine fehlgeschlagene Anfrage wird erneut ausgeführt. |
| **Retry Storm** | Überlastung durch Wiederholungen | Viele automatische Wiederholungen verstärken eine bestehende Störung. |
| **Exponential Backoff** | Exponentiell steigende Wartezeit | Zwischen Wiederholungen wird die Wartezeit schrittweise erhöht. |
| **Circuit Breaker** | Schutzschalter für Dienstaufrufe | Unterbricht vorübergehend Aufrufe an einen fehlerhaften Dienst, um weitere Belastung zu vermeiden. |
| **Timeout** | Zeitüberschreitung | Ein Vorgang wird nicht unbegrenzt abgewartet. |
| **Race Condition** | Nebenläufigkeitsfehler | Das Ergebnis hängt vom zeitlichen Ablauf konkurrierender Vorgänge ab. |
| **Deadlock** | Verklemmung | Prozesse oder Threads warten dauerhaft aufeinander beziehungsweise auf Ressourcen. |
| **Cascading Failure** | Kaskadenausfall | Der Ausfall einer Komponente löst weitere Ausfälle aus. |
| **Graceful Degradation** | Kontrollierte Einschränkung | Kernfunktionen bleiben bei Teilausfällen verfügbar, optionale Funktionen können ausfallen. |
| **Eventual Consistency** | Zeitverzögerte Konsistenz | Replikate können vorübergehend unterschiedliche Daten zeigen und sich später angleichen. |
| **Race-to-failure** | Kein einheitlich definierter Fachbegriff | Nicht als feststehende technische Definition verwenden; besser den konkreten Fehlermechanismus benennen. |

Quelle: [AWS – Zuverlässige Interaktionen in verteilten Systemen](https://wa.aws.amazon.com/wellarchitected/2020-07-02T19-33-23/wat.question.REL_4.en.html).

## Releases und Projektarbeit

| Begriff | Bedeutung | Kurz erklärt |
|---|---|---|
| **MVP** | Minimum Viable Product | Früheste Produktversion, die genügend Nutzen bietet, um Annahmen mit echten Nutzern zu prüfen. |
| **PoC** | Proof of Concept | Prüft, ob eine Idee oder technische Lösung grundsätzlich funktioniert. |
| **Prototype** | Prototyp | Frühes Modell zur Prüfung von Bedienung, Konzept oder Technik. |
| **Scope Creep** | Unkontrollierte Ausweitung des Umfangs | Zusätzliche Anforderungen kommen hinzu, ohne Umfang, Zeit und Ressourcen entsprechend anzupassen. |
| **Bottleneck** | Engpass | Eine begrenzte Ressource bestimmt die Gesamtleistung. |
| **Quick Win** | Schnell erreichbarer Nutzen | Maßnahme mit vergleichsweise geringem Aufwand und kurzfristig erkennbarem Nutzen. |
| **KPI** | Key Performance Indicator | Kennzahl zur Bewertung eines wichtigen Leistungs- oder Geschäftsziels. |
| **Proof of Value** | Nachweis des Nutzens | Prüft, ob eine Lösung einen erwarteten geschäftlichen Nutzen liefert. |
| **Staging** | Vorproduktionsumgebung | Testumgebung, die der Produktionsumgebung möglichst ähnlich ist. |
| **Production / Prod** | Produktivumgebung | Umgebung, in der die Anwendung für echte Nutzer betrieben wird. |
| **Rollback** | Rücknahme einer Änderung | Kehrt nach einem fehlerhaften Release zu einer vorherigen Version oder Konfiguration zurück. |
| **Canary Release** | Schrittweise Veröffentlichung | Neue Version wird zunächst einem kleinen Teil der Nutzer oder Infrastruktur bereitgestellt. |
| **Blue-Green Deployment** | Zwei parallele Umgebungen | Neue Version wird in einer zweiten Umgebung vorbereitet; der Verkehr wird anschließend umgeschaltet. |
| **Feature Flag** | Funktion per Schalter aktivieren | Eine Funktion kann unabhängig vom Code-Release aktiviert oder deaktiviert werden. |
| **Dogfooding** | Eigenes Produkt selbst nutzen | Das Team verwendet sein eigenes Produkt, um Probleme aus Nutzersicht zu erkennen. |

Quelle: [Martin Fowler – Canary Release](https://www.martinfowler.com/bliki/CanaryRelease.html).

## Infrastruktur und Konfiguration

| Begriff | Bedeutung | Kurz erklärt |
|---|---|---|
| **Infrastructure as Code (IaC)** | Infrastruktur als Code | Server, Netzwerke und andere Ressourcen werden deklarativ oder per Code definiert und verwaltet. |
| **Configuration Drift** | Konfigurationsabweichung | Ein System weicht im Laufe der Zeit von der vorgesehenen Konfiguration ab. |
| **Immutable Infrastructure** | Unveränderliche Infrastruktur | Bestehende Instanzen werden bei Änderungen möglichst ersetzt statt manuell angepasst. |
| **Golden Image** | Standardisiertes Systemabbild | Vorgefertigtes Image mit definiertem Betriebssystem und festgelegter Grundkonfiguration. |
| **Container Image** | Container-Abbild | Paket aus Anwendung und benötigten Dateien, aus dem Container gestartet werden. |
| **Ephemeral** | Kurzlebig | Ressource ist für eine begrenzte Lebensdauer vorgesehen und kann ersetzt werden. |
| **Stateless** | Zustandslos | Eine Instanz speichert keinen für spätere Anfragen notwendigen Sitzungszustand lokal. |
| **Stateful** | Zustandsbehaftet | Eine Komponente verwaltet dauerhaften oder sitzungsbezogenen Zustand. |
| **Autoscaling** | Automatische Skalierung | Anzahl oder Größe von Ressourcen wird anhand definierter Regeln angepasst. |
| **Load Balancing** | Lastverteilung | Anfragen werden auf mehrere Server oder Instanzen verteilt. |
| **Capacity Planning** | Kapazitätsplanung | Ressourcenbedarf wird anhand von Last, Wachstum und Reserven geplant. |

## Security-Slang

| Begriff | Bedeutung | Kurz erklärt |
|---|---|---|
| **Attack Surface** | Angriffsfläche | Alle erreichbaren Schnittstellen, Komponenten und Wege, über die ein Angriff möglich sein könnte. |
| **Defense in Depth** | Mehrschichtige Sicherheit | Mehrere unabhängige Schutzmaßnahmen begrenzen die Wirkung eines einzelnen Versagens. |
| **Least Privilege** | Minimalprinzip | Benutzer und Dienste erhalten nur die Rechte, die sie tatsächlich benötigen. |
| **Zero-day Vulnerability** | Noch nicht behobene Sicherheitslücke | Sicherheitslücke, für die zum relevanten Zeitpunkt kein wirksamer Patch verfügbar ist. Der Begriff wird nicht in jeder Quelle exakt gleich abgegrenzt. |
| **Hardening** | Systemhärtung | Deaktiviert unnötige Dienste und reduziert unsichere Konfigurationen und Funktionen. |
| **Misconfiguration** | Fehlkonfiguration | Eine unsichere oder fehlerhafte Einstellung erzeugt ein Risiko. |
| **Lateral Movement** | Seitliche Ausbreitung | Angreifer bewegen sich nach einem ersten Zugriff zu weiteren Systemen oder Konten. |
| **Privilege Escalation** | Rechteausweitung | Ein Benutzer oder Prozess erlangt höhere Berechtigungen als vorgesehen. |
| **Patch Management** | Patch-Verwaltung | Sicherheits- und Fehlerkorrekturen werden bewertet, getestet und kontrolliert ausgerollt. |
| **Attack Chain** | Angriffskette | Mehrere Schritte werden kombiniert, um ein Angriffsziel zu erreichen. |

## Begriffe, die in Gesprächen besonders viel ausdrücken

| Statt langer Erklärung | Präziser Begriff |
|---|---|
| „Der Code funktioniert, ist aber schwer zu ändern.“ | Technical Debt |
| „Die neue Änderung hat etwas Altes kaputtgemacht.“ | Regression |
| „Ein Server ist der Grund, warum alles langsam ist.“ | Bottleneck |
| „Wenn diese eine Komponente ausfällt, fällt der Dienst aus.“ | Single Point of Failure |
| „Der Fehler eines Dienstes bringt andere Dienste mit herunter.“ | Cascading Failure |
| „Wir beheben nur das Symptom, nicht die eigentliche Ursache.“ | Workaround statt Root-Cause-Fix |
| „Wir testen eine neue Version erst mit wenigen Nutzern.“ | Canary Release |
| „Die Serverkonfiguration ist nicht mehr überall gleich.“ | Configuration Drift |
| „Das Team verbringt zu viel Zeit mit wiederholten manuellen Aufgaben.“ | Toil |
| „Wir sammeln Daten, können aber nicht erkennen, warum das System fehlerhaft ist.“ | Unzureichende Observability |
| „Wir bauen immer mehr Funktionen ein, ohne den vereinbarten Umfang anzupassen.“ | Scope Creep |
| „Die Anwendung soll bei Ausfall einer Zusatzfunktion weiterarbeiten.“ | Graceful Degradation |
| „Ein wiederholter API-Aufruf darf keine Doppelbuchung auslösen.“ | Idempotency |
| „Zu viele Warnungen verdecken die wichtigen Alarme.“ | Alert Fatigue |

**Merksatz:** Fachbegriffe sind nur dann nützlich, wenn sie präzise verwendet werden. Bei Begriffen wie *MTTR*, *Zero-day* oder *MVP* sollte die konkrete Definition im jeweiligen Team oder Projekt geklärt sein.
