# OSI

## In 30 Sekunden

- 7 Schichten, von unten nach oben: **Physical, Data Link, Network, Transport, Session, Presentation, Application**.
- Merksatz: *Please Do Not Throw Salami Pizza Away* – Schicht 1 bis 7.
- Die Prüfung fragt fast immer: **„Ordne Gerät oder Protokoll einer Schicht zu“** oder **„was macht Schicht X“**.
- Switch = Layer 2 (MAC), Router = Layer 3 (IP), Proxy = Layer 7.
- TLS ist **keine** eigene OSI-Schicht – schreib „zwischen 4 und 7“, dann ist es nicht falsch.

## Kernwissen

| # | Schicht | Aufgabe | Beispiele | Geräte |
|---:|---|---|---|---|
| 7 | Anwendung | Dienste für Programme | HTTP, DNS, SMTP, DHCP | Proxy, Gateway |
| 6 | Darstellung | Format, Zeichen, Kompression, Verschlüsselung | UTF-8, JPEG, TLS* | – |
| 5 | Sitzung | Sitzungen aufbauen, halten, beenden | RPC, NetBIOS | – |
| 4 | Transport | Ende-zu-Ende, Ports, Segmentierung | TCP, UDP | – |
| 3 | Vermittlung | logische Adressen, Routing | IPv4, IPv6, ICMP | Router |
| 2 | Sicherung | Frames im lokalen Netz, MAC, Fehlererkennung | Ethernet, WLAN | Switch, Bridge |
| 1 | Bitübertragung | Bits als Signal übertragen | Kupfer, Glasfaser, Funk | Repeater, Hub |

**OSI vs. TCP/IP (kommt gern als Vergleich)**
| OSI | TCP/IP |
|---|---|
| 1 + 2 | Netzwerkzugriff |
| 3 | Internet |
| 4 | Transport |
| 5 + 6 + 7 | Anwendung |

**Einordnung üben**
| Aussage | Schicht |
|---|---|
| „Switch lernt MAC-Adressen“ | 2 |
| „Router wählt den nächsten Hop“ | 3 |
| „TCP segmentiert und nummeriert“ | 4 |
| „Proxy terminiert TLS und prüft HTTP“ | 7 |
| „Hub verteilt Signale an alle Ports“ | 1 |

## Prüfungsfrage

**Frage:** Ein Paket geht vom Client über einen Switch und einen Router zum Webserver. Auf welchen Schichten arbeiten die Geräte? Begründe.

**Musterantwort:** Der Switch arbeitet auf Schicht 2 und leitet Frames anhand seiner MAC-Adresstabelle weiter, ohne die IP-Adresse zu verändern. Der Router arbeitet auf Schicht 3, liest die Ziel-IP und wählt anhand der Routingtabelle den nächsten Hop. Dabei wird die Layer-2-Adresse pro Strecke neu geschrieben, die IP-Adresse bleibt Ende-zu-Ende erhalten.

## Falle

- „TLS ist Schicht 6“ – strittig, also besser „zwischen 4 und 7“ schreiben.
- Hub und Switch verwechseln: Hub = Schicht 1, Switch = Schicht 2.
- „Router arbeitet auf Schicht 2, weil er MAC-Adressen umschreibt“ → nein, das Umschreiben ist nur die neue Frameadresse.
- Schicht 5 und 7 vertauschen: DNS und DHCP sind Anwendung, nicht Sitzung.

## Merksatz

**Please Do Not Throw Salami Pizza Away** – und dazu: **Switch = 2, Router = 3, Proxy = 7.**

## Verwandt

- [MAC](mac.md) – Schicht 2 konkret
- [IPv4](ipv4.md) – Schicht 3 konkret
- [Proxy](proxy.md) – Schicht 7 konkret
- [Firewall](firewall.md) – welcher Typ auf welcher Schicht filtert
