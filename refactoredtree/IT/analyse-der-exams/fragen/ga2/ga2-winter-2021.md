## 1. Aufgabe (Netzwerk erweitern)

### a) Subnetting 172.16.102.0/24 – 4 Abteilungen
- **Abteilung 1 (80 Hosts):** Benötigt 7 Host-Bits (2^7 − 2 = 126). Netz: **172.16.102.0/25** (Maske 255.255.255.128).
- **Abteilung 2 (50 Hosts):** 6 Host-Bits (2^6 − 2 = 62). Netz: **172.16.102.128/26** (Maske 255.255.255.192).
- **Abteilung 3 (20 Hosts):** 5 Host-Bits (2^5 − 2 = 30). Netz: **172.16.102.192/27** (Maske 255.255.255.224).
- **IT (10 Hosts):** 4 Host-Bits (2^4 − 2 = 14). Netz: **172.16.102.224/28** (Maske 255.255.255.240).
- **Freier Block:** 172.16.102.240/28 (16 Adressen) bleibt am Ende.

| Bereich | Anzahl Hosts | Netzdresse | Subnetzmaske (dezimal) |
|---|---|---|---|
| Abteilung 1 | 80 | 172.16.102.0 | 255.255.255.128 |
| Abteilung 2 | 50 | 172.16.102.128 | 255.255.255.192 |
| Abteilung 3 | 20 | 172.16.102.192 | 255.255.255.224 |
| IT | 10 | 172.16.102.224 | 255.255.255.240 |

### b) Statische Routen Router Erfurt (minimale Hops)
- **Topologie:** Erfurt → Köln (10.10.10.41/30), Erfurt → Hamburg (10.10.10.53/30). Hamburg–Berlin direkt **nicht verfügbar**; Berlin über Köln erreichbar.
- **Route zu Köln (172.16.64.0/19):** direkt über eth0 (10.10.10.41) → Next-Hop 10.10.10.42 (Köln-Router).
- **Route zu Köln DMZ (203.0.113.8/29):** über Köln-Router (10.10.10.42), da DMZ an Köln hängt.
- **Route zu Hamburg (172.16.96.0/22):** über eth1 (10.10.10.53) → Next-Hop 10.10.10.54.
- **Route zu Berlin (172.16.100.0/23):** Hamburg–Berlin-Direktlink fehlt → über Köln (10.10.10.42), weil Köln→Berlin existiert.
- **Default-Route (0.0.0.0/0):** über Köln (10.10.10.42), da Köln den Internet-Uplink hat.

| Netzwerk | Subnetzmaske | Schnittstelle | Next-Hop |
|---|---|---|---|
| 172.16.64.0/19 | 255.255.224.0 | eth0 | 10.10.10.42 |
| 203.0.113.8/29 | 255.255.255.248 | eth0 | 10.10.10.42 |
| 172.16.96.0/22 | 255.255.252.0 | eth1 | 10.10.10.54 |
| 172.16.100.0/23 | 255.255.254.0 | eth0 | 10.10.10.42 |
| 0.0.0.0/0 | 0.0.0.0 | eth0 | 10.10.10.42 |

### c) SPI-Firewall-Regeln (eingehend)
| Richtung | Quell-IP | Ziel-IP | Quell-Port | Ziel-Port | Protokoll | Regel |
|---|---|---|---|---|---|---|
| eingehend | any | 203.0.113.10 | any | 443 | TCP | accept |
| eingehend | any | 203.0.113.11 | any | 993 | TCP | accept |
| eingehend | any | 203.0.113.11 | any | 465 | TCP | accept |
| eingehend | any | 203.0.113.11 | any | 587 | TCP | accept |
| eingehend | any | any | any | any | any | drop |

- **Logik:** Webserver (203.0.113.10) nur HTTPS; Mailserver (203.0.113.11) IMAPS (993), SMTPS (465), Submission (587). Alles andere verwerfen.

### d) NGFW – 2 Vorteile vs. Paketfilter
- **Application-Layer-Erkennung (DPI):** Identifiziert Anwendungen unabhängig von Ports (z.B. Facebook over HTTPS) und filtert Inhalte/URLs.
- **IPS/Threat Prevention:** Erkennt und blockiert Exploits, Malware, Command-and-Control-Traffic in Echtzeit; Paketfilter kann das nicht.

### e) Statisch vs. Dynamisch – Entscheidung
- **Empfehlung:** **Dynamisches Routing (OSPF)**.
- **Argument 1 – Redundanz/Failover:** Bei Ausfall der Direktverbindung Hamburg–Berlin oder Erfurt–Köln konvergiert OSPF automatisch und wählt Alternativpfade. Statische Routen müssten manuell angepasst werden.
- **Argument 2 – Skalierbarkeit/Wartbarkeit:** Neue Netze/Leitungen werden automatisch propagiert; weniger Administrationsaufwand und Fehleranfälligkeit.

---

## 2. Aufgabe (WLAN)

### a) Accesspoint vs. WLAN-Controller
- **Accesspoint:** Funk-Schnittstelle; verbindet WLAN-Clients mit dem LAN. Sendet Beacons, verschlüsselt Funkstrecke (WPA2/3), authentifiziert Clients (802.1X).
- **WLAN-Controller:** Zentrale Verwaltung aller APs. Konfiguration (SSID, Kanäle, Power), Firmware-Verteilung, Roaming-Steuerung, Authentifizierung (RADIUS-Anbindung), Monitoring.

### b) WPA2-PSK vs. WPA2-Enterprise – Entscheidung
- **Empfehlung:** **WPA2-Enterprise (802.1X/RADIUS)**.
- **Begründung:** Individuelle Credentials pro Mitarbeiter (AD-Login/Zertifikat) statt gemeinsamer PSK. Kein Schlüssel-Leak bei Weitergabe; zentrale Rechteverwaltung; sofortiger Entzug bei Offboarding. PSK ist bei mehreren Mitarbeitern unsicher.

### c) RADIUS AAA
- **Authentication:** Identitätsprüfung des Clients (EAP, Zertifikat, User/Passwort).
- **Authorization:** Rechtevergabe nach Auth (VLAN-Zuweisung, Bandbreite, Zeitfenster).
- **Accounting:** Protokollierung der Sitzung (Login-Zeit, Dauer, Traffic) für Audit/Abrechnung.

### d) Voucher-System – Definition & 2 Vorteile
- **Definition:** Gast-WLAN mit Captive Portal. Gast erhält zeitlich/volumenmäßig begrenzten Code (Voucher) vom Gastgeber. Portal authentifiziert und schaltet Internet frei.
- **Vorteil 1 – Rechtliche Absicherung:** Nutzungsbedingungen/Haftungsausschluss werden akzeptiert; DSGVO-konform.
- **Vorteil 2 – Kontrolle:** Zeitliche Begrenzung, Bandbreitenlimit, Gerätebegrenzung pro Voucher; kein offenes WLAN.

### e) Switch-Auswahl (NICHT in Frage kommend)
- **Switch A – nicht geeignet:** Nur 8 × 1 Gbit/s RJ45; **keine SFP+**-Uplinks zum Core-Switch. 8 APs × 17,9 W = 143,2 W PoE+ → 250 W Budget reicht, aber fehlende SFP+ verhindert 10G-Uplink.
- **Switch B – nicht geeignet:** PoE-Budget **60 W** reicht nicht für 8 APs × 17,9 W = **143,2 W**. Nur 2 PoE+-Ports.
- **Switch C – geeignet:** 24 × 1 Gbit/s RJ45, PoE+-Budget 180 W (> 143,2 W), 2 × SFP+ für Core-Uplink.
- **Fazit:** **Switch A und B kommen NICHT in Frage.**

---

## 3. Aufgabe (TLS & Zertifikate)

### a) X.509-Zertifikat – Inhalte (4)
- **Subject:** Identität (CN, O, OU, E-Mail) des Webservers.
- **Public Key:** Öffentlicher Schlüssel des Servers.
- **Signatur der CA:** Digitale Signatur der ausstellenden CA.
- **Gültigkeitszeitraum:** Not Before / Not After.
- **Serial Number:** Eindeutige Seriennummer.
- **Extensions:** SAN (Subject Alternative Names), Key Usage.

### c) Hybrides Verfahren (TLS 1.3)
- **Prinzip:** Asymmetrische Kryptographie (z.B. ECDHE) für den **Schlüsselaustausch**; symmetrische Kryptographie (z.B. AES-GCM) für die **Datenverschlüsselung**.
- **Grund:** Asymmetrisch ist sicher (kein gemeinsamer Schlüssel vorab nötig), aber langsam. Symmetrisch ist schnell, aber Schlüsselverteilung problematisch. Hybrid kombiniert beide Vorteile.

---

## 4. Aufgabe (Monitoring & Fehleranalyse)

### aa) SNMP – zwei weitere Werte
| Auszulesender Wert | Datentyp | Beispielhafter Rückgabewert |
|---|---|---|
| CPU-Auslastung | Integer | 45 % |
| Freier Festplattenspeicher | Integer | 120 GB |
| Uptime | TimeTicks | 123456789 |
| Interface-Status | Integer | 1 (up) |

### ab) Nachteil snmpget
- **Prinzip:** Polling – der Manager fragt periodisch jeden Wert einzeln ab. Hohe Latenz, Netzwerklast, verpasst kurzzeitige Events (z.B. Spitzen) zwischen Abfragen.

### ac) Umgehung des Nachteils
- **SNMP Traps/Informs:** Agent sendet bei Ereignis (z.B. Temperatur überschritten) sofort eine Nachricht an den Manager. Event-basiert statt Polling.

### ba) Ping – Zeitüberschreitung
- **Ursache:** ICMP Echo Requests werden vom Ziel oder auf dem Pfad blockiert (Firewall/Provider). DNS-Auflösung funktionierte (IP 141.88.222.152), aber ICMP ist gefiltert. Kein Beweis für Ausfall.

### bb) nslookup – Zeilen 2–7
| Zeile | Erklärung |
|---|---|
| 2 | Verwendeter DNS-Server: router.local |
| 3 | IP-Adresse des DNS-Servers: fe80::1 (Link-Local IPv6) |
| 4 | Nicht autorisierende Antwort: aus Cache, nicht vom autoritativen Server |
| 5 | Aufgelöster Name: www.google.de |
| 6 | IPv6-Adresse (AAAA-Record) |
| 7 | IPv4-Adresse (A-Record) |

### a) Tracert – Zeile mit `* * *`
- **Grund:** Router/Feuerwehr auf dem Pfad blockiert ICMP TTL-Exceeded oder antwortet nicht. Tracert wartet auf Antwort und zeigt nach Timeout `* * *`. Die IP des Hops ist unbekannt, aber der Hop existiert.

### b) Unterschiedliche Zeilenzahl – Ursache
- **Grund:** **Load Balancing / Anycast**: Google betreibt mehrere Server (unterschiedliche IPs: ...815::2003 vs. ...80b::2003). Der zweite Pfad ist länger/kürzer (14 statt 13 Hops), da der DNS-Record eine andere IP liefert und der Traffic über andere Router läuft. Zudem blockieren manche Router ICMP unregelmäßig.

---

**Merksatz:** IHK-Aufgaben sind **Kausalitätsketten** – wer Subnetting (/25–/28), X.509-SAN, RADIUS-AAA und SNMP-Traps nicht als Hard-Facts sitzen hat, verliert Punkte an der Oberfläche. Lerne Standards als Vokabeln, nicht als Prosa.
