# VLAN (Virtual LAN)

> Grundlagen zum Switch als Gerät (MAC-Tabelle, Domänen, Auswahl, Härtung) stehen in [Switches – Grundlagen](switches-und-vlans.md). Dieses Blatt behandelt die **VLAN-Details**.

## In 30 Sekunden

- Ein **VLAN** teilt ein physisches Switch-Netz in mehrere **logische Netze = getrennte Broadcast-Domänen**; jedes VLAN bekommt ein **eigenes IP-Subnetz**.
- **Access-Port** = genau ein VLAN, **untagged** (Endgeräte). **Trunk-Port** = mehrere VLANs, **tagged nach IEEE 802.1Q** (Switch–Switch, Switch–Router, Switch–Hypervisor).
- Das **802.1Q-Tag** ist **4 Byte (32 Bit)** groß und enthält eine **12-Bit-VLAN-ID** (1–4094) und 3 Bit Priorität.
- Zwischen VLANs kommuniziert man nur über **Layer 3**: **Router-on-a-Stick** (Subinterfaces) oder **Layer-3-Switch** (SVI).
- VLAN kam in **6 von 10 GA2-Terminen** vor (2022S, 2023S, 2023W, 2024W, 2025S, 2025W) – Gründe, Portmodi, natives VLAN, Relay, Konfigurationsfehler.

## Kernwissen

### Gründe für VLANs (fast immer „nennen Sie zwei/drei“)

| Grund | Erklärung |
|---|---|
| **Sicherheit / Segmentierung** | Abteilungen, Gäste, VoIP, Server, Management getrennt; Übergänge nur über Router/Firewall mit Regeln |
| **Weniger Broadcast-Last** | Broadcasts bleiben im eigenen VLAN → bessere Performance |
| **Flexibilität** | Zuordnung per Konfiguration statt Umpatchen; Abteilung über mehrere Etagen möglich |
| **QoS** | eigenes Voice-VLAN, Sprachverkehr priorisierbar (2025W) |
| **Kosten** | ein Switch statt mehrerer physisch getrennter Netze |

### Portmodi

| Modus | VLANs | Tagging | Einsatz |
|---|---|---|---|
| **Access** (untagged) | 1 | Switch entfernt/setzt Tag intern, Endgerät sieht keins | PC, Drucker, AP mit nur einer SSID |
| **Trunk** (tagged) | viele | 802.1Q-Tag pro Frame; **ein natives VLAN untagged** | Uplinks, Router-on-a-Stick, ESXi/Hyper-V, AP mit mehreren SSIDs |
| **Hybrid / Voice-VLAN** | 1 untagged + 1 tagged | Daten untagged, Telefon tagged | IP-Telefon mit durchgeschleiftem PC |

### 802.1Q-Tag

```
| Ziel-MAC | Quell-MAC | 802.1Q-Tag (4 Byte) | EtherType | Daten | FCS |
                        |TPID 0x8100 (16)|PCP (3)|DEI (1)|VID (12)|
```
- Das Tag wird **zwischen Quell-MAC und EtherType** eingefügt → Frame wird 4 Byte größer, FCS neu berechnet, **MAC-Adressen bleiben gleich** (2023W).
- **VID 12 Bit** → 4.096 Werte, nutzbar **1–4094** (0 und 4095 reserviert).
- **PCP 3 Bit** = Priorität **802.1p** (0–7) → QoS für Sprache.
- **Eingehend** am Trunk: Tag bestimmt das VLAN; untagged → natives VLAN. **Ausgehend**: Tag wird gesetzt, außer für das native VLAN (2024W).

### Natives VLAN und Default-VLAN (GA2 Winter 2024)

- **Natives VLAN:** VLAN, dessen Frames auf dem Trunk **ohne Tag** laufen (Standard VLAN 1). Muss auf beiden Trunk-Seiten gleich sein (sonst *Native VLAN Mismatch*).
- **Default-VLAN:** VLAN, in dem alle Ports ab Werk sind (VLAN 1).
- **Warum nicht gleich lassen?** **VLAN-Hopping per Double Tagging:** Angreifer im nativen VLAN sendet einen Frame mit zwei Tags; der erste Switch entfernt das äußere (natives VLAN), der zweite liest das innere → Frame landet im Ziel-VLAN. Gegenmaßnahme: **natives VLAN = ungenutztes VLAN** (z. B. 999), keine Endgeräte darin, ggf. natives VLAN taggen.

### VLAN-Zuordnung: statisch vs. dynamisch (2023S, 2024W)

| | Portbasiert (statisch) | Dynamisch (802.1X / MAC) |
|---|---|---|
| Prinzip | Admin legt Port fest auf VLAN X | RADIUS weist nach Anmeldung das VLAN zu |
| Folge | Gerät erhält VLAN durch die Dose, an der es steckt | Benutzer/Gerät „nimmt sein VLAN mit“ |
| Vorteil | einfach, transparent | flexibel, unbekannte Geräte → Gäste-/Quarantäne-VLAN |
| Nachteil | Umzug = Umkonfiguration; fremde Geräte bekommen Zugang | RADIUS-Infrastruktur, Supplicant nötig |

### Inter-VLAN-Routing

| Variante | Umsetzung | Bewertung |
|---|---|---|
| **Router-on-a-Stick / One-armed Router** (2025W) | ein Router-Port als Trunk, je VLAN ein **Subinterface** mit `encapsulation dot1Q <ID>` und Gateway-IP | günstig; der eine Link ist Flaschenhals |
| **Layer-3-Switch** | je VLAN ein **SVI** (`interface vlan 10`) mit Gateway-IP, `ip routing` | schnell (Hardware), Standard im Unternehmen |
| Router mit je einem Port pro VLAN | physisch getrennte Interfaces | skaliert schlecht |

**Warum `encapsulation dot1Q` zwingend? (2025W)** Ohne die Angabe weiß das Subinterface nicht, welche getaggten Frames es annehmen und mit welchem Tag es antworten soll – die VLANs wären nicht unterscheidbar.

### Konfigurationsbeispiel (Cisco)

```
! Switch
vlan 10
 name BUERO
vlan 20
 name VOIP
interface gi0/1
 switchport mode access
 switchport access vlan 10
 switchport voice vlan 20
interface gi0/24
 switchport mode trunk
 switchport trunk allowed vlan 10,20
 switchport trunk native vlan 999

! Router-on-a-Stick
interface g0/0.10
 encapsulation dot1Q 10
 ip address 192.168.10.1 255.255.255.0
 ip helper-address 10.0.0.5
interface g0/0.20
 encapsulation dot1Q 20
 ip address 192.168.20.1 255.255.255.0
```
Linux-Schreibweise der Subinterfaces: `eth1.10`, `eth1.20` (2025S).

### Typische Fehler

| Symptom | Ursache |
|---|---|
| Clients im neuen VLAN erreichen nichts außerhalb, `nslookup` Timeout (2023W) | Subinterface/SVI fehlt, Gateway-IP falsch, VLAN auf dem Trunk nicht erlaubt |
| Gateway-IP passt nicht zum Subnetz (2025W) | Subinterface `.20` mit IP aus `192.168.30.0/24` |
| Kein DHCP in anderen VLANs (2023S, 2024W) | **DHCP-Relay** (`ip helper-address`) fehlt |
| Ping zwischen VLANs geht nicht, Gateway-Ping geht | Ziel-Port im falschen Access-VLAN, ACL/Firewall, Host-Firewall |
| Sporadische Ausfälle am Trunk | Native VLAN Mismatch |

## Prüfungsfrage

**Frage (angelehnt an GA2 Winter 2025):** Ein Switch verbindet Büro-PCs (VLAN 231), Werkstatt (VLAN 232) und IP-Telefone (VLAN 233); Port G0/24 führt zum Router. Geben Sie für G0/1 (Büro-PC), G0/10 (Telefon) und G0/24 Modus und VLANs an und begründen Sie, warum der Router für G0/24 Subinterfaces benötigt.

**Musterantwort:** G0/1: Access-Port, VLAN 231, untagged. G0/10: Access-Port, VLAN 233, untagged (bzw. als Voice-VLAN tagged, wenn ein PC am Telefon hängt). G0/24: Trunk-Port, VLANs 231, 232, 233 **tagged** (802.1Q). Der Router hat nur eine physische Verbindung, muss aber Gateway für drei Subnetze sein. Dafür erhält er pro VLAN ein Subinterface (z. B. `g0/0.231`) mit `encapsulation dot1Q 231` und der Gateway-IP dieses VLANs. So kann er anhand des Tags zuordnen, aus welchem VLAN ein Frame kommt, und zwischen den VLANs routen (Router-on-a-Stick).

## Falle

- „VLANs verbinden sich automatisch“ → für Kommunikation zwischen VLANs ist **immer Layer 3** nötig.
- VLAN-Tag mit „12 Bit“ beantworten, wenn nach der **Taggröße** gefragt ist → Tag **32 Bit**, davon VLAN-ID **12 Bit**.
- Endgeräte-Ports als Trunk konfigurieren → Einfallstor für **Switch Spoofing** (Angreifer handelt per DTP einen Trunk aus). Access-Ports fest auf `mode access`, DTP aus.
- Natives VLAN = Benutzer-VLAN oder VLAN 1 lassen → Double-Tagging-Gefahr.
- „VLAN ist eine vollständige Sicherheitsmaßnahme“ → nur Segmentierung; Regeln am Übergang (ACL/Firewall) machen die Sicherheit.

## Merksatz

**„Access = ein VLAN, ungetaggt. Trunk = viele VLANs, getaggt. Zwischen VLANs hilft nur Layer 3.“**

## Verwandt

- [Switches – Grundlagen](switches-und-vlans.md) – Switch-Funktionsweise, Auswahl, Härtung
- [Routing](routing.md) – Routingtabellen hinter den Gateways
- [DHCP](../OSI-IP-mac-bit-byte/dhcp.md) – Relay über VLAN-Grenzen
- [Subnetting](../OSI-IP-mac-bit-byte/subnetting-ipv4-ipv6.md) – ein Subnetz pro VLAN
- [Port-Security & 802.1X](port-security-8021x.md) – dynamische VLAN-Zuweisung
- [VoIP & QoS](voip-qos.md) – Voice-VLAN und 802.1p
- [WLAN](wlan.md) – SSID-zu-VLAN-Zuordnung, Gast-VLAN
- [Firewall](../Sicherheit/firewall/firewall.md) – Regeln zwischen den Segmenten
- [Lektion 15 – VLAN & Routing](../../analyse-der-exams/L15_VLAN_Routing.md)
