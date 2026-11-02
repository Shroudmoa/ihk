# IPv6 SLAAC & DHCPv6

## In 30 Sekunden

- **SLAAC** (Stateless Address Autoconfiguration): Host bildet seine Adresse selbst aus **Präfix aus dem Router Advertisement + eigener 64-Bit-Interface-ID**.
- **DHCPv6 stateful:** Server vergibt Adressen und protokolliert sie; **stateless:** Adresse per SLAAC, DHCPv6 liefert nur Optionen (DNS, NTP …).
- Das **Default-Gateway kommt immer per Router Advertisement** – nie per DHCPv6. Gateway = Link-Local-Adresse des Routers.
- **NDP** (Neighbor Discovery, ICMPv6) ersetzt ARP: RS/RA, NS/NA, DAD.
- SLAAC braucht zwingend ein **/64**-Präfix (GA2 2022S, 2024W, 2025S).

## Kernwissen

### SLAAC-Ablauf Schritt für Schritt (GA2 Sommer 2024)

| Schritt | Was passiert | Nachricht / Adresse |
|---:|---|---|
| 1 | Host bildet **Link-Local-Adresse** `fe80::` + Interface-ID | – |
| 2 | **DAD**: prüft, ob die Adresse schon belegt ist | Neighbor Solicitation an Solicited-Node-Multicast |
| 3 | Host fragt nach Routern | **Router Solicitation (RS)** an `ff02::2` |
| 4 | Router antwortet mit Präfix, Lebensdauer, Flags | **Router Advertisement (RA)** an `ff02::1` (Absender = Link-Local des Routers) |
| 5 | Host bildet **globale Adresse**: Präfix (/64) + Interface-ID, erneut DAD | z. B. `2001:db8:1:10::21a:2bff:fe3c:4d5e` |
| 6 | Default-Gateway = Absender des RA | `fe80::1` |

Router senden RAs zusätzlich periodisch ungefragt.

### Interface-ID bilden

**a) EUI-64 aus der MAC (GA2 Sommer 2024, „Bildung der fe80-Adresse“)**

Beispiel MAC `00:1A:2B:3C:4D:5E`:
1. MAC in der Mitte teilen: `001A2B` | `3C4D5E`
2. `FFFE` einfügen: `001A2B` `FFFE` `3C4D5E`
3. **7. Bit** des ersten Bytes invertieren (U/L-Bit): `00` = `0000 0000` → `0000 0010` = `02`
4. Ergebnis: `021a:2bff:fe3c:4d5e` → Link-Local `fe80::21a:2bff:fe3c:4d5e`

**b) Zufällig / Privacy Extensions (RFC 4941, RFC 7217):** Windows und moderne Systeme nutzen meist zufällige bzw. stabile, nicht aus der MAC ableitbare IDs – Datenschutz, weil die MAC sonst weltweit verfolgbar wäre. Erkennbar daran, dass **kein `ff:fe`** in der Mitte steht.

### RA-Flags steuern die Methode

| Flag im RA | Bedeutung | Ergebnis |
|---|---|---|
| A = 1 (Prefix Info) | Host darf per SLAAC Adresse bilden | SLAAC |
| M = 1 (Managed) | Adresse per DHCPv6 holen | DHCPv6 **stateful** |
| O = 1 (Other) | weitere Optionen per DHCPv6 holen | DHCPv6 **stateless** |
| RDNSS-Option | DNS-Server direkt im RA (RFC 8106) | DNS ohne DHCPv6 |

### Methoden im Vergleich

| Verfahren | Adresse von | Optionen (DNS, NTP …) | Zentrale Übersicht | Einsatz |
|---|---|---|---|---|
| SLAAC | Host selbst | RDNSS im RA | nein | Heimnetz, einfache LANs |
| SLAAC + DHCPv6 stateless | Host selbst | DHCPv6 | nur Optionen | Unternehmens-Clients |
| DHCPv6 stateful | DHCPv6-Server | DHCPv6 | **ja** (Leases, Logging) | Server-/Client-Netze mit Nachweispflicht |
| Manuell | Admin | Admin | ja | Server, Router, Infrastruktur |

**DHCPv6-Details:** UDP **546** (Client) / **547** (Server); Clients werden über die **DUID** statt der MAC identifiziert; Anfragen an Multicast `ff02::1:2`. Ablauf **SARR**: Solicit – Advertise – Request – Reply (Gegenstück zu DORA).

### Warum trotzdem DHCPv6? (GA2 Sommer 2024 – „drei Gründe“, GA2 Sommer 2026)

- **Zentrale Verwaltung und Protokollierung:** wer hatte wann welche Adresse (Nachvollziehbarkeit, Audit).
- **Zusatzoptionen** verteilen: DNS-Suchdomäne, NTP-/Zeitserver, TFTP-/Bootserver (PXE), SIP-Server.
- **Reservierungen** für bestimmte Geräte, Adresspools steuern.
- Einheitliche Konfiguration über mehrere Subnetze per **DHCPv6-Relay**.

### NDP-Nachrichten (ICMPv6)

| Typ | Nachricht | Zweck |
|---:|---|---|
| 133 | Router Solicitation (RS) | „Gibt es einen Router?“ |
| 134 | Router Advertisement (RA) | Präfix, Gateway, Flags |
| 135 | Neighbor Solicitation (NS) | MAC zu IPv6 erfragen, DAD |
| 136 | Neighbor Advertisement (NA) | Antwort mit MAC |
| 137 | Redirect | besseren Next Hop mitteilen |

**Sicherheit:** Gefälschte RAs (Rogue RA) leiten Clients auf ein falsches Gateway → Schutz durch **RA Guard** und **DHCPv6 Guard** am Switch.

## Prüfungsfrage

**Frage (angelehnt an GA2 Sommer 2024):** Im Firmennetz wird IPv6 per SLAAC eingeführt. Beschreiben Sie, wie ein Client seine globale Adresse erhält, und nennen Sie drei Gründe, warum zusätzlich ein DHCPv6-Server betrieben werden soll.

**Musterantwort:** Der Client bildet zunächst eine Link-Local-Adresse (`fe80::` + Interface-ID) und prüft per Duplicate Address Detection, ob sie frei ist. Dann sendet er eine Router Solicitation; der Router antwortet mit einem Router Advertisement, das das /64-Präfix enthält. Der Client hängt seine Interface-ID (EUI-64 oder zufällig) an das Präfix und erhält so seine globale Adresse; der Absender des RA wird sein Default-Gateway. DHCPv6 ist zusätzlich sinnvoll, um (1) Adressvergaben zentral zu protokollieren, (2) weitere Parameter wie DNS-Suchdomäne, NTP- und TFTP-Server zu verteilen und (3) Reservierungen bzw. eine zentrale Steuerung über mehrere Subnetze zu ermöglichen.

## Falle

- „DHCPv6 verteilt das Standardgateway“ → **falsch**, das kommt per RA.
- SLAAC mit /56- oder /72-Präfix im LAN → SLAAC funktioniert nur mit **/64**.
- EUI-64: das 7. Bit vergessen zu kippen (`00` → `02`, `02` → `00`).
- NDP auf Schicht 2 einordnen → NDP ist **ICMPv6**, also Schicht 3 (löst aber Layer-2-Adressen auf).
- Stateless DHCPv6 mit „DHCPv6 ohne Server“ verwechseln – auch stateless braucht einen Server, er merkt sich nur keine Adressen.

## Merksatz

**„Router sagt das Präfix, Host baut die Adresse, DHCPv6 liefert nur, was der Router nicht kann.“**

## Verwandt

- [IPv6](ipv6.md) – Notation, Adresstypen, Dual Stack
- [DHCP](dhcp.md) – DORA bei IPv4 zum Vergleich
- [ARP](arp.md) – IPv4-Gegenstück zu NDP
- [MAC](mac.md) – Grundlage für EUI-64
- [Port-Security & 802.1X](../Netzwerk-Administration/port-security-8021x.md) – RA Guard, DHCP-Snooping
- [Lektion 08 – IPv6](../../analyse-der-exams/08_IPv6.md)
