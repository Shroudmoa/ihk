## 1. Aufgabe (VLANs, DHCP & Switching)

### a) Gründe für VLANs (2)
- **Sicherheit/Isolierung:** Broadcast-Domänen werden getrennt. Verkehr zwischen Verkauf, Werkstatt, Verwaltung und Servern muss über einen Layer-3-Switch (Inter-VLAN-Routing) laufen, wo Firewall-Regeln greifen.
- **Flexibilität/Administration:** Logische Gruppierung unabhängig von physischer Verkabelung. Änderungen der Bereichszugehörigkeit erfolgen per Switch-Konfiguration statt Patchfeld.

### ac) Statisch vs. 802.1X
- **Statisch:** Port wird manuell einem VLAN zugewiesen (z.B. Port 1–10 = VLAN Verkauf). Einfach, aber unflexibel; bei Umzug muss der Admin den Port ändern.
- **IEEE 802.1X:** Port-basierte Authentifizierung via RADIUS. Der Switch weist das VLAN dynamisch nach erfolgreicher Authentifizierung zu (z.B. MAC/User). Unbekannte Geräte landen im Guest-VLAN oder werden blockiert.

### ba) DHCP – 4 Kommunikationsschritte (DORA)
1. **Discover:** Client sendet Broadcast (UDP 67/68) auf der Suche nach einem DHCP-Server.
2. **Offer:** Server antwortet mit einem IP-Adressvorschlag (inkl. Lease-Dauer, Maske, Gateway).
3. **Request:** Client fordert die angebotene IP-Adresse formal an (Broadcast).
4. **Acknowledge (ACK):** Server bestätigt die Zuweisung; Client konfiguriert das Interface.

### bb) DHCP über VLAN-Grenzen – Problem & Lösung
- **Problem:** DHCP-Discover ist ein **Layer-2-Broadcast**. Router (Layer 3) leiten Broadcasts standardmäßig **nicht** weiter. Der DHCP-Server im Server-VLAN empfängt die Anfrage aus dem Benutzer-VLAN nicht.
- **Lösung:** Am Layer-3-Switch (Gebäude-Verteiler) **DHCP-Relay (IP-Helper)** pro Benutzer-VLAN konfigurieren. Der Relay wandelt den Broadcast in einen Unicast zum DHCP-Server um.

### ca) SFP+ vs. SFP
- **SFP+:** Kleines, hot-swap-fähiges Transceiver-Modul für Glasfaser/Kupfer. SFP+ unterstützt bis **10 Gbit/s**; SFP bis 1 Gbit/s. SFP+ ist elektrisch und mechanisch kompatibel zu SFP, aber für höhere Datenraten spezifiziert.

### cb) Glasfaserstecker
- **LC-Stecker** (klein, Verriegelung). Passt zum angegebenen Modul (Multimode, 1310 nm, LC).

### cc) STP vs. Link-Aggregation
| Protokoll | Funktionsweise (störungsfrei) | Reaktion bei Leitungsausfall |
|---|---|---|
| **Spanning Tree (STP)** | Eine Leitung wird logisch blockiert (Blocking); nur eine ist aktiv (Forwarding). Verhindert Layer-2-Schleifen. | Blockierte Leitung wird aktiviert (Listening→Forwarding). Umschaltung dauert (Sekunden). |
| **Link-Aggregation (802.3ad)** | Beide Leitungen sind aktiv und werden gebündelt (LACP). Lastverteilung über beide Links. | Traffic wird auf die verbleibende Leitung verteilt. Kein Ausfall, nur reduzierte Bandbreite. |

---

## 2. Aufgabe (Routing & IP-Konfiguration)

### aa) Routentabelle Router Essen (statisch)
| Netzwerkziel | Netzmaske | Gateway/Next-Hop | Interface |
|---|---|---|---|
| 10.1.0.0 | 255.255.0.0 | direct | IF1 |
| 10.2.0.0 | 255.255.0.0 | 192.168.0.30 | IF2 |
| 10.3.0.0 | 255.255.0.0 | 192.168.4.30 | IF3 |
| 10.4.0.0 | 255.255.0.0 | 192.168.0.30 | IF2 (oder IF4 via 192.168.0.30) |
| 0.0.0.0 | 0.0.0.0 | 210.10.10.1 | dsl |

- **Hinweis:** Next-Hop für 10.2.0.0/16 ist der Router Bochum (192.168.0.30). Für 10.3.0.0/16 ist es Router Duisburg (192.168.4.30). Für 10.4.0.0/16 ist es Router Dortmund (192.168.0.30 via IF2 oder direkt via IF4 192.168.0.30).

### ab) Dynamisches Routing – Begründung
- **Redundanz:** Bei Ausfall einer Standleitung (z.B. Duisburg–Essen) muss der Traffic automatisch über die neue Leitung Bochum–Duisburg umgeleitet werden. Statisches Routing kennt keine Alternativrouten; dynamisches Routing (OSPF) konvergiert und wählt den besten Pfad.
- **Performance:** Dynamisches Routing berücksichtigt Link-Kosten (Bandbreite). Die 500-Mbit/s-Leitung wird teurer bewertet als 10-Gbit/s-Leitungen; Traffic wird optimal verteilt.

### ac) Distance-Vector vs. Link-State
- **Link-State (OSPF):** Erforderlich. Jeder Router kennt die vollständige Topologie (Link-State-Datenbank) und berechnet kürzeste Pfade (Dijkstra). Schnelle Konvergenz bei Topologieänderungen; geeignet für redundante Netze mit unterschiedlichen Bandbreiten.
- **Distance-Vector (RIP):** Kennt nur Nachbar-Router und Distanzen; langsame Konvergenz, Schleifenbildung (Count-to-Infinity). Ungeeignet für dieses Netz.

### ba) File-Server nicht erreichbar – Analyse
- **Analyse:** `ipconfig /all` zeigt **169.254.102.223** (APIPA/Link-Local) statt der statischen 10.1.0.10. Das bedeutet: Die statische Konfiguration wurde nicht übernommen oder das Interface hat keine Verbindung zum Netz (Kabel/Port defekt).
- **ARP-Probe:** Der Server sendet ARP-Probes für 10.1.0.10 und 169.254.102.223. Ein anderer Host antwortet: **10.1.0.10 is at 00:0d:59:57:11:05** → **IP-Konflikt**. Ein anderes Gerät im Netz verwendet bereits 10.1.0.10.
- **Folge:** Der Server erhält eine APIPA-Adresse, weil die statische IP als „in use“ erkannt wird.

### bb) Vorgehen zur Behebung
- **Schritt 1:** Das konkurrierende Gerät identifizieren (MAC 00:0d:59:57:11:05) und dessen IP ändern oder aus dem Netz nehmen.
- **Schritt 2:** Am File-Server `ipconfig /release` und `ipconfig /renew` ausführen oder Interface neu starten.
- **Schritt 3:** Statische IP 10.1.0.10/16 erneut setzen und mit `ping`/`arp -a` prüfen, ob kein Konflikt mehr besteht.

---

## 3. Aufgabe (VPN)

### aa) VPN-Arten – Beispiele
- **End-to-Site:** Homeoffice-Mitarbeiter verbindet sich per VPN-Client (z.B. WireGuard, OpenVPN) mit dem Firmennetz in Essen. Einzelner Client → Firmen-Gateway.
- **Site-to-Site:** Die Filiale Duisburg ist über einen IPsec-Tunnel dauerhaft mit der Zentrale Essen verbunden. Zwei Standorte (Router zu Router) werden logisch verbunden.

### ab) X.509-Zertifikat – 3 Bestandteile
- **Subject:** Identität des Inhabers (CN, O, OU, E-Mail).
- **Public Key:** Öffentlicher Schlüssel des Inhabers.
- **Signatur der CA:** Digitale Signatur der ausstellenden CA über den Zertifikatsinhalt.
- **Gültigkeitszeitraum:** Not Before / Not After.
- **Serial Number:** Eindeutige Seriennummer.

### b) VPN am Hotspot scheitert – Ursache & Lösung
- **Ursache:** Der Hotspot blockiert den für VPN benötigten Port (z.B. UDP 500/4500 für IPsec, UDP 1194 für OpenVPN, UDP 51820 für WireGuard). Viele Hotspots erlauben nur HTTP/HTTPS (TCP 80/443).
- **Lösung:** VPN auf Port 443/TCP umkonfigurieren (z.B. OpenVPN over TCP 443, WireGuard über TCP oder SSL-VPN). Alternativ: Provider/Hotspot-Betreiber kontaktieren, um Portfreigabe bitten.

### ca) Ungefilterter Traffic trotz Proxy – Ursache
- **Ursache:** Der `tracert` zeigt **IPv6-Adressen** (2001:db8:...). Der Proxy ist nur für **IPv4** konfiguriert. Der Client nutzt IPv6 und umgeht den IPv4-Proxy komplett. Der Router im Homeoffice verteilt IPv6 per SLAAC; der Traffic geht direkt ins Internet.

### cb) Lösung zur Unterbindung
- **IPv6 deaktivieren** auf dem Client (oder im Router) oder **IPv6-Proxy** in der Zentrale einrichten. Alternativ: Firewall-Regel, die IPv6-Traffic blockiert, bis ein IPv6-Proxy verfügbar ist.

---

## 4. Aufgabe (DNS & DNSSEC)

### aa) Aufgaben des DNS
- **Namensauflösung:** Übersetzung von Domainnamen (z.B. www.ihk.de) in IP-Adressen (A/AAAA-Records) und umgekehrt (PTR).
- **Weitere Records:** MX (Mail), CNAME (Alias), TXT (SPF/DKIM), NS (Nameserver).

### ab) DNS-Weiterleitung (Forwarder)
- **Sinn:** Der lokale DNS-Server leitet Anfragen für externe Domains an den DNS-Server des Providers weiter (Forwarder). Reduziert die Last auf Root-/TLD-Server; beschleunigt die Auflösung durch Provider-Cache.

### ac) Iterative Auflösung von www.ihk.de
1. **Root-Server:** Fragt Root (`.`), antwortet mit Verweis auf TLD-Server für `.de`.
2. **TLD-Server (.de):** Fragt `.de`-Server, antwortet mit Verweis auf autoritativen Nameserver für `ihk.de`.
3. **Autoritativer Server (ihk.de):** Fragt den Nameserver von `ihk.de`, antwortet mit **A-Record** (IP-Adresse für `www.ihk.de`).
4. **Antwort an Client:** Der Provider-DNS cached das Ergebnis und liefert es an den Client.

### ba) MX-Record fehlt – E-Mail-Empfang
- **Tabelle:** Der MX-Record für `fahrguth.gmbh` fehlt (oder zeigt auf falschen Host). Ohne MX-Record wissen externe Mailserver nicht, wohin sie E-Mails für die Domain zustellen sollen. Zustellung scheitert (Bounce).

### b) SPF – Funktion & Newsletter-Problem
- **Funktion:** SPF (TXT-Record) autorisiert, welche IP-Adressen/Hosts E-Mails für die Domain versenden dürfen. Empfänger prüfen die Absender-IP gegen den SPF-Record.
- **Problem:** Der externe Newsletter-Versender (`mail.newsletterversand.de`) ist **nicht im SPF-Record** von `fahrguth.gmbh` eingetragen. Empfänger stufen die Mail als nicht autorisiert ein → Reject/Spam. Lösung: SPF-Record um die IP/Domain des Newsletter-Versenders erweitern (`include:newsletterversand.de`).

### c) DNS-Angriffsmethode (Cache Poisoning / DNS Spoofing)
- **Methode:** **DNS Cache Poisoning** – Ein Angreifer schleust gefälschte DNS-Antworten in den Cache eines Resolvers ein. Clients erhalten dann die IP der gefälschten Webseite. Alternativ: **DNS Spoofing** (Man-in-the-Middle) – gefälschte Antworten auf DNS-Anfragen.

### d) DNSSEC – Authentizität & Integrität
- **Authentizität:** DNSSEC signiert DNS-Antworten kryptografisch. Der Client kann verifizieren, dass die Antwort tatsächlich vom autoritativen Nameserver der Domain stammt (Ursprung).
- **Integrität:** Die Signatur stellt sicher, dass die DNS-Daten (Records) nicht manipuliert wurden (keine Änderung auf dem Transportweg).

---

**Merksatz:** IHK-Aufgaben sind **Kausalitätsketten** – wer DHCP-DORA, VLAN-Trunking, DNS-Records (A, MX, SPF) und APIPA/169.254 nicht als Hard-Facts sitzen hat, verliert Punkte an der Oberfläche. Lerne Standards als Vokabeln, nicht als Prosa.
