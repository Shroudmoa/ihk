# IPv4

# IPv4, Subnetting und IPv6 – IHK AP2

## IPv4 – Grundlagen

**IPv4 (Internet Protocol Version 4)** adressiert Geräte auf Layer 3 des OSI-Modells. Eine IPv4-Adresse besteht aus **32 Bit**, aufgeteilt in vier Oktette mit jeweils 8 Bit.

- Beispiel: `192.168.1.10`
- Binär: `11000000.10101000.00000001.00001010`
- Adressbereich: `0.0.0.0` bis `255.255.255.255`
- Theoretischer Adressraum: \(2^{32} = 4.294.967.296\) Adressen
- Schreibweise: Dezimal mit Punkten, z. B. `192.168.1.10/24`

Eine IP-Adresse besteht logisch aus **Netzanteil** und **Hostanteil**. Die Subnetzmaske bzw. der CIDR-Präfix legt fest, welche Bits zum Netz gehören.

## IPv4-Adressklassen – historisches System

Die Klassen A bis E sind historisch relevant. Moderne Netzwerke verwenden **CIDR** statt fester Klassen, um Netzgrößen flexibel festzulegen.

| Klasse | Historischer Bereich | Historische Verwendung | Beispiel |
|---|---|---|---|
| A | `1.0.0.0` – `126.255.255.255` | Große Unicast-Netze | `10.0.0.0/8` ist privat |
| B | `128.0.0.0` – `191.255.255.255` | Mittlere Unicast-Netze | `172.16.0.0/12` ist privat |
| C | `192.0.0.0` – `223.255.255.255` | Kleine Unicast-Netze | `192.168.0.0/16` ist privat |
| D | `224.0.0.0` – `239.255.255.255` | Multicast | `224.0.0.1` |
| E | `240.0.0.0` – `255.255.255.255` | Reserviert bzw. spezielle Zwecke | `240.0.0.1` |

**Wichtig:**
- `127.0.0.0/8` ist für Loopback reserviert, z. B. `127.0.0.1`.
- `0.0.0.0` hat spezielle Bedeutungen, z. B. als nicht spezifizierte Adresse.
- `255.255.255.255` ist die begrenzte IPv4-Broadcast-Adresse.
- Die historischen Klassenbereiche sind keine zuverlässige Methode, um die Netzgröße einer heutigen IP-Adresse zu bestimmen. Dafür nutzt man den Präfix, z. B. `/20` oder `/27`.

Quelle: [RFC 4632 – CIDR](https://www.rfc-editor.org/rfc/rfc4632)

## Private IPv4-Adressen – RFC 1918

Private IPv4-Adressen werden in internen Netzen verwendet und sind **nicht direkt global über das Internet routbar**. Für den Internetzugang wird häufig NAT am Router eingesetzt.

| Privater Bereich | CIDR | Subnetzmaske | Anzahl Adressen |
|---|---|---|---:|
| `10.0.0.0` – `10.255.255.255` | `10.0.0.0/8` | `255.0.0.0` | 16.777.216 |
| `172.16.0.0` – `172.31.255.255` | `172.16.0.0/12` | `255.240.0.0` | 1.048.576 |
| `192.168.0.0` – `192.168.255.255` | `192.168.0.0/16` | `255.255.0.0` | 65.536 |

**Merke:** Nicht der gesamte Bereich `172.x.x.x` ist privat. Privat ist nur `172.16.0.0` bis `172.31.255.255`.

Quelle: [RFC 1918 – Private Address Allocation](https://www.rfc-editor.org/rfc/rfc1918)

## Subnetting – ein großes Netz aufteilen

**Subnetting** teilt einen IP-Adressbereich in kleinere logische Netze. Das verbessert die Adressplanung, trennt Broadcast-Domänen und ermöglicht eine strukturierte Netzwerkverwaltung.

### CIDR-Präfix und Subnetzmaske

Der CIDR-Präfix gibt an, wie viele der 32 Bits zum Netzanteil gehören.

Beispiel: `192.168.1.0/24`

- Netzanteil: 24 Bit
- Hostanteil: \(32 - 24 = 8\) Bit
- Subnetzmaske: `255.255.255.0`
- Gesamtadressen: \(2^8 = 256\)
- Üblicherweise nutzbare Hostadressen: \(256 - 2 = 254\)

Die zwei abgezogenen Adressen sind normalerweise die Netzadresse und die Broadcast-Adresse. Sonderfälle wie `/31` für Punkt-zu-Punkt-Verbindungen und `/32` für eine einzelne Adresse folgen anderen Regeln.

### Wichtige Präfixe

| CIDR | Subnetzmaske | Gesamtadressen | Übliche nutzbare Hosts |
|---|---|---:|---:|
| `/8` | `255.0.0.0` | 16.777.216 | 16.777.214 |
| `/16` | `255.255.0.0` | 65.536 | 65.534 |
| `/24` | `255.255.255.0` | 256 | 254 |
| `/25` | `255.255.255.128` | 128 | 126 |
| `/26` | `255.255.255.192` | 64 | 62 |
| `/27` | `255.255.255.224` | 32 | 30 |
| `/28` | `255.255.255.240` | 16 | 14 |
| `/29` | `255.255.255.248` | 8 | 6 |
| `/30` | `255.255.255.252` | 4 | 2 |

### Formeln für Subnetting

- **Hostbits:** `32 − Präfix`
- **Gesamtadressen:** \(2^{\text{Hostbits}}\)
- **Übliche nutzbare Hosts:** \(2^{\text{Hostbits}} - 2\)
- **Anzahl gleich großer Subnetze:** \(2^{\text{zusätzlich ausgeliehene Netzbits}}\)

Die Hostformel gilt für gewöhnliche IPv4-Subnetze, nicht unverändert für `/31`- und `/32`-Sonderfälle.

### Beispiel: `192.168.1.0/26`

Ein `/26`-Netz hat 6 Hostbits.

- Gesamtadressen: \(2^6 = 64\)
- Nutzbare Hosts: \(64 - 2 = 62\)
- Subnetzmaske: `255.255.255.192`
- Blockgröße im letzten Oktett: `256 − 192 = 64`

| Eigenschaft | Wert |
|---|---|
| Netzadresse | `192.168.1.0` |
| Erste Hostadresse | `192.168.1.1` |
| Letzte Hostadresse | `192.168.1.62` |
| Broadcast-Adresse | `192.168.1.63` |

Die nächsten `/26`-Netze beginnen bei `.64`, `.128` und `.192`.

### Subnetting mit einer bestimmten Hostanzahl

Beispiel: Ein Teilnetz soll mindestens 50 Geräte aufnehmen.

- 5 Hostbits ergeben \(2^5 - 2 = 30\) Hosts – zu wenig.
- 6 Hostbits ergeben \(2^6 - 2 = 62\) Hosts – ausreichend.
- Benötigter Präfix: `32 − 6 = /26`.

**Ergebnis:** Ein `/26`-Subnetz bietet 62 übliche Hostadressen.

## Netzadresse und Broadcast-Adresse

In einem gewöhnlichen IPv4-Subnetz haben zwei Adressen besondere Aufgaben.

- **Netzadresse:** Hostbits sind alle `0`. Sie bezeichnet das Subnetz.
- **Broadcast-Adresse:** Hostbits sind alle `1`. Sie adressiert alle Geräte im lokalen IPv4-Subnetz.
- **Hostadressen:** Adressen zwischen Netz- und Broadcast-Adresse, normalerweise für Geräte verwendbar.

Beispiel für `192.168.10.0/24`:

| Typ | Adresse |
|---|---|
| Netzadresse | `192.168.10.0` |
| Hostbereich | `192.168.10.1` – `192.168.10.254` |
| Broadcast | `192.168.10.255` |

Ein Router leitet einen lokalen Broadcast normalerweise nicht einfach in andere Subnetze weiter. Für die Kommunikation zwischen IP-Netzen wird Routing benötigt.

## IPv4 vs. IPv6

IPv6 wurde unter anderem entwickelt, um den stark begrenzten IPv4-Adressraum zu erweitern und die Netzwerkkonfiguration weiterzuentwickeln.

| Eigenschaft | IPv4 | IPv6 |
|---|---|---|
| Adresslänge | 32 Bit | 128 Bit |
| Schreibweise | Dezimal, vier Oktette | Hexadezimal, acht Gruppen |
| Beispiel | `192.168.1.10` | `2001:db8::10` |
| Adressraum | \(2^{32}\) | \(2^{128}\) |
| Broadcast | Ja | Nein |
| Multicast | Ja | Ja |
| Automatische Konfiguration | DHCP, teils automatische Verfahren | SLAAC und/oder DHCPv6 |
| NAT | Häufig eingesetzt | Für globale IPv6-Kommunikation normalerweise nicht erforderlich |
| ARP | Für IPv4 im lokalen Netz | Kein ARP; Neighbor Discovery nutzt ICMPv6 |
| Header | Variabel, mindestens 20 Byte | Fester Basisheader mit 40 Byte |
| Fragmentierung | Router und Endgeräte können fragmentieren | Nur der Sender fragmentiert, falls nötig |
| Loopback | `127.0.0.1` | `::1` |
| Private/lokale Adressen | RFC-1918-Adressen | Unique Local Addresses, typischerweise `fc00::/7` |
| Link-local | `169.254.0.0/16` | `fe80::/10` |

`2001:db8::/32` ist für Dokumentation und Beispiele reserviert und keine normale öffentliche Zuweisung.

### IPv6: wichtige Unterschiede

- IPv6 verwendet **128 Bit** und bietet dadurch einen sehr großen Adressraum.
- IPv6 kennt keinen Broadcast; vergleichbare Aufgaben übernehmen unter anderem Multicast und Anycast.
- SLAAC ermöglicht Geräten, selbstständig eine IPv6-Adresse zu konfigurieren. DHCPv6 kann ergänzend oder alternativ eingesetzt werden.
- IPv6 verwendet ICMPv6 Neighbor Discovery statt ARP.
- NAT ist bei IPv6 nicht grundsätzlich erforderlich, aber Firewalls bleiben wichtig. Eine globale IPv6-Adresse bedeutet nicht, dass ein Gerät ungeschützt erreichbar sein sollte.

## Vorteile und Nachteile

### IPv4

**Vorteile**
- Sehr weit verbreitet und von nahezu allen gängigen Netzwerksystemen unterstützt.
- Bekannte Werkzeuge, ausgereifte Infrastruktur und umfangreiche Betriebserfahrung.
- NAT ermöglicht mehreren privaten Geräten die gemeinsame Nutzung einer öffentlichen IPv4-Adresse.

**Nachteile**
- Begrenzter Adressraum; öffentliche IPv4-Adressen sind knapp.
- NAT erhöht die Komplexität bestimmter eingehender Verbindungen und kann Fehlersuche erschweren.
- Subnetting und manuelle Adressplanung können bei großen Netzen aufwendig werden.

### IPv6

**Vorteile**
- Sehr großer Adressraum mit 128 Bit.
- Kein Broadcast; Multicast und Neighbor Discovery übernehmen entsprechende Netzwerkaufgaben.
- SLAAC ermöglicht automatische Adresskonfiguration.
- Unterstützt moderne, hierarchische Adressplanung und Routing-Aggregation.

**Nachteile**
- Dual-Stack-Betrieb kann vorübergehend zusätzliche Komplexität verursachen.
- Administratoren müssen IPv6-Adressierung, Routing, ICMPv6 und Sicherheitsregeln beherrschen.
- Ältere Geräte oder Anwendungen können IPv6 unvollständig unterstützen.
- Die längeren Adressen sind für Menschen schwieriger zu lesen und manuell zu verwalten.

## IHK-Merksätze

- IPv4 hat 32 Bit, IPv6 hat 128 Bit.
- CIDR `/26` bedeutet: 26 Netzbits und 6 Hostbits.
- In einem gewöhnlichen IPv4-Subnetz sind Netz- und Broadcast-Adresse nicht als normale Hostadressen nutzbar.
- RFC 1918 definiert private IPv4-Bereiche; private Adressen sind nicht global eindeutig.
- Ein Router verbindet unterschiedliche IP-Netze; ein Switch verbindet Geräte innerhalb eines lokalen Netzes.
- IPv6 besitzt keinen Broadcast und nutzt ICMPv6 Neighbor Discovery statt ARP.
- NAT ist kein Ersatz für eine Firewall.
- Subnetting kann Broadcast-Domänen verkleinern; die Kommunikation zwischen Subnetzen benötigt Routing.

## Quellen

- [RFC 1918 – Private IPv4-Adressbereiche](https://www.rfc-editor.org/rfc/rfc1918)
- [RFC 4632 – CIDR und klassenlose Adressierung](https://www.rfc-editor.org/rfc/rfc4632)
- [RFC 791 – Internet Protocol (IPv4)](https://www.rfc-editor.org/rfc/rfc791)
- [RFC 8200 – Internet Protocol, Version 6 (IPv6)](https://www.rfc-editor.org/rfc/rfc8200)
