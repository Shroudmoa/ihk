## 1. Aufgabe (Netzwerkplan & Routing)

### aa) Routing-Tabelle Router (IHK-Multimedia)
| Netz-ID | Subnetzmaske (dezimal) | Next-Hop oder Interface |
|---|---|---|
| LAN | 172.16.0.0 | 255.255.0.0 | LAN (172.16.0.254) |
| DMZ | 10.10.0.0 | 255.255.255.192 | DMZ (10.10.0.1) |
| WAN | 203.0.113.64 | 255.255.255.252 | WAN (203.0.113.65) |
| Default | 0.0.0.0 | 0.0.0.0 | ISP-Gateway |

- **Logik:** Router hat drei angeschlossene Netze (LAN, DMZ, WAN). Default-Route zeigt zum ISP.

### ab) Broadcast-Adressen (3 Netze)
- **LAN 172.16.0.0/16:** Broadcast = **172.16.255.255**
- **DMZ 10.10.0.0/26:** Blockgröße 64 → 10.10.0.0–10.10.0.63 → Broadcast = **10.10.0.63**
- **WAN 203.0.113.64/30:** Blockgröße 4 → 203.0.113.64–67 → Broadcast = **203.0.113.67**

### ac) Unicast, Multicast, Broadcast
- **Unicast:** 1:1-Kommunikation. Beispiel: Client 1 sendet HTTP-Request an Webserver 10.10.0.10.
- **Multicast:** 1:n-Kommunikation an eine Gruppe. Beispiel: IPTV-Stream an mehrere Empfänger; IPv6 Neighbor Discovery (Solicited-Node).
- **Broadcast:** 1:alle im selben L2-Segment. Beispiel: ARP-Request („Wer hat 172.16.0.254?“) oder DHCP-Discover.

### ba) Portweiterleitung (DNAT) für Web/Mail
- **Maßnahme:** Am Router **Port Forwarding (Destination NAT)** einrichten:
  - Externe IP:443 → 10.10.0.10:443 (Webserver)
  - Externe IP:465 → 10.10.0.11:465 (SMTPs)
  - Externe IP:993 → 10.10.0.11:993 (IMAPS)
- **Zusätzlich:** Firewall-Regeln erlauben eingehend nur diese Ports auf diese Ziele.

### bb) Paket nach Router-Verlassen (Client 1 → www.ihk.de)
- **Source-IP:** 203.0.113.65 (öffentliche WAN-IP des Routers; NAT/PAT).
- **Destination-IP:** 51.38.104.145 (www.ihk.de).
- **TTL:** 64 (dekrementiert vom Client).
- **Weitere Felder:** Version 4, IHL 5, Protocol 6 (TCP), Source-Port (zufällig), Dest-Port 443.

### bc) Client-Entscheidung Default-Gateway
- **Prinzip:** Client prüft Ziel-IP gegen eigene Netzmaske (172.16.0.0/16). Ziel 10.10.0.10 liegt **außerhalb** des lokalen Subnetzes. Daher sendet er das Paket an den Default-Gateway (Router 172.16.0.254).

### bd) ARP-Einsatz
- **Prinzip:** Client muss die **MAC-Adresse** des Default-Gateways ermitteln. Er sendet ARP-Broadcast („Wer hat 172.16.0.254?“). Der Router antwortet mit seiner MAC. Diese wird im ARP-Cache gespeichert; das Ethernet-Frame wird damit adressiert.

---

## 2. Aufgabe (Fehleranalyse & Mail-Client)

### a) IPv4 169.254.93.253 – Grund
- **APIPA/Link-Local:** Der Client hat keine IP vom DHCP-Server erhalten (DHCP-Timeout, kein DHCP erreichbar, Kabel/Port defekt). Windows vergibt automatisch eine Adresse aus 169.254.0.0/16. Kein Internetzugang möglich.

### ab) IPv6-Adresse fe80::... – Typ und Bildung
- **Typ:** **Link-Local Unicast** (fe80::/10).
- **Bildung:** Automatisch aus der MAC-Adresse via **EUI-64** (MAC in 64-Bit Interface-ID umgewandelt, 7. Bit geflippt, ff:fe in Mitte eingefügt).

### ac) Lösungsweg für funktionierende Netzwerkkonfiguration
- **Schritt 1:** Physische Verbindung prüfen (Kabel, Switch-Port, Link-LED).
- **Schritt 2:** `ipconfig /release` und `ipconfig /renew` ausführen; DHCP-Server erreichbar?
- **Schritt 3:** DHCP-Relay/Switch-Konfiguration prüfen; ggf. statische IP als Fallback setzen.

### b) FQDN-Zugriff fehlgeschlagen – DNS-Problem
- **Analyse:** `DNS-Server = 8.8.8.8` (Google Public DNS). Dieser kennt den **internen** FQDN `dateiserver.ihk-media.local` nicht. Interne Namensauflösung erfordert den **internen DNS** (Domain Controller, z.B. 172.16.1.11).
- **bb) Lösung:** DNS-Server am Notebook auf **172.16.1.11** (interner DC) ändern oder zusätzlich eintragen. `ipconfig /flushdns` ausführen.

### c) Mail-Client – manuelle Konfiguration
- **Protokoll:** **IMAP** (Telnet-Antwort: `* OK [CAPABILITY IMAP4rev1 ...]`).
- **Verbindungssicherheit:** **STARTTLS** (Server bietet `STARTTLS` in CAPABILITY an; Port 143 mit Upgrade).
- **Authentifizierungsmethode:** **Passwort, normal** (Server bietet `AUTH=PLAIN AUTH=LOGIN`; einfache Passwort-Authentifizierung).

---

## 3. Aufgabe (IPv6)

### aa) Grund für IPv6-Umstellung beim Provider
- **Adressknappheit:** IPv4-Adressen sind global erschöpft (IANA 2011). IPv6 bietet 2^128 Adressen; zukunftssichere Skalierung.

### ab) Drei Vorteile IPv6
- **Größerer Adressraum:** Kein NAT mehr nötig; Ende-zu-Ende-Konnektivität.
- **Autokonfiguration (SLAAC):** Clients generieren Adressen selbst ohne DHCP; Plug-and-Play.
- **Effizienteres Routing:** Aggregierbare Präfixe; kleinerer Routing-Tabellen-Overhead; kein Broadcast (Multicast/Anycast).
- **Integriertes IPsec:** Sicherheit auf Layer 3 standardisiert.

### ba) Zwei Gründe für Subnetze
- **Broadcast-/Multicast-Domänen begrenzen:** Reduziert Traffic und erhöht Performance.
- **Sicherheit/Isolation:** Trennung von Abteilungen; Firewall-Regeln zwischen Subnetzen.
- **Administrative Struktur:** Logische Gruppierung.

### bb) Vier gleich große Subnetze aus 2001:DB8:CAFE::/48
- **/48 → /50:** 2 Bits für 4 Subnetze (2^2 = 4).
- **Subnetz 1:** 2001:DB8:CAFE:0000::/50 → **2001:DB8:CAFE::/50**
- **Subnetz 2:** 2001:DB8:CAFE:4000::/50
- **Subnetz 3:** 2001:DB8:CAFE:8000::/50
- **Subnetz 4:** 2001:DB8:CAFE:C000::/50
- **Hinweis:** /50 → 4. Hextett: 0000, 4000, 8000, C000.

### ca) SLAAC – Funktionsweise
- **Prinzip:** Client sendet Router Solicitation (RS); Router antwortet mit Router Advertisement (RA) inkl. Präfix (/64). Client bildet Interface-ID via EUI-64 oder Privacy Extensions (RFC 4941) und kombiniert mit Präfix zur Global Unicast Address. Kein DHCP-Server nötig.

### cb) Drei Gründe für DHCPv6
- **Zentrale Verwaltung:** DNS-Server, NTP, Domain Search List können per DHCPv6 verteilt werden (SLAAC liefert nur Präfix).
- **Stateful Addressing:** Zuweisung und Tracking von Adressen (Lease-Datenbank); Audit.
- **Kompatibilität:** Ältere Geräte/OS ohne vollständiges SLAAC; Richtlinien zur Adressvergabe.

---

## 4. Aufgabe (WLAN)

### aa) WPA2-Personal – 3 Nachteile
- **Shared PSK:** Ein gemeinsames Passwort für alle Nutzer; bei Weitergabe ist das gesamte WLAN kompromittiert. Keine individuelle Rückverfolgbarkeit.
- **Kein zentrales Management:** Passwortwechsel erfordert Neuverteilung an alle Clients; keine dynamische Rechtevergabe.
- **Anfällig für Offline-Brute-Force:** Handshake kann mitgeschnitten und offline angegriffen werden (PMKID-Angriff); schwache Passphrasen sind knackbar.

### ab) Sicherheit verbessern
- **WPA2/WPA3-Enterprise (802.1X/RADIUS):** Individuelle Authentifizierung per Benutzerzertifikat oder AD-Credentials; dynamische VLAN-Zuweisung.

### ba) Drei Gründe für separates WLAN (Gast/BYOD)
- **Sicherheit/Isolation:** Gäste und private Geräte erhalten keinen Zugriff auf interne Server (AD, Fileserver). Eigenes VLAN/Subnetz.
- **Compliance/DSGVO:** Private Geräte dürfen keine Firmendaten sehen; Trennung verhindert Datenabfluss.
- **Performance/QoS:** Gast-Traffic kann bandbreitenbegrenzt werden; interne Anwendungen bleiben priorisiert.

### bb) Kameras verbinden nicht mehr – 2 Ursachen
- **Kanal-Überlappung/Interferenz:** Neue APs nutzen denselben 2,4-GHz-Kanal wie die Kameras; Störungen durch Ko-Kanal-Interferenz.
- **Sicherheitsstandard inkompatibel:** Neue APs fordern WPA3; Kameras unterstützen nur WPA2. Oder: 802.11ax-only-Modus schließt ältere 802.11n-Clients aus.

### cb) Performanceproblem – Grund aus Scan
- **Grund:** **Kanal 1 ist überfüllt.** SSIDs „iPhone von Thomas“, „<hiden>“, „MEDIA-AGENTUR“ und „Hotspot1“ nutzen alle **Kanal 1 (2,4 GHz)**. Ko-Kanal-Interferenz reduziert Durchsatz und erhöht Latenz.

### cc) Hidden SSID – Grund
- **Prinzip:** Der AP sendet die SSID nicht in Beacon-Frames (nur in Probe Responses). Erschwert triviales Auffinden des Netzes; **keine echte Sicherheitsmaßnahme** (SSID ist im Handshake sichtbar). Reduziert nur „Sichtbarkeit“ für Laien.

### da) Voucher-System – Definition
- **Prinzip:** Gast-WLAN mit Captive Portal. Der Gast erhält einen zeitlich/volumenmäßig begrenzten Zugangscode (Voucher) vom Gastgeber. Authentifizierung am Portal; Freischaltung des Internetzugangs für die Voucher-Dauer.

### db) Drei weitere Möglichkeiten des Voucher-Systems
- **Bandbreitenbegrenzung:** Max. Up-/Download pro Gast (QoS).
- **Zeitfenster:** Zugang nur zu bestimmten Uhrzeiten.
- **Gerätebegrenzung:** Max. Anzahl gleichzeitiger Geräte pro Voucher.
- **Logging/Reporting:** Nutzungsstatistiken pro Voucher (DSGVO-konform).

---

**Merksatz:** IHK-Aufgaben sind **Kausalitätsketten** – wer Subnetting-Broadcast, NAT/PAT, SLAAC und WLAN-Kanalinterferenz nicht als Hard-Facts sitzen hat, verliert Punkte an der Oberfläche. Lerne Standards als Vokabeln, nicht als Prosa.
