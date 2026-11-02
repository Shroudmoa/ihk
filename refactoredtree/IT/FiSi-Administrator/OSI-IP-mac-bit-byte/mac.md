# MAC-Adresse und Ethernet-Frame

## In 30 Sekunden

- Die **MAC-Adresse** (Media Access Control) ist die **Layer-2-Adresse** einer Netzwerkschnittstelle: **48 Bit = 6 Byte = 12 Hexziffern**, z. B. `00:1A:2B:3C:4D:5E`.
- Aufbau: **24 Bit OUI** (Hersteller) + **24 Bit** gerätespezifischer Teil.
- Gilt nur **im lokalen Segment** – an jedem Router wird das Frame mit neuen MAC-Adressen neu gebaut, die IP bleibt.
- Switches lernen MACs aus der **Quelladresse** und leiten anhand ihrer **MAC-Adresstabelle** weiter.
- MACs lassen sich fälschen (**MAC-Spoofing**) → MAC-Filter ist keine echte Authentifizierung.

## Kernwissen

### Aufbau

| Bestandteil | Länge | Bedeutung |
|---|---:|---|
| OUI (Organizationally Unique Identifier) | 24 Bit | dem Hersteller von der IEEE zugeteilt (z. B. `00:50:56` = VMware) |
| NIC-spezifischer Teil | 24 Bit | vom Hersteller vergeben |

**Besondere Bits im ersten Byte**
| Bit | Wert 0 | Wert 1 |
|---|---|---|
| niederwertigstes Bit (I/G) | Unicast | Multicast/Broadcast |
| zweitniederwertigstes Bit (U/L) | universell (vom Hersteller) | lokal verwaltet (z. B. zufällige WLAN-MAC, VMs) |

**Sonderadressen:** `FF:FF:FF:FF:FF:FF` = Broadcast, `01:00:5E:…` = IPv4-Multicast, `33:33:…` = IPv6-Multicast.

**Zufällige MAC (Privacy):** Smartphones und Windows nutzen im WLAN oft pro SSID zufällige, lokal verwaltete MACs → MAC-Filter, DHCP-Reservierungen und Voucher-Bindungen können dadurch „brechen“.

### Ethernet-Frame (IEEE 802.3 / Ethernet II)

| Feld | Größe | Inhalt |
|---|---:|---|
| Präambel + SFD | 8 Byte | Taktsynchronisation |
| Ziel-MAC | 6 Byte | Empfänger |
| Quell-MAC | 6 Byte | Absender |
| *802.1Q-Tag (optional)* | *4 Byte* | *VLAN-ID (12 Bit) + Priorität (3 Bit) – nur auf Trunks* |
| EtherType | 2 Byte | `0x0800` IPv4, `0x86DD` IPv6, `0x0806` ARP, `0x8100` = VLAN-Tag folgt |
| Nutzdaten | 46–1500 Byte | z. B. IP-Paket (**MTU 1500**) |
| FCS | 4 Byte | CRC-Prüfsumme → Fehlererkennung (nicht -korrektur) |

Das VLAN-Tag verändert den Frame auf **Schicht 2**, die MAC-Adressen selbst bleiben gleich (GA2 Winter 2023).

### Wie ein Switch mit MACs arbeitet

1. **Lernen:** Quell-MAC + Eingangsport in die MAC-Tabelle (CAM) eintragen, Alterung meist 300 s.
2. **Weiterleiten:** Ziel-MAC bekannt → nur an diesen Port (**Forwarding**).
3. **Fluten:** Ziel unbekannt, Broadcast oder Multicast → an alle Ports im VLAN außer dem Eingangsport.
4. **Filtern:** Ziel liegt am Eingangsport → verwerfen.

**MAC-Flooding:** Angreifer füllt die Tabelle mit Fantasie-MACs → Switch flutet alles wie ein Hub → Mitlesen möglich. Schutz: **Port-Security** (maximale Anzahl MACs pro Port).

### MAC vs. IP

| | MAC-Adresse | IP-Adresse |
|---|---|---|
| OSI-Schicht | 2 (Sicherung) | 3 (Vermittlung) |
| Länge | 48 Bit | 32 Bit (IPv4) / 128 Bit (IPv6) |
| Gültigkeit | lokales Segment, ändert sich pro Hop | Ende-zu-Ende (außer bei NAT) |
| Vergabe | Hersteller (fest „eingebrannt“) | Admin, DHCP, SLAAC |
| Gerät | Switch | Router |
| Verknüpfung | ARP (IPv4), NDP (IPv6) | |

## Prüfungsfrage

**Frage:** Ein Paket geht von PC A über Switch S und Router R zum Server B in einem anderen Netz. Welche Quell- und Ziel-MAC bzw. -IP stehen im Frame zwischen A und R sowie zwischen R und B?

**Musterantwort:** Zwischen A und R: Quell-MAC = MAC von A, Ziel-MAC = MAC des Router-Interfaces (per ARP ermittelt); Quell-IP = A, Ziel-IP = B. Der Switch S ändert nichts, er leitet nur anhand der Ziel-MAC weiter. Zwischen R und B: Quell-MAC = MAC des ausgehenden Router-Interfaces, Ziel-MAC = MAC von B; die IP-Adressen bleiben A → B (ohne NAT). MAC-Adressen gelten also nur pro Abschnitt, IP-Adressen Ende-zu-Ende.

## Falle

- „Der Router behält die MAC-Adressen“ → nein, er baut pro Hop einen neuen Frame.
- MAC-Filter als sichere Zugangskontrolle bezeichnen → MAC-Spoofing ist trivial; besser **802.1X**.
- OUI mit „Gerätenummer“ verwechseln: die **ersten** drei Byte sind der Hersteller.
- Die MTU (1500 Byte) mit der maximalen Framegröße (1518 Byte, mit Tag 1522) verwechseln.

## Merksatz

**„MAC = Hausnummer im Flur, IP = Postadresse.“** Die MAC wechselt an jedem Router, die IP reist mit.

## Verwandt

- [ARP](arp.md) – IP zu MAC auflösen
- [OSI](osi.md) – Einordnung in Schicht 2
- [Switches und VLANs](../Netzwerk-Administration/switches-und-vlans.md) – MAC-Tabelle und Switching
- [VLAN](../Netzwerk-Administration/vlan.md) – 802.1Q-Tag im Frame
- [Port-Security & 802.1X](../Netzwerk-Administration/port-security-8021x.md) – MAC-Begrenzung, Sticky-MAC
- [IPv6 SLAAC & DHCPv6](ipv6-slaac-dhcpv6.md) – EUI-64 aus der MAC
- [Hacker-Angriffe](../Sicherheit/hacking/hackers-can.md) – MAC- und ARP-Spoofing
