# MAC

## MAC-Adressen (IHK)

Eine **MAC-Adresse (Media Access Control Address)** ist eine Layer-2-Adresse des OSI-Modells. Sie identifiziert eine Netzwerkschnittstelle innerhalb eines lokalen Netzwerks.

### Aufbau

- **48 Bit = 6 Byte = 12 Hexadezimalstellen**
- Darstellung: `00:1A:2B:3C:4D:5E`

| Bestandteil | Länge | Bedeutung |
|---|---:|---|
| OUI | 24 Bit | Kennzeichnet den zugewiesenen Hersteller-Adressbereich |
| Interface-ID | 24 Bit | Identifiziert die Schnittstelle innerhalb dieses Bereichs |

*Hinweis: Bei universell verwalteten MAC-Adressen gilt dieser klassische Aufbau. Lokal verwaltete MAC-Adressen müssen keinen realen Hersteller erkennen lassen.*

### IHK-Prüfungswissen

- **OSI:** Schicht 2 – Sicherungsschicht.
- **Switch:** Leitet Ethernet-Frames anhand der MAC-Adresstabelle weiter.
- **Router:** Verbindet IP-Netzwerke; für den nächsten Übertragungsabschnitt wird eine passende Layer-2-Adresse verwendet.
- **ARP:** Ermittelt im IPv4-LAN die MAC-Adresse zu einer bekannten IPv4-Adresse.
- **IPv6:** Verwendet Neighbor Discovery (NDP) statt ARP.
- **Sicherheit:** MAC-Adressen sind keine sichere Authentifizierung, da sie gefälscht werden können (MAC Spoofing).

### MAC vs. IP

| MAC-Adresse | IP-Adresse |
|---|---|
| OSI Layer 2 | OSI Layer 3 |
| Lokale Frame-Zustellung | Logische Adressierung und Routing |
| Beispiel: `00:1A:2B:3C:4D:5E` | Beispiel: `192.168.1.10` |
| Switch | Router |

**Merksatz:** MAC = lokale Zustellung im Layer-2-Netz; IP = logische Adressierung zwischen Netzwerken.

## Verwandt

- [OSI](osi.md) – Einordnung in Schicht 2
- [Netzwerktypen](netzwerktypen.md) – Adressierung im lokalen Netz
- [IPv6](ipv6.md) – Neighbor Discovery statt ARP
- [hackers-can](hackers-can.md) – ARP Spoofing / MAC Spoofing

