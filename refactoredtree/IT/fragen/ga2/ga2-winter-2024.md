## 1. Aufgabe (VLANs & Switching)

### aa) Drei Vorteile von VLANs
- **Sicherheit/Isolation:** Broadcast-Domänen werden getrennt; Abteilungen (Verwaltung, Forschung, Labor) können nicht ungefiltert aufeinander zugreifen. Inter-VLAN-Traffic muss über Layer-3-Router/Firewall.
- **Flexibilität:** Logische Gruppierung unabhängig von physischer Verkabelung. Umzüge erfordern nur Konfigurationsänderung am Switch, kein Patchfeld.
- **Performance:** Reduzierung von Broadcast-Traffic; kleinere Broadcast-Domänen senken die Last auf Endgeräten.

### ab) Portbasiert vs. Dynamisch
- **Portbasiert (statisch):** Der Administrator weist jedem Switch-Port manuell eine VLAN-ID zu. Geräte am Port gehören automatisch zum VLAN (z.B. Port 1–10 = VLAN 10). Einfach, aber unflexibel.
- **Dynamisch:** Zuordnung erfolgt automatisch, z.B. per **802.1X (RADIUS)** oder MAC-basiert. Der Switch fragt den RADIUS-Server; nach Authentifizierung wird der Port dem passenden VLAN zugewiesen. Ideal für mobile Geräte.

### ba) 802.1Q-Tagging – Auswirkungen
- **Eingehend:** Frames mit 802.1Q-Tag werden vom Switch anhand der VLAN-ID dem entsprechenden VLAN zugeordnet. Untagged Frames landen im Native VLAN.
- **Ausgehend:** Frames werden mit dem 802.1Q-Tag (VLAN-ID) versehen, bevor sie den Trunk-Port verlassen. Der Empfänger-Switch kann sie korrekt zuordnen.

### bb) Core-Switch – getaggte Ports
- **Alle Ports, die zu anderen Switches (Trunks) führen:** Port 1, Port 2, Port 3, Port 4, Port 5, Port 6, Port 7 (Uplinks zu den Etagen-Switches und zum Router). Diese Ports müssen getaggt werden, da sie mehrere VLANs transportieren.

### ca) Native VLAN – Auswirkung auf Trunk
- **Wirkung:** Frames im Native VLAN werden **ohne 802.1Q-Tag** über den Trunk gesendet. Der Empfänger ordnet untagged Frames dem konfigurierten Native VLAN zu. Nur ein Native VLAN pro Trunk möglich.

### cb) Native VLAN ≠ Default VLAN (VLAN 1)
- **Grund:** **VLAN-Hopping-Angriff.** Wenn Native VLAN = VLAN 1 (Default), kann ein Angreifer double-tagged Frames senden und in andere VLANs springen. Ein anderes Native VLAN (z.B. 99) verhindert dies, da untagged Frames nicht in VLAN 1 landen.

### d) Switch-Zugriff absichern (4 Einstellungen)
- **Management-IP + VLAN:** Dediziertes Management-VLAN (z.B. VLAN 99); IP nur dort erreichbar.
- **Starke Passwörter/AAA:** Lokale User mit starken Passwörtern oder RADIUS/TACACS+ für Admin-Login.
- **SSH statt Telnet:** Verschlüsselte Remote-Administration; Telnet deaktivieren.
- **Port-Security/802.1X:** Nur autorisierte MAC-Adressen/Geräte an Access-Ports; ungenutzte Ports shutdown.
- **ACL für Management:** Zugriff nur von Admin-Subnetz.

---

## 2. Aufgabe (IP-Adressierung)

### aa) 172.16.0.0/12 in 4 gleich große Subnetze
- **/12 → /14:** 2 Bits für 4 Subnetze.
- **Subnetz 1 (VLAN 10):** 172.16.0.0/14 (Maske 255.252.0.0)
- **Subnetz 2 (VLAN 20):** 172.20.0.0/14 (Maske 255.252.0.0)
- **Subnetz 3 (VLAN 30):** 172.24.0.0/14 (Maske 255.252.0.0)
- **Subnetz 4 (VLAN 40):** 172.28.0.0/14 (Maske 255.252.0.0)

| VLAN | Netzwerk-ID | Subnetzmaske (dezimal) |
|---|---|---|
| 10 | 172.16.0.0 | 255.252.0.0 |
| 20 | 172.20.0.0 | 255.252.0.0 |
| 30 | 172.24.0.0 | 255.252.0.0 |
| 40 | 172.28.0.0 | 255.252.0.0 |

### b) Standard-Gateways (jeweils letzte IP)
| VLAN | IP des Standard-Gateways |
|---|---|
| 10 | 172.19.255.254 |
| 20 | 172.23.255.254 |
| 30 | 172.27.255.254 |
| 40 | 172.31.255.254 |
| 99 (10.0.0.0/8) | 10.255.255.254 |

- **Logik:** Broadcast-Adresse minus 1 = letzte nutzbare Host-IP.

### ac) Dritter RFC-1918-Bereich
- **Netz-ID und CIDR:** **192.168.0.0/16** (Maske 255.255.0.0). Broadcast: 192.168.255.255.

### ba) Client mit 169.254.134.79 – Grund
- **APIPA/Link-Local:** Der Client hat **kein DHCP-Offer** erhalten. Ursache: DHCP-Server nicht erreichbar (VLAN-Trunk fehlt, DHCP-Relay nicht konfiguriert, Server down). Windows vergibt automatisch 169.254.0.0/16.

### bb) Alle Clients in VLAN 10–40 erhalten APIPA – Ursache & Lösung
- **Ursache:** Der **DHCP-Relay** am Core-Switch (Layer-3) ist für die VLANs 10–40 nicht konfiguriert. DHCP-Discover (Broadcast) wird nicht ins VLAN 99 (Server) weitergeleitet. Nur VLAN 99 funktioniert, weil der DHCP-Server dort direkt im selben Broadcast-Segment sitzt.
- **Lösung:** Auf dem Core-Switch für jedes VLAN 10–40 ein **DHCP-Relay (IP-Helper)** mit der IP des DHCP-Servers (10.0.0.x) konfigurieren.

### ca) Default-Route am Core-Switch
- **Grund:** Der Core-Switch ist Layer-3-Gateway für alle VLANs. Alle unbekannten Ziele (Internet) werden an den nächsten Router (192.168.250.2) gesendet. Direkt angeschlossene Netze (VLANs) kennt er bereits; nur die Route ins Internet fehlt.

### cb) Route in Routingtabelle
| Netzwerk-ID | Subnetzmaske (dezimal) | Next-Hop |
|---|---|---|
| 0.0.0.0 | 0.0.0.0 | 192.168.250.2 |

### da) Eine öffentliche IP für alle Geräte – NAT/PAT
- **Prinzip:** **NAT (Network Address Translation)** mit **PAT (Port Address Translation)**. Der Router übersetzt private Quell-IPs in die eine öffentliche IP und vergibt eindeutige Quell-Ports. Antworten werden anhand der Port-Tabelle zurückübersetzt. Bis zu 65.535 gleichzeitige Verbindungen pro öffentlicher IP.

### ea) IPv6-Subnetze aus 2001:db8:1234::/56
- **/56 → /64:** 8 Bits für Subnetze → 2^8 = **256 Subnetze**.

### eb) Erstes und letztes Subnetz
- **1. Subnetz:** 2001:db8:1234:0000::/64 → **2001:db8:1234::/64**
- **Letztes Subnetz:** 2001:db8:1234:00ff::/64 → **2001:db8:1234:ff::/64**

---

## 3. Aufgabe (Fehlersuche)

### aa) Ping-Ausgabe – Rückschluss
- **Rückschluss:** Der Router ist über **IPv6** erreichbar (Ziel 2001:db8:1234::1). Das verwendete Protokoll ist **ICMPv6**. Die Namensauflösung funktioniert (FQDN → IPv6). Kein Paketverlust; Latenz < 1 ms. IPv6 ist aktiv und funktioniert intern.

### b) nslookup – Zeilen 2–7
| Zeile | Erklärung |
|---|---|
| 2 | Verwendeter DNS-Server: ns.ihk-med.de |
| 3 | IP-Adresse des DNS-Servers: 2001:db8:1234::53 (IPv6) |
| 4 | Nicht autorisierende Antwort: aus Cache, nicht vom autoritativen Server |
| 5 | Aufgelöster Name: www.google.de |
| 6 | IPv6-Adresse (AAAA-Record) |
| 7 | IPv4-Adresse (A-Record) |

### b) Übertragungsrate 100 Mbit/s – 2 Ursachen
- **Ursache 1 – Kabel:** Cat-5-Kabel (nur 100 Mbit/s) statt Cat-5e/Cat-6; oder defektes Kabel mit nur 2 Adernpaaren.
- **Ursache 2 – Switch-Port:** Port ist manuell auf 100 Mbit/s (Full-Duplex) konfiguriert statt Auto-Negotiation; oder Fehler an der NIC (Treiber).

### c) Ping fehlschlägt, HTTP funktioniert – Ursache
- **Ursache:** **ICMP wird blockiert** (Firewall/Provider). Der Webserver ist erreichbar (TCP 80 funktioniert via Telnet/Test-NetConnection), aber ICMP Echo Requests werden gefiltert. Ping ist kein zuverlässiger Test für HTTP-Erreichbarkeit.

### d) 404 – File not found (content parameter is null)
- **Ursache:** Der Webserver sucht eine Ressource (z.B. `/index.html`), die nicht existiert oder der Content-Parameter ist falsch/leer. Die URL ist syntaktisch korrekt, aber der Pfad zeigt ins Leere (fehlende Datei, falsche Konfiguration im Reverse Proxy).

### d) FTP-Server nicht erreichbar – DNS-Problem
- **Ursache:** `ftp.ihk-med.de` ist ein **CNAME** auf `ihk-med.de`. Der A-Record von `ihk-med.de` zeigt auf **203.0.113.80** (alten Webserver), nicht auf den neuen FTP-Server `ftp1.ihk-med.de` (203.0.113.21). Clients verbinden sich zum falschen Host.
- **Behebung:** CNAME von `ftp.ihk-med.de` auf **`ftp1.ihk-med.de`** ändern. Dann löst der Name auf 203.0.113.21 auf.

---

## 4. Aufgabe (SSL/TLS Inspection)

### a) Wie NGFW HTTPS untersucht
- **Prinzip:** **Man-in-the-Middle (MITM)**. Die NGFW terminiert die TLS-Verbindung des Clients (präsentiert ein von der NGFW-CA signiertes Zertifikat für die Zielseite). Gleichzeitig baut sie eine zweite TLS-Verbindung zum echten Webserver auf. Der Traffic wird entschlüsselt, inspiziert (IPS, AV, DPI) und wieder verschlüsselt.

### ab) Sicherheitsvorteil SSL/TLS Inspection
- **Vorteil:** **Malware/Exploits in HTTPS erkennen.** Ohne Inspection bleibt der Inhalt verschlüsselt; Schadsoftware kann sich über HTTPS verbreiten (z.B. C2-Traffic, Drive-by-Downloads). NGFW kann Signaturen/Verhalten im Klartext prüfen.

### ac) Zwei Nachteile
- **Datenschutz/DSGVO:** Entschlüsselung persönlicher Daten (Banking, Gesundheitsdaten) erfordert Rechtsgrundlage; Vertrauensverlust der Mitarbeiter.
- **Performance:** TLS-Interception kostet CPU; Latenz steigt. Zudem brechen manche Anwendungen (Certificate Pinning, mTLS) bei MITM ab.

### ad) Rechtliche/organisatorische Maßnahme
- **Betriebsvereinbarung/Einwilligung:** Mitarbeiter informieren (Transparenz), Betriebsrat einbeziehen (§ 87 BetrVG). Datenschutz-Folgenabschätzung (DSFA) durchführen. Nur inspizieren, was rechtlich zulässig ist (keine privaten Banking-Sessions).

### ba) SEC_ERROR_UNKNOWN_ISSUER
- **Ursache:** Der Client vertraut der **NGFW-Intermediate-CA** nicht. Das von der NGFW präsentierte Zertifikat ist von einer CA signiert, die nicht im Trust Store des Clients ist.
- **Behebung:** NGFW-CA (Root/Intermediate) im **Trust Store** aller Clients importieren (GPO/MDM).

### bb) SEC_ERROR_EXPIRED_ISSUER_CERTIFICATE
- **Ursache:** Das **CA-Zertifikat der NGFW ist abgelaufen**. Die NGFW signiert Zertifikate mit einer abgelaufenen Intermediate-CA.
- **Behebung:** Neues CA-Zertifikat auf der NGFW erzeugen/erneuern; altes widerrufen; Clients aktualisieren.

### bc) SSL_ERROR_VERSION_OR_CIPHER_MISMATCH
- **Ursache:** **TLS-Version oder Cipher-Suite nicht kompatibel.** Die NGFW unterstützt z.B. nur TLS 1.2, der Client fordert TLS 1.3 (oder umgekehrt). Oder die Cipher-Suite des Servers wird von der NGFW nicht unterstützt.
- **Behebung:** NGFW-Konfiguration anpassen: TLS 1.2 und 1.3 aktivieren; Cipher-Suiten erweitern (z.B. AES-GCM, ChaCha20).

---

**Merksatz:** IHK-Aufgaben sind **Kausalitätsketten** – wer 802.1Q-Tagging, DHCP-Relay, CNAME-Ketten und TLS-MITM nicht als Hard-Facts sitzen hat, verliert Punkte an der Oberfläche. Lerne Standards als Vokabeln, nicht als Prosa.
