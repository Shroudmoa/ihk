# Netzwerktypen und Topologien

## In 30 Sekunden

- Nach **Ausdehnung**: **PAN** (persönlicher Nahbereich) → **LAN** (Gebäude) → **CAN** (Campus) → **MAN** (Stadt) → **WAN** (Länder) → **GAN** (weltweit).
- Nach **Topologie**: heute fast immer **(erweiterter) Stern** im LAN, **Mesh** im WAN/Backbone; Bus und Ring sind historisch.
- Typische Technik: PAN = Bluetooth, LAN = Ethernet/WLAN, CAN = Glasfaser + VLANs, WAN = MPLS, SD-WAN, Site-to-Site-VPN.
- Die Netzgröße bestimmt **nicht** automatisch die Angriffsart – entscheidend sind Architektur, Protokolle und Konfiguration.
- Prüfungsbezug meist indirekt: Standortkopplung (VPN, WAN-Anschluss), Gebäudevernetzung (LWL), Segmentierung (VLAN, DMZ).

## Kernwissen

### Netzwerktypen nach Ausdehnung

| Typ | Name | Ausdehnung | Verbindung / Beispiel | Typische Risiken |
|---|---|---|---|---|
| PAN | Personal Area Network | wenige Meter | Bluetooth, USB-Tethering | unbefugtes Pairing, Bluejacking |
| LAN | Local Area Network | Raum, Gebäude | Ethernet, WLAN, Switches | ARP-Spoofing, Rogue DHCP, Sniffing |
| CAN | Campus Area Network | mehrere Gebäude eines Geländes | Glasfaser zwischen Gebäuden, VLANs | VLAN-Hopping, Ausbreitung von Malware |
| MAN | Metropolitan Area Network | Stadt, Region | Metro-Ethernet, Provider-Glasfaser | Abhören unverschlüsselter Strecken, DDoS |
| WAN | Wide Area Network | Länder, Kontinente | MPLS, SD-WAN, Site-to-Site-VPN, Internet | MitM, Routing-Manipulation, DDoS |
| GAN | Global Area Network | weltweit | globale Unternehmensnetze, Satellit | kompromittierte Endpunkte, Spionage |

### Topologien

| Topologie | Aufbau | Vorteil | Nachteil |
|---|---|---|---|
| **Stern** | alle Geräte an einem zentralen Switch | einfache Fehlersuche, Ausfall eines Kabels betrifft nur ein Gerät | zentraler Switch = Single Point of Failure |
| Erweiterter Stern / Baum | Core → Distribution → Access | skalierbar, strukturiert | Redundanz muss geplant werden (STP, LACP) |
| Bus | gemeinsame Leitung (Koax) | wenig Kabel | ein Fehler legt alles lahm – veraltet |
| Ring | jeder mit zwei Nachbarn | deterministisch | Unterbrechung ohne Redundanzring = Ausfall |
| **Vermascht (Mesh)** | mehrere Wege zwischen Knoten | hohe Ausfallsicherheit | teuer, komplex (Routing nötig) – WAN, Internet, WLAN-Mesh |

### Verbindungsarten

| Technologie | Zweck | Typischer Bezug |
|---|---|---|
| Bluetooth | Kurzstrecke | PAN |
| Ethernet (IEEE 802.3) | kabelgebundenes LAN | LAN/CAN |
| WLAN (IEEE 802.11) | drahtloses LAN | LAN |
| Glasfaser | hohe Bandbreite, große Distanz | CAN/MAN/WAN |
| MPLS | Provider-Netz mit Label-Switching, QoS | WAN |
| IPsec-VPN | verschlüsselter Tunnel über das Internet | WAN |
| SD-WAN | softwaregesteuerte Nutzung mehrerer WAN-Leitungen | WAN/GAN |

### Angriffe und Schutzmaßnahmen

| Angriff | Bereich | Schutz |
|---|---|---|
| Bluejacking | PAN | Sichtbarkeit aus, unbekannte Anfragen ablehnen |
| ARP-Spoofing | LAN, Schicht 2 | Dynamic ARP Inspection, DHCP-Snooping |
| Rogue Access Point | WLAN | WPA3/802.1X, Rogue-AP-Erkennung am Controller |
| VLAN-Hopping | LAN/CAN | Trunks gezielt, natives VLAN ungenutzt, ungenutzte Ports aus |
| Man-in-the-Middle | LAN/WAN | TLS mit Zertifikatsprüfung, IPsec |
| DDoS | MAN/WAN/GAN | DDoS-Schutz des Providers, Rate Limiting |
| Routing-Manipulation | WAN | Routing-Filter, Authentifizierung der Routing-Protokolle, Monitoring |

## Prüfungsfrage

**Frage:** Ein Unternehmen hat eine Zentrale mit drei Gebäuden auf einem Gelände und zwei Filialen in anderen Städten. Ordnen Sie die Vernetzung den Netzwerktypen zu und schlagen Sie jeweils eine geeignete Übertragungstechnik vor.

**Musterantwort:** Die Vernetzung innerhalb eines Gebäudes ist ein **LAN** (Ethernet mit Switches in Sterntopologie, WLAN). Die drei Gebäude auf dem Gelände bilden zusammen ein **CAN**; sie werden über **Glasfaser** (Singlemode) miteinander verbunden, da Kupfer auf 100 m begrenzt ist und LWL galvanisch trennt. Die Filialen in anderen Städten werden über ein **WAN** angebunden, z. B. per **Site-to-Site-VPN (IPsec)** über Internetanschlüsse oder per MPLS bzw. SD-WAN des Providers.

## Falle

- CAN und MAN verwechseln: **Campus** = ein Gelände eines Betreibers, **Metropolitan** = Stadtgebiet.
- „VLAN macht das Netz sicher“ → VLAN segmentiert, Sicherheit entsteht erst durch Regeln am Übergang.
- Stern als ausfallsicher bezeichnen → der zentrale Switch ist ein Single Point of Failure.
- Internet als „GAN des Unternehmens“ bezeichnen → das Internet ist ein öffentliches WAN; ein GAN ist ein weltweites Netz eines Betreibers/Unternehmens.

## Merksatz

**„Person – Local – Campus – Metropolitan – Wide – Global: von der Hosentasche bis um die Welt.“**

## Verwandt

- [OSI](OSI-IP-mac-bit-byte/osi.md) – Schichtenmodell als Grundlage
- [Netzwerkgeräte](Netzwerk-Administration/netzwerkgeraete.md) – was in welchem Netz steht
- [Verkabelung](Netzwerk-Administration/verkabelung-kupfer-lwl.md) – Kupfer im LAN, Glasfaser im CAN
- [WAN-Anschlussarten](Netzwerk-Administration/wan-anschlussarten.md) – DSL, Kabel, Glasfaser, LTE
- [VLAN](Netzwerk-Administration/vlan.md) – Segmentierung im LAN/CAN
- [VPN](../nice-to-know/VPN/readme.md) – Standortkopplung im WAN
- [Firewall](Sicherheit/firewall/firewall.md) – Trennung zwischen Netzen
- [Proxy](Netzwerk-Administration/proxy.md) – kontrollierter Zugriff nach außen
- [IPv6](OSI-IP-mac-bit-byte/ipv6.md) – Adressierung im LAN/WAN
