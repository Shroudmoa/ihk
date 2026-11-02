# Skalierbarkeit und Elastizität

## In 30 Sekunden

- **Skalierbarkeit:** Ein System kann bei steigender Last durch zusätzliche Ressourcen mehr leisten.
- **Vertikal (Scale-up):** **größerer** Server (mehr CPU, RAM). Einfach, aber begrenzt, oft mit Downtime, bleibt ein SPOF.
- **Horizontal (Scale-out):** **mehr** Server hinter einem Load Balancer. Nahezu unbegrenzt und ausfallsicherer, aber komplexer.
- **Elastizität:** Ressourcen werden **automatisch** nach Bedarf **hoch und wieder herunter** gefahren (Auto-Scaling). Typisch für die Cloud mit Pay-per-Use.
- Geprüft: horizontale vs. vertikale Skalierung 2023S, Skalierbarkeit als Cloud-Vorteil 2021W/2024S.

## Kernwissen

### Vertikal vs. horizontal

| Merkmal | **Vertikal – Scale-up / Scale-down** | **Horizontal – Scale-out / Scale-in** |
|---|---|---|
| Prinzip | Ressourcen eines bestehenden Servers erweitern/reduzieren | weitere Server/Instanzen hinzufügen oder entfernen |
| Beispiel | VM von 8 auf 16 vCPU und von 32 auf 64 GB RAM | zusätzlicher Webserver im Pool |
| Grenze | maximale Hardware des Hosts | theoretisch nahezu unbegrenzt |
| Downtime | häufig nötig (Neustart, Einbau) | meist ohne Unterbrechung |
| Anwendung | keine Änderung nötig | muss verteilbar sein (zustandslos, Sessions zentral) |
| Ausfallsicherheit | bleibt **SPOF** | höher durch Verteilung |
| Komplexität | gering | höher (Load Balancer, Datenabgleich) |
| Einsatz | Datenbank, monolithische Anwendung | Webserver, Microservices, Container |

Für horizontales Skalieren wird ein [Load Balancer](../load-balancer.md) eingesetzt, der selbst redundant sein muss.

### Skalierbarkeit vs. Elastizität

| Kriterium | Skalierbarkeit | Elastizität |
|---|---|---|
| Charakter | **Fähigkeit** zu wachsen | **automatische** Anpassung an die aktuelle Last |
| Steuerung | geplant, oft manuell | Regeln/Metriken (z. B. CPU > 70 % → +1 Instanz) |
| Zeitrahmen | langfristiges Wachstum | kurzfristige Spitzen und Täler |
| Richtung | vor allem nach oben | hoch **und** runter |
| Kosteneffekt | Kapazität wird vorgehalten | ungenutzte Ressourcen werden freigegeben → Kosten sinken |

**Beispiel:** Ein Onlineshop läuft normal mit 2 Webservern, am Black Friday skaliert er automatisch auf 20 und danach wieder zurück.

### Cloud ist nicht automatisch günstiger

| Lastprofil | Konsequenz |
|---|---|
| dauerhaft hohe, gleichmäßige Last | eigene Hardware oder reservierte Instanzen oft günstiger |
| stark schwankende Last | Elastizität spart Überkapazität |
| schlechte Kostenkontrolle | vergessene Instanzen und Datenverkehr (Egress) treiben die Rechnung |
| proprietäre Dienste | Vendor Lock-in, teure Migration |

Entscheidend sind [TCO](../../../Management/tco.md), Lastprofil, Betriebsaufwand und Wechselkosten.

## Prüfungsfrage

**Frage:** Der Webshop stößt zu Spitzenzeiten an seine Grenzen. Erläutern Sie horizontale und vertikale Skalierung und empfehlen Sie eine Variante.

**Musterantwort:** Bei der vertikalen Skalierung (Scale-up) erhält der bestehende Server mehr Ressourcen, etwa zusätzliche CPU-Kerne oder mehr Arbeitsspeicher. Das ist einfach umzusetzen und erfordert keine Änderung der Anwendung, stößt aber an die Grenzen der Hardware, erfordert oft einen Neustart, und der Server bleibt ein Single Point of Failure. Bei der horizontalen Skalierung (Scale-out) werden weitere Server-Instanzen hinzugefügt, und ein Load Balancer verteilt die Anfragen. Das ist nahezu beliebig erweiterbar und erhöht gleichzeitig die Ausfallsicherheit, setzt aber eine verteilbare Anwendung mit zentraler Sitzungsverwaltung voraus. Für einen Webshop mit Lastspitzen ist die horizontale Skalierung zu empfehlen, idealerweise elastisch per Auto-Scaling, damit zusätzliche Instanzen nach der Spitze wieder entfernt werden.

## Falle

- Vertikal und horizontal vertauschen: **up = größer**, **out = mehr**.
- Skalierbarkeit und Elastizität gleichsetzen. Elastizität ist **automatisch und in beide Richtungen**.
- Scale-out ohne Load Balancer oder mit lokal gespeicherten Sitzungen planen.
- Behaupten, Scale-out ginge immer. Eine Anwendung, die nicht verteilt laufen kann, profitiert nicht.

## Merksatz

**„Up = dicker, Out = mehr, elastisch = atmet von selbst.“**

## Verwandt

- [Load Balancer](../load-balancer.md) – unverzichtbar für Scale-out
- [Cloud Computing](../Cloud/cloud-computing.md) – Elastizität als Kernmerkmal
- [HA-Hochverfügbarkeit](../ha-hochverfuegbarkeit.md)
- [Cluster](../cluster.md)
- [Container & Docker](container-docker.md) – schnelles horizontales Skalieren
- [Virtualisierung](virtualisierung.md)
- [Blue-Green-Deployment](../blue-green-deployment.md)
- [TCO](../../../Management/tco.md)
- [Lektion L28 GA1 Praxisthemen](../../../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md)
