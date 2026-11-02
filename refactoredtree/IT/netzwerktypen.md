# Netzwerktypen

## Netzwerktypen nach geografischer Ausdehnung (IHK)

| Typ | Vollständiger Name | Ausdehnung | Verbindung / Beispiel | Typische Sicherheitsrisiken |
|---|---|---|---|---|
| PAN | Personal Area Network | Persönlicher Nahbereich | Bluetooth, USB-Tethering | Bluetooth-Hijacking, Bluejacking, unbefugtes Pairing |
| LAN | Local Area Network | Raum, Gebäude oder Gelände | Ethernet, WLAN, Switch | ARP Spoofing, MAC Spoofing, Rogue DHCP, Sniffing |
| CAN | Campus Area Network | Mehrere Gebäude eines Standorts | Glasfaser zwischen Gebäuden, VLANs | VLAN Hopping, unbefugter Netzwerkzugriff, Malware-Ausbreitung |
| MAN | Metropolitan Area Network | Stadt oder Ballungsraum | Metro-Ethernet, Glasfaser, Provider-Netze | DDoS, Routing-Angriffe, Abhören unverschlüsselter Daten |
| WAN | Wide Area Network | Länder und Kontinente | MPLS, SD-WAN, Site-to-Site-VPN | DDoS, MitM, VPN-Angriffe, Routing-Manipulation |
| GAN | Global Area Network | Weltweite Vernetzung | Globale Unternehmensnetze, Satellitenverbindungen | DDoS, kompromittierte Endpunkte, globale Spionageangriffe |

## IHK-Prüfungswissen

- **PAN:** Verbindet persönliche Geräte über kurze Distanzen. Bluetooth sollte nur bei Bedarf aktiviert und unbekanntes Pairing abgelehnt werden.
- **LAN:** Verbindet Geräte innerhalb eines lokalen Netzes. VLANs segmentieren das Netz; Firewalls und 802.1X kontrollieren Zugriffe.
- **CAN:** Verbindet mehrere LANs auf einem Campus. VLANs, ACLs und Netzsegmentierung begrenzen die Ausbreitung von Angriffen.
- **MAN:** Verbindet Standorte innerhalb einer Stadt oder Region. Verschlüsselung und Provider-Sicherheitsmechanismen schützen die Datenübertragung.
- **WAN:** Verbindet geografisch entfernte Netzwerke. Site-to-Site-VPNs mit IPsec schützen die Kommunikation über öffentliche Netze.
- **GAN:** Ermöglicht globale Vernetzung. Zentrale Sicherheitsrichtlinien, MFA, Monitoring und Zero-Trust-Konzepte reduzieren Risiken.

## Verbindungsarten und Protokolle

| Technologie | Zweck | Typischer Bezug |
|---|---|---|
| Bluetooth | Kurzstreckenkommunikation | PAN |
| Ethernet (IEEE 802.3) | Kabelgebundenes LAN | LAN/CAN |
| WLAN (IEEE 802.11) | Drahtloses LAN | LAN |
| Glasfaser | Hohe Bandbreite über größere Distanzen | CAN/MAN/WAN |
| MPLS | Provider-basiertes Weiterleiten von Datenverkehr | WAN |
| IPsec VPN | Verschlüsselter Tunnel über IP-Netze | WAN |
| SD-WAN | Zentrale, softwarebasierte Steuerung von WAN-Verbindungen | WAN/GAN |

## Typische Angriffe und Schutzmaßnahmen

| Angriff | Betroffene Ebene / Bereich | Schutzmaßnahme |
|---|---|---|
| Bluejacking | PAN / Bluetooth | Unbekannte Anfragen ablehnen, Sichtbarkeit deaktivieren |
| ARP Spoofing | LAN / OSI 2 | Dynamic ARP Inspection, DHCP Snooping |
| Rogue Access Point | WLAN / LAN | WPA3, 802.1X, Rogue-AP-Erkennung |
| VLAN Hopping | LAN/CAN / OSI 2 | Switchports korrekt konfigurieren, ungenutzte Ports deaktivieren |
| Man-in-the-Middle | LAN/WAN | TLS, Zertifikatsprüfung, IPsec VPN |
| DDoS | LAN/MAN/WAN/GAN | DDoS-Schutz, Rate Limiting, Traffic-Filterung |
| Routing-Manipulation | MAN/WAN/GAN | Routing-Filter, sichere Protokollkonfiguration, Monitoring |

## IHK-Merksätze

- **PAN bis GAN:** Die geografische Reichweite nimmt grundsätzlich zu.
- **VPN:** Schützt die Vertraulichkeit und Integrität der Datenübertragung, sofern korrekt konfiguriert.
- **VLAN:** Trennt Broadcast-Domänen, ist aber allein keine vollständige Sicherheitsmaßnahme.
- **Wichtig:** Die Netzwerkgröße bestimmt nicht automatisch die Angriffsart. Angriffe hängen von Architektur, Protokollen und Sicherheitskonfiguration ab.

## Verwandt

- [OSI](osi.md) – Schichtenmodell als Grundlage
- [MAC](mac.md) – Adressierung im LAN
- [Firewall](firewall.md) – Trennung zwischen Netzen
- [Proxy](proxy.md) – kontrollierter Zugriff nach außen
- [IPv6](ipv6.md) – Adressierung im WAN/LAN

