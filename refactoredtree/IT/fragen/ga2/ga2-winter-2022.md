## 1. Aufgabe (DMZ & Firewall)

### aa) IP-Adressen der Router-Schnittstellen
- **Netz:** 203.0.113.64/29 → Blockgröße 8: 64–71. Nutzbare Hosts: .65–.70.
- **R- Rusticus- ext (erste IP):** **203.0.113.65**
- **R- Rusticus- int (letzte IP):** **203.0.113.70**

| Router, Schnittstelle | IP-Adresse |
|---|---|
| R- Rusticus- ext, ETH | 203.0.113.65 |
| R- Rusticus- int, ETH0 | 203.0.113.70 |

### ab) Routingtabelle R- Rusticus- ext
| Netzwerk-ID | Subnetzmaske | Schnittstelle oder Next-Hop |
|---|---|---|
| 203.0.113.64 | 255.255.255.248 | direkt (DMZ) |
| 0.0.0.0 | 0.0.0.0 | ISP-Gateway (z.B. 203.0.113.1) |
| internes Firmennetz | (laut Plan) | 203.0.113.70 (R- Rusticus- int) |

- **Logik:** Default-Route zum ISP; Route zum internen Firmennetz über R- Rusticus- int; direkt angeschlossenes DMZ-Netz (kein Next-Hop).

### b) SPI vs. ohne SPI
- **Ohne SPI (reiner Paketfilter):** Jedes Paket wird isoliert anhand von Regeln (IP, Port, Protokoll) geprüft. Antwortpakete auf ausgehende Verbindungen müssen durch **explizite Regeln** erlaubt werden. State wird nicht berücksichtigt.
- **Mit SPI:** Führt eine **State Table** (Verbindungstabelle). Ausgehende Verbindungen werden dynamisch erlaubt; eingehende Antwortpakete werden automatisch zugelassen, solange die Session aktiv ist. Verhindert Spoofing/Port-Scanning effektiver.

### c) Firewall-Regeln (aus Internet regulieren)
| Aktion | Protokoll | Quelle | Ziel | Quellport | Zielport | Von Interface | Nach Interface |
|---|---|---|---|---|---|---|---|
| accept | TCP | any | Webserver | any | 80 | ext | int |
| accept | TCP | any | Webserver | any | 443 | ext | int |
| accept | TCP | any | Mailserver | any | 25 | ext | int |
| accept | TCP | any | Mailserver | any | 143 | ext | int |
| drop | any | any | any | any | any | ext | any |

- **Logik:** Nur HTTP/HTTPS (Web) und SMTP/IMAP (Mail) von außen erlauben. Alles andere verwerfen.

---

## 2. Aufgabe (IPv6 & Netzwerkanalyse)

### a) Warum zuerst DNS-Query?
- **Grund:** Der Client kennt nur den FQDN `www.google.de`, nicht die IP-Adresse. Für den Ping (ICMPv6) wird die IP benötigt. Also fragt der Client zuerst den DNS-Resolver (A/AAAA-Query), um die IPv6-Adresse zu erhalten.

### b) Rückgabewert der DNS-Antwort (Frame 126)
- **Antwort:** Die DNS-Response enthält den **AAAA-Record** mit der IPv6-Adresse **2a00:1450:4001:812::2003** für `www.google.de`.

### c) Lokale IPv4- und genutzte IPv6-Adresse des Clients
| IPv4 | IPv6 |
|---|---|
| 192.168.12.254 (oder laut Mitschnitt) | 2001:db8:... (Link-Local / Global aus Mitschnitt) |

- **Hinweis:** Aus dem Mitschnitt: Source der DNS-Query = lokale IPv6-Adresse (z.B. `fe80::...` oder `2001:...`). IPv4 aus ARP/anderen Frames.

### d) IP-Adresse des lokalen DNS-Servers
- **Adresse:** 192.168.12.254 (aus Frame 126: Destination der DNS-Query). Alternativ die IPv6-Adresse des Routers (Link-Local).

### e) Warum PING an 2a00:...812::2003 statt ...4016:809::2003?
- **Grund:** Der DNS-Resolver hat für `www.google.de` den AAAA-Record **2a00:1450:4001:812::2003** zurückgegeben. Google nutzt **Anycast/Load Balancing** mit vielen IPs; der Resolver cached/erhält genau diese Adresse. Der Client pingt die vom DNS gelieferte IP.

### f) Anzahl Hops (aus Frames)
- **Berechnung:** IPv6 Hop Limit im Ping-Request (z.B. 64) minus Hop Limit in der Ping-Antwort (z.B. 51) = **13 Hops**.
- **Formel:** Hops = Initial-TTL − Empfangs-TTL.

---

## 3. Aufgabe (TLS)

### a) Zwei Eigenschaften TLS vs. TCP/IP
- **Verschlüsselung (Vertraulichkeit):** Daten werden symmetrisch (AES) verschlüsselt; Lauschangriffe sehen nur Ciphertext.
- **Integrität/Authentizität:** HMAC/AEAD verhindert Manipulation; X.509-Zertifikat authentifiziert den Server (Chain of Trust).

### b) Warum TLS 1.2 nicht mehr zulassen? (3 Argumente aus RFC 8446)
- **Veraltete Algorithmen entfernt:** TLS 1.3 entfernt Legacy-Cipher (z.B. RC4, 3DES, SHA-1). TLS 1.2 erlaubt noch schwache Suites.
- **Forward Secrecy (PFS):** Statisches RSA/DH wurde entfernt; alle Key-Exchanges bieten PFS. Bei TLS 1.2 ohne PFS kann ein später kompromittierter Private Key vergangene Sessions entschlüsseln.
- **Verschlüsselter Handshake:** In TLS 1.3 sind alle Handshake-Nachrichten nach ServerHello verschlüsselt (weniger Metadaten-Leak). TLS 1.2 überträgt Zertifikat/Handshake teils im Klartext.

### c) Interne vs. externe CA – Entscheidung
- **Empfehlung:** **Externe CA** (z.B. DigiCert, Let's Encrypt).
- **Argument 1 – Vertrauen:** Öffentliche Clients (Außendienst) vertrauen externen CAs bereits (Root-Store). Interne CA müsste auf jedem Client manuell importiert werden.
- **Argument 2 – Management/Aufwand:** Externe CA übernimmt Revocation (CRL/OCSP), Compliance, Rotation. Interne CA erfordert eigene PKI-Infrastruktur und Personal.

### d) Client-Authentifizierung – je ein Vorteil
| Verfahren | Vorteil |
|---|---|
| 1. Username + Password | Einfach, keine zusätzliche Hardware; überall verfügbar. |
| 2. Username + Password + Client-Zertifikat | Zwei Faktoren (Wissen + Besitz); Client-Zertifikat ist fälschungssicher, an Gerät gebunden. |
| 3. Username + Password + Krypto-Smart-Card | Physischer Faktor; Smart-Card ist kopiergeschützt, PIN-geschützt; sehr hohe Sicherheit. |
| 4. Username + Password + OTP (Authenticator-App) | Zeitbasierter Einmalcode; phishing-resistent (Code nur kurz gültig); keine Hardware nötig. |

---

## 4. Aufgabe (VoIP & Videokonferenz)

### a) Bandbreite VoIP (G.722, 10% Overhead)
- **Codec:** G.722 = 64 kbit/s pro Gespräch.
- **Overhead:** 10% → 64 × 1,10 = **70,4 kbit/s** pro Gespräch.
- **Gleichzeitige Gespräche:** 5 intern + 15 extern = **20 Gespräche**.
- **Gesamt:** 20 × 70,4 = **1.408 kbit/s**.

### ba) Bandbreite nicht ausreichend – Verhalten
- **Verhalten:** Die Software priorisiert **Audio vor Video**. Audio bleibt stabil; Videoqualität wird reduziert (niedrigere Auflösung, weniger fps) oder Video deaktiviert.

### bb) Drei Faktoren für aktuelle Bandbreite
- **Video-Layout:** Anzahl gleichzeitig sichtbarer Teilnehmer (Galerie vs. Spotlight).
- **Video-Auflösung:** 1080p vs. 720p vs. 360p.
- **Frames pro Sekunde (fps):** 30 fps vs. 15 fps.
- **Inhaltsfreigabe (Content Sharing):** Bildschirmfreigabe erhöht Bandbreite.

### c) Zusätzlicher Traffic (10 Personen, empfohlene Bandbreite)
- **Tabelle (Recommended, Meetings):** Audio 58/58 kbit/s, Video **2.500/4.000 kbit/s** up/down.
- **Upload (pro Teilnehmer):** 58 + 2.500 = **2.558 kbit/s**.
- **Download (pro Teilnehmer):** 58 + 4.000 = **4.058 kbit/s**.
- **Gesamt für 10 Teilnehmer (zentraler Internetanschluss):** Upload = 10 × 2.558 = **25.580 kbit/s**; Download = 10 × 4.058 = **40.580 kbit/s**.

### d) Fax über VoIP – Problem
- **Ursache:** Fax (T.30) nutzt **T.38** oder **G.711-Passthrough**. VoIP komprimiert (G.722, Opus) und paketiert; **Jitter, Packet Loss und Latenz** führen zu Synchronisationsfehlern. Fax-Modems sind für **verlustfreie, konstante Bitrate** (64 kbit/s G.711) ausgelegt. Bei mehrseitigen Faxen summieren sich Fehler → unvollständige Übertragung.

---

**Merksatz:** IHK-Aufgaben sind **Kausalitätsketten** – wer Subnetting (/29), TLS-1.3-PFS, SPI-State-Table und VoIP-Bandbreitenformel nicht als Hard-Facts sitzen hat, verliert Punkte an der Oberfläche. Lerne Standards als Vokabeln, nicht als Prosa.
