# ARP (Address Resolution Protocol)

## In 30 Sekunden

- ARP ermittelt im **IPv4-LAN** zu einer bekannten **IP-Adresse die MAC-Adresse** – Voraussetzung, um überhaupt ein Ethernet-Frame zu adressieren.
- **ARP-Request = Broadcast** (`FF:FF:FF:FF:FF:FF`, „Wer hat 172.16.0.254?“), **ARP-Reply = Unicast** („172.16.0.254 ist bei 00:11:22:…“).
- Ergebnisse landen im **ARP-Cache** (`arp -a`); für Ziele in fremden Netzen wird die MAC des **Default-Gateways** aufgelöst.
- Sonderformen aus Prüfungen: **ARP-Probe** (IP-Konflikt, 2023S), **Gratuitous ARP** (2025S), **ARP-Poisoning** (2026S).
- ARP hat **keine Authentifizierung** → anfällig für Spoofing; Schutz per **Dynamic ARP Inspection**. IPv6 nutzt stattdessen **NDP**.

## Kernwissen

### Ablauf

```
PC 172.16.10.1 möchte zu 10.10.0.10 (fremdes Netz) -> braucht MAC des Gateways 172.16.0.254

1. ARP-Cache prüfen -> kein Eintrag
2. ARP-Request  (Broadcast)  Src-MAC: PC  Dst-MAC: FF:FF:FF:FF:FF:FF
                "Who has 172.16.0.254? Tell 172.16.10.1"
3. ARP-Reply    (Unicast)    Src-MAC: Router  Dst-MAC: PC
                "172.16.0.254 is at 00:1b:44:11:3a:b7"
4. Eintrag im Cache, Frame an Router-MAC, Ziel-IP bleibt 10.10.0.10
```

**Einordnung:** ARP arbeitet zwischen Schicht 2 und 3 (oft als „Layer 2,5“ bezeichnet). Es ist direkt in Ethernet gekapselt (EtherType `0x0806`), nutzt **weder IP noch Ports** und wird daher **nicht geroutet**.

### Varianten

| Variante | Was passiert | Zweck | Prüfung |
|---|---|---|---|
| **ARP-Probe** | Host fragt nach der eigenen künftigen IP, Absender-IP `0.0.0.0` | Prüfen, ob die IP schon belegt ist; antwortet jemand → **IP-Konflikt** | 2023S |
| **Gratuitous ARP** | Host kündigt ungefragt die eigene IP-MAC-Zuordnung an | Konflikte erkennen, ARP-Caches und Switch-MAC-Tabellen aktualisieren (nach DHCP, Failover, VM-Migration, VRRP-Wechsel) | 2025S |
| **Proxy-ARP** | Router antwortet stellvertretend für Hosts in anderen Netzen | Notlösung bei falschen Masken – heute meist deaktiviert | – |
| **Statischer Eintrag** | `arp -s` | feste Zuordnung für kritische Systeme | – |

### IP-Konflikt erkennen (GA2 Sommer 2023)

Ein Server mit statischer IP `10.1.0.10` zeigt plötzlich `169.254.102.223`. Im Mitschnitt antwortet auf seine ARP-Probe ein anderes Gerät: `10.1.0.10 is at 00:0d:59:57:11:05`. → **Die Adresse ist doppelt vergeben**, Windows weicht auf APIPA aus. Lösung: Gerät mit der fremden MAC finden (MAC-Tabelle des Switches, OUI), dessen IP ändern, danach Adresse neu setzen und mit `arp -a` prüfen.

### ARP-Poisoning / ARP-Spoofing (GA2 Sommer 2026)

| Aspekt | Inhalt |
|---|---|
| Angriff | Angreifer sendet gefälschte ARP-Replies „Gateway-IP ist bei **meiner** MAC“ an Opfer (und ggf. umgekehrt an das Gateway) |
| Folge | Verkehr läuft über den Angreifer → **Man-in-the-Middle**: mitlesen, manipulieren, Sessions übernehmen |
| Erkennen | Im Mitschnitt/`arp -a`: **eine MAC-Adresse für mehrere IP-Adressen** (z. B. `.6` und Gateway `.254`), auffällig viele unaufgeforderte Replies, IDS/SIEM-Alarm |
| Analyse | Verkehr über einen **Monitoring-/SPAN-Port** am Switch passiv mitschneiden (Wireshark) |
| Schutz | **Dynamic ARP Inspection** (prüft gegen DHCP-Snooping-Tabelle), **Port-Security** (Sticky-MAC, Violation Shutdown), 802.1X, statische Einträge für Gateways, Segmentierung, Ende-zu-Ende-Verschlüsselung (TLS) |

### Befehle

| Befehl | Zweck |
|---|---|
| `arp -a` | ARP-Cache anzeigen (Windows/Linux) |
| `arp -d *` | Cache löschen (Windows, Admin) |
| `ip neigh` | Nachbar-Tabelle unter Linux (IPv4 und IPv6) |
| `Get-NetNeighbor` | PowerShell |

## Prüfungsfrage

**Frage (angelehnt an GA2 Sommer 2026):** Das SIEM meldet einen möglichen MitM-Angriff im Netz `192.168.12.0/24`. Im Mitschnitt sehen Sie die Zeilen `192.168.12.254 is at 00:50:56:aa:bb:cc` und `192.168.12.6 is at 00:50:56:aa:bb:cc`. Erläutern Sie den Angriff und nennen Sie zwei Gegenmaßnahmen.

**Musterantwort:** Es handelt sich um **ARP-Poisoning**: Ein Gerät mit der MAC `00:50:56:aa:bb:cc` gibt sich gleichzeitig als Gateway `.254` und als Host `.6` aus. Die Opfer tragen diese falsche Zuordnung in ihren ARP-Cache ein und schicken ihre Frames an den Angreifer, der sie mitlesen und weiterleiten kann (Man-in-the-Middle). Gegenmaßnahmen: (1) **Dynamic ARP Inspection** am Switch, die ARP-Pakete gegen die DHCP-Snooping-Bindungstabelle prüft und gefälschte verwirft; (2) **Port-Security** mit Sticky-MAC und Shutdown bei Verletzung bzw. 802.1X, damit fremde Geräte keinen Netzzugang erhalten.

## Falle

- „ARP-Reply ist ein Broadcast“ → der **Request** ist Broadcast, der **Reply** Unicast.
- ARP für einen Server im Internet → aufgelöst wird die MAC des **Gateways**, nie die des entfernten Servers.
- ARP einen Port zuordnen → ARP hat **keinen Port** und kein IP-Header.
- ARP bei IPv6 → dort übernimmt **NDP** (Neighbor Solicitation/Advertisement).
- Gratuitous ARP und ARP-Probe gleichsetzen: Probe fragt mit Absender `0.0.0.0` (vor der Nutzung), Gratuitous ARP verkündet die eigene, bereits genutzte Adresse.

## Merksatz

**„ARP fragt laut, antwortet leise“** – Request Broadcast, Reply Unicast. Eine MAC mit vielen IPs = Poisoning-Verdacht.

## Verwandt

- [MAC](mac.md) – Layer-2-Adressen
- [IPv4](ipv4.md) – lokal oder Gateway
- [IPv6 SLAAC & DHCPv6](ipv6-slaac-dhcpv6.md) – NDP statt ARP
- [Port-Security & 802.1X](../Netzwerk-Administration/port-security-8021x.md) – DAI, Sticky-MAC, SPAN
- [FHRP / VRRP](../Netzwerk-Administration/fhrp-vrrp-hsrp.md) – Gratuitous ARP beim Gateway-Wechsel
- [Hacker-Angriffe](../Sicherheit/hacking/hackers-can.md) – ARP-Spoofing als Bedrohung
- [Lektion 06 – IPv4 & Routing](../../analyse-der-exams/06_IPv4_Adressierung_Routing.md)
