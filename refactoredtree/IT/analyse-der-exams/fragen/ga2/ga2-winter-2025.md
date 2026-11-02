## 1. Aufgabe (Netzwerkplan & NAT)

### a) Router-IPs und nutzbare Adressen
| Router | Interface | IP-Adresse des Interface | Anzahl nutzbarer IP-Adressen |
|---|---|---|---|
| Zentrale | eth1 – LAN | 10.0.0.254 | 16.777.214 (10.0.0.0/8) |
| Zentrale | eth2 – DMZ | 192.168.199.254 | 254 (192.168.199.0/24) |
| Reisebüro | eth1 – LAN | 172.16.10.2 | 2 (172.16.10.0/30) |

- **Logik:** Letzte nutzbare IP = Broadcast − 1. Bei /30 sind 2 Hosts nutzbar (.1 und .2).

### b) Routingtabelle Router Zentrale
| Netzwerk-ID | CIDR | Schnittstelle | Next-Hop |
|---|---|---|---|
| 10.0.0.0 | /8 | eth1 | – |
| 192.168.199.0 | /24 | eth2 | – |
| 31.101.17.40 | /30 | eth0 | – |
| 172.16.10.0 | /30 | tun0 | – |
| 172.16.20.0 | /30 | tun1 | – |
| 192.168.50.0 | /24 | tun1 | – |
| 172.16.0.0 | /30 | tun0 | – |
| 0.0.0.0 | /0 | eth0 | 31.101.17.41 |

- **Logik:** Direkt angeschlossene Netze (LAN, DMZ, WAN, VPN-Tunnel) ohne Next-Hop. Route zu Werkstatt (172.16.0.0/30) über tun0. Route zum Reisebüro (192.168.50.0/24) über tun1. Default-Route zum ISP.

### c) NAT für LAN → Internet
- **Grund:** Interne IPs (10.0.0.0/8, RFC 1918) sind im Internet nicht routbar. NAT übersetzt sie in die öffentliche IP des Routers (31.101.17.41), damit Antworten zurückkommen. Ohne NAT würden ISP-Router private Pakete verwerfen.

### d) NAT/PAT-Prozess
**Abschnitt 2 (Client → NAT-Router):**
| Source | Destination |
|---|---|
| 10.0.0.1:45123 | 12.7.51.9:443 |
| 10.0.0.2:45123 | 12.7.51.9:443 |

**Abschnitt 3 (NAT-Router → Server):**
| Source | Destination |
|---|---|
| 31.101.17.41:50001 | 12.7.51.9:443 |
| 31.101.17.41:50002 | 12.7.51.9:443 |

**Abschnitt 4 (Server → NAT-Router → Client):**
| Source | Destination |
|---|---|
| 12.7.51.9:443 | 31.101.17.41:50001 |
| 12.7.51.9:443 | 31.101.17.41:50002 |

- **Logik:** PAT ändert Quell-IP auf öffentliche Router-IP und Quell-Port auf eindeutige Ports (50001, 50002). Rückrichtung wird anhand der Session-Tabelle zurückübersetzt.

### e) Warum NAT trotz TLS funktioniert
- **Grund:** NAT arbeitet auf **Layer 3/4** (IP-Header, TCP/UDP-Header). Die TLS-Verschlüsselung schützt nur den **Payload** (Layer 5–7). Der IP-Header und der TCP-Port-Header bleiben unverschlüsselt und können von NAT gelesen/geändert werden.

### f) Portforwarding
- **Definition:** **Destination NAT (DNAT)** – eingehende Pakete an die öffentliche IP:Port werden auf eine interne IP:Port umgeschrieben.
- **Anwendungsfall:** Webserver in der DMZ soll von außen über HTTPS (Port 443) erreichbar sein. Router leitet 31.101.17.41:443 → 192.168.199.10:443 weiter.

---

## 2. Aufgabe (VPN)

### aa) VPN-Typ Werkstatt ↔ Zentrale
- **Typ:** **Site-to-Site VPN** (IPsec).
- **Geräte:** Eingerichtet auf den ** Routern** beider Standorte (Router Werkstatt und Router Zentrale). Transparenter Tunnel zwischen den Netzen.

### ab) VPN-Typ für Busfahrer/Reiseleiter
- **Typ:** **End-to-Site VPN** (Remote Access VPN, z.B. IPsec oder SSL-VPN).
- **Geräte:** VPN-Client auf dem **mobilen Endgerät** (Laptop/Smartphone) des Mitarbeiters; VPN-Gateway auf dem **Router/Firewall der Zentrale**.

### b) Zwei Schutzziele VPN
- **Vertraulichkeit:** Verschlüsselung (IPsec ESP, AES) verhindert Mitlesen auf dem Transportweg.
- **Integrität/Authentizität:** HMAC/ICV erkennt Manipulation; IKE-Authentifizierung (PSK/Zertifikat) stellt sicher, dass nur autorisierte Peers den Tunnel aufbauen.

### c) Neuer VPN-Nutzer – 2 Fehlerursachen
- **Ursache 1 – Falsche Credentials/PSK:** Der neue Nutzer hat falschen Username/Passwort oder falschen PSK. **Behebung:** Credentials prüfen und neu setzen.
- **Ursache 2 – Fehlende Firewall-Regel:** Der neue Nutzer hat eine andere öffentliche IP (z.B. Mobilfunk), die nicht in der VPN-Gateway-ACL erlaubt ist. **Behebung:** ACL um den neuen Adressbereich erweitern oder Authentifizierung (Zertifikat) statt IP-Filter nutzen.

### da) Passwort-Policy – 2 weitere Regeln
- **Mindestlänge:** Mindestens 12 Zeichen.
- **Sonderzeichen:** Mindestens ein Sonderzeichen (z.B. !, §, $, %).
- **Kein Wörterbuch/keine Wiederholung:** Keine bekannten Wörter oder Tastaturmuster (z.B. „123456“, „qwertz“).
- **Ablaufdatum:** Passwortwechsel alle 90 Tage.

### db) MFA – 2 weitere Methoden (neben Wissen)
- **Besitz:** Hardware-Token (FIDO2/YubiKey) oder Smart-Card mit Zertifikat. Physischer Faktor, kopiergeschützt.
- **Sein (Biometrie):** Fingerabdruck, Gesichtserkennung (Windows Hello). Schwer zu fälschen, kein Merken nötig.
- **Besitz (Software):** TOTP-Authenticator-App (zeitbasierter Einmalcode).

### e) Zwei Angriffsmethoden
- **Phishing:** Gefälschte E-Mail/Website imitiert vertrauenswürdige Quelle, um Zugangsdaten/Finanzdaten zu stehlen. Schutz: Awareness, MFA, SPF/DKIM/DMARC.
- **Trojaner:** Schadprogramm, das sich als nützliche Software tarnt und im Hintergrund Backdoor/Zugriff ermöglicht. Schutz: AV, Application Whitelisting, keine unbekannten Anhänge öffnen.

---

## 3. Aufgabe (VLANs & Router-on-a-Stick)

### aa) Zwei Gründe für VLANs
- **Sicherheit/Isolation:** Trennung von Büro, Werkstatt und VoIP; Broadcast-Domänen werden getrennt; kein ungefilterter Zugriff.
- **QoS/Performance:** VoIP-Traffic kann priorisiert werden (eigenes VLAN); Broadcast-Stürme bleiben auf ein VLAN beschränkt.

### ab) Switchports – VLAN & Modus
| Interface | VLAN-Nr | Switchportmodus |
|---|---|---|
| G0/1 | 231 | untagged (Access) |
| G0/2 | 232 | untagged (Access) |
| G0/10 | 233 | untagged (Access) |
| G0/22 | 231, 232, 233 | **tagged** (Trunk zum Router) |
| G0/23 | 231, 232, 233 | **tagged** (Trunk zum Router) |
| G0/24 | 231, 232, 233 | **tagged** (Trunk zum Router) |

- **Logik:** Access-Ports (Endgeräte) untagged; Trunk-Ports (Router, weitere Switches) tagged für alle VLANs.

### ba) Router-on-a-Stick – Konzept
- **Prinzip:** Ein physisches Router-Interface wird in mehrere **Subinterfaces** (802.1Q) aufgeteilt. Jedes Subinterface ist einem VLAN zugeordnet und fungiert als Gateway. Der Router routet zwischen VLANs über dieselbe physische Leitung. Erfordert Trunk-Port am Switch.

### bb) Fehler in Router-Konfiguration
- **Fehler:** Die Subinterfaces haben IP-Adressen aus **/29-Netzen** (255.255.255.248), aber die PCs im LAN nutzen vermutlich ein anderes Subnetz (laut Plan 192.168.23.x/24?). Zudem fehlt evtl. die Zuordnung der VLANs zu den richtigen Subnetzen.
- **Lösungsvorschlag:** IP-Adressen der Subinterfaces an das tatsächliche Subnetz anpassen (z.B. 192.168.23.1/24 für VLAN 231). Oder: VLAN-Zuordnung im Switch prüfen (Access-Ports müssen im richtigen VLAN sein).

### bc) Warum `encapsulation dot1q` zwingend?
- **Grund:** Das Subinterface muss wissen, welche Frames zu welchem VLAN gehören. `encapsulation dot1q <VLAN-ID>` weist das Subinterface an, Frames mit diesem 802.1Q-Tag zu verarbeiten und ausgehende Frames mit diesem Tag zu versehen. Ohne diese Konfiguration kann der Router die VLANs nicht unterscheiden.

### c) VoIP-VLAN – 2 Gründe
- **QoS/Priorisierung:** VoIP benötigt geringe Latenz/Jitter. Eigenes VLAN ermöglicht Priorisierung (802.1p/CoS, DSCP) und Bandbreitengarantie.
- **Sicherheit/Isolation:** VoIP-Telefone sind oft schlecht patchbar; Trennung vom Datenverkehr verhindert Angriffe auf Telefone, die auf das LAN überspringen.

---

## 4. Aufgabe (Verfügbarkeit & Fehlersuche)

### a) SLA 99% – erlaubte Ausfallzeit
- **Jahr:** 365 × 24 = 8.760 h.
- **Ausfallzeit:** 1% von 8.760 h = **87,6 h** pro Jahr.

### b) Gesamtverfügbarkeit ≥ 99,99% – zwei Anbieter
- **Formel:** A_gesamt = 1 − (1 − A1) × (1 − A2). Benötigt: A_gesamt ≥ 0,9999.
- **o3-Internet:** A1 = 0,99 → (1 − A1) = 0,01.
- **Berechnung für jeden Anbieter:**
  - Fonedavo (98,5%): 1 − (0,01 × 0,015) = 1 − 0,00015 = **0,99985 (99,985%)** → < 99,99%, **nicht geeignet**.
  - Lahrstink (99%): 1 − (0,01 × 0,01) = 1 − 0,0001 = **0,9999 (99,99%)** → **geeignet**.
  - S-Mobile (98%): 1 − (0,01 × 0,02) = 1 − 0,0002 = **0,9998 (99,98%)** → < 99,99%, **nicht geeignet**.
  - Telegeh (99%): 1 − (0,01 × 0,01) = **0,9999 (99,99%)** → **geeignet**.
- **Ergebnis:** **Lahrstink (Satellit)** und **Telegeh (FTTH GPON)** kommen in Betracht.

### b) Shared Medium – Nachteil
- **Nachteil:** Die Bandbreite wird mit anderen Kunden geteilt. Bei hoher Last (Peak-Zeiten) sinkt die verfügbare Bandbreite; Latenz steigt. Keine garantierte Performance (Best Effort).

### b) Übertragungsdauer 15 GiB bei 250 Mbit/s (10% Overhead)
- **Datenmenge:** 15 GiB = 15 × 1024 × 1024 × 1024 × 8 Bit = 128.849.018.880 Bit.
- **Mit Overhead:** 128.849.018.880 × 1,10 = 141.733.920.768 Bit.
- **Bandbreite:** 250 Mbit/s = 250.000.000 Bit/s.
- **Dauer:** 141.733.920.768 / 250.000.000 = 566,936 s.
- **In Minuten/Sekunden:** 566,936 / 60 = 9 min 26,9 s → **9 min 27 s**.

### bd) CGNAT/DS-Lite – Maßnahme für eingehenden Tunnel
- **Maßnahme:** **VPN über IPv6** aufbauen (DS-Lite bietet natives IPv6). Oder: **Port-Control-Protocol (PCP)** / **UPnP** nutzen, um Ports durch CGNAT zu mappen. Alternativ: **Relay-Server** (Cloud) als Vermittler.

### ca) Erhöhte Port-Aktivität – 2 Ursachen
- **Ursache 1 – Loop/Broadcast-Sturm:** Fehlerhafte Verkabelung (Loop) oder defekte NIC sendet Broadcast-Sturm. **Behebung:** STP prüfen, Loop beseitigen.
- **Ursache 2 – Malware/Kompromittierung:** Infizierter Client sendet Daten exfiltriert oder Scan-Traffic. **Behebung:** Port shutdown, Client isolieren, AV-Scan.

### cb) SFP-Uplink funktioniert nicht – 2 Ursachen
- **Ursache 1 – SFP-Modul inkompatibel:** Neues SFP-Modul (z.B. 10G SR) passt nicht zur Glasfaser (Multimode vs. Singlemode) oder zur Gegenstelle. **Behebung:** Kompatibles SFP-Modul verwenden (Wellenlänge, Reichweite, Typ prüfen).
- **Ursache 2 – Konfiguration nicht übernommen:** Die gesicherte Konfiguration enthält Interface-Namen/Port-Nummern des alten Switches (z.B. Gi1/0/25), die beim neuen Switch anders sind. **Behebung:** Konfiguration anpassen; Uplink-Port manuell konfigurieren (Trunk, Speed, Duplex).

---

**Merksatz:** IHK-Aufgaben sind **Kausalitätsketten** – wer NAT/PAT-Session-Tabelle, Router-on-a-Stick (802.1Q), SLA-Ausfallzeit und SFP-Kompatibilität nicht als Hard-Facts sitzen hat, verliert Punkte an der Oberfläche. Lerne Standards als Vokabeln, nicht als Prosa.
