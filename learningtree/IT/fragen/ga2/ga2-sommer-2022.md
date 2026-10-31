## 1. Aufgabe (Netzwerkplanung & VPN)

### a) IPv4-Subnetz 203.0.113.180/27
- **Netz-ID:** 203.0.113.160 (Blockgröße 32: 160–191)
- **Erste Host-IP:** 203.0.113.161
- **Letzte Host-IP:** 203.0.113.190
- **Broadcast:** 203.0.113.191
- **Subnetzmaske (dezimal):** 255.255.255.224

### b) IPv6-Subnetze aus 2001:db8:ca::/56
- **SLAAC-Vorgabe:** Subnetz-Präfix muss **/64** sein (64 Bit Interface-ID für SLAAC).
- **Verfügbare Bits:** /56 → 8 Bits für Subnetze (64 − 56 = 8).
- **Maximale Subnetze:** 2^8 = **256 Subnetze**.
- **Erstes Subnetz:** 2001:db8:ca:0000::/64 → **2001:db8:ca::/64**
- **Letztes Subnetz:** 2001:db8:ca:00ff::/64 → **2001:db8:ca:ff::/64**

### ba) IP-Adresstypen
| | Öffentliche IPv4 | Private IPv4 | Global Unicast IPv6 | Link-Local IPv6 |
|---|---|---|---|---|
| **Herr Meier** | X (142.250.186.130) | | X (2003:ee:...) | |
| **Frau Schmitt** | | X (172.30.160.253) | X (2002:...) | |
| **Herr Pfeiffer** | | X (10.16.130.129) | X (2001:...) | |

- **Hinweis:** 172.16.0.0/12 und 10.0.0.0/8 sind private IPv4-Bereiche. IPv6-Adressen mit 2000::/3 sind Global Unicast.

### bb) Internet-Anbindung
| | IPv4 only | IPv6 only | Dual Stack |
|---|---|---|---|
| **Herr Meier** | | | X (IPv4 + IPv6) |
| **Frau Schmitt** | | | X (IPv4 + IPv6) |
| **Herr Pfeiffer** | | | X (IPv4 + IPv6) |

### bc) Übertragungsmedium „Letzte Meile“
| | Mobilfunk | Telefonkabel | Koaxialkabel |
|---|---|---|---|
| **Herr Meier (VDSL)** | | X | |
| **Frau Schmitt (DOCSIS)** | | | X |
| **Herr Pfeiffer (LTE)** | X | | |

### ca) VPN – Fachbegriff und Definition
- **Fachbegriff:** Virtual Private Network.
- **Definition:** Logisches, verschlüsseltes Netzwerk über ein unsicheres Medium (Internet). Aufbau eines Tunnels zwischen Client und Firmennetz (z.B. IPsec, OpenVPN, WireGuard); gewährleistet Vertraulichkeit, Integrität und Authentizität.

### cb) Vorteile Full-Tunnel-VPN (2)
- **Zentrale Sicherheitskontrolle:** Jeglicher Traffic (auch privater) läuft über die Firmen-Firewall; Malware/Phishing wird zentral gefiltert. Kein Bypass über unsichere Netze.
- **Schutz vor Datenabfluss (Data Leakage):** Firmendaten verlassen das Netz nur verschlüsselt über den VPN-Tunnel. Split-Tunneling wird verhindert; sensibler Traffic wird nicht im Klartext über öffentliche WLANs gesendet.

### cc) VPN-Subnetze aus 172.20.0.0/16 (RFC 3021, /31)
- **RFC 3021:** /31-Netze für Point-to-Point; 2 gültige IP-Adressen, kein Broadcast.
- **Verfügbare Adressen:** 172.20.0.0/16 = 65.536 Adressen.
- **Pro VPN:** 2 Adressen (/31).
- **Maximale VPNs:** 65.536 / 2 = **32.768 VPNs**.

---

## 2. Aufgabe (WLAN & VLANs)

### aa) Technische Aspekte WLAN-Planung (3)
- **Frequenzplanung/Kanäle:** 2,4 GHz (nur 3 überlappungsfreie Kanäle: 1, 6, 11) vs. 5 GHz (mehr Kanäle, weniger Störung). Vermeidung von Ko-Kanal-Interferenzen.
- **Abdeckung/Signalstärke:** Site-Survey (Heatmap); Wandmaterialien (Beton, Metall) dämpfen; Anzahl APs und Positionierung.
- **Kapazität/Client-Dichte:** Anzahl gleichzeitiger Clients pro AP; Bandbreitenbedarf (VoIP, Video).
- **Sicherheit:** WPA2/WPA3-Enterprise (RADIUS), Gast-VLAN, Captive Portal.

### ab) RADIUS AAA – Komponenten
- **Authentication (Authentifizierung):** Identitätsprüfung des Benutzers (z.B. Benutzername/Passwort, Zertifikat, EAP). Wer bist du?
- **Authorization (Autorisierung):** Festlegung der Rechte nach erfolgreicher Authentifizierung (z.B. VLAN-Zuweisung, Zeitfenster, Bandbreite).
- **Accounting (Abrechnung/Protokollierung):** Erfassung von Sitzungsdaten (Login-Zeit, Dauer, Traffic) für Audit und Abrechnung.

### ac) RADIUS AAA vs. PSK – 2 Vorteile
- **Individuelle Credentials:** Jeder Benutzer hat eigene Zugangsdaten (User/Passwort oder Zertifikat). Kein gemeinsamer PSK, der bei Weitergabe kompromittiert ist.
- **Zentrale Rechteverwaltung:** Autorisierung pro Benutzer/Gruppe; dynamische VLAN-Zuweisung; sofortiger Entzug bei Offboarding. Accounting für Audit.

### ad) Gast-WLAN – Voraussetzungen
| Voraussetzung | Begründung |
|---|---|
| **Eigene SSID** | Trennung des Gast-Traffics vom Firmennetz; eigene Verschlüsselung/Policy. Gäste sehen nicht die interne SSID. |
| **Eigener IP-Adressbereich (VLAN)** | Isolation auf Layer 3; Gäste erreichen keine internen Server (AD, DB). Firewall-Regeln zwischen VLANs. |
| **Captive Portal (Voucher)** | Rechtliche Absicherung (Nutzungsbedingungen, Haftungsausschluss); Zugangskontrolle per Voucher/Code; zeitliche Begrenzung. |

### ba) VLAN-Vorteile (2)
- **Sicherheit/Isolation:** Trennung von Abteilungen (z.B. HR, Entwicklung, Gast) auf Layer 2. Broadcast-Domänen werden getrennt; kein unbefugter Zugriff.
- **Flexibilität:** Logische Gruppierung unabhängig von physischer Verkabelung; einfache Änderung per Konfiguration statt Patchfeld.

### bb) Uplink Etagen-Switch ↔ Core-Switch
- **Einstellung:** **Trunk-Port** (802.1Q). Tagged Frames für alle VLANs; Native VLAN für untagged Traffic. Erlaubt Übertragung mehrerer VLANs über eine physische Verbindung.

### bc) Schutz vor fremden Endgeräten an Netzwerkdosen
- **802.1X (Port-Based Network Access Control):** Authentifizierung des Endgeräts/Benutzers am Switch-Port via RADIUS. Ohne gültige Credentials bleibt der Port im VLAN „Guest“ oder wird deaktiviert.
- **MAC-Filtering/Port-Security:** Nur bekannte MAC-Adressen; bei fremder MAC wird Port gesperrt (Sticky MAC).

---

## 3. Aufgabe (Proxy & DMZ)

### a) HTTPS-Scanoptionen
- **Option „Entschlüssel und scannen“:** Erforderlich, um Schadsoftware im HTTPS-Traffic zu erkennen (SSL/TLS-Interception via MITM). Proxy agiert als Man-in-the-Middle mit eigenem CA-Zertifikat; Clients müssen die Proxy-CA vertrauen.
- **Nur URL-Filterung:** Reicht nicht; HTTPS-Inhalt bleibt verschlüsselt. Schadsoftware kann nicht erkannt werden. Widerspricht der Vorgabe „Schadsoftware scannen“.

### ba) Fehlermeldung NET::ERR_CERT_AUTHORITY_INVALID
- **Ursache:** Der Proxy entschlüsselt HTTPS (MITM) und präsentiert ein Zertifikat, das von der Proxy-CA signiert wurde. Der Client vertraut dieser CA nicht (nicht im Trust Store). Browser warnt vor „nicht vertrauenswürdiger Zertifizierungsstelle“.

### bb) Vermeidung der Fehlermeldung
- **Proxy-CA im Trust Store installieren:** Das Root-Zertifikat der Proxy-CA muss auf allen Clients (Windows, macOS, Browser) als vertrauenswürdig importiert werden (GPO/Intune). Dann wird die Signatur akzeptiert.

### ca) Reverse-Proxy – 2 Funktionen
- **Lastverteilung/TLS-Terminierung:** Eingehende HTTPS-Anfragen werden am Reverse-Proxy terminiert (TLS-Offloading) und auf mehrere Backend-Webserver verteilt. Entlastet die Webserver von Krypto.
- **Sicherheit/Filtering:** Verbirgt die Backend-IPs; Web Application Firewall (WAF) filtert Angriffe (SQLi, XSS) vor dem Webserver. Caching statischer Inhalte.

### cb) DMZ – Funktionsweise
- **Prinzip:** Pufferzone zwischen unsicherem Internet und internem LAN. Externe Dienste (Web, Mail) stehen in der DMZ; Firewall-Regeln erlauben Zugriff von außen nur auf DMZ-Hosts, nicht direkt ins LAN. Bei Kompromittierung eines DMZ-Hosts bleibt das interne Netz geschützt (Segmentierung).

### cc) DMZ vs. internes Netz
- **DMZ:** **Web-Server** (und ggf. **Reverse-Proxy**). Begründung: Direkt aus dem Internet erreichbar; hohes Risiko. Muss von außen zugänglich sein, aber isoliert vom LAN.
- **Intern:** **AD-Server** (und **Datenbank-Server**). Begründung: Enthält sensible Identitätsdaten; darf niemals direkt aus dem Internet erreichbar sein. Nur über Proxy/App-Server aus der DMZ ansprechbar (Layer-3-Firewall).

---

## 4. Aufgabe (WAN & Mail-Filter)

### aa) Videokonferenz-Störungen (16 Mbit/s ADSL)
- **Ursache:** Zu geringe Upload-Bandbreite (ADSL asymmetrisch: 16 Mbit/s down, oft nur 1–2 Mbit/s up). Videokonferenzen benötigen symmetrische Bandbreite (z.B. 2–4 Mbit/s up/down). Bei Überlastung: Packet Loss, Jitter, Latenz → schlechte Bild-/Sprachqualität.

### ab) Übertragungsdauer 1.634 MiB bei 16 Mbit/s
- **Umrechnung:** 1.634 MiB × 8 = 13.072 Mibit (Megabit).
- **Dauer:** 13.072 Mbit / 16 Mbit/s = 817 s.
- **In Minuten:** 817 / 60 ≈ 13,62 → **14 Minuten** (aufgerundet).

### ba) Spamfilter-Kriterien (3)
- **Betreff-/Header-Analyse:** Verdächtige Keywords („Rechnung“, „dringend“), fehlende/gefälschte Header (`Received`-Kette).
- **Link-/URL-Reputation:** Links zu bekannten Phishing-Domains (DNSBL/URIBL); URL-Shortener.
- **Anhang-Typ/Inhalt:** Ausführbare Dateien (.exe, .js), Makros (.docm), Passwort-geschützte Archive.
- **Textanalyse (Bayes):** Statistische Klassifikation von Spam-Texten; Heuristik.
- **SPF/DKIM/DMARC-Fail:** Absender-Domain nicht autorisiert.

### bb) Briefgeheimnis – Bedingung für Mail-Scan
- **Bedingung:** Einwilligung des Mitarbeiters (Betriebsvereinbarung) oder **konkreter Verdacht** auf Straftat/Verstoß. Dann ist eine gezielte Prüfung durch den Administrator zulässig (§ 206 StGB, BDSG). Automatisierte Filterung (Spam/Malware) ist durch berechtigtes Interesse (Art. 6 DSGVO) gedeckt; inhaltliche Analyse nur bei begründetem Anlass.

### bc) Sandbox – Funktion
- **Prinzip:** Unbekannte Anhänge werden in einer isolierten virtuellen Umgebung (Sandbox) ausgeführt. Verhalten wird überwacht (Datei-/Registry-Änderungen, Netzwerkverbindungen). Bei verdächtigem Verhalten wird die Mail blockiert/quarantäniert. Schützt vor Zero-Day-Malware, die signaturbasiert nicht erkannt wird.

---

**Merksatz:** IHK-Aufgaben sind **Kausalitätsketten** – wer Subnetting (/27, /56→/64), RFC 3021 (/31) und Proxy-MITM (CA-Trust) nicht als Hard-Facts sitzen hat, verliert Punkte an der Oberfläche. Lerne Standards als Vokabeln, nicht als Prosa.
