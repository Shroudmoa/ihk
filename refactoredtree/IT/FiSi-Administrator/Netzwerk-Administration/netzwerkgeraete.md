# Netzwerkgeräte – Hub, Switch, Router, L3-Switch, AP & Co.

## In 30 Sekunden

- Geräte werden nach der **OSI-Schicht** eingeordnet, auf der sie Entscheidungen treffen: **Hub/Repeater 1, Switch/Bridge/AP 2, Router/L3-Switch 3, Proxy/NGFW 7**.
- **Hub** verteilt Signale an alle Ports, **Switch** leitet gezielt nach **MAC**, **Router** verbindet **Netze** nach **IP**.
- Ein **Layer-3-Switch** routet zwischen VLANs in Hardware (SVI) – schnell, aber meist ohne NAT/VPN/Firewall-Funktionen eines Routers.
- Ein **Access Point** ist eine **Bridge** zwischen Funk (802.11) und Kabel (802.3); viele APs werden über einen **WLAN-Controller** verwaltet.
- Prüfungsfragen: „Ordnen Sie zu“, „Aufgaben eines Routers mit Firewall“ (2026S), „AP vs. WLAN-Controller“ (2021W).

## Kernwissen

### Übersicht

| Gerät | OSI | Entscheidet nach | Kollisionsdomäne | Broadcast-Domäne | Aufgabe |
|---|---:|---|---|---|---|
| Repeater | 1 | – | – | – | Signal verstärken, Reichweite verlängern |
| Hub | 1 | – | **eine** für alle Ports | eine | Multiport-Repeater, sendet alles an alle (veraltet) |
| Medienkonverter | 1 | – | – | – | Kupfer ↔ Glasfaser |
| Bridge | 2 | MAC | je Port | eine | zwei Segmente koppeln |
| Switch | 2 | MAC | je Port | eine pro VLAN | gezielte Weiterleitung, VLANs |
| Access Point | 2 | MAC | Funkzelle (geteilt) | wie angebundenes VLAN | WLAN ↔ LAN |
| WLAN-Controller | 2–7 | – | – | – | zentrale AP-Verwaltung |
| Layer-3-Switch | 2 + 3 | MAC + IP | je Port | je VLAN/SVI | Switching + Inter-VLAN-Routing |
| Router | 3 | IP | je Interface | **je Interface** | Netze verbinden, Routing, oft NAT, VPN |
| Firewall (Paketfilter/SPI) | 3–4 | IP, Port, Zustand | – | – | Verkehr nach Regeln filtern |
| NGFW / Proxy | bis 7 | Anwendung, URL, Inhalt | – | – | Anwendungserkennung, TLS-Inspection, Caching |
| Modem / ONT | 1–2 | – | – | – | Signal der Leitung (DSL, Kabel, Glasfaser) umsetzen |

### Router vs. Layer-3-Switch

| | Router | Layer-3-Switch |
|---|---|---|
| Ports | wenige, verschiedene Medien (WAN, LTE, DSL, SFP) | viele Ethernet-Ports |
| Routing | Software/CPU, flexibel | Hardware (ASIC), sehr schnell |
| Funktionen | NAT/PAT, VPN, Firewall, dynamisches Routing, QoS | Inter-VLAN-Routing, DHCP-Relay, ACLs, teils OSPF |
| Einsatz | Netzgrenze, Internet, Standortkopplung | Core/Distribution im LAN |

### Aufgaben eines Routers mit Firewall (GA2 Sommer 2026, „nenne drei“)

- **Routing** zwischen LAN, DMZ und Internet.
- **NAT/PAT**: private Adressen auf die öffentliche IP übersetzen, Portweiterleitung.
- **Paketfilter/SPI**: Verbindungen nach Regeln erlauben oder verwerfen.
- **VPN-Gateway** (Site-to-Site, Remote Access).
- Dienste wie **DHCP-Server/-Relay, DNS-Forwarder, NTP**.

### Access Point vs. WLAN-Controller (GA2 Winter 2021)

| Access Point | WLAN-Controller |
|---|---|
| Funkschnittstelle, sendet Beacons/SSID | konfiguriert alle APs zentral (SSIDs, VLANs, Security) |
| verschlüsselt die Funkstrecke, bridged ins LAN | Kanal- und Sendeleistungsplanung, Roaming-Steuerung |
| leitet 802.1X-Anfragen an RADIUS weiter | Firmware-Verteilung, Monitoring, Rogue-AP-Erkennung |

### Weitere Begriffe

| Begriff | Bedeutung |
|---|---|
| **Gateway** | allgemein: Übergang zwischen Netzen/Protokollen; im IP-Kontext = Default-Gateway (Router) |
| **Load Balancer** | verteilt Anfragen auf mehrere Server (Layer 4 oder 7) |
| **Patchfeld** | passive Rangierstelle der Gebäudeverkabelung (kein aktives Gerät) |
| **TAP** | passives Messgerät, kopiert Verkehr einer Leitung für Analysen |
| **PoE-Injektor** | speist Strom ins Kabel ein, wenn der Switch kein PoE hat |

## Prüfungsfrage

**Frage:** In einem Altnetz hängen 16 PCs an einem Hub, der über einen Router mit dem Server-Netz verbunden ist. Der Hub soll durch einen Switch ersetzt werden. Erläutern Sie den Unterschied und die Auswirkung auf Kollisions- und Broadcast-Domänen.

**Musterantwort:** Ein Hub arbeitet auf Schicht 1 und gibt jedes Signal an alle Ports weiter. Alle 16 PCs teilen sich eine Kollisionsdomäne und die Bandbreite im Halbduplex; jeder Rechner kann den Verkehr der anderen mitlesen. Ein Switch arbeitet auf Schicht 2, lernt die MAC-Adressen und leitet Frames nur an den Zielport weiter. Jeder Port wird zu einer eigenen Kollisionsdomäne (16 statt 1), Vollduplex ist möglich und Mitlesen deutlich erschwert. Die Broadcast-Domäne bleibt **eine** – die trennt erst ein Router oder eine VLAN-Aufteilung.

## Falle

- Switch trennt Broadcast-Domänen → nur mit VLANs; **Router** trennt sie immer.
- Access Point als Router bezeichnen → ein AP ist eine **Bridge**; der Heim-„WLAN-Router“ ist ein Kombigerät (Router + Switch + AP + Modem).
- L3-Switch mit Firewall gleichsetzen → ACLs sind zustandslos, keine SPI/NGFW.
- Modem mit Router verwechseln → Modem setzt nur das Leitungssignal um.

## Merksatz

**„Hub schreit, Switch flüstert, Router vermittelt.“** – Schicht 1, 2, 3.

## Verwandt

- [OSI](../OSI-IP-mac-bit-byte/osi.md) – Schichten und Geräte
- [Switches – Grundlagen](switches-und-vlans.md) – Switch im Detail
- [Routing](routing.md) – was der Router entscheidet
- [WLAN](wlan.md) – AP, Controller, SSID
- [Firewall](../Sicherheit/firewall/firewall.md) – Paketfilter, SPI, NGFW
- [Proxy](proxy.md) – Forward- und Reverse-Proxy
- [Load Balancer](../Server/load-balancer.md) – Lastverteilung
- [Verkabelung](verkabelung-kupfer-lwl.md) – Medien und Stecker
