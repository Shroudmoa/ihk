# Routing

## In 30 Sekunden

- Ein **Router** (Schicht 3) leitet Pakete anhand der **Ziel-IP** und seiner **Routingtabelle** zwischen Netzen weiter.
- Routenarten: **direkt angeschlossen**, **statisch**, **Default-Route `0.0.0.0/0`**, **dynamisch** (RIP, OSPF, BGP).
- Auswahlregel: **Longest Prefix Match** – die spezifischste passende Route gewinnt; die Default-Route nur, wenn nichts anderes passt.
- Der **Next-Hop** muss in einem **direkt angeschlossenen** Netz liegen – und das Ziel braucht eine **Rückroute**.
- „Routingtabelle ergänzen/korrigieren“ und „statisch vs. dynamisch begründen“ kommen in fast jeder GA2 (2021W, 2022W, 2023S, 2023W, 2024S, 2025S, 2025W).

## Kernwissen

### Aufbau einer Routingtabelle

| Spalte | Inhalt |
|---|---|
| Zielnetz (Netz-ID) | z. B. `192.168.50.0` |
| Subnetzmaske / Präfix | `255.255.255.0` bzw. `/24` |
| Next-Hop (Gateway) | IP des nächsten Routers – bei direkt angeschlossenen Netzen leer/„direkt“ |
| Interface | ausgehende Schnittstelle (eth0, tun0, …) |
| Metrik | Kosten der Route (Hops, Bandbreite) – bei mehreren Wegen gewinnt die kleinere |

### Beispiel Schritt für Schritt (angelehnt an GA2 Sommer 2024 / Winter 2025)

Router R1: `eth0` WAN `203.0.113.66/30` (ISP-Router `.65`), `eth1` LAN `172.16.0.254/16`, `eth2` DMZ `10.10.0.1/26`, `eth3` Transfernetz `10.255.0.1/30` zu R2 (`10.255.0.2`), hinter R2 liegt die Filiale `192.168.50.0/24`.

1. **Direkt angeschlossene Netze** aus den Interface-Adressen ableiten (Netz-ID berechnen!).
2. **Entfernte Netze** mit dem Next-Hop des benachbarten Routers eintragen.
3. **Default-Route** zum Provider.

| Zielnetz | Maske | Next-Hop | Interface |
|---|---|---|---|
| 172.16.0.0 | 255.255.0.0 | direkt | eth1 |
| 10.10.0.0 | 255.255.255.192 | direkt | eth2 |
| 203.0.113.64 | 255.255.255.252 | direkt | eth0 |
| 10.255.0.0 | 255.255.255.252 | direkt | eth3 |
| 192.168.50.0 | 255.255.255.0 | 10.255.0.2 | eth3 |
| 0.0.0.0 | 0.0.0.0 | 203.0.113.65 | eth0 |

R2 braucht umgekehrt Routen zu `172.16.0.0/16` und `10.10.0.0/26` über `10.255.0.1` (oder eine Default-Route dorthin) – sonst kommen Antworten nie zurück. Bei VPN-Tunneln steht statt eines Next-Hops oft nur das Tunnel-Interface (`tun0`).

### Longest Prefix Match

Tabelle: `10.0.0.0/8 → A`, `10.1.0.0/16 → B`, `10.1.2.0/24 → C`, `0.0.0.0/0 → D`

| Ziel-IP | passende Routen | Gewinner |
|---|---|---|
| 10.1.2.7 | /8, /16, /24, /0 | **C** (/24) |
| 10.1.9.1 | /8, /16, /0 | **B** (/16) |
| 10.200.0.1 | /8, /0 | **A** (/8) |
| 8.8.8.8 | /0 | **D** (Default) |

### Statisch vs. dynamisch (GA2 2021W, 2023S, 2023W)

| | Statisch | Dynamisch |
|---|---|---|
| Pflege | manuell durch Admin | Router tauschen Routen automatisch aus |
| Ausfall einer Leitung | keine automatische Umleitung | **Konvergenz** auf Alternativpfad |
| Skalierung | kleine, stabile Netze | viele Standorte, redundante Wege |
| Ressourcen | keine CPU/Bandbreite für Protokoll | Protokollverkehr, CPU, Know-how |
| Sicherheit | vorhersehbar, nichts wird „gelernt“ | Routing-Updates absichern (Authentifizierung) |
| Typisch | Default-Route am Kundenrouter, Stub-Netze | OSPF im Unternehmen, BGP im Internet |

### Dynamische Routingprotokolle

| Protokoll | Typ | Metrik | Merkmale |
|---|---|---|---|
| **RIP** v2 | Distance Vector | Hop-Anzahl (max. 15) | einfach, langsame Konvergenz, Count-to-Infinity |
| **OSPF** | Link State | Kosten (aus Bandbreite) | Dijkstra/SPF, schnelle Konvergenz, Areas, offener Standard |
| **EIGRP** | Advanced Distance Vector | Bandbreite + Verzögerung | Cisco-Ursprung |
| **BGP** | Path Vector, **EGP** | Pfadattribute (AS-Pfad) | Routing **zwischen** Autonomen Systemen (Internet) |

**Distance Vector vs. Link State (GA2 Sommer 2023):**
- **Distance Vector:** Router kennt nur Entfernung und Richtung („Routing by rumor“) – tauscht periodisch seine Tabelle mit Nachbarn aus. Langsam, schleifengefährdet.
- **Link State:** Jeder Router kennt die **gesamte Topologie** (Link-State-Datenbank) und berechnet selbst den kürzesten Weg. Schnelle Konvergenz, berücksichtigt **Bandbreiten** → richtig, wenn Leitungen unterschiedlich schnell sind (z. B. 500 Mbit/s vs. 10 Gbit/s).

**Administrative Distanz** (Vertrauenswert der Quelle, Cisco): direkt 0, statisch 1, OSPF 110, RIP 120 – kennt ein Router dasselbe Netz aus zwei Quellen, gewinnt der kleinere Wert. Eine statische Route mit höherer Distanz (z. B. 200) dient als **Floating Static Route** (Backup-Weg).

### Befehle

| Befehl | System |
|---|---|
| `route print` / `Get-NetRoute` | Windows |
| `ip route` / `ip route add 192.168.50.0/24 via 10.255.0.2` | Linux |
| `show ip route` / `ip route 192.168.50.0 255.255.255.0 10.255.0.2` | Cisco |
| `tracert` / `traceroute` | Weg der Pakete prüfen |

## Prüfungsfrage

**Frage (angelehnt an GA2 Winter 2023):** Router Hamburg hat die Interfaces `eth0 10.1.1.1/30` (zu Router Berlin `10.1.1.2`) und `eth1 203.0.113.14/30` (ISP `203.0.113.13`). Das Berliner LAN ist `172.16.64.0/22`. Die Tabelle enthält: `172.16.64.0/22 → 10.2.2.2` und `0.0.0.0/0 → 203.0.113.14`. Finden und korrigieren Sie zwei Fehler.

**Musterantwort:**
1. Der Next-Hop `10.2.2.2` liegt in keinem direkt angeschlossenen Netz von Hamburg und ist damit nicht erreichbar. Richtig ist der Berliner Router im Transfernetz: `172.16.64.0/22 → 10.1.1.2` über `eth0`.
2. Die Default-Route zeigt auf `203.0.113.14` – das ist die **eigene** Adresse des Routers. Richtig ist der ISP-Router: `0.0.0.0/0 → 203.0.113.13` über `eth1`.
Zusätzlich muss Berlin eine Rückroute bzw. Default-Route Richtung Hamburg (`10.1.1.1`) besitzen.

## Falle

- **Eigene Interface-IP als Next-Hop** eintragen → Next-Hop ist immer der **Nachbar**.
- Host-IP statt **Netz-ID** als Zielnetz eintragen (`172.16.0.254` statt `172.16.0.0`).
- **Rückroute vergessen** → Hinweg klappt, Antworten gehen ins Leere (oder über die Default-Route ins Internet).
- Default-Route doppelt oder zum falschen Interface.
- Bei „minimale Hops“-Aufgaben einen Link nutzen, der laut Aufgabe **nicht verfügbar** ist (GA2 Winter 2021).
- RIP für Netze mit unterschiedlichen Bandbreiten empfehlen → RIP zählt nur Hops; **OSPF** nimmt Bandbreite.

## Merksatz

**„Spezifisch schlägt allgemein, Nachbar statt ich selbst, und ohne Rückweg kein Gespräch.“**

## Verwandt

- [Subnetting](../OSI-IP-mac-bit-byte/subnetting-ipv4-ipv6.md) – Netz-IDs und Masken für die Tabelle
- [IPv4](../OSI-IP-mac-bit-byte/ipv4.md) – Gateway-Entscheidung des Clients
- [Netzwerkgeräte](netzwerkgeraete.md) – Router und Layer-3-Switch
- [VLAN](vlan.md) – Inter-VLAN-Routing
- [FHRP / VRRP](fhrp-vrrp-hsrp.md) – redundantes Default-Gateway
- [NAT & PAT](../Sicherheit/firewall/nat-pat.md) – Adressübersetzung am Internet-Router
- [VPN](../../nice-to-know/VPN/readme.md) – Routen über Tunnel-Interfaces
- [Lektion 06 – IPv4 & Routing](../../analyse-der-exams/06_IPv4_Adressierung_Routing.md)
- [Lektion 15 – VLAN & Routing](../../analyse-der-exams/L15_VLAN_Routing.md)
