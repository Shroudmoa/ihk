# IPv6

## IPv6 – Adressvergabe und Neighbor Discovery (IHK)

IPv6-Adressen besitzen **128 Bit** und werden hexadezimal dargestellt, z. B. `2001:db8::10`.

### Wie erhält ein Gerät eine IPv6-Adresse?

| Verfahren | Funktionsweise | Vorteile | Nachteile |
|---|---|---|---|
| SLAAC | Host erstellt seine Adresse anhand von Router Advertisements (RA). | Automatisch, kein DHCPv6-Server erforderlich. | Weniger zentrale Kontrolle über Adressen. |
| DHCPv6 Stateful | DHCPv6-Server weist IPv6-Adressen und Optionen zu. | Zentrale Verwaltung, Reservierungen und Protokollierung möglich. | Server und Konfiguration erforderlich. |
| DHCPv6 Stateless | Host erzeugt seine Adresse per SLAAC; DHCPv6 liefert zusätzliche Parameter. | Zentrale Verteilung zusätzlicher Optionen. | Adressvergabe bleibt dezentral. |
| Manuell | Administrator konfiguriert die IPv6-Adresse. | Volle Kontrolle, geeignet für statische Systeme. | Verwaltungsaufwand, Fehlergefahr bei großen Netzen. |

### Ist DHCPv6 besser?

**Nicht grundsätzlich.** Es hängt von den Anforderungen ab.

- **DHCPv6 Stateful:** Sinnvoll, wenn zentrale Adressverwaltung, Reservierungen und Protokollierung wichtig sind.
- **SLAAC:** Sinnvoll, wenn Geräte sich automatisch und ohne DHCPv6-Server konfigurieren sollen.
- **Kombination:** SLAAC und DHCPv6 können gemeinsam eingesetzt werden.

**Wichtig für die IHK:**
- DHCPv6 weist normalerweise **kein Standardgateway** zu. Dieses wird über Router Advertisements bekannt gegeben.
- DNS-Server können über DHCPv6 oder über Router Advertisements (RDNSS) bekannt gegeben werden.
- DHCPv6-Reservierungen und Protokollierung hängen von Server und Konfiguration ab.

### IPv6: ARP vs. NDP

IPv6 verwendet **kein ARP**. Stattdessen übernimmt **Neighbor Discovery Protocol (NDP)** auf Basis von ICMPv6 unter anderem folgende Aufgaben:

| Funktion | Protokoll / Mechanismus |
|---|---|
| Nachbarn und deren Link-Layer-Adressen ermitteln | Neighbor Solicitation (NS) / Neighbor Advertisement (NA) |
| Router finden | Router Solicitation (RS) / Router Advertisement (RA) |
| Erreichbarkeit von Nachbarn prüfen | NDP Neighbor Unreachability Detection |
| IPv6-Duplikate erkennen | Duplicate Address Detection (DAD) |

### IHK-Merksätze

- IPv6 = 128 Bit; IPv4 = 32 Bit.
- SLAAC = automatische Adressbildung über Router Advertisements.
- Stateful DHCPv6 = Server weist Adressen zu.
- NDP ersetzt ARP-Funktionen bei IPv6.
- **Sicherheitsbezug:** Rogue Router Advertisements können Clients auf einen falschen Router lenken. Schutz bieten beispielsweise RA Guard und korrekt konfigurierte Switches.


## Dual Stack (IPv4 und IPv6) – IHK

**Definition:** Dual Stack bedeutet, dass IPv4 und IPv6 parallel auf demselben Host oder Netzwerk betrieben werden.

### Funktionsweise

- **IPv4:** 32 Bit, z. B. `192.168.1.10`
- **IPv6:** 128 Bit, z. B. `2001:db8::10`
- **DNS A-Record:** Liefert eine IPv4-Adresse.
- **DNS AAAA-Record:** Liefert eine IPv6-Adresse.
- **Happy Eyeballs (RFC 8305):** Ermöglicht Clients, IPv4 und IPv6 zeitnah auszuprobieren und eine funktionierende Verbindung mit geringer Verzögerung zu bevorzugen.

### DHCPv6 vs. SLAAC

| Verfahren | Funktion | Vorteil |
|---|---|---|
| DHCPv6 | Verteilt IPv6-Adressen und/oder weitere Konfigurationsparameter. | Zentrale Verwaltung und Protokollierung durch den DHCPv6-Server. |
| SLAAC | Hosts bilden IPv6-Adressen anhand von Router Advertisements. | Automatische Konfiguration ohne DHCPv6-Server. |

**Wichtig:** DHCPv6 ersetzt bei IPv6 nicht die Router Advertisements. Das Standardgateway wird normalerweise über Router Advertisements bekannt gegeben.

### Vorteile
- Kompatibilität mit IPv4- und IPv6-Diensten.
- Schrittweise IPv6-Migration ohne sofortige Abschaltung von IPv4.
- Happy Eyeballs reduziert Verzögerungen bei Problemen mit einem Protokoll.

### Nachteile
- Doppelte Konfiguration und Fehleranalyse.
- Zusätzliche Firewall- und Monitoring-Regeln erforderlich.
- IPv6 kann unbeabsichtigt Sicherheitsregeln umgehen, wenn nur IPv4 abgesichert wird.

### IHK-Prüfungsbeispiel

Ein Client erhält über DNS einen A- und einen AAAA-Record. Er verwendet Dual Stack und kann den Server über IPv4 oder IPv6 erreichen. Happy Eyeballs reduziert Verzögerungen, indem beide Verbindungswege zeitnah berücksichtigt werden.

**Merksatz:** Dual Stack = IPv4 und IPv6 parallel; A = IPv4; AAAA = IPv6; DHCPv6 = optionale zentrale IPv6-Konfiguration; SLAAC = automatische IPv6-Konfiguration.

## Verwandt

- [MAC](mac.md) – Link-Layer-Adressen im Nachbarverfahren
- [Netzwerktypen](netzwerktypen.md) – Einsatzbereich und Adressierung
- [OSI](osi.md) – Schicht 3: logische Adressierung

