# DHCP (Dynamic Host Configuration Protocol)

## In 30 Sekunden

- DHCP verteilt automatisch **IP-Adresse, Subnetzmaske, Default-Gateway, DNS-Server** und weitere Optionen – für eine begrenzte Zeit (**Lease**).
- Ablauf **DORA**: **D**iscover – **O**ffer – **R**equest – **A**cknowledge; Transport **UDP 67 (Server) / 68 (Client)**.
- Discover ist ein **Broadcast** → endet an der Router-/VLAN-Grenze → über Netzgrenzen braucht es ein **DHCP-Relay** (`ip helper-address`).
- Kein Server erreichbar → Windows vergibt **APIPA `169.254.x.x`**.
- Schutz gegen fremde DHCP-Server: **DHCP-Snooping** am Switch.

## Kernwissen

### DORA im Detail (GA2 Sommer 2023, Sommer 2026 mit Wireshark-Mitschnitt)

| Schritt | Nachricht | Quelle → Ziel (IP) | Inhalt |
|---:|---|---|---|
| 1 | **Discover** | `0.0.0.0:68` → `255.255.255.255:67` | „Gibt es einen DHCP-Server?“ + Client-MAC |
| 2 | **Offer** | Server → Client (Broadcast oder Unicast) | angebotene IP, Maske, Gateway, DNS, Lease-Zeit, Server-ID |
| 3 | **Request** | `0.0.0.0` → `255.255.255.255` (Broadcast) | „Ich nehme das Angebot von Server X“ – andere Server ziehen ihr Angebot zurück |
| 4 | **Acknowledge (ACK)** | Server → Client | Bestätigung, Lease beginnt |

**Warum ist der Discover ein Broadcast? (GA2 Sommer 2025)** Der Client hat noch keine IP-Adresse und kennt den DHCP-Server nicht – ein gezielter Unicast ist unmöglich, also sendet er an alle.

Weitere Nachrichten: **NAK** (Server lehnt Request ab, z. B. falsches Netz), **Decline** (Client hat per ARP einen Konflikt erkannt), **Release** (Client gibt Adresse zurück, `ipconfig /release`), **Inform** (nur Optionen abfragen).

Nach dem ACK sendet der Client oft ein **Gratuitous ARP**, um Konflikte zu erkennen und ARP-Caches zu aktualisieren (2025S) → siehe [ARP](arp.md).

### Lease-Verlängerung

| Zeitpunkt | Aktion |
|---|---|
| **T1 = 50 %** der Lease | Client fragt **seinen** Server per Unicast nach Verlängerung (Request → ACK) |
| **T2 = 87,5 %** | Rebinding: Request per Broadcast an **jeden** Server |
| 100 % | Lease abgelaufen → Adresse weg, neuer Discover |

**Folge (GA2 Sommer 2026):** Änderungen am Server (z. B. neuer DNS-Eintrag) erreichen Clients erst bei der nächsten Verlängerung → sofort übernehmen mit `ipconfig /release` + `ipconfig /renew`.

### Wichtige DHCP-Optionen

| Option | Inhalt |
|---:|---|
| 1 | Subnetzmaske |
| 3 | Router (Default-Gateway) |
| 6 | DNS-Server |
| 15 | DNS-Domänenname |
| 42 | NTP-Server |
| 43 | herstellerspezifisch (z. B. WLAN-Controller-Adresse für APs) |
| 51 | Lease-Dauer |
| 53 | Nachrichtentyp (Discover = 1, Offer = 2, Request = 3, ACK = 5) |
| 54 | Server-Identifier (IP des DHCP-Servers) |
| 66 / 67 | TFTP-Server / Bootdatei (PXE-Boot) |
| 82 | Relay Agent Information |

### Begriffe der Serverkonfiguration

| Begriff | Bedeutung |
|---|---|
| **Scope / Bereich** | Adresspool eines Subnetzes, z. B. `192.168.10.100 – .200` |
| **Ausschluss** | Adressen im Scope, die nicht vergeben werden (statische Server, Drucker) |
| **Reservierung** | feste IP für eine bestimmte MAC – zentral verwaltet, trotzdem „immer gleich“ |
| **Lease-Dauer** | kurz (Gast-WLAN: 1–8 h), lang (Büro: 8 Tage) |
| **Relay-Agent** | Router/L3-Switch leitet Broadcast als **Unicast** an den Server weiter und trägt seine Interface-IP (`giaddr`) ein → Server weiß, aus welchem Scope er vergeben muss |
| **Failover / Split-Scope** | zwei Server teilen sich den Pool (z. B. 80/20) oder synchronisieren Leases → Redundanz |

### DHCP-Relay (GA2 Sommer 2023, Winter 2024)

```
Client (VLAN 10) --Broadcast--> L3-Switch SVI 10 (ip helper-address 10.0.0.5)
                                      --Unicast--> DHCP-Server 10.0.0.5 (VLAN 99)
```
Typisches Fehlerbild: **Nur das VLAN, in dem der DHCP-Server steht, funktioniert**, alle anderen VLANs bekommen APIPA → Relay fehlt.

### Typische Fehlerbilder

| Symptom | Ursache | Abhilfe |
|---|---|---|
| `169.254.x.x` (APIPA) | Kein Server erreicht: Kabel, VLAN, Relay, Server down | Link prüfen, Relay konfigurieren, `ipconfig /renew` |
| Neue Clients ohne IP, alte funktionieren | **Scope erschöpft** | Pool vergrößern, Lease-Zeit verkürzen |
| Falsches Gateway/DNS, fremdes Netz | **Rogue DHCP-Server** (z. B. privater Router) | DHCP-Snooping, Port identifizieren |
| DNS-Änderung kommt nicht an | **alter Lease** | `ipconfig /release` + `/renew` |
| IP-Konflikt | statische IP im DHCP-Pool | Ausschluss oder Reservierung setzen |

**DHCP-Snooping:** Der Switch erlaubt DHCP-Offers/ACKs nur auf als **trusted** markierten Ports (Uplink zum echten Server); an Access-Ports werden sie verworfen. Die dabei entstehende Binding-Tabelle (MAC ↔ IP ↔ Port) nutzt auch **Dynamic ARP Inspection**.

## Prüfungsfrage

**Frage (angelehnt an GA2 Winter 2024):** Nach Einführung der VLANs 10–40 erhalten alle Clients dieser VLANs eine Adresse `169.254.x.x`. Nur Clients im Server-VLAN 99, in dem auch der DHCP-Server steht, funktionieren. Erläutern Sie Ursache und Lösung.

**Musterantwort:** Der DHCP-Discover ist ein Broadcast und wird vom Layer-3-Switch nicht in andere VLANs weitergeleitet. Der DHCP-Server im VLAN 99 empfängt die Anfragen aus den VLANs 10–40 daher nicht, die Clients fallen auf APIPA zurück. Lösung: Auf dem Layer-3-Switch für jedes VLAN-Interface (SVI 10–40) ein **DHCP-Relay** (`ip helper-address <IP des DHCP-Servers>`) konfigurieren und auf dem Server je VLAN einen passenden Scope anlegen. Das Relay wandelt den Broadcast in einen Unicast an den Server um.

## Falle

- DORA-Reihenfolge vertauschen oder „Answer“ statt **Acknowledge** schreiben.
- Ports vertauschen: **Server 67, Client 68**.
- „Request ist Unicast“ → beim ersten Bezug ist der Request ein **Broadcast** (alle Server sollen ihn sehen).
- APIPA als Konfigurationsfehler des Clients deuten → meist ist der **Server nicht erreichbar**.
- Relay auf dem Client oder dem DHCP-Server einrichten → es gehört auf das **Gateway-Interface des Client-Netzes**.
- DHCP für Server-Infrastruktur (DNS, DC, Router) → diese bekommen **statische** Adressen oder Reservierungen.

## Merksatz

**„DORA ruft laut (Broadcast) – über die Grenze hilft nur der Helper.“**

## Verwandt

- [DNS](dns.md) – Option 6 liefert den Resolver
- [ARP](arp.md) – Gratuitous ARP nach dem Lease
- [IPv6 SLAAC & DHCPv6](ipv6-slaac-dhcpv6.md) – das IPv6-Gegenstück
- [VLAN](../Netzwerk-Administration/vlan.md) – Broadcast-Domänen und Relay
- [Netzwerkanalyse & Troubleshooting](../Netzwerk-Administration/netzwerkanalyse-und-troubleshooting.md) – APIPA systematisch eingrenzen
- [Port-Security & 802.1X](../Netzwerk-Administration/port-security-8021x.md) – DHCP-Snooping
- [Lektion 17 – DHCP & DNS](../../analyse-der-exams/L17_DHCP_DNS.md)
- [Lektion 07 – Netzwerk-Fehlersuche](../../analyse-der-exams/07_Netzwerk_Fehlersuche.md)
