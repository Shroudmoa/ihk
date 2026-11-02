# Skalierbarkeit & Elastizität

## Skalierbarkeit (Scalability)

**Skalierbarkeit** beschreibt die Fähigkeit eines IT-Systems, bei steigender Last mehr Leistung bereitzustellen, indem Ressourcen angepasst werden.

## Vertikale vs. horizontale Skalierung

| Merkmal | **Vertikal – Scale Up / Scale Down** | **Horizontal – Scale Out / Scale In** |
|---|---|---|
| **Prinzip** | Ressourcen eines bestehenden Servers erweitern/reduzieren (CPU, RAM) | Hinzufügen oder Entfernen weiterer Server/Instanzen |
| **Beispiel** | Mehr RAM oder stärkere CPU einbauen/zuweisen | Zusätzlichen Webserver zum Pool hinzufügen |
| **Hardwaregrenze** | Durch maximale Hardwarekapazität begrenzt | Theoretisch nahezu unbegrenzt |
| **Downtime** | Häufig nötig (bes. bei physischer Aufrüstung) | Meist ohne vollständige Downtime möglich |
| **Komplexität** | Geringer | Höher (Lastverteilung, Sessions, Datenabgleich) |
| **Ausfallsicherheit** | Höheres Risiko eines **SPOF** (Single Point of Failure) | Höhere Ausfallsicherheit durch Verteilung |
| **Einsatz** | Kleinere Erweiterungen, monolithische Systeme | Hochverfügbare, verteilte Systeme |

## Scale Up vs. Scale Out im Detail

- **Scale Up (vertikal):** „Stärkerer Server“ – mehr Leistung pro Einheit
- **Scale Down (vertikal):** Ressourcen reduzieren
- **Scale Out (horizontal):** „Mehr Server“ – Lastverteilung über mehrere Einheiten
- **Scale In (horizontal):** Überflüssige Instanzen entfernen

Für horizontales Skalieren wird meist ein **[Load Balancer](../load-balancer.md)** eingesetzt.

## Elastizität (Elasticity)

**Elastizität** geht über die Skalierbarkeit hinaus: Das System passt Ressourcen **automatisch und bedarfsgerecht** an – dynamisch nach oben und unten.

**Beispiel:** Ein Onlineshop skaliert während eines Black Fridays automatisch auf 20 Webserver hoch und fährt nach Ende der Aktion wieder auf 2 Server herunter.

## Unterschied Skalierbarkeit vs. Elastizität

| Kriterium | **Skalierbarkeit** | **Elastizität** |
|---|---|---|
| **Charakter** | Potenzial zur Leistungssteigerung | Dynamische, automatische Anpassung an aktuellen Bedarf |
| **Steuerung** | Meist manuell oder geplant | Meist automatisch (Auto-Scaling) |
| **Zeitrahmen** | Langfristig (Wachstum) | Kurzfristig (Lastspitzen/-täler) |
| **Richtung** | Fokus auf Vergrößern | Fokus auf **hoch und runter** skalieren |
| **Cloud-Bezug** | Allgemeines Konzept | Typisch für Public Cloud (Pay-per-Use) |

## IHK-Merksätze

- **Scale-Up = vertikal (größer)** | **Scale-Out = horizontal (mehr)**
- **Elastisch = wächst und schrumpft automatisch**
- **Horizontale Skalierung + Load Balancer = hohe Verfügbarkeit & Ausfallsicherheit**
- **Nur elastisch heißt:** Ressourcen werden bei Nichtbedarf **wieder freigegeben** (Kostenersparnis)

## Verwandt

- [Load Balancer](../load-balancer.md) – unverzichtbar für Scale-Out
- [Cloud Computing](../Cloud/cloud-computing.md) – Elastizität als Kernmerkmal
- [HA-Hochverfügbarkeit](../ha-hochverfuegbarkeit.md)
- [Virtualisierung](virtualisierung.md)
- [Hypervisor-Typen](hypervisor-typen.md)


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
