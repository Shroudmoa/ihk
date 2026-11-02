# OpenVPN

## In 30 Sekunden

- **OpenVPN** ist ein Open-Source-VPN auf Basis von **TLS** (OpenSSL): TLS-Handshake für Authentifizierung und Schlüsselaustausch, symmetrische Verschlüsselung (z. B. AES-256-GCM) für die Daten.
- Standardport **UDP 1194**; alternativ **TCP 443** – sieht dann wie HTTPS aus und kommt durch restriktive Hotspots und Firewalls (GA2 Sommer 2023).
- Authentifizierung über eine **PKI** (Server- und Client-Zertifikate), optional zusätzlich Benutzername/Passwort und MFA.
- Läuft auf praktisch jedem Betriebssystem; typisch für **Remote Access**, auch Site-to-Site möglich.
- Nachteil: mehr Overhead und klassisch geringerer Durchsatz als WireGuard/IPsec; TCP-Betrieb kann bei Paketverlust stark bremsen.

## Kernwissen

### Funktionsweise

| Kanal | Aufgabe |
|---|---|
| **Control Channel** | TLS-Handshake: Zertifikate prüfen, Schlüssel aushandeln, Konfiguration übertragen (IP, Routen, DNS) |
| **Data Channel** | Nutzdaten, symmetrisch verschlüsselt und mit Integritätsschutz |

| Begriff | Bedeutung |
|---|---|
| **tun** | Layer-3-Tunnel (IP-Pakete, geroutet) – Standard |
| **tap** | Layer-2-Tunnel (Ethernet-Frames, gebridged) – selten, mehr Overhead |
| `tls-crypt` / `tls-auth` | zusätzlicher statischer Schlüssel schützt den Handshake gegen Scans und DoS |
| Push-Optionen | Server verteilt Routen, DNS-Server, Full- oder Split-Tunnel an die Clients |
| `easy-rsa` | Werkzeug für die eigene CA |

### UDP oder TCP?

| | UDP 1194 | TCP 443 |
|---|---|---|
| Leistung | besser, kein doppeltes Nachsenden | schlechter: **TCP-über-TCP** – bei Paketverlust wiederholen innere und äußere Verbindung gleichzeitig („TCP Meltdown“) |
| Firewall/Hotspot | wird oft blockiert | kommt fast immer durch |
| Empfehlung | Standard | Fallback für restriktive Netze |

### Vor- und Nachteile

- **Vorteile:** plattformunabhängig, sehr flexibel (UDP/TCP, beliebiger Port, Push-Konfiguration), ausgereift und gut dokumentiert, PKI mit Widerruf (CRL), MFA-Anbindung (RADIUS/LDAP).
- **Nachteile:** Konfiguration und PKI-Aufbau aufwendiger als WireGuard, höherer Protokoll-Overhead, klassisch im User-Space und dadurch langsamer (neuere Versionen können den Datenkanal in den Kernel auslagern), Client-Software nötig.

### Firewallregel (GA2 Sommer 2026)

Eingehend auf die WAN-IP des VPN-Gateways nur **UDP 1194** (bzw. das konfigurierte Protokoll/Port) erlauben, alles andere eingehend verwerfen – die SPI-Firewall lässt die Antworten automatisch zu.

## Prüfungsfrage

**Frage:** Mitarbeiter sollen sich aus Hotels, Zügen und Kundennetzen zuverlässig per VPN mit der Firma verbinden. Begründen Sie, warum OpenVPN mit TCP 443 als Fallback geeignet ist, und nennen Sie einen Nachteil dieser Betriebsart.

**Musterantwort:** Viele öffentliche Netze lassen nur Webverkehr (TCP 80/443) zu und sperren typische VPN-Ports wie UDP 500/4500 oder 1194. OpenVPN kann auf **TCP 443** betrieben werden und nutzt TLS, sodass die Verbindung wie normales HTTPS behandelt wird und durchkommt. Nachteil: Wird TCP im Tunnel über eine TCP-Verbindung transportiert, wiederholen bei Paketverlust beide Ebenen ihre Übertragungen; Durchsatz und Latenz verschlechtern sich deutlich. Daher wird UDP 1194 als Standard und TCP 443 nur als Fallback eingesetzt.

## Falle

- OpenVPN mit IPsec gleichsetzen → OpenVPN nutzt **TLS**, nicht IKE/ESP.
- „Port 443 = automatisch sicher“ → sicher wird es durch TLS und Zertifikatsprüfung, nicht durch den Port.
- tun und tap verwechseln: **tun = Layer 3**, tap = Layer 2.
- Client-Zertifikate ohne Widerrufsprozess ausgeben → verlorenes Notebook bleibt berechtigt.

## Merksatz

**„OpenVPN spricht TLS, mag UDP 1194 und schleicht sich zur Not über 443.“**

## Verwandt

- [VPN – Überblick](readme.md) – Vergleich aller Protokolle
- [IPsec](IPsec.md) – Standard für Site-to-Site
- [WireGuard](wireguard.md) – schnellere Alternative
- [TCP vs. UDP](../../FiSi-Administrator/OSI-IP-mac-bit-byte/tcp-udp.md) – warum TCP-über-TCP bremst
- [PKI](../../FiSi-Administrator/Sicherheit/Verschluesselung/pki.md) – CA und Client-Zertifikate
- [Lektion 27 – PKI, TLS & VPN](../../analyse-der-exams/L27_PKI_TLS_Verschluesselung_VPN.md)
