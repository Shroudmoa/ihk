# 6. IPv4-Adressierung & Routing

*Quelle: GA2 Aufgabe 1 (2024 Sommer)*

> ✅ **Verifiziert am echten Prüfungskorpus:** IPv4-Subnetting kommt in
> **JEDEM** der 10 geprüften Jahrgänge (2021–2026) vor – der mit Abstand
> häufigste GA2-Themenblock. Reale Beispiele aus den Originalprüfungen:
> - **2021W:** `172.16.102.0/24` per **VLSM** in 4 Subnetze für Abteilungen
>   mit 80/50/20/10 Hosts aufteilen → Ergebnis /25, /26, /27, /28
> - **2022S:** `203.0.113.180/27` – Netz-ID, erste/letzte Host-IP,
>   Broadcast, Maske bestimmen; zusätzlich RFC 3021 (Punkt-zu-Punkt-Links
>   mit **/31**-Masken ohne Netz-/Broadcast-Verlust), 2^15 = 32.768 mögliche
>   /31-Netze
> - **2022W:** DMZ `203.0.113.64/29`
> - **2024S:** Router mit WAN/30, LAN/16, DMZ/26 – Broadcast aller drei
>   Netze berechnen; Multicast-Bereich 224.0.0.0–239.255.255.255 (Klasse D,
>   führendes Bitmuster `1110`)
> - **2024W:** `172.16.0.0/12` in 4 gleich große VLAN-Subnetze aufteilen
>   (Maske `255.252.0.0` = /14); RFC1918-Bereich `192.168.0.0/16`
> - **2025S:** feste Subnetzgrößen /24, /26, /25, /29 → Maske + nutzbare
>   Hosts berechnen (254 / 62 / 126 / 6)
> - **2025W:** `10.0.0.0/10`, `192.168.199.0/24` – nutzbare Adressen als
>   Zweierpotenz ausdrücken
> - **2026S:** Netz- und Broadcast-Adresse von `10.0.0.0/16` bestimmen,
>   beide übrigen RFC1918-Bereiche (`172.16.0.0/12`, `192.168.0.0/16`)
>   nennen

## 6.1 Grundlagen

- IPv4-Adresse = 32 Bit, Punkt-Dezimal (z. B. `172.16.0.254`)
- **Netz-ID** + **Host-Anteil**, getrennt durch **Subnetzmaske**
- **Broadcast-Adresse** = höchste Adresse im Netz (Host-Bits alle auf 1)

## 6.2 Übertragungsarten

| Art | Bedeutung | Beispiel |
|-----|-----------|----------|
| **Unicast** | 1 zu 1 | Webseite abrufen |
| **Multicast** | 1 zu Gruppe | IPTV-Livestream (224.0.0.0–239.255.255.255) |
| **Broadcast** | 1 zu alle | ARP-Request im lokalen Netz |

**IPv4-Multicast-Bereich:** `224.0.0.0` bis `239.255.255.255`

> Diese Dreiteilung wird **explizit nur in 2024S** so abgefragt
> (Unicast = Webseitenabruf, Multicast = IPTV-Livestream, Broadcast =
> ARP-Request). **2026S** unterscheidet zusätzlich **Broadcast auf Layer 3**
> (IP-Broadcast-Adresse) vom **Ethernet-Broadcast auf Layer 2**
> (Ziel-MAC `FF:FF:FF:FF:FF:FF`). In allen anderen Jahrgängen taucht das
> Thema nur implizit über DHCP/ARP auf.

## 6.3 Routing-Tabelle ergänzen (Beispiel 2024)

Gegeben: Router mit WAN `203.0.113.65/30`, LAN `172.16.0.0/16`,
DMZ `10.10.0.0/26`.

| Netz-ID | Subnetzmaske (dezimal) | Next-Hop / Interface |
|---------|------------------------|----------------------|
| 203.0.113.64 | 255.255.255.252 | WAN |
| 172.16.0.0 | 255.255.0.0 | LAN |
| 10.10.0.0 | 255.255.255.192 | DMZ |
| 0.0.0.0 | 0.0.0.0 | WAN (Default-Route) |

**Broadcast-Adressen:**
- WAN `/30` → `.67`
- LAN `/16` → `172.16.255.255`
- DMZ `/26` → `10.10.0.63`

## 6.4 Portforwarding / NAT (DMZ von außen erreichbar)

Webserver (443) und Mailserver (465/993) sollen von außen erreichbar sein:
→ Am Router **Portforwarding / Destination-NAT** einrichten: Pakete auf
öffentlicher IP + Port werden auf interne IP (10.10.0.10 / .11) umgeschrieben.

> ✅ **Portforwarding kommt in 2021W, 2022W, 2025S und 2026S** praktisch
> identisch dran: **SPI-Firewall + Destination-NAT** für Web-/Mailserver in
> der DMZ, typische Ports **80/443** (Web) und **25/465/587/993** (Mail).
> **2026S** grenzt zusätzlich den **Reverse Proxy** von einfacher
> Portweiterleitung ab: Ein Reverse Proxy übernimmt zusätzlich
> **TLS-Terminierung**, **Filterung** des Datenverkehrs und
> **Load-Balancing** – eine reine Portweiterleitung leitet Pakete nur 1:1
> unverändert weiter.

### NAT-Varianten in der Praxis

- **2023S:** Hotspot-NAT als typische Fehlerursache bei VPN-Verbindungen;
  IPsec mit **ESP** benötigt **NAT-Traversal**, da ESP selbst keine
  Portnummern besitzt und durch NAT sonst nicht korrekt zugeordnet werden
  kann.
- **2023W:** Rechenübung zum Header-Umschreiben durch NAT (TTL, Port, IP);
  zusätzlich **Carrier Grade NAT (CGN)** mit Adressen aus `100.64.0.0/10`
  bzw. intern `10.x.x.x` im Mobilfunknetz.
- **2024W:** **Masquerading** (PAT) – Vor- und Nachteil gegenüber
  klassischem NAT nennen (Vorteil: nur 1 öffentliche IP nötig; Nachteil:
  keine direkte Erreichbarkeit interner Hosts von außen ohne
  Portforwarding).
- **2025W:** Ausführlicher **NAT/PAT-Trace** (Quell-IP/-Port vor/nach dem
  Router); zusätzlich die Erkenntnis, dass eine **TLS-Portmanipulation**
  durch NAT möglich ist, weil NAT nur die **Header** (IP/Port), nicht aber
  die verschlüsselte **Payload** verändert.

## 6.5 Paketfluss & Entscheidung Default-Gateway

Client vergleicht seine **Netz-ID** mit der des Ziels. Liegen beide in
**verschiedenen Netzen** → Paket geht an das **Default-Gateway** (Router).

## 6.6 ARP

Client kennt die MAC des Gateways nicht → sendet **ARP-Request** (Broadcast):
„Wer hat IP X?". Gateway antwortet mit seiner MAC (**ARP-Reply**).

> ✅ **ARP taucht in fast jedem Jahrgang in anderem Kontext auf:**
> - **2023S:** ARP-Probe-Trace zur Erkennung eines **IP-Adresskonflikts**
> - **2024S:** ganz normales ARP beim Senden ans Default-Gateway (siehe
>   oben)
> - **2025S:** **Gratuitous ARP**, das ein Client nach erfolgreichem
>   DHCP-Bezug ins Netz sendet (Konflikterkennung / MAC-Tabellen
>   aktualisieren)
> - **2026S:** **ARP-Poisoning** ausführlich als Angriffsszenario:
>   SIEM-Alarm, Erkennung im Wireshark-Trace an **doppelter
>   MAC-zu-IP-Zuordnung**, Abwehr über **Monitoring-Port (SPAN/Mirror)**
>   zur Analyse sowie **Port-Security** mit **Sticky-MAC** und
>   **Violation-Shutdown** am Switch

## 6.7 Quell-IP nach NAT (Paket verlässt Router Richtung Internet)

Beispiel: Client `172.16.10.1` ruft `www.ihk.de` (51.38.104.145) auf.

| Feld | Wert nach Router |
|------|------------------|
| Source-IP | **203.0.113.65** (öffentliche IP des Routers) |
| Destination-IP | 51.38.104.145 |
| Protocol | 6 (TCP) |

## Typische Prüfungsaufgabe

> „Ermitteln Sie für die drei Netze jeweils die Broadcast-Adresse."

→ Host-Bits zählen (aus /Präfix), Rest = 1 setzen. Rechenweg zeigen!
