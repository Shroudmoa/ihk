# Skalierbarkeit-Elastizität

# Skalierbarkeit und Elastizität – IHK AP2

## Skalierbarkeit (Scalability)

Skalierbarkeit beschreibt, wie gut ein IT-System steigende oder sinkende Last bewältigen kann, indem Ressourcen angepasst werden.

## Vertikale Skalierung vs. horizontale Skalierung

| Merkmal | Vertikal – Scale Up / Down | Horizontal – Scale Out / In |
|---|---|---|
| Prinzip | Ressourcen eines Servers erweitern oder reduzieren | Server hinzufügen oder entfernen |
| Beispiel | Mehr RAM, stärkere CPU | Einen weiteren Webserver bereitstellen |
| Hardware | Oft Upgrade bestehender Hardware | Zusätzliche Server oder Instanzen |
| Downtime | Bei physischen Upgrades häufig nötig | Häufig ohne vollständige Downtime möglich |
| Grenze | Durch maximale Hardwarekapazität begrenzt | Durch Architektur und Infrastruktur begrenzt |
| Komplexität | Meist einfacher | Höher, z. B. Verteilung und Synchronisation |
| Kosten | Teure leistungsfähige Einzelhardware möglich | Mehrere kleinere Instanzen; zusätzlicher Betriebsaufwand |
| Ausfallsicherheit | Einzelserver kann Single Point of Failure sein | Mehrere Instanzen können Ausfälle abfangen |

### Vertikal skalieren – Scale Up

Ein Server bekommt mehr CPU-Leistung, RAM oder schnelleren Speicher.

- **Physisch:** Hardwaretausch oder RAM-Erweiterung kann Abschaltung und Wartungsfenster erfordern.
- **Virtualisiert:** CPU- und RAM-Zuweisungen lassen sich häufig schnell anpassen. Je nach Hypervisor, Gastbetriebssystem und Änderung kann trotzdem ein Neustart nötig sein.
- **Grenze:** Ein einzelner Server kann nicht unbegrenzt wachsen.

### Horizontal skalieren – Scale Out

Zusätzliche Server oder virtuelle Maschinen übernehmen einen Teil der Last.

- Ein Load Balancer verteilt Anfragen auf mehrere Server.
- Fällt eine Instanz aus, können andere weiterarbeiten, sofern die Architektur entsprechend ausgelegt ist.
- Anwendungen müssen häufig für mehrere Instanzen ausgelegt sein. Sessions, Datenbanken und gemeinsam genutzte Dateien müssen berücksichtigt werden.

**Scale In** bedeutet, Instanzen zu entfernen, wenn weniger Leistung benötigt wird.

## Elastizität (Elasticity)

Elastizität beschreibt, wie dynamisch Ressourcen an den aktuellen Bedarf angepasst werden – nach oben und nach unten.

**Beispiel:** Ein Onlineshop benötigt während einer Verkaufsaktion zehn Server statt zwei. Nach dem Ende der Aktion werden die zusätzlichen Instanzen wieder entfernt.

| Skalierbarkeit | Elastizität |
|---|---|
| System kann mit steigender Last umgehen | Ressourcen passen sich dynamisch an die Last an |
| Kann manuell oder automatisch erfolgen | Häufig automatisiert und bedarfsabhängig |
| Fokus: Wachstum und Kapazität | Fokus: flexible Anpassung nach oben und unten |

**Merksatz:** Skalierbarkeit ist die Fähigkeit zu wachsen. Elastizität ist die dynamische Anpassung an den tatsächlichen Bedarf.

## Cloud: schneller – aber zu welchem Preis?

Cloud-Dienste können Ressourcen oft innerhalb von Minuten oder Sekunden bereitstellen. Das hängt vom Anbieter, Dienst und der Architektur ab.

### Vorteile

- Schnelle Bereitstellung ohne vorherigen Hardwarekauf.
- Automatische Skalierung bei Lastspitzen.
- Abrechnung häufig nutzungsabhängig (OPEX).
- Weniger eigene Hardwarewartung, abhängig vom Cloud-Service-Modell.

### Nachteile und Kosten

- Laufende Kosten können bei dauerhaft hoher Auslastung erheblich sein.
- Unbeachtete Ressourcen können weiter Kosten verursachen.
- Datenübertragung aus der Cloud kann zusätzliche Gebühren verursachen.
- Proprietäre Dienste können Vendor Lock-in erzeugen.
- Abhängigkeit von Internetverbindung, Anbieter und dessen Verfügbarkeit.
- Datenschutz und Compliance müssen geprüft werden, z. B. Auftragsverarbeitung und Drittlandtransfers nach DSGVO.

### Kostenvergleich

| Szenario | Typische Konsequenz |
|---|---|
| Dauerhaft hohe Last | Eigene oder reservierte Kapazität kann wirtschaftlich sein |
| Stark schwankende Last | Elastische Cloud-Ressourcen können Kosten und Überkapazität reduzieren |
| Schlechte Kostenkontrolle | Unbenutzte Instanzen und Datenverkehr erhöhen die Rechnung |
| Abhängigkeit von proprietären Cloud-Diensten | Migration kann aufwendig und teuer werden |

Cloud ist daher nicht automatisch günstiger. Entscheidend sind TCO, Lastprofil, Betriebsaufwand, Verfügbarkeit und mögliche Wechselkosten.

## Load Balancer

Ein Load Balancer verteilt eingehende Anfragen auf mehrere Backend-Server.

**Typische Verfahren:**
- **Round Robin:** Anfragen werden der Reihe nach verteilt.
- **Least Connections:** Bevorzugt den Server mit den wenigsten aktiven Verbindungen.
- **Health Checks:** Prüfen, ob ein Backend erreichbar ist; fehlerhafte Instanzen können aus der Verteilung genommen werden.

Ein Load Balancer ermöglicht horizontale Skalierung, garantiert aber allein keine Hochverfügbarkeit. Er selbst muss ebenfalls gegen Ausfälle abgesichert sein.

## IHK-Merksatz

Vertikale Skalierung erweitert die Ressourcen eines einzelnen Systems (Scale Up), horizontale Skalierung fügt weitere Systeme hinzu (Scale Out). Elastizität beschreibt die dynamische Anpassung dieser Ressourcen an den Bedarf. Cloud-Dienste ermöglichen häufig eine schnelle Bereitstellung, verursachen aber nutzungsabhängige Kosten und können Abhängigkeiten vom Anbieter schaffen.
