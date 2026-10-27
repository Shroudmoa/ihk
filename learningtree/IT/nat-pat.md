# NAT & PAT

## In 30 Sekunden

- **NAT (Network Address Translation)** = Übersetzung von privaten zu öffentlichen IP-Adressen
- **PAT (Port Address Translation)** = NAT mit Portnummern. Auch bekannt als **NAT Overload**
- Ermöglicht vielen Clients die Nutzung **einer einzigen öffentlichen IP**
- Findet meist am **Router/Firewall**
- **Private IPs** (RFC 1918) sind ohne NAT nicht direkt aus dem Internet erreichbar

## Kernwissen

### NAT vs PAT

| Merkmal | **NAT (Static/Dynamic)** | **PAT (NAT Overload)** |
|---|---|---|
| **Übersetzung** | 1:1 (IP zu IP) | Viele:1 (Viele private IPs zu einer öffentlichen IP) |
| **Portnummern** | Werden meist nicht verändert | Werden zur Unterscheidung genutzt |
| **Verwendung** | Selten, z. B. für Server im LAN, der von außen erreichbar sein soll | Sehr verbreitet im Heimnetz/Unternehmen |
| **Anzahl Clients** | Begrenzt durch öffentliche IPs | Sehr viele Clients mit nur 1 öffentlicher IP |
| **Bezeichnung** | Network Address Translation | Port Address Translation / NAPT / IP Masquerading |

### Begriffe

| Begriff | Erklärung |
|---|---|
| **Inside Local** | Private IP des internen Geräts (z. B. 192.168.1.10) |
| **Inside Global** | Öffentliche IP, die nach außen genutzt wird (z. B. 84.200.X.X) |
| **Outside Local/Global** | Adressen auf der Internetseite |
| **Masquerading** | Linux-Begriff für PAT (iptables) |

### Static NAT vs Dynamic NAT vs PAT

| Art | Erklärung | Einsatz |
|---|---|---|
| **Static NAT** | Feste Zuordnung: 1 private <-> 1 öffentliche IP | Webserver im LAN, der permanent erreichbar sein muss |
| **Dynamic NAT** | Aus einem Pool öffentlicher IPs wird dynamisch eine zugewiesen | seltener heute |
| **PAT** | Alle nutzen dieselbe öffentliche IP, getrennt über Ports | Standard heute (DSL-Router, Unternehmensfirewall) |

## Prüfungsfrage

**Frage:** Worin liegt der Unterschied zwischen NAT und PAT?

**Musterantwort:** Bei klassischem NAT wird eine private IP-Adresse eins zu eins auf eine öffentliche IP-Adresse übersetzt. Bei PAT (NAT Overload) werden hingegen viele private IP-Adressen auf eine einzige öffentliche IP-Adresse übersetzt, wobei der Router die Verbindungen über unterschiedliche Quellports unterscheidet. Dadurch können sehr viele Clients mit nur einer öffentlichen IP ins Internet.

## Falle

- **NAT ist KEINE Firewall** – NAT versteckt nur Adressen, filtert aber nicht automatisch
- **Port Forwarding** – Nur über NAT/PAT möglich, wenn externer Zugriff auf internen Server gewünscht ist
- **PAT läuft über Portnummern** – Ohne Ports wäre keine Unterscheidung möglich
- **Von außen kein direkter Zugriff** – Ohne Port-Forwarding ist der interne Client nicht von außen erreichbar

## Merksatz

**„NAT = Adressen tauschen. PAT = Adressen + Ports tauschen.“**

## Verwandt

- [Firewall](firewall.md) – Filterung vs Adressübersetzung
- [IPv4](ipv4.md) – Private vs öffentliche Adressen
- [Proxy](proxy.md) – Alternativer Weg zum Zugriff ins Internet
- [Netzwerktypen](netzwerktypen.md) – WAN/LAN
