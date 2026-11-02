# VPN (Virtual Private Network)

## In 30 Sekunden

- Ein **VPN** ist ein **logisches, verschlüsseltes Netz über ein unsicheres Netz** (Internet): Ein **Tunnel** kapselt die Originalpakete und schützt **Vertraulichkeit, Integrität und Authentizität** (GA2 Sommer 2022, Winter 2025).
- Topologien: **Site-to-Site** (Standort ↔ Standort, auf den Routern/Firewalls), **End-to-Site / Remote Access** (Client ↔ Firmen-Gateway), **End-to-End** (Host ↔ Host).
- **Full Tunnel** = gesamter Verkehr durch den Tunnel (zentrale Filterung); **Split Tunnel** = nur Firmennetze durch den Tunnel (entlastet, aber weniger Kontrolle).
- Protokolle: **IPsec** (Standard für Site-to-Site, UDP 500/4500), **OpenVPN** (TLS, UDP 1194 oder TCP 443), **WireGuard** (UDP 51820, schlank und schnell).
- Typische Fehler: Hotspot blockiert VPN-Ports, NAT ohne **NAT-Traversal**, falsche Zugangsdaten/Zertifikate (2023S, 2025W).

## Kernwissen

### Topologien

| Typ | Endpunkte | Eingerichtet auf | Beispiel |
|---|---|---|---|
| **Site-to-Site** | Netz ↔ Netz | Router/Firewall beider Standorte, dauerhaft | Filiale ↔ Zentrale per IPsec |
| **End-to-Site** (Remote Access) | einzelner Client ↔ Firmennetz | VPN-Client auf dem Notebook + VPN-Gateway in der Zentrale | Homeoffice, Außendienst |
| **End-to-End** | Host ↔ Host | beide Endgeräte | Admin-PC ↔ einzelner Server |

### Full Tunnel vs. Split Tunnel (GA2 Sommer 2022)

| | Full Tunnel | Split Tunnel |
|---|---|---|
| Verkehr durch den Tunnel | **alles**, auch Internet | nur Firmennetze |
| Vorteile | zentrale Sicherheitsfilter (Proxy, Firewall, AV) für alle Zugriffe; kein ungeschützter Verkehr im Hotel-WLAN; Data-Leakage-Schutz | weniger Last auf Zentrale und Leitung, schnellere Internetnutzung |
| Nachteile | hohe Bandbreite in der Zentrale, Latenz | Client ist parallel direkt im Internet → Angriffsweg ins Firmennetz |

### Protokolle im Vergleich

| | IPsec (IKEv2) | OpenVPN | WireGuard | SSL-VPN (Browser/Client) |
|---|---|---|---|---|
| Basis | IETF-Standard, Schicht 3 | TLS (OpenSSL) | Noise-Protokoll, moderne Kryptografie | TLS über HTTPS |
| Ports | UDP 500 (IKE), UDP 4500 (NAT-T), ESP = IP-Protokoll 50 | UDP 1194 (Standard), TCP 443 möglich | UDP 51820 (Standard) | TCP 443 |
| Authentifizierung | PSK oder Zertifikate (X.509) | Zertifikate (PKI), optional Benutzer/Passwort/MFA | statische Public Keys je Peer | Benutzer, MFA, Zertifikate |
| Stärken | in fast jeder Firewall/Router nativ, Hardwarebeschleunigung | flexibel, läuft durch restriktive Firewalls (TCP 443) | sehr schnell, wenig Code, einfaches Roaming | clientlos bzw. einfach, Hotspot-tauglich |
| Schwächen | komplexe Konfiguration, NAT-Probleme ohne NAT-T | mehr Overhead, klassisch langsamer | nur UDP, keine eingebaute Benutzer-/Zertifikatsverwaltung | an Hersteller gebunden |
| Typisch | **Site-to-Site** | Remote Access | Remote Access, Site-to-Site | Remote Access |

Veraltet: **PPTP** (TCP 1723, gebrochen), **L2TP/IPsec** (UDP 1701 + IPsec) nur noch für Altgeräte.

### Schutzziele (GA2 Winter 2025)

| Schutzziel | Umsetzung im VPN |
|---|---|
| Vertraulichkeit | symmetrische Verschlüsselung (AES-GCM, ChaCha20) |
| Integrität | HMAC/AEAD erkennt Manipulation |
| Authentizität | PSK, Zertifikate, Public Keys der Gegenstelle |
| (Verfügbarkeit) | **nicht** durch VPN – eher zusätzliche Abhängigkeit |

### Adressierung im Tunnel (GA2 Sommer 2022)

Für Punkt-zu-Punkt-Tunnel genügen **/31-Netze** (RFC 3021, beide Adressen nutzbar) oder /30. Aus `172.20.0.0/16` lassen sich 2^(31−16) = **32.768** /31-Tunnelnetze bilden. In der Routingtabelle zeigen Routen zu entfernten Netzen auf das **Tunnel-Interface** (`tun0`).

### Typische Fehlerursachen

| Symptom | Ursache | Lösung |
|---|---|---|
| VPN im Hotel/Hotspot baut nicht auf (2023S) | UDP 500/4500/1194/51820 gesperrt, nur 80/443 offen | VPN über **TCP 443** (OpenVPN, SSL-VPN) |
| IPsec hinter NAT scheitert | **ESP hat keine Ports** → NAT kann nicht zuordnen | **NAT-Traversal**: ESP in UDP 4500 kapseln |
| Neuer Nutzer kommt nicht rein (2025W) | falsche Zugangsdaten/PSK, Zertifikat fehlt/abgelaufen, Benutzer nicht in der berechtigten Gruppe, Quell-IP nicht freigegeben | Konto/Zertifikat prüfen, Gruppenzuordnung, Firewallregel |
| Tunnel steht, aber kein Zugriff | fehlende Routen/Rückroute, überlappende Subnetze (beide Standorte `192.168.1.0/24`) | Routen ergänzen, eindeutige Adressbereiche planen |
| Eingehendes VPN zum Homeoffice unmöglich | Anschluss mit **CGNAT/DS-Lite** | Verbindung von innen nach außen aufbauen, IPv6, feste IP buchen |

## Prüfungsfrage

**Frage (angelehnt an GA2 Winter 2025):** Die Werkstatt soll dauerhaft mit der Zentrale verbunden werden; Busfahrer sollen unterwegs mit Tablets auf das Buchungssystem zugreifen. Nennen Sie jeweils den VPN-Typ, die Geräte, auf denen er eingerichtet wird, und zwei Schutzziele, die ein VPN gewährleistet.

**Musterantwort:** Werkstatt ↔ Zentrale: **Site-to-Site-VPN** (z. B. IPsec), eingerichtet auf den **Routern/Firewalls beider Standorte**; die Endgeräte benötigen keine VPN-Software. Busfahrer: **End-to-Site-VPN (Remote Access)**, VPN-Client auf dem **Tablet** und VPN-Gateway auf der **Firewall der Zentrale**. Schutzziele: **Vertraulichkeit** durch Verschlüsselung der Daten im Tunnel und **Integrität/Authentizität** durch kryptografische Prüfsummen und die Authentifizierung der Gegenstelle per Zertifikat oder Schlüssel.

## Falle

- „VPN = anonym surfen“ → im Unternehmen geht es um **sichere Kopplung**, nicht Anonymität.
- Site-to-Site mit Client-Software auf jedem PC beschreiben → Site-to-Site läuft auf den **Gateways**.
- VPN erhöht die **Verfügbarkeit** → nein, eher eine zusätzliche Fehlerquelle.
- IPsec ESP einen Port zuordnen → ESP ist **IP-Protokoll 50**, Ports gibt es erst durch NAT-T (UDP 4500).
- Split Tunnel als „sicherer“ bezeichnen → er ist bequemer, Full Tunnel gibt mehr Kontrolle.

## Merksatz

**„Site-to-Site verbindet Netze auf Routern, End-to-Site holt den Laptop ins Büro – Tunnel schützt Vertraulichkeit und Integrität, nicht Verfügbarkeit.“**

## Verwandt

- [IPsec](IPsec.md) – Modi, AH/ESP, IKEv2, NAT-T
- [OpenVPN](openvpn.md) – TLS-basiertes VPN
- [WireGuard](wireguard.md) – schlankes UDP-VPN
- [PKI](../../FiSi-Administrator/Sicherheit/Verschluesselung/pki.md) – Zertifikate für die Authentifizierung
- [Symmetrische und asymmetrische Verschlüsselung](../../FiSi-Administrator/Sicherheit/Verschluesselung/symmetrische-und-asymmetrische.md) – hybride Verfahren
- [Firewall](../../FiSi-Administrator/Sicherheit/firewall/firewall.md) – Regeln für VPN-Ports
- [NAT & PAT](../../FiSi-Administrator/Sicherheit/firewall/nat-pat.md) – warum IPsec NAT-T braucht
- [Routing](../../FiSi-Administrator/Netzwerk-Administration/routing.md) – Routen über Tunnel-Interfaces
- [WAN-Anschlussarten](../../FiSi-Administrator/Netzwerk-Administration/wan-anschlussarten.md) – CGNAT
- [Lektion 27 – PKI, TLS & VPN](../../analyse-der-exams/L27_PKI_TLS_Verschluesselung_VPN.md)
