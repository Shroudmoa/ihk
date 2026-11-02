## 1. Aufgabe (RZ-Anbindung & DNS)

### aa) Netzadresse & Broadcast (10.0.0.0/16)
- **Netzadresse:** 10.0.0.0
- **Broadcast-Adresse:** 10.0.255.255

### ab) Broadcast (Layer 3) & Layer-2-Realisierung
- **Aufgabe Layer 3:** Ein Paket an die Broadcast-IP wird an **alle Hosts** im selben Subnetz zugestellt. Wird z.B. für ARP oder DHCP-Discover genutzt.
- **Layer-2-Realisierung:** Broadcast-IP wird in einen **Ethernet-Broadcast-Frame** mit Ziel-MAC `FF:FF:FF:FF:FF:FF` gekapselt. Alle Geräte im selben L2-Segment (Broadcast-Domäne) empfangen und verarbeiten das Frame; Router leiten es nicht weiter.

### a (Fortsetzung) RFC 1918 – zwei weitere private Bereiche
| Netzadresse und CIDR | Broadcast-Adresse |
|---|---|
| 172.16.0.0/12 | 172.31.255.255 |
| 192.168.0.0/16 | 192.168.255.255 |

- **Hinweis:** 10.0.0.0/8 ist bereits im Einsatz. Zweiter Bereich: 172.16.0.0/12 (1.048.576 Adressen), dritter: 192.168.0.0/16 (65.536 Adressen).

### ba) Zwei Internet-Anbindungen – Grund
- **Redundanz/Ausfallsicherheit:** Fällt Provider 1 (LWL) aus, übernimmt Provider 2 (DSL). Verhindert Single Point of Failure; erhöht Verfügbarkeit.
- **Lastverteilung/Performance:** Traffic kann auf beide Leitungen verteilt werden (Load Balancing).

### bb) FHRP (VRRP) – Aufgabe & Funktionsweise
- **Aufgabe:** Stellt ein **virtuelles Standard-Gateway** (VRRP-IP 10.0.99/16) für die Hosts bereit. Fällt der aktive Router aus, übernimmt der Standby-Router die virtuelle IP und MAC.
- **Funktionsweise:** Router bilden eine VRRP-Gruppe mit Prioritäten. Master sendet periodisch Advertisements; bleibt dies aus, wird der Backup zum Master. Hosts merken nichts vom Failover (transparent).

### bc) Standard-Gateway für Server
- **IP-Adresse:** **10.0.0.99** (die VRRP-virtuelle IP). Nicht die physischen Router-IPs (10.0.0.1 oder 10.0.0.2), da diese bei Ausfall wegbrechen.

### c) Split-Horizon DNS
- **a) Funktion:** Ein DNS-Name (z.B. shop.bookworm.de) liefert **unterschiedliche IP-Adressen** je nach Quelle der Anfrage. Interne Clients erhalten die private IP (z.B. 10.0.1.10); externe Clients die öffentliche IP (z.B. 203.0.113.10). Realisiert durch getrennte Zonen/Views auf dem DNS-Server.
- **b) Auswirkungen:**
  - **Admin-Team (intern):** Erhält die private IP; Zugriff direkt über LAN, geringe Latenz, kein Umweg über Firewall/NAT.
  - **Kunde (extern):** Erhält die öffentliche IP; Zugriff über Internet/Firewall, ggf. höhere Latenz.
- **c) Reverse Proxy vs. Port-Weiterleitung:**
  - **Reverse Proxy:** Vermittelt eingehende Anfragen an Backend-Server. Terminiert TLS, cached Inhalte, verteilt Last, filtert (WAF), verbirgt Backend-IPs.
  - **Port-Weiterleitung (DNAT):** Leitet nur Pakete 1:1 an einen internen Host weiter. Keine TLS-Terminierung, kein Caching, keine Filterung. Backend ist direkter Angriffspunkt.
- **d) Dual-Stack:** Paralleler Betrieb von **IPv4 und IPv6** auf derselben Infrastruktur. Hosts erhalten beide Adressen; Anwendungen wählen das passende Protokoll (Happy Eyeballs). Kein NAT bei IPv6 nötig.

---

## 2. Aufgabe (Firewall)

### aa) Drei Aufgaben Router mit Firewall
- **Routing:** Verbindet LAN (192.168.100.0/24) mit WAN (Internet); leitet Pakete zwischen Netzen weiter.
- **NAT/PAT:** Übersetzt private LAN-IPs in die öffentliche IP (203.0.113.1) für Internetzugriff.
- **Paketfilterung/SPI:** Erlaubt/blockiert Traffic nach Regeln (Ports, IPs, Zustand); schützt vor unautorisiertem Zugriff.
- **DHCP/DNS-Relay:** Kann DHCP- oder DNS-Dienste für das LAN bereitstellen.

### ab) Zwei Bedrohungen ohne Firewall-Schutz
- **Phishing/Social Engineering:** Firewall filtert keine E-Mails mit bösartigen Links/Anhängen, die ein Benutzer öffnet.
- **Innentäter/Insider-Threat:** Firewall schützt nicht vor autorisierten Benutzern, die Daten missbrauchen (Data Leakage).
- **Malware auf Endgeräten:** Firewall blockiert keine bereits infizierten Clients im LAN (Ost-West-Traffic).
- **Physischer Diebstahl:** Kein Schutz vor gestohlenen Geräten/Datenträgern.

### ac) SPI vs. NGFW – OSI-Modell
- **SPI (Stateful Packet Inspection):** Arbeitet auf **Layer 3/4** (IP/Port). Führt eine State Table; erlaubt Rückverkehr dynamisch. Erkennt keine Anwendungen/Inhalte.
- **NGFW:** Arbeitet bis **Layer 7** (Application Layer). Deep Packet Inspection (DPI), Anwendungserkennung (z.B. Facebook over HTTPS), IPS, TLS-Interception, URL-Filtering, User-Identität.

### ba) Firewall-Regeln eth1 (WAN → LAN)
| Regel | Protokoll | Quell-IP | Ziel-IP | Quell-Port | Ziel-Port | Richtung |
|---|---|---|---|---|---|---|
| allow | tcp | any | 203.0.113.1/32 | any | 1194 | in |
| deny | any | any | any | any | any | in |

- **Logik:** Nur OpenVPN (UDP/TCP 1194) von außen auf die WAN-IP erlauben. Alles andere eingehend blockieren. Antworten auf ausgehende Verbindungen erlaubt SPI automatisch.

### bb) Firewall-Regeln eth0 (LAN → WAN)
| Regel | Protokoll | Quell-IP | Ziel-IP | Quell-Port | Ziel-Port | Richtung |
|---|---|---|---|---|---|---|
| allow | tcp | 192.168.100.0/24 | any | any | 80 | in (out) |
| allow | tcp | 192.168.100.0/24 | any | any | 443 | in (out) |
| allow | udp | 192.168.100.0/24 | 10.0.4.1/32 | any | 53 | in (out) |
| allow | tcp | 192.168.100.0/24 | 10.0.3.1/32 | any | 993 | in (out) |
| allow | tcp | 192.168.100.0/24 | 10.0.3.1/32 | any | 465 | in (out) |
| deny | any | any | any | any | any | in |

- **Logik:** HTTP/HTTPS ins Internet; DNS zum internen Nameserver (10.0.4.1); IMAPS (993) und SMTPS (465) zum Mailserver im RZ (10.0.3.1). Alles andere blockieren.

---

## 3. Aufgabe (MitM & ARP-Poisoning)

### a) MitM – Begriff & Gefahr
- **Begriff:** Man-in-the-Middle. Angreifer positioniert sich zwischen zwei Kommunikationspartnern und leitet Daten mit (oder manipuliert sie), ohne dass die Parteien es merken.
- **Gefahr:** Abgreifen von Zugangsdaten, Session-Cookies, Kreditkartendaten; Manipulation von Transaktionen; Einschleusen von Malware.

### ba) Monitoring-Port – Funktionsweise
- **Prinzip:** Ein Switch-Port wird so konfiguriert, dass er **eine Kopie des gesamten Traffics** anderer Ports (oder eines bestimmten VLANs) erhält. Auch „SPAN-Port“ (Switched Port Analyzer). Ermöglicht passives Mitschneiden ohne den Datenfluss zu stören.

### bb) ARP-Poisoning
- **Begriff:** Angreifer sendet gefälschte ARP-Antworten („IP X ist bei MAC Y“) an Opfer. Die Opfer aktualisieren ihren ARP-Cache und senden Traffic an die MAC des Angreifers statt an das echte Ziel. Ermöglicht MitM/Sniffing.

### bc) Verdacht anhand Paketmitschnitt (Anlage 1)
- **Analyse:** Mehrere ARP-Antworten („is at“) von **derselben MAC-Adresse** für **unterschiedliche IP-Adressen** (z.B. 192.168.12.6 und 192.168.12.254). Ein einzelnes Gerät gibt sich als Gateway und als anderer Host aus.
- **Verdacht:** ARP-Poisoning für **192.168.12.254** (Gateway) und **192.168.12.6**. Der Angreifer mappt mehrere IPs auf seine MAC, um Traffic abzugreifen.

---

## 4. Aufgabe (Netzwerk-Support & DHCP)

### aa) DNS-Auflösung – Zeilennummern
- **Anfrage:** Zeile 1 (Standard query 0xa25c A ihk.de).
- **Antwort:** Zeile 2 (Standard query response 0xa25c A ihk.de A 51.38.104.145).

### ab) IPv6-Adressen (DNS & Ziel)
- **Genutzter DNS-Server:** 2001:db8::1 (aus dem Mitschnitt – Source/Destination der DNS-Query).
- **Angepingte Adresse (Ziel):** 2a02:26f0:1300:19a::356e (Akamai; aus ICMPv6 Echo Request/Reply).

### ac) Lokale und öffentliche IPv6 (Client)
- **Lokale IPv6 (Link-Local):** fe80::4257:a46c:5622:bb4 (aus ipconfig).
- **Öffentliche IPv6 (Global Unicast):** 2001:db8:381c:21:7642:7fff:fefc:62f8 (aus Mitschnitt, Source der Query).

### ba) DHCP-Parameter aus Mitschnitt (Anlage 2)
| Parameter | Wert |
|---|---|
| IP-Adresse | 192.168.5.101 |
| Netzwerkmaske | 255.255.255.0 |
| IP des DHCP-Servers | 192.168.5.1 |
| IP des DNS-Servers | 192.168.5.1 |
| Lease Time | 00:00:09:55 (9 min 55 s, aus `ipconfig`: 09:35:36 bis 09:45:31) |
| Netzwerkname | Frankfurt.IHK-BookWorm-GmbH.Local |
| IP des Routers | 192.168.5.1 |

### bb) Falscher Parameter
- **Parameter:** **IP des DNS-Servers** (bzw. des Standard-Gateways). Der DHCP-Server übergibt **192.168.5.1** als DNS. Der Domänen-Controller (DNS) steht aber unter einer anderen IP (nicht 192.168.5.1). Anmeldung am DC scheitert, weil der Client den DC-Namen nicht auflösen kann.

### bc) Ursache für unterschiedliche Parameter
- **Möglichkeit:** **Rogue DHCP-Server** im Netz (z.B. ein falsch konfigurierter Router oder ein Angreifer), der schnellere DHCP-Offers sendet als der legitime Server. Der Client akzeptiert das erste Offer (Race Condition) und erhält falsche Parameter (falsches Gateway/DNS).

### bd) Vorgehen für gültige Parameter
- **Schritt 1:** `ipconfig /release` und `ipconfig /renew` am Client (neues DHCP-Offer anfordern).
- **Schritt 2:** Rogue DHCP-Server identifizieren (Switch: `show ip dhcp snooping binding`, Port-Security) und blockieren.
- **Schritt 3:** **DHCP Snooping** auf den Switches aktivieren; nur vertrauenswürdige Ports (Uplink zum echten DHCP) erlauben DHCP-Offers.

---

**Merksatz:** IHK-Aufgaben sind **Kausalitätsketten** – wer Subnetting-Broadcast, VRRP-VIP, SPI-State-Table und ARP-Poisoning nicht als Hard-Facts sitzen hat, verliert Punkte an der Oberfläche. Lerne Standards als Vokabeln, nicht als Prosa.
