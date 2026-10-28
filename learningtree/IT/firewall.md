# Firewall

## In 30 Sekunden

- Eine Firewall erlaubt oder verbietet Verkehr nach Regeln – sie ist **kein Rundumschutz**.
- **Paketfilter** (Layer 3/4) sieht IP, Port, Protokoll. **Stateful** merkt sich Verbindungen. **NGFW** schaut bis Layer 7.
- Regeln werden **von oben nach unten** gelesen, die erste passende gewinnt, unten steht **deny all**.
- Zonen: Internet → DMZ (Web/Mail) → LAN. Gäste nie ins interne LAN.
- Phishing, USB-Sticks und bereits infizierte Clients fängt keine Firewall.

## Kernwissen

| Typ | Schicht | Erkennt | Grenze |
|---|---|---|---|
| Paketfilter | 3/4 | IP, Port, Protokoll | kein Zusammenhang zwischen Paketen |
| Stateful (SPI) | 3/4 | Verbindungszustand | Inhalt bleibt unsichtbar |
| Proxy / ALG | 7 | Anwendungsprotokoll | Aufwand, möglicher Flaschenhals |
| WAF | 7 | HTTP-Inhalte | kein Ersatz für sichere Programmierung |
| NGFW | 3–7 | App-Erkennung, IDS/IPS | teuer, komplex |
| Host-Firewall | 3–7 | lokal pro Gerät | schützt nicht das Netz |

**Regel lesen und schreiben**
`Quelle → Ziel | Port | Protokoll | Aktion`

| # | Quelle | Ziel | Port | Aktion |
|---:|---|---|---|---|
| 1 | LAN `192.168.1.0/24` | any | 443 | allow |
| 2 | any | Webserver | 443 | allow |
| 3 | Gast `192.168.9.0/24` | LAN | any | deny |
| 4 | any | any | any | deny |

Regel 1–3 sind Beispiele, Regel 4 ist die implizite Grundhaltung: **alles andere ist verboten**.

**Zonen und Richtung**
- Internet → DMZ: nur die Dienste, die veröffentlicht werden sollen (443, 25 …).
- DMZ → LAN: möglichst gar nichts, sonst nur einzeln begründet.
- LAN → Internet: meist erlaubt, aber mit Egress-Filter.

## Prüfungsfrage

**Frage:** Warum reicht eine Stateful Firewall nicht, um den Datenabfluss eines kompromittierten Clients zu verhindern?

**Musterantwort:** Die Stateful Firewall prüft nur IP-Adresse, Port und Verbindungszustand. Der Client baut eine erlaubte Verbindung nach außen auf, zum Beispiel HTTPS auf Port 443; den Inhalt sieht die Firewall nicht. Gegen Datenabfluss braucht man Layer-7-Prüfung über NGFW oder Proxy, Egress-Filterung und Endpoint-Schutz.

## Falle

- „Die Firewall schützt vor Viren und Phishing“ → falsch, dafür sind Endpoint-Schutz und Mailfilter zuständig.
- Eine `allow any → any`-Regel oben → alles darunter ist wirkungslos.
- NAT mit Firewall verwechseln: NAT übersetzt Adressen, es filtert nicht.
- Stateful mit „prüft Inhalte“ erklären → tut es nicht.

## Merksatz

**„Oben gewinnt, unten deny.“** Regeln immer von speziell nach allgemein sortieren.

## Verwandt

- [Proxy](proxy.md) – Application-Level Gateway und Reverse Proxy
- [Netzwerktypen](netzwerktypen.md) – LAN, WAN und Segmentierung
- [hackers-can](hackers-can.md) – welche Angriffe abgewehrt werden
- [OSI](osi.md) – Zuordnung der Firewall-Typen zu Schichten
- [Ports](ports.md) – welche Ports gefiltert werden
- [IPv4](ipv4.md) – IP-Filterung
- [VLAN](vlan.md) – Netzsegmentierung mit Firewalls
