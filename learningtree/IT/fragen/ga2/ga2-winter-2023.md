## 1. Aufgabe (Routing & WLAN)

### aa) Zwei Fehler in der Routingtabelle
- **Fehler 1 – Fehlende Route zum Netz 172.16.64.0/22:** Die Zeile `172.16.64.0/22` hat als Next-Hop **10.2.2.2** (falsch, gehört nicht zum Berliner Netz). Korrekt: Next-Hop **10.1.1.2** (über Eth 1).
- **Fehler 2 – Fehlende Route zu 10.0.0.0/8 oder Default-Route falsch:** Die Default-Route `0.0.0.0/0` zeigt auf **203.0.113.14**, was nicht der ISP-Gateway (203.0.113.14 im /30 = Router R- Rusticus- ext) ist. Zudem fehlt eine Route zu `10.0.0.0/8` (Berliner LAN), sodass Traffic zu Berlin ins Internet geroutet wird.
- **Behebung:** Next-Hop der 172.16.64.0/22-Route auf 10.1.1.2 korrigieren; Default-Route auf ISP-Gateway (203.0.113.13) setzen; ggf. Route zu 10.0.0.0/8 über 10.1.1.2 ergänzen.

### ab) Dynamisches Routing – 2 Vorteile
- **Automatische Konvergenz:** Bei Ausfall einer Leitung wird automatisch eine Alternativroute gewählt. Kein manueller Eingriff.
- **Skalierbarkeit:** Neue Netze/Router werden automatisch propagiert; weniger Administrationsaufwand und Fehleranfälligkeit als bei statischen Routen.

### ad) Erreichbarkeit DMZ-Server (private IPs)
- **Möglichkeit:** **Destination NAT (Port Forwarding)** am Router RB1. Eingehende Anfragen auf die öffentliche IP (203.0.113.13) werden auf die private DMZ-IP umgeschrieben (z.B. 203.0.113.13:443 → 192.168.100.10:443). Firewall-Regeln erlauben nur diese Ports.

### ba) WPA2/3-Enterprise
- **Verfahren:** 802.1X mit RADIUS. Client authentifiziert sich mit individuellen Credentials (EAP, Zertifikat, User/Passwort). AP leitet Auth an RADIUS weiter; bei Erfolg wird der Port/VLAN freigeschaltet. Schlüssel werden dynamisch pro Session generiert.

### bb) WPA2/3-Enterprise vs. PSK – 2 Vorteile
- **Individuelle Credentials:** Kein gemeinsamer PSK; bei Verlust/Weitergabe ist nur ein Benutzer betroffen.
- **Zentrale Rechteverwaltung:** Dynamische VLAN-Zuweisung, Accounting, sofortiger Entzug bei Offboarding.

---

## 2. Aufgabe (NAT/PAT & IPv6)

### aa) NAT vs. PAT
- **NAT (Network Address Translation):** 1:1-Übersetzung privater IPs in öffentliche IPs. Jeder interne Host erhält eine eigene öffentliche IP.
- **PAT (Port Address Translation):** n:1-Übersetzung. Mehrere interne Hosts teilen sich eine öffentliche IP; Unterscheidung über **Quell-Ports** (Port-Multiplexing).

### ab) Header auf Weg Client → Router RB2
| Feld | Wert |
|---|---|
| TTL | 128 (dekrementiert auf 127) |
| Protokoll | TCP (6) |
| Header-Checksum | wird neu berechnet |
| Quell-IP | 192.168.100.x (interne Client-IP) |
| Ziel-IP | 141.88.222.152 (www.ihk.de) |
| Quell-Port | 49152 |
| Ziel-Port | 443 (HTTPS) |

### ac) Header nach NAT auf Weg Router → ISP
| Feld | Wert |
|---|---|
| TTL | 127 (dekrementiert) |
| Protokoll | TCP (6) |
| Header-Checksum | neu berechnet |
| Quell-IP | 203.0.113.13 (öffentliche NAT-IP) |
| Ziel-IP | 141.88.222.152 |
| Quell-Port | **50000** (vom Router zugewiesen) |
| Ziel-Port | 443 |

### ad) Warum anderer Quell-Port?
- **Grund:** **PAT.** Der Router übersetzt die Quell-IP von 192.168.100.x auf 203.0.113.13 und ändert den Quell-Port auf einen eigenen (50000). So kann der Router Antworten eindeutig dem ursprünglichen Client zuordnen (Session-Tabelle).

### ba) DMZ SSH-Zugriff (aus Internet)
- **IP-Paket ankommend:** Quell-IP 62.155.243.193; Ziel-IP **203.0.113.13** (öffentliche IP); Quell-Port zufällig; Ziel-Port **22** (SSH).

### bb) IP-Paket nach Destination-NAT
- **Quell-IP:** 62.155.243.193 (bleibt).
- **Ziel-IP:** **192.168.100.10** (private DMZ-Server-IP).
- **Ziel-Port:** 22 (bleibt).

### c) Carrier-Grade NAT (10.x.x.x)
- **Erläuterung:** Provider übersetzt private Kunden-IPs (10.x.x.x) in öffentliche IPs, da IPv4-Adressen knapp sind. Mehrere Kunden teilen sich eine öffentliche IP (CGNAT).
- **Nachteil:** Kein direkter eingehender Zugriff (kein Port-Forwarding) möglich; einige Dienste (VPN, Online-Gaming, Peer-to-Peer) funktionieren nicht.

---

## 3. Aufgabe (VLANs & Switching)

### a) Glasfaser vs. Kupfer – 2 Gründe
- **Distanz:** 150 m übersteigt Twisted-Pair-Limit (100 m). Glasfaser überbrückt km ohne Signalverlust.
- **Bandbreite/EMV:** 10 Gbit/s SFP+ über Glasfaser; immun gegen elektromagnetische Interferenzen (EMI).

### bb) VLAN-Trunking zwischen Switches
- **Einstellung:** **Trunk-Port (802.1Q)** auf den Uplink-Ports beider Switches. Tagged Frames für alle VLANs; Native VLAN für untagged Traffic. Erlaubt gebäudeübergreifende VLANs.

### bc) OSI Layer 2 – Veränderung
- **Veränderung:** Ethernet-Frames werden mit einem **802.1Q-Tag** (4 Byte) erweitert. Enthält VLAN-ID (12 Bit) und Priority (3 Bit). Switches können Frames dem richtigen VLAN zuordnen.

### ca) nslookup Timeout – Ursache
- **Ursache:** DNS-Server 192.168.100.1 ist nicht erreichbar. Vermutlich **fehlendes Inter-VLAN-Routing** oder **falsches Gateway** im neuen VLAN. Der Router (Gateway) hat keine Route zum DNS-Server oder VLAN-Trunk ist nicht korrekt konfiguriert.

### cb) IP-Adresse des Clients (aus Trace)
- **Client-IP:** 192.168.100.x (aus Ping-Trace: Source der ICMP-Pakete).

### cc) VLAN-Nummer (aus Trace)
- **VLAN:** Aus 802.1Q-Tag im Trace ablesbar (z.B. **VLAN 20** oder **VLAN 100** – je nach Konfiguration).

### cd) Fehlerursache & Behebung (Internet)
- **Fehlerursache:** Das neue VLAN hat **kein Routing zum Internet**. Der Router (192.168.100.1) hat kein Subinterface für das neue VLAN oder die Default-Route fehlt.
- **Behebung:** Subinterface auf dem Router für das neue VLAN anlegen (802.1Q); Default-Route zum ISP konfigurieren; VLAN-Trunk zum Router sicherstellen.

---

## 4. Aufgabe (PKI & TLS)

### a) Zertifikate vs. PSK – 2 Vorteile
- **Individuelle Identität:** Jeder Client hat ein eigenes Zertifikat; bei Kompromittierung nur ein Client betroffen. PSK ist ein gemeinsames Geheimnis.
- **Widerruf (Revocation):** Zertifikate können via CRL/OCSP widerrufen werden. PSK kann nicht gezielt für einen Client gesperrt werden.

### ba) Hashfunktionen in X.509
- **Rolle:** Die CA signiert das Zertifikat mit einem Hash über den Inhalt (Subject, Public Key, etc.). Der Hash gewährleistet **Integrität**: Jede Änderung am Zertifikat würde die Signatur invalidieren.

### bb) Collision Attack & MD5
- **Collision Attack:** Zwei unterschiedliche Eingaben erzeugen denselben Hashwert. Bei MD5 ist dies in Sekunden möglich.
- **Unsicherheit:** Ein Angreifer kann ein **gefälschtes Zertifikat** mit demselben Hashwert wie ein legitimes Zertifikat erstellen. Die CA-Signatur wäre gültig, obwohl der Inhalt manipuliert ist.

### ca) Fehler des Kollegen (CA_Private_Key versendet)
- **Fehler:** Der **private Schlüssel der CA** wurde per E-Mail versendet. Der private CA-Key darf niemals das CA-System verlassen.
- **Sicherheitslücke:** Wer den CA-Key hat, kann beliebige Zertifikate im Namen der CA ausstellen (MITM, Identitätsdiebstahl). Die gesamte PKI ist kompromittiert.

### cb) Korrektur des Fehlers
- **Schritt 1:** CA-Key als kompromittiert betrachten; **neue CA** aufsetzen (neues Keypair).
- **Schritt 2:** Altes CA-Zertifikat widerrufen (CRL/OCSP); alle ausgestellten Client-Zertifikate neu ausstellen und verteilen.
- **Schritt 3:** Private Keys niemals per E-Mail versenden; Übergabe nur verschlüsselt/offline (HSM, USB).

### da) Diffie-Hellman – Funktion
- **Prinzip:** Beide Parteien einigen sich über einen gemeinsamen **Session Key**, ohne ihn über das Netz zu übertragen. Jede Seite berechnet den Schlüssel aus eigenem privaten und öffentlichem Schlüssel der Gegenseite. Bietet **Forward Secrecy** (PFS).

### db) Asymmetrische Verschlüsselung in TLS 1.3
- **Zweck:** **Schlüsselaustausch** (Diffie-Hellman) und **Authentifizierung** (Server-Zertifikat, Signatur). Die eigentlichen Nutzdaten werden symmetrisch (AES-GCM) verschlüsselt. Asymmetrisch ist zu langsam für Massendaten.

---

**Merksatz:** IHK-Aufgaben sind **Kausalitätsketten** – wer NAT/PAT-Port-Tabelle, 802.1Q-Tag, MD5-Collision und CA-Key-Handling nicht als Hard-Facts sitzen hat, verliert Punkte an der Oberfläche. Lerne Standards als Vokabeln, nicht als Prosa.
