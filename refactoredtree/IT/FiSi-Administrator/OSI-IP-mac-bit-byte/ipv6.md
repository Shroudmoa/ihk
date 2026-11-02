# IPv6

## In 30 Sekunden

- IPv6 hat **128 Bit**, geschrieben als **8 Gruppen à 4 Hexziffern** (je 16 Bit), getrennt durch Doppelpunkte.
- **Kürzen:** führende Nullen pro Gruppe weglassen, **eine** zusammenhängende Nullfolge durch `::` ersetzen – nur einmal pro Adresse.
- Typischer Aufbau: **/48 oder /56 vom Provider → /64 pro Subnetz → 64 Bit Interface-ID**.
- Adresstypen erkennen: `2000::/3` global, `fe80::/10` link-local, `fc00::/7` (`fd..`) Unique Local, `ff00::/8` Multicast.
- **Kein Broadcast, kein ARP** – stattdessen Multicast und **NDP** (ICMPv6). Autokonfiguration per SLAAC/DHCPv6.

## Kernwissen

### Notation und Kürzungsregeln

| Regel | Beispiel |
|---|---|
| Volle Schreibweise | `2001:0db8:0000:0000:0000:ff00:0042:8329` |
| 1. Führende Nullen je Gruppe weglassen | `2001:db8:0:0:0:ff00:42:8329` |
| 2. Längste Folge von Null-Gruppen → `::` (nur **einmal**) | `2001:db8::ff00:42:8329` |
| Kleinbuchstaben empfohlen (RFC 5952) | `2001:DB8::1` = `2001:db8::1` |
| Präfix mit `/` | `2001:db8:cafe::/48` |
| In URLs eckige Klammern | `https://[2001:db8::10]:8443/` |

**Expandieren (Rückweg) – Schritt für Schritt:** `2001:db8:a::1`
1. Gruppen zählen: `2001`, `db8`, `a`, `1` = 4 Gruppen → `::` ersetzt 8 − 4 = **4** Null-Gruppen.
2. Auffüllen: `2001:0db8:000a:0000:0000:0000:0000:0001`.

**Ungültig:** `2001::db8::1` (zwei `::`), `2001:db8:0:0:0:0:0:0:1` (9 Gruppen), `2001:db8::g1` (`g` ist kein Hex).

**Hinweis zu RFC 5952:** Bei zwei gleich langen Nullfolgen wird die **erste** gekürzt; eine **einzelne** Null-Gruppe soll als `0` stehen bleiben (`2001:db8:0:1::1`).

### Adresstypen

| Präfix | Typ | Erkennen | Zweck |
|---|---|---|---|
| `2000::/3` | Global Unicast (GUA) | beginnt mit `2` oder `3` | weltweit routbar (z. B. `2003:…` Telekom) |
| `fe80::/10` | Link-Local | `fe80::…` | nur im Segment, jedes Interface hat eine; Gateway-Adresse in RAs |
| `fc00::/7` (praktisch `fd00::/8`) | Unique Local (ULA) | `fd…` | intern, nicht im Internet routbar (≈ RFC 1918) |
| `ff00::/8` | Multicast | `ff…` | `ff02::1` alle Knoten, `ff02::2` alle Router |
| `::1/128` | Loopback | `::1` | localhost |
| `::/128` | unspezifiziert | `::` | „noch keine Adresse“ (z. B. bei DAD) |
| `2001:db8::/32` | Dokumentation | `2001:db8:…` | Beispiele in Prüfungen |
| `2002::/16` | 6to4 (Übergangstechnik) | `2002:…` | IPv6 über IPv4 getunnelt |

### Aufbau einer globalen Adresse

```
2001:0db8:cafe : 0012 : 0211:22ff:fe33:4455
|-- 48 Bit ---| |16 Bit| |------ 64 Bit ------|
 Global Routing   Subnet     Interface-ID
 Prefix (Provider) ID      (EUI-64 oder zufällig)
```
- **/48:** typische Zuteilung an Unternehmen → 2^16 = 65.536 /64-Netze.
- **/56:** typische Zuteilung an Privat-/Kleinkunden → 256 /64-Netze.
- **/64:** ein LAN-Segment; Pflicht für SLAAC.

### IPv4 vs. IPv6

| Merkmal | IPv4 | IPv6 |
|---|---|---|
| Adresslänge | 32 Bit | 128 Bit |
| Schreibweise | dezimal, Punkte | hexadezimal, Doppelpunkte |
| Broadcast | ja | **nein** (Multicast stattdessen) |
| MAC-Auflösung | ARP | NDP (ICMPv6) |
| Autokonfiguration | DHCP / APIPA | SLAAC, DHCPv6 |
| NAT | üblich | grundsätzlich nicht nötig |
| Header | variabel, mit Checksumme | feste 40 Byte, ohne Checksumme, TTL heißt **Hop Limit** |
| DNS-Record | A | AAAA |

### Vorteile von IPv6 (nenne 3)

- Riesiger Adressraum → Ende-zu-Ende-Erreichbarkeit ohne NAT.
- Autokonfiguration (SLAAC) → weniger Verwaltungsaufwand.
- Vereinfachter Header, hierarchische Präfixe → effizienteres Routing.
- Kein Broadcast → weniger Last im Segment.
- IPsec ist Teil der Spezifikation (Nutzung bleibt optional).

### Dual Stack

**Dual Stack** = IPv4 und IPv6 parallel auf denselben Hosts und Netzen (GA2 Sommer 2026). Ein Client bekommt A- und AAAA-Record und bevorzugt meist IPv6; **Happy Eyeballs** (RFC 8305) probiert beide Wege fast gleichzeitig und nimmt den schnelleren.

**Sicherheitsfalle Dual Stack (GA2 Sommer 2023):** Proxy- und Firewallregeln nur für IPv4 → der Client umgeht sie über einen **IPv6-Breakout** (im `tracert` an IPv6-Hops erkennbar). Lösung: Regeln und Proxy auch für IPv6, oder IPv6 gezielt sperren.

## Prüfungsfrage

**Frage (angelehnt an GA2 Sommer 2022):** Ordnen Sie die Adressen zu (Global Unicast, Link-Local, Unique Local, Multicast) und kürzen Sie die erste maximal: `2001:0db8:0000:0042:0000:0000:0000:0001`, `fe80::4257:a46c:5622:bb4`, `fdbf:1a2b:3c4d::10`, `ff02::2`.

**Musterantwort:**
- `2001:0db8:0000:0042:0000:0000:0000:0001` → **Global Unicast** (Bereich `2000::/3`; `2001:db8::/32` ist das Dokumentationspräfix). Gekürzt: **`2001:db8:0:42::1`** (die längere Nullfolge hinten wird zu `::`, die einzelne Null-Gruppe bleibt `0`).
- `fe80::4257:a46c:5622:bb4` → **Link-Local**, nur im eigenen Segment gültig.
- `fdbf:1a2b:3c4d::10` → **Unique Local**, intern, nicht im Internet routbar.
- `ff02::2` → **Multicast** an alle Router im Segment.

## Falle

- `::` **zweimal** verwenden → Adresse ist nicht mehr eindeutig.
- Nullen **am Ende** einer Gruppe weglassen (`db80` → `db8`) → falsch, nur **führende** Nullen dürfen weg.
- `fd…`-Adressen als Link-Local bezeichnen → das sind **Unique Local** Adressen; Link-Local beginnt mit `fe80`.
- „IPv6 hat Broadcast an `ff:ff…`“ → nein, IPv6 nutzt Multicast (`ff02::1`).
- Bei Subnetting Netz- und Broadcastadresse abziehen → bei IPv6 nicht.
- Firewall nur für IPv4 pflegen → IPv6 umgeht die Regeln.

## Merksatz

**„Acht Gruppen, vier Hex, Nullen vorne weg, Doppelpunkt-Doppelpunkt nur einmal.“** – `2` global, `fe80` lokal, `fd` intern, `ff` Multicast.

## Verwandt

- [IPv6 SLAAC & DHCPv6](ipv6-slaac-dhcpv6.md) – Autokonfiguration, NDP, EUI-64
- [Subnetting IPv4 & IPv6](subnetting-ipv4-ipv6.md) – /48 → /50, /56 → /64 rechnen
- [IPv4](ipv4.md) – Vergleich 32 Bit
- [DNS](dns.md) – AAAA-Record
- [MAC](mac.md) – Grundlage für EUI-64
- [Firewall](../Sicherheit/firewall/firewall.md) – Regeln auch für IPv6
- [Lektion 08 – IPv6](../../analyse-der-exams/08_IPv6.md)
