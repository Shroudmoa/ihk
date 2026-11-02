## 1. Aufgabe (Netzwerk-Analyse)

### aa) Subnetzmasken & max. nutzbare IP-Adressen
| VLAN | Name | Subnetz | Maske (dezimal) | Max. nutzbare IP-Adressen |
|---|---|---|---|---|
| 1 | default | 192.168.1.0/24 | 255.255.255.0 | 254 |
| 11 | Technik | 192.168.11.0/26 | 255.255.255.192 | 62 |
| 12 | Vertrieb | 192.168.12.128/25 | 255.255.255.128 | 126 |
| 66 | Management | 192.168.66.8/29 | 255.255.255.248 | 6 |
| Filial-Netze | - | 172.16.200.0/22 | 255.255.252.0 | 1022 |

- **Logik:** Nutzbare Hosts = 2^(32−Präfix) − 2 (Netz-ID + Broadcast).
- **/24:** 2^8 − 2 = 254. **/26:** 2^6 − 2 = 62. **/25:** 2^7 − 2 = 126. **/29:** 2^3 − 2 = 6. **/22:** 2^10 − 2 = 1022.

### ab) Gateway-IPs (jeweils letzte IP im Subnetz)
| VLAN | Name | Subnetz | Gateway-IP |
|---|---|---|---|
| 1 | default | 192.168.1.0/24 | 192.168.1.254 |
| 11 | Technik | 192.168.11.0/26 | 192.168.11.62 |
| 12 | Vertrieb | 192.168.12.128/25 | 192.168.12.254 |
| 66 | Management | 192.168.66.8/29 | 192.168.66.14 |

### ac) Subinterface – Erläuterung
- **Definition:** Logische, virtuelle Schnittstelle auf einem physischen Interface (z.B. `eth1.11`). Mehrere Subinterfaces auf einem Port ermöglichen **VLAN-Trunking** (802.1Q): Jedes Subinterface ist einem VLAN zugeordnet und trägt das VLAN-Tag.
- **Grund:** R1 hat nur eine physische Verbindung zum Switch, muss aber Gateway für mehrere VLANs sein. Subinterfaces erlauben **Inter-VLAN-Routing (Router-on-a-Stick)**.

### ba) Link Aggregation – Begriff & Vorteile
- **Begriff:** Bündelung mehrerer physischer Leitungen zu einem logischen Link (IEEE 802.3ad / LACP). Lastverteilung über alle Mitglieder.
- **Vorteil 1 – Höhere Bandbreite:** Zwei 1-Gbit/s-Links = 2 Gbit/s aggregiert (kein Flaschenhals zwischen Switches).
- **Vorteil 2 – Redundanz:** Fällt eine Leitung aus, läuft der Traffic über die verbleibende(n) weiter. Kein Ausfall, nur reduzierte Bandbreite.

### bb) MLS1-Port zu MLS2 geblockt – Grund
- **Ursache:** **Spanning Tree Protocol (STP)**. Zwischen MLS1, MLS2, MLS3 und MLS4 existieren redundante Pfade (Schleifen). STP blockiert einen Port (Blocking-State), um Layer-2-Broadcast-Stürme zu verhindern. Der geblockte Port ist funktionsfähig, aber logisch deaktiviert, bis die aktive Leitung ausfällt.

### c) Default-Route in R1
| Netzwerk | Schnittstelle | Next-Hop |
|---|---|---|
| 0.0.0.0/0 | eth0 | 203.0.113.89 (ISP-Gateway) |

- **Logik:** Ohne Default-Route kennt R1 kein Ziel für unbekannte Netze (Internet). Next-Hop = ISP-Router im /30-Netz (203.0.113.89).

---

## 2. Aufgabe (Internet-Anbindung & IPv6)

### aa) Geschäfts-DSL vs. Privat-DSL (3)
- **SLA/Garantien:** Garantierte Bandbreite, Verfügbarkeit (z.B. 99,5%) und Entstörzeiten (z.B. 8h) – Privatkunden haben „Best Effort“.
- **Feste öffentliche IP:** Geschäftskunden erhalten statische IPv4/IPv6-Präfixe; Privatkunden meist dynamische IPs.
- **Symmetrie/Upload:** Höherer Upload (z.B. 40 Mbit/s) für VPN, Server, Cloud-Backup.
- **Support:** Business-Hotline, priorisierte Störungsbearbeitung.

### ab) Shared Medium (GPON) – Problem
- **Definition:** Die Glasfaser wird von mehreren Teilnehmern geteilt (PON: Passive Optical Network). Bandbreite wird aufgesplittet (z.B. 1:32).
- **Problem:** Bei hoher Last anderer Teilnehmer sinkt die verfügbare Bandbreite für die Filiale. Keine garantierte Performance; Latenz steigt. Ungünstig für geschäftskritische Anwendungen (POS, VPN).

### ac) LTE als Übergangslösung – 2 Aspekte
- **Datenvolumen/Limit:** LTE-Tarife haben oft Volumenbegrenzung (z.B. 50 GB); nach Verbrauch Drosselung. Nicht für dauerhaften Filialbetrieb geeignet.
- **Latenz/Jitter:** Mobilfunk hat höhere, schwankende Latenz als DSL/Glasfaser; problematisch für POS-Terminals und VPN.
- **Zuverlässigkeit:** Funkloch/Wetterabhängigkeit; keine garantierte Bandbreite (Shared Medium).

### ba) Viele Geräte über eine öffentliche IPv4 – NAT/PAT
- **Prinzip:** **NAT (Network Address Translation)** mit **PAT (Port Address Translation)**. Der Router übersetzt private Quell-IPs (z.B. 192.168.x.x) in die eine öffentliche IP und vergibt eindeutige Quell-Ports. Antworten werden anhand der Port-Tabelle zurückübersetzt.
- **Ergebnis:** Bis zu 65.535 gleichzeitige Verbindungen pro öffentlicher IP möglich.

### bb) IPv6-Subnetze aus 2001:db8:9876::/56
- **/56 → /64:** 8 Bits für Subnetze → 2^8 = **256 Subnetze**.
- **1. Subnetz:** 2001:db8:9876:0000::/64 → **2001:db8:9876::/64**
- **2. Subnetz:** 2001:db8:9876:0001::/64
- **Vorletztes (255.):** 2001:db8:9876:00fe::/64
- **Letztes (256.):** 2001:db8:9876:00ff::/64

### bc) DMZ – 2 Eigenschaften
- **Isolation:** Pufferzone zwischen Internet und internem LAN. Externe Dienste (Web, Mail) stehen in der DMZ; bei Kompromittierung bleibt das LAN geschützt.
- **Kontrollierter Zugriff:** Firewall-Regeln erlauben nur definierte Ports/Protokolle von außen in die DMZ; kein direkter Zugriff ins LAN.

### bd) IP-Adressen in /64-DMZ
- **2^64 ≈ 18,4 Trillionen** Adressen (genauer: 18.446.744.073.709.551.616). Praktisch unbegrenzt.

### ca) Rekursive DNS-Auflösung (Client → Router)
- **Prinzip:** Client fragt seinen konfigurierten DNS-Resolver (Router). Der Router übernimmt die vollständige Auflösung (iterativ) und liefert dem Client das Endergebnis (IP-Adresse). Client muss keine weiteren Server fragen.

### cb) Iterative DNS-Auflösung (Router)
1. **Root-Server:** Router fragt Root (`.`), erhält Verweis auf `.de`-TLD-Server.
2. **TLD-Server (.de):** Fragt `.de`-Server, erhält Verweis auf autoritativen Nameserver für `ihk.de`.
3. **Autoritativer Server (ihk.de):** Fragt Nameserver von `ihk.de`, erhält **A-Record** für `www.ihk.de` (IP-Adresse).
4. **Antwort an Client:** Router cached das Ergebnis und liefert es an den Client.

---

## 3. Aufgabe (Firewall & VPN)

### aa) Stateful Packet Inspection vs. Paketfilter
- **Paketfilter:** Prüft jedes Paket isoliert anhand von Regeln (IP, Port, Protokoll). Kennt keinen Verbindungszustand.
- **SPI:** Führt eine **State Table** (Verbindungstabelle). Erlaubt Rückverkehr dynamisch (z.B. Antwortpakete auf eine ausgehende Verbindung) ohne explizite Regel. Verhindert Spoofing/Port-Scanning effektiver.

### ab) SPI-Regeln
| Regel | Protokoll | Quell-IP | Ziel-IP | Quell-Port | Ziel-Port | Richtung |
|---|---|---|---|---|---|---|
| allow | TCP | any | any | any | 443 | out |
| allow | TCP | any | any | any | 587 | out |
| allow | TCP | any | any | any | 993 | out |
| allow | TCP | any | any | any | 4711 | out |
| allow | TCP | any | 192.0.2.64/32 | any | 443 | in |
| deny | any | any | any | any | any | out |
| deny | any | any | any | any | any | in |

- **Logik:** Ausgehend nur benötigte Ports; eingehend nur HTTPS zum Webserver; alles andere blockieren.

### ac) Next Generation Firewall – 2 Vorteile
- **Application-Layer-Erkennung (DPI):** Erkennt Anwendungen unabhängig vom Port (z.B. Facebook over HTTPS) und kann sie filtern/blockieren.
- **Intrusion Prevention System (IPS):** Erkennt und blockiert Angriffsmuster (Exploits, Malware) in Echtzeit; nicht nur Port-/IP-basiert.

### ba) VPN-Software-Auswahl
- **Empfehlung:** **VPN-Software 1 (X.509)**.
- **Begründung:** X.509-Zertifikate basieren auf einer **PKI/CA** (Chain of Trust). Skalierbar, widerrufbar (CRL/OCSP), automatisierbar (z.B. SCEP). Software 2 mit manuell hinterlegten Public Keys ist fehleranfällig (kein Widerruf, hoher Admin-Aufwand).

### bb) Hardware-Kriterien für VPN
- **Hardware-Krypto:** AES-NI/Offload beschleunigt Verschlüsselung; ohne Hardware-Support bricht der Durchsatz bei hoher VPN-Last ein.
- **Anzahl Netzwerkports:** Bestimmt Anbindung an LAN/WAN/DMZ; mindestens WAN + LAN + ggf. DMZ.
- **Firmware-Updates:** Garantierte Sicherheitsupdates schließen CVEs; ohne Updates ist die Firewall angreifbar.
- **Leistungsaufnahme:** Relevant für Kühlung/Energiekosten im 24/7-Betrieb; hohe Watt = höhere Betriebskosten.

---

## 4. Aufgabe (WLAN & IoT)

### aa) SSID-Broadcast – Funktionsweise & Zweck
- **Funktionsweise:** Der AP sendet periodisch **Beacon-Frames** mit der SSID, damit Clients das Netz finden und sich verbinden können.
- **Zweck:** Sichtbarkeit des WLANs für Endgeräte. Bei deaktiviertem Broadcast (Hidden SSID) sendet der AP keine SSID in Beacons; Clients müssen die SSID manuell eingeben. **Keine echte Sicherheitsmaßnahme** (SSID im Handshake sichtbar).

### ab) Authentifizierung vs. Vertraulichkeit (WLAN)
- **Authentifizierung:** Nachweis der Identität des Clients gegenüber dem AP (z.B. PSK, 802.1X/EAP, Zertifikat). Verhindert unbefugten Zugang.
- **Vertraulichkeit:** Verschlüsselung der Funkstrecke (z.B. AES-CCMP bei WPA2, GCMP bei WPA3). Verhindert Mitlesen (Sniffing).

### ac) WPA – Beispiele
- **Authentifizierung:** WPA2-Personal → PSK (Pre-Shared Key); WPA2-Enterprise → 802.1X/EAP mit RADIUS.
- **Verschlüsselung:** WPA2 → AES-CCMP; WPA3 → AES-GCMP (192 Bit).

### ba) DHCP-Discover als Broadcast – Grund
- **Prinzip:** Der Client hat noch keine IP-Adresse und kennt den DHCP-Server nicht. Broadcast (255.255.255.255) erreicht alle Geräte im L2-Segment. Der DHCP-Server antwortet per Unicast (Offer).
- **Grund:** Ohne IP keine gezielte Unicast-Kommunikation möglich; Broadcast ist der einzige Weg.

### bb) Gratuitous ARP – Sinn
- **Prinzip:** Der Client sendet einen ARP-Request für seine **eigene** IP-Adresse (ohne vorherige Anfrage).
- **Zweck 1:** **Konflikterkennung:** Prüfen, ob ein anderes Gerät dieselbe IP verwendet (Duplicate Address Detection).
- **Zweck 2:** **ARP-Cache-Update:** Andere Geräte aktualisieren ihren ARP-Cache mit der neuen MAC-IP-Zuordnung (z.B. nach Failover/VM-Migration).

### ca) MQTT auf TCP – Grund
- **Zuverlässigkeit:** TCP garantiert Reihenfolge, Vollständigkeit und Wiederholung verlorener Pakete (Retransmission). MQTT benötigt **Delivery Guarantees** (QoS 0/1/2). UDP bietet keine Zustellgarantie; bei Sensordaten wären Verluste kritisch.
- **Verbindungsorientierung:** MQTT nutzt persistente Sessions (CONNECT/CONNACK); TCP liefert die nötige Verbindungsorientierung.

### c) Bandbreitenberechnung IoT
- **Pro Gerät:** 66 Byte / 60 s = 1,1 Byte/s.
- **500 Geräte:** 500 × 1,1 = 550 Byte/s.
- **In Bit:** 550 × 8 = **4.400 bit/s = 4,4 kbit/s**.
- **Vergleich:** 4,4 kbit/s < 10 kbit/s → **Bandbreite ist ausreichend** (ca. 56% Reserve).

---

**Merksatz:** IHK-Aufgaben sind **Kausalitätsketten** – wer Subnetting (/26, /29), NAT/PAT, SPI-State-Table und MQTT-QoS nicht als Hard-Facts sitzen hat, verliert Punkte an der Oberfläche. Lerne Standards als Vokabeln, nicht als Prosa.
