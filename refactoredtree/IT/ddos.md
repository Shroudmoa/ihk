# DDoS (Distributed Denial of Service)

## Was ist ein DDoS-Angriff?

Ein **Distributed Denial of Service (DDoS)** ist ein Angriff, bei dem viele kompromittierte Systeme (Botnet) gleichzeitig massenhaft Anfragen an ein Ziel senden, um dessen Ressourcen zu überlasten. Ziel ist, **legitime Nutzer am Zugriff zu hindern** und damit die **Verfügbarkeit** zu beeinträchtigen.

## Unterschied DoS vs. DDoS

| Kriterium | **DoS (Denial of Service)** | **DDoS (Distributed DoS)** |
|---|---|---|
| **Herkunft** | Ein einzelnes System | Viele verteilte Systeme (Botnet) |
| **Skalierung** | Geringer | Sehr groß |
| **Erkennung/Abwehr** | Relativ einfacher | Schwieriger |
| **Auswirkung** | Oft geringer | Sehr hoch |

## Angriffsarten (wichtig für IHK)

| Angriffsart | Schicht | Beschreibung |
|---|---|---|
| **Volumetrisch (z. B. UDP-Flood)** | L3/L4 | Überflutung mit hohem Datenvolumen, Bandbreite wird erschöpft |
| **Protokollbasiert (z. B. SYN-Flood)** | L3/L4 | Ausnutzung von TCP-Handshake, Verbindungsressourcen erschöpft |
| **Anwendungsbasiert (z. B. HTTP-Flood)** | L7 | Sehr gezielt gegen Webanwendungen, wirkt legitim, schwerer zu erkennen |

## Gegenmaßnahmen

| Maßnahme | Erklärung |
|---|---|
| **Rate Limiting** | Begrenzung der Anfragen pro IP/Zeitintervall |
| **Traffic Scrubbing / DDoS-Protection** | Reinigungsdienst filtert schädlichen Traffic heraus |
| **Anycast** | Verteilung des Traffics auf mehrere Standorte → Last wird verteilt |
| **CDN (Content Delivery Network)** | Zwischenspeicherung statischer Inhalte, entlastet Ursprung |
| **Load Balancer** | Verteilt Anfragen auf mehrere Server |
| **Firewall/IPS/NGFW** | Erkennung und Filterung verdächtigen Traffics |
| **Blackholing/RTBH** | Weiterleitung des gesamten Traffics ins Nirwana (Notlösung) |
| **Geo-Blocking** | Sperren von Regionen, aus denen kein legitimer Traffic erwartet wird |

## Schutzziel

DDoS greift primär das CIA-Schutzziel **Verfügbarkeit (Availability)** an.

## IHK-Merksätze

- **Botnet = verteilte Angreiferquelle**
- **Ziel = Verfügbarkeitsverlust, nicht direkt Datendiebstahl**
- **HTTP-Flood ist schwer zu filtern** da Anfragen legitim aussehen
- **Kombination aus Maßnahmen** ist effektiv (z. B. CDN + Scrubbing + Load Balancer)

## Verwandt

- [Load Balancer](load-balancer.md) – Lastverteilung
- [Firewall](firewall.md) – Filterung
- [Hackers-Can](hackers-can.md) – Angriffsübersicht
- [Schutzziele](schutzziele.md) – Verfügbarkeit
- [Netzwerkanalyse und Troubleshooting](netzwerkanalyse-und-troubleshooting.md)
