# Load Balancer (Lastverteiler)

## In 30 Sekunden

- Ein Load Balancer nimmt Anfragen unter einer **virtuellen IP (VIP)** an und verteilt sie auf einen **Pool** von Backend-Servern.
- Ziele: **Lastverteilung**, **Ausfallsicherheit** (Health Checks nehmen defekte Server aus dem Pool) und **horizontale Skalierung**.
- Wichtigste Verfahren: **Round Robin** (reihum), **Least Connections** (wenigste aktive Verbindungen), **IP-Hash** (gleicher Client → gleicher Server).
- **Layer 4** entscheidet nach IP und Port, **Layer 7** nach Inhalt (URL, Header, Cookie).
- Der Load Balancer selbst muss redundant sein, sonst ist er der neue Single Point of Failure.
- Geprüft: Funktionsweise und Methoden 2026S, Umschaltung beim Blue-Green-Deployment 2023S.

## Kernwissen

### Was ist ein Load Balancer?

Ein **Load Balancer** verteilt eingehende Client-Anfragen auf mehrere Backend-Server. Er tritt als **virtueller Endpunkt (VIP)** auf und sorgt für Lastverteilung, Ausfallsicherheit und Skalierbarkeit.

### Aufgaben eines Load Balancers

- **Lastverteilung:** Gleichmäßige Verteilung auf verfügbare Server
- **Hochverfügbarkeit:** Erkennt ausgefallene Server über Health Checks und entfernt sie aus dem Pool
- **Skalierbarkeit:** Ermöglicht horizontales Skalieren (Scale-Out)
- **Session-Stickyness:** Kann Clients dauerhaft dem gleichen Backend-Server zuweisen
- **SSL/TLS-Terminierung:** Entschlüsselt HTTPS und leitet entpackt weiter
- **Health Monitoring:** Kontinuierliche Überprüfung der Serververfügbarkeit

### Load-Balancing-Algorithmen

| Algorithmus | Beschreibung | Einsatz |
|---|---|---|
| **Round Robin** | Gleichmäßige Verteilung reihum auf alle Server | Gleich leistungsstarke Server |
| **Weighted Round Robin (WRR)** | Berücksichtigt Servergewichtung | Unterschiedlich leistungsstarke Server |
| **Least Connections** | Verteilung an den Server mit den **wenigsten aktiven Verbindungen** | Unterschiedliche Verbindungsdauern |
| **Weighted Least Connections** | Kombination aus Gewichtung und wenigsten Verbindungen | Heterogene Serverlandschaft |
| **IP Hash** | Hash der Client-IP bestimmt den Server | Benötigte **Session-Persistenz** (Stickyness) |
| **Least Response Time** | An Server mit kürzester Antwortzeit + wenigsten Verbindungen | Performance-optimiert |

### Layer 4 vs. Layer 7

| Kriterium | **Layer 4 (Transport, TCP/UDP)** | **Layer 7 (Application, HTTP/HTTPS)** |
|---|---|---|
| **Kriterium** | IP-Adresse, Port | URL-Pfad, Header, Cookies, HTTP-Methode |
| **Geschwindigkeit** | Schneller, weniger Overhead | Etwas langsamer, aber intelligenter |
| **SSL-Terminierung** | Meist nicht möglich | Häufig genutzt |
| **Content-basiert** | Nein | Ja (z. B. `/api/*` → API-Pool) |
| **Einsatz** | Generische TCP/UDP-Dienste | Webanwendungen, APIs |

### Health Checks

- **Zweck:** Prüfen, ob Backend-Server funktionsfähig sind
- **Typen:** TCP-Check, HTTP/HTTPS-Check
- **Verhalten:** Bei Ausfall → Entfernung aus Pool
- **Wiederaufnahme:** Nur nach erfolgreichem Health Check

### Redundanz des Load Balancers

Zwei Load Balancer als **Active/Passive-Paar** teilen sich die VIP (z. B. per VRRP/keepalived). Fällt der aktive aus, übernimmt der passive die VIP.

## Prüfungsfrage

**Frage:** Erläutern Sie die Funktionsweise eines Load Balancers und die Methoden Round Robin, Least Connections und IP-Hashing.

**Musterantwort:** Ein Load Balancer ist der zentrale Eintrittspunkt für Clients. Er nimmt die Anfragen unter einer virtuellen IP entgegen und leitet sie an einen von mehreren Backend-Servern weiter. Per Health Check erkennt er ausgefallene Server und schickt ihnen keine Anfragen mehr. Bei **Round Robin** werden die Anfragen reihum verteilt (1, 2, 3, 1, 2, 3, …). Das ist einfach, berücksichtigt aber weder Last noch Leistungsfähigkeit der Server. Bei **Least Connections** geht jede neue Anfrage an den Server mit den aktuell wenigsten aktiven Verbindungen. Das eignet sich bei unterschiedlich langen Sitzungen. Beim **IP-Hashing** wird aus der Client-IP ein Hashwert gebildet, der den Server bestimmt. Derselbe Client landet so immer auf demselben Server (Session-Persistenz), die Verteilung kann aber ungleichmäßig sein, etwa wenn viele Clients hinter einer NAT-Adresse stehen.

## Falle

- Load Balancer und Firewall gleichsetzen. Ein LB verteilt, er filtert nicht primär (eine WAF kann zusätzlich integriert sein).
- Round Robin bei sehr unterschiedlich starken Servern ohne Gewichtung einsetzen → Weighted Round Robin.
- Session-Daten nur lokal auf einem Server halten und Round Robin nutzen. Der Nutzer verliert beim nächsten Request seinen Warenkorb. Lösung: Sticky Sessions oder zentraler Session-Store.
- Einen einzelnen Load Balancer als HA-Lösung verkaufen, obwohl er selbst ein SPOF ist.

## Merksatz

**„Round Robin reihum, Least Connections zum Faulsten, IP-Hash immer zum Selben. Health Check wirft die Kranken raus.“**

## Verwandt

- [HA-Hochverfügbarkeit](ha-hochverfuegbarkeit.md)
- [Cluster](cluster.md)
- [Skalierbarkeit-Elastizität](virtualisierung/skalierbarkeit-elastizitaet.md)
- [DDoS](../Sicherheit/hacking/ddos.md)
- [Firewall](../Sicherheit/firewall/firewall.md)
- [Blue-Green-Deployment](blue-green-deployment.md) – Umschaltung per Load Balancer
- [Verfügbarkeit & Kennzahlen](verfuegbarkeit-kennzahlen.md) – Parallelschaltung rechnen
- [Lektion L28 GA1 Praxisthemen](../../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md)
