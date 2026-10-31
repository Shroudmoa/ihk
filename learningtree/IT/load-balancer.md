# Load Balancer (Lastverteiler)

## Was ist ein Load Balancer?

Ein **Load Balancer** verteilt eingehende Client-Anfragen auf mehrere Backend-Server. Er tritt als **virtueller Endpunkt (VIP)** auf und sorgt für Lastverteilung, Ausfallsicherheit und Skalierbarkeit.

## Aufgaben eines Load Balancers

- **Lastverteilung:** Gleichmäßige Verteilung auf verfügbare Server
- **Hochverfügbarkeit:** Erkennt ausgefallene Server über Health Checks und entfernt sie aus dem Pool
- **Skalierbarkeit:** Ermöglicht horizontales Skalieren (Scale-Out)
- **Session-Stickyness:** Kann Clients dauerhaft dem gleichen Backend-Server zuweisen
- **SSL/TLS-Terminierung:** Entschlüsselt HTTPS und leitet entpackt weiter
- **Health Monitoring:** Kontinuierliche Überprüfung der Serververfügbarkeit

## Load-Balancing-Algorithmen

| Algorithmus | Beschreibung | Einsatz |
|---|---|---|
| **Round Robin** | Gleichmäßige Verteilung reihum auf alle Server | Gleich leistungsstarke Server |
| **Weighted Round Robin (WRR)** | Berücksichtigt Servergewichtung | Unterschiedlich leistungsstarke Server |
| **Least Connections** | Verteilung an den Server mit den **wenigsten aktiven Verbindungen** | Unterschiedliche Verbindungsdauern |
| **Weighted Least Connections** | Kombination aus Gewichtung und wenigsten Verbindungen | Heterogene Serverlandschaft |
| **IP Hash** | Hash der Client-IP bestimmt den Server | Benötigte **Session-Persistenz** (Stickyness) |
| **Least Response Time** | An Server mit kürzester Antwortzeit + wenigsten Verbindungen | Performance-optimiert |

## Layer 4 vs. Layer 7

| Kriterium | **Layer 4 (Transport, TCP/UDP)** | **Layer 7 (Application, HTTP/HTTPS)** |
|---|---|---|
| **Kriterium** | IP-Adresse, Port | URL-Pfad, Header, Cookies, HTTP-Methode |
| **Geschwindigkeit** | Schneller, weniger Overhead | Etwas langsamer, aber intelligenter |
| **SSL-Terminierung** | Meist nicht möglich | Häufig genutzt |
| **Content-basiert** | Nein | Ja (z. B. `/api/*` → API-Pool) |
| **Einsatz** | Generische TCP/UDP-Dienste | Webanwendungen, APIs |

## Health Checks

- **Zweck:** Prüfen, ob Backend-Server funktionsfähig sind
- **Typen:** TCP-Check, HTTP/HTTPS-Check
- **Verhalten:** Bei Ausfall → Entfernung aus Pool
- **Wiederaufnahme:** Nur nach erfolgreichem Health Check

## IHK-Merksätze

- **Load Balancer verteilt Verkehr + erhöht Verfügbarkeit**
- **Health Checks** verhindern Weiterleitung an defekte Server
- **Layer 7 = contentbasiertes Routing**
- **IP Hash = Sticky Session**
- **Oft kombiniert mit Cluster für HA**

## Verwandt

- [HA-Hochverfügbarkeit](ha-hochverfuegbarkeit.md)
- [Cluster](cluster.md)
- [Skalierbarkeit-Elastizität](skalierbarkeit-elastizitaet.md)
- [DDoS](ddos.md)
- [Firewall](firewall.md)
