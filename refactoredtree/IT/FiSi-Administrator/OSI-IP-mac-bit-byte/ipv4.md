# IPv4

## In 30 Sekunden

- IPv4 ist **32 Bit**, vier Oktette (Punkt-Dezimal), aufgeteilt in **Netz- und Hostanteil** über die Subnetzmaske bzw. den Präfix.
- Nutzbare Hosts: **2^Hostbits − 2** – Netz- und Broadcastadresse zählen nicht.
- Privat nach RFC 1918: `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16` – im Internet nicht routbar, daher **NAT/PAT** nötig.
- Sonderbereiche erkennen: `127.0.0.0/8` Loopback, `169.254.0.0/16` **APIPA**, `100.64.0.0/10` **CGNAT**, `224.0.0.0/4` **Multicast**.
- Ein Host sendet an ein fremdes Netz immer über sein **Default-Gateway**; im eigenen Netz direkt (nach ARP).

## Kernwissen

### Aufbau und Begriffe

| Begriff | Bedeutung | Beispiel |
|---|---|---|
| Adresse | 32 Bit, 4 × 8 Bit | `192.168.1.10` |
| Präfix (CIDR) | Anzahl Netzbits | `/26` |
| Subnetzmaske | Netzbits = 1, Hostbits = 0 | `255.255.255.192` |
| Netz-ID | alle Hostbits = 0 | `192.168.1.0` |
| Broadcast | alle Hostbits = 1 | `192.168.1.63` |
| Default-Gateway | Router-IP im eigenen Netz | meist erste oder letzte Host-IP |

Rechenwege (Blockgröße, VLSM) stehen ausführlich in [Subnetting](subnetting-ipv4-ipv6.md).

### Adressbereiche, die man erkennen muss

| Bereich | Bedeutung | Prüfungsbezug |
|---|---|---|
| `10.0.0.0/8` | privat (RFC 1918), 16.777.216 Adressen | 2025W, 2026S |
| `172.16.0.0/12` | privat, **172.16.0.0 – 172.31.255.255** | 2024W, 2026S |
| `192.168.0.0/16` | privat, 65.536 Adressen | 2024W, 2026S |
| `100.64.0.0/10` | Carrier-Grade NAT (Provider-intern) | 2023W |
| `127.0.0.0/8` | Loopback (`127.0.0.1` = localhost) | – |
| `169.254.0.0/16` | APIPA / Link-Local – **kein DHCP erreicht** | 2023S, 2024S, 2024W |
| `224.0.0.0 – 239.255.255.255` | Multicast (ehem. Klasse D, Bitmuster `1110`) | 2024S |
| `255.255.255.255` | Limited Broadcast (nur lokales Segment) | DHCP-Discover |
| `192.0.2.0/24`, `198.51.100.0/24`, `203.0.113.0/24` | Dokumentationsnetze (TEST-NET) – in IHK-Aufgaben als „öffentliche“ IPs | fast jede GA2 |

**Historische Klassen** (nur noch zur Einordnung): A `0–127` (/8), B `128–191` (/16), C `192–223` (/24), D `224–239` (Multicast), E `240–255` (reserviert). Heute gilt **klassenlos (CIDR)**.

### Übertragungsarten (GA2 Sommer 2024, Sommer 2026)

| Art | Empfänger | Beispiel | Layer-2-Ziel |
|---|---|---|---|
| **Unicast** | genau einer | Webseite abrufen | MAC des Ziels/Gateways |
| **Multicast** | Gruppe | IPTV-Livestream, OSPF, VRRP (`224.0.0.18`) | `01:00:5E:…` |
| **Broadcast** | alle im Subnetz | ARP-Request, DHCP-Discover | `FF:FF:FF:FF:FF:FF` |
| **Anycast** | „nächster“ von mehreren | DNS-Root-Server, CDNs | – |

**Layer 3 vs. Layer 2 Broadcast (2026S):** Ein IP-Broadcast (z. B. `10.0.255.255`) wird auf Layer 2 in einen Ethernet-Frame mit Ziel-MAC `FF:FF:FF:FF:FF:FF` gekapselt. Alle Geräte der **Broadcast-Domäne** verarbeiten ihn; **Router leiten Broadcasts nicht weiter**.

### Entscheidung „lokal oder Gateway?“

1. Host verknüpft eigene IP und Ziel-IP jeweils per UND mit seiner Maske.
2. Gleiche Netz-ID → Ziel ist lokal → ARP nach der Ziel-MAC.
3. Unterschiedlich → Paket an das **Default-Gateway** → ARP nach der Gateway-MAC.
4. Die Ziel-IP im Paket bleibt die des Endziels; nur die Ziel-MAC ist die des Gateways.

### IPv4-Header – Felder, die vorkommen

| Feld | Bedeutung |
|---|---|
| Version | 4 |
| TTL | wird pro Router um 1 verringert; bei 0 verworfen (ICMP Time Exceeded) |
| Protocol | 1 = ICMP, 6 = TCP, 17 = UDP |
| Header Checksum | wird von jedem Router neu berechnet (TTL ändert sich) |
| Source / Destination IP | Quell- und Zieladresse (NAT ändert sie) |

**Hop-Rechnung (2022W, 2023S):** Start-TTL (Windows 128, Linux 64) − empfangene TTL = Anzahl Router. 128 − 118 = **10 Hops**.

## Prüfungsfrage

**Frage (angelehnt an GA2 Sommer 2024):** Client `172.16.10.1/16` ruft den Webserver `10.10.0.10` in der DMZ auf. Erläutern Sie, woher der Client weiß, dass er das Default-Gateway nutzen muss, und welches Protokoll er vorher benötigt.

**Musterantwort:** Der Client vergleicht die Netz-ID seiner Adresse (`172.16.0.0/16`) mit der Netz-ID des Ziels. `10.10.0.10` liegt nicht in `172.16.0.0/16`, also sendet er das Paket an sein Default-Gateway. Dafür benötigt er dessen MAC-Adresse und ermittelt sie per **ARP**: Er sendet einen ARP-Request als Broadcast („Wer hat 172.16.0.254?“), das Gateway antwortet per Unicast mit seiner MAC. Das Frame geht an die Gateway-MAC, die Ziel-IP bleibt `10.10.0.10`.

## Falle

- `172.32.x.x` als privat einordnen → falsch, privat endet bei **172.31**.
- `169.254.x.x` als „falsch konfigurierte statische IP“ deuten → es ist **APIPA**: DHCP aktiv, aber kein Server erreicht.
- `100.64.x.x` ist weder RFC 1918 noch öffentlich nutzbar → CGNAT des Providers.
- Netz- und Broadcastadresse als Hosts mitzählen → eine Antwort zu viel.
- „Router leitet Broadcasts weiter“ → nein, genau deshalb braucht DHCP über Netzgrenzen ein **Relay**.

## Merksatz

**„10, 172.16 bis 31, 192.168 – alles andere ist öffentlich oder speziell.“** Fremdes Netz = Gateway, eigenes Netz = direkt.

## Verwandt

- [Subnetting IPv4 & IPv6](subnetting-ipv4-ipv6.md) – alle Rechenwege und VLSM
- [ARP](arp.md) – MAC zur IP ermitteln
- [IPv6](ipv6.md) – 128 Bit, kein Broadcast
- [NAT & PAT](../Sicherheit/firewall/nat-pat.md) – private Adressen ins Internet
- [DHCP](dhcp.md) – automatische Adressvergabe, APIPA
- [Routing](../Netzwerk-Administration/routing.md) – wohin das Gateway weiterleitet
- [OSI](osi.md) – Einordnung auf Schicht 3
- [Lektion 06 – IPv4 & Routing](../../analyse-der-exams/06_IPv4_Adressierung_Routing.md)
