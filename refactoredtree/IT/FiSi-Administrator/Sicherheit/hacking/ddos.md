# DDoS (Distributed Denial of Service)

## In 30 Sekunden

- Bei einem **DDoS-Angriff** überlasten viele kompromittierte Systeme (**Botnetz**) gleichzeitig ein Ziel, bis legitime Nutzer es nicht mehr erreichen.
- Angegriffen wird das Schutzziel **Verfügbarkeit** – nicht primär die Vertraulichkeit.
- Arten: **volumetrisch** (Bandbreite füllen), **protokollbasiert** (z. B. SYN-Flood), **anwendungsbasiert** (HTTP-Flood auf Layer 7).
- Abwehr: **Rate Limiting**, **DDoS-Schutzdienst/Scrubbing** beim Provider, **CDN/Anycast**, Load Balancer, Firewall/IPS (Prüfung 2026S GA1 zusammen mit Load Balancer).

## Kernwissen

### Was ist ein DDoS-Angriff?

Ein **Distributed Denial of Service (DDoS)** ist ein Angriff, bei dem viele kompromittierte Systeme (Botnet) gleichzeitig massenhaft Anfragen an ein Ziel senden, um dessen Ressourcen zu überlasten. Ziel ist, **legitime Nutzer am Zugriff zu hindern** und damit die **Verfügbarkeit** zu beeinträchtigen.

### Unterschied DoS vs. DDoS

| Kriterium | **DoS (Denial of Service)** | **DDoS (Distributed DoS)** |
|---|---|---|
| **Herkunft** | Ein einzelnes System | Viele verteilte Systeme (Botnet) |
| **Skalierung** | Geringer | Sehr groß |
| **Erkennung/Abwehr** | Relativ einfacher | Schwieriger |
| **Auswirkung** | Oft geringer | Sehr hoch |

### Angriffsarten

| Angriffsart | Schicht | Beschreibung |
|---|---|---|
| **Volumetrisch (z. B. UDP-Flood)** | L3/L4 | Überflutung mit hohem Datenvolumen, Bandbreite wird erschöpft |
| **Protokollbasiert (z. B. SYN-Flood)** | L3/L4 | Ausnutzung von TCP-Handshake, Verbindungsressourcen erschöpft |
| **Anwendungsbasiert (z. B. HTTP-Flood)** | L7 | Sehr gezielt gegen Webanwendungen, wirkt legitim, schwerer zu erkennen |

### Gegenmaßnahmen

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

### Schutzziel

DDoS greift primär das CIA-Schutzziel **Verfügbarkeit (Availability)** an.

**Warum der eigene Server allein nicht hilft:** Ein volumetrischer Angriff füllt bereits die Internetleitung zum Rechenzentrum. Filtern muss man deshalb **vor** dem Engpass – beim Provider oder einem Scrubbing-/CDN-Dienst.

### Kurz gemerkt

- **Botnet = verteilte Angreiferquelle**
- **Ziel = Verfügbarkeitsverlust, nicht direkt Datendiebstahl**
- **HTTP-Flood ist schwer zu filtern** da Anfragen legitim aussehen
- **Kombination aus Maßnahmen** ist effektiv (z. B. CDN + Scrubbing + Load Balancer)

## Prüfungsfrage

**Frage:** Erläutern Sie das Prinzip eines DDoS-Angriffs auf einen Webshop und nennen Sie zwei Gegenmaßnahmen.

**Musterantwort:** Bei einem DDoS-Angriff senden sehr viele kompromittierte Rechner eines Botnetzes gleichzeitig Anfragen an den Webshop oder überfluten seine Internetanbindung. Die Ressourcen wie Bandbreite, Verbindungstabellen oder Rechenleistung werden erschöpft, sodass echte Kunden den Shop nicht mehr erreichen – das Schutzziel Verfügbarkeit ist verletzt. Gegenmaßnahmen sind erstens ein DDoS-Schutzdienst des Providers oder eines CDN, der den Verkehr vor dem Rechenzentrum filtert, und zweitens Rate Limiting zusammen mit einem Load Balancer, der die legitime Last auf mehrere Server verteilt.

## Falle

- DoS und DDoS gleichsetzen: der Unterschied ist die **verteilte Quelle** – einzelne IPs zu sperren hilft bei DDoS kaum.
- „Firewall reicht“ → bei volumetrischen Angriffen ist die Leitung schon vor der Firewall voll.
- DDoS als Datendiebstahl bezeichnen – es geht um **Verfügbarkeit**; allerdings dient er manchmal als Ablenkung für andere Angriffe.

## Merksatz

**„Viele gegen einen – gefiltert wird vor dem Engpass.“**

## Verwandt

- [Load Balancer](../../Server/load-balancer.md) – Lastverteilung
- [Firewall](../firewall/firewall.md) – Filterung
- [Hackers-Can](hackers-can.md) – Angriffsübersicht
- [Schutzziele](../schutzziele.md) – Verfügbarkeit
- [Netzwerkanalyse und Troubleshooting](../../Netzwerk-Administration/netzwerkanalyse-und-troubleshooting.md)
- [IDS und IPS](../firewall/ids-ips.md)
- [Lektion 28 GA1-Praxisthemen (Load Balancer und DDoS)](../../../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md)
