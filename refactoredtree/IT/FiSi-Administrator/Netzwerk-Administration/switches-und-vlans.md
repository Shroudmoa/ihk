# Switches – Grundlagen, Auswahl und Absicherung

> Dieses Blatt behandelt den **Switch als Gerät** (Funktionsweise, Kollisions-/Broadcast-Domänen, Auswahl, Härtung). Alles zu **VLANs** (Access/Trunk, 802.1Q, natives VLAN, Inter-VLAN-Routing, VLAN-Hopping) steht im Blatt [VLAN](vlan.md).

## In 30 Sekunden

- Ein Switch arbeitet auf **Schicht 2**: Er lernt **Quell-MACs** pro Port und leitet Frames gezielt anhand der **Ziel-MAC** weiter.
- Jeder Switchport ist eine eigene **Kollisionsdomäne**; alle Ports (eines VLANs) bilden **eine Broadcast-Domäne** – erst VLANs bzw. Router teilen sie.
- **Managed Switches** bieten VLANs, STP, LACP, 802.1X, Port-Security, SNMP, Port-Mirroring – **Layer-3-Switches** zusätzlich Routing.
- Auswahlkriterien in der Prüfung: **Portanzahl/-geschwindigkeit, SFP/SFP+-Uplinks, PoE-Budget, Managementfunktionen** (GA2 Winter 2021).
- Absicherung: **Management-VLAN, SSH statt Telnet, AAA/RADIUS, ungenutzte Ports abschalten, Port-Security** (GA2 Winter 2024).

## Kernwissen

### Arbeitsweise

| Schritt | Aktion |
|---|---|
| Lernen | Quell-MAC + Port in die MAC-Tabelle (CAM) |
| Weiterleiten | bekannte Ziel-MAC → nur an diesen Port |
| Fluten | Broadcast, Multicast, unbekanntes Ziel → alle Ports des VLANs |
| Altern | Einträge verfallen nach Inaktivität (typisch 300 s) |

**Weiterleitungsverfahren**
| Verfahren | Funktionsweise | Eigenschaft |
|---|---|---|
| Store-and-Forward | ganzen Frame empfangen, FCS prüfen, dann senden | fehlerhafte Frames werden verworfen, etwas mehr Latenz – Standard |
| Cut-Through | nach Ziel-MAC sofort weiterleiten | minimale Latenz, Fehler werden mitgeschleppt |
| Fragment-Free | erste 64 Byte prüfen | Kompromiss |

### Kollisions- und Broadcast-Domänen

| Gerät | Kollisionsdomänen | Broadcast-Domänen |
|---|---|---|
| Hub (8 Ports) | 1 | 1 |
| Switch (24 Ports, 1 VLAN) | 24 | 1 |
| Switch (24 Ports, 3 VLANs) | 24 | 3 |
| Router (3 Interfaces) | 3 | 3 |

Im **Vollduplex**-Betrieb (heute Standard) gibt es praktisch keine Kollisionen mehr; CSMA/CD ist nur bei Halbduplex relevant.

### Duplex und Geschwindigkeit (GA2 Winter 2024)

- **Auto-Negotiation:** Beide Seiten handeln Speed und Duplex aus.
- **Duplex-Mismatch** (eine Seite fest, andere auto) → Halbduplex auf einer Seite, späte Kollisionen, CRC-Fehler, massiver Leistungsverlust.
- **Nur 100 Mbit/s statt 1 Gbit/s:** Kabel nutzt nur 2 Adernpaare oder ist beschädigt (Gigabit braucht **alle 4 Paare**), Cat-5-Kabel, Port fest auf 100 konfiguriert, NIC-Treiber.

### Bauformen und Einsatzebenen

| Merkmal | Ausprägungen |
|---|---|
| Verwaltung | unmanaged (Plug & Play), smart/web-managed, voll managed (CLI, SNMP) |
| Schicht | Layer 2 (Switching), **Layer 3** (zusätzlich Routing per SVI, DHCP-Relay) |
| Hierarchie | **Access** (Endgeräte, PoE, 802.1X) → **Distribution** (Aggregation, Routing zwischen VLANs) → **Core** (schneller Backbone, 10/25/40/100G) |
| Uplinks | SFP (1 Gbit/s), SFP+ (10 Gbit/s), SFP28 (25 Gbit/s), QSFP+ (40 Gbit/s) |
| Redundanz | Stacking (mehrere Switches als ein logisches Gerät), redundante Netzteile, LACP |
| Stromversorgung von Endgeräten | PoE / PoE+ / PoE++ mit **PoE-Budget** |

### Auswahl nach Anforderungen (GA2 Winter 2021)

Vorgehen: Jede Anforderung als **K.o.-Kriterium** prüfen und Rechnungen zeigen.
| Kriterium | Prüfung |
|---|---|
| Anzahl/Speed Ports | genug 1-Gbit/s-RJ45 für alle Geräte + Reserve? |
| Uplink | SFP+ für 10G zum Core vorhanden? |
| PoE-Budget | Σ Leistungsaufnahme aller PoE-Geräte ≤ Budget, z. B. 8 APs × 17,9 W = **143,2 W** |
| PoE-Klasse je Port | Gerät braucht PoE+ (bis 25,5 W am Gerät) → Port muss 802.3at liefern |
| Funktionen | VLAN, 802.1X, LACP, SNMP, Layer 3 nötig? |

### Switch absichern (GA2 Winter 2024 – „vier Einstellungen“)

- Verwaltung nur über ein eigenes **Management-VLAN** und per **ACL** nur aus dem Admin-Netz.
- **SSH/HTTPS** statt Telnet/HTTP; Telnet deaktivieren.
- **Starke, individuelle Admin-Konten** bzw. zentrale Anmeldung per **RADIUS/TACACS+**; Standardpasswörter ändern.
- **Ungenutzte Ports deaktivieren** und in ein ungenutztes „Parkplatz“-VLAN legen.
- **Port-Security / 802.1X** an Access-Ports, **DHCP-Snooping**, **BPDU Guard**.
- Firmware aktuell halten, Konfiguration sichern, Logging per **Syslog**, Zeit per **NTP**.

### Port-Mirroring (SPAN)

Ein Switchport erhält eine **Kopie** des Verkehrs anderer Ports oder VLANs → Analyse mit Wireshark/IDS, ohne den Betrieb zu stören (GA2 Sommer 2026, ARP-Poisoning-Analyse).

## Prüfungsfrage

**Frage (angelehnt an GA2 Winter 2021):** Für eine Etage werden 8 Access Points (je 17,9 W, PoE+) und 12 PCs angeschlossen. Der Uplink zum Core soll mit 10 Gbit/s über Glasfaser laufen. Switch A: 24 × 1G RJ45, PoE+-Budget 250 W, keine SFP-Ports. Switch B: 24 × 1G RJ45, 2 × SFP+, PoE-Budget 60 W. Switch C: 24 × 1G RJ45, 2 × SFP+, PoE+-Budget 180 W. Welche Switches kommen nicht in Frage? Begründen Sie.

**Musterantwort:** Der PoE-Bedarf beträgt 8 × 17,9 W = 143,2 W, benötigt werden 20 Ports und ein 10-Gbit/s-Glasfaser-Uplink. **Switch A** scheidet aus, weil er keinen SFP+-Port für den 10G-Uplink hat. **Switch B** scheidet aus, weil sein PoE-Budget von 60 W für 143,2 W nicht reicht. **Switch C** erfüllt alle Anforderungen (24 Ports, 2 × SFP+, 180 W ≥ 143,2 W).

## Falle

- „Switch trennt Broadcast-Domänen“ → nur **mit VLANs**; ohne VLAN ist der ganze Switch eine Broadcast-Domäne.
- Hub und Switch gleichsetzen → Hub (Schicht 1) sendet alles an alle Ports.
- PoE-Budget nur pro Port prüfen und die **Summe** vergessen (oder umgekehrt).
- SFP und SFP+ verwechseln: SFP = **1 Gbit/s**, SFP+ = **10 Gbit/s**.
- „100 Mbit/s statt Gigabit“ nur auf den Switch schieben → Kabel und Gegenstelle gehören zur Ursachenliste.

## Merksatz

**„Der Switch lernt von der Quelle und liefert ans Ziel – trennen kann er erst mit VLANs.“**

## Verwandt

- [VLAN](vlan.md) – VLAN-Details: Access/Trunk, 802.1Q, natives VLAN, Inter-VLAN-Routing
- [Netzwerkgeräte](netzwerkgeraete.md) – Hub, Switch, Router, L3-Switch, AP im Vergleich
- [STP](stp-spanning-tree.md) – Schleifen verhindern
- [Link Aggregation (LACP)](link-aggregation-lacp.md) – Uplinks bündeln
- [PoE](poe.md) – Standards und Budget
- [Port-Security & 802.1X](port-security-8021x.md) – Zugangskontrolle an Switchports
- [Verkabelung](verkabelung-kupfer-lwl.md) – Kupfer, LWL, SFP-Module
- [MAC](../OSI-IP-mac-bit-byte/mac.md) – MAC-Tabelle und Frame-Aufbau
- [Lektion 15 – VLAN & Routing](../../analyse-der-exams/L15_VLAN_Routing.md)
