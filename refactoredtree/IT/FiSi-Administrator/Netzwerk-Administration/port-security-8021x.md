# Port-Security, 802.1X & RADIUS

> **Abgrenzung:** Dieses Blatt zeigt die **Switch-/Netzwerk-Sicht** (Port-Konfiguration, Violation-Modi, DHCP-Snooping, DAI). Rollen, EAP-Methoden und RADIUS aus **Sicherheits-Sicht** → [Netzwerkzugangskontrolle: 802.1X & RADIUS](../Sicherheit/netzwerkzugang-802-1x-radius.md).

## In 30 Sekunden

- Ziel: **Fremde Geräte an Netzwerkdosen** sollen keinen Zugang bekommen (GA2 Sommer 2022) und Angriffe im LAN (ARP-Poisoning, Rogue DHCP) sollen ins Leere laufen.
- **Port-Security:** Switch erlaubt pro Port nur **bestimmte/wenige MAC-Adressen** (statisch oder **Sticky-MAC**), bei Verstoß z. B. **Shutdown** (GA2 Sommer 2026).
- **IEEE 802.1X:** portbasierte Authentifizierung – **Supplicant** (Client) → **Authenticator** (Switch/AP) → **Authentication Server** (RADIUS); erst nach Erfolg wird der Port freigeschaltet, optional mit **dynamischem VLAN** (2023S).
- **RADIUS** (UDP **1812** Auth, **1813** Accounting) liefert **AAA**: Authentication, Authorization, Accounting.
- Ergänzend am Switch: **DHCP-Snooping**, **Dynamic ARP Inspection**, **BPDU Guard**, ungenutzte Ports abschalten.

## Kernwissen

### Port-Security

| Einstellung | Wirkung |
|---|---|
| maximale MAC-Anzahl | z. B. 1 (PC) oder 2 (Telefon + PC) |
| statische MAC | Admin trägt erlaubte MAC fest ein |
| **Sticky-MAC** | Switch lernt die erste(n) MAC(s) und speichert sie in der Konfiguration |
| Verletzungsmodus **protect** | fremde Frames still verwerfen |
| **restrict** | verwerfen + Log/SNMP-Trap |
| **shutdown** (Standard) | Port geht in *err-disabled*, Admin muss ihn wieder freigeben |

```
interface gi0/5
 switchport mode access
 switchport port-security
 switchport port-security maximum 1
 switchport port-security mac-address sticky
 switchport port-security violation shutdown
```
**Grenze:** MAC-Adressen lassen sich fälschen → Port-Security ist eine Hürde, keine starke Authentifizierung. Schützt aber gut gegen **MAC-Flooding** und spontan angesteckte Geräte.

### 802.1X – Ablauf

```
Client (Supplicant)  --EAPoL-->  Switch/AP (Authenticator)  --RADIUS-->  RADIUS-Server (z. B. NPS + AD)
```
1. Gerät wird angesteckt; Port lässt nur **EAPoL**-Verkehr durch (unautorisiert).
2. Switch fordert die Identität an, der Client antwortet (Benutzer/Zertifikat).
3. Switch reicht die EAP-Nachrichten in **RADIUS**-Paketen an den Server weiter.
4. Server prüft gegen AD/LDAP bzw. Zertifikat → **Access-Accept** (ggf. mit VLAN-ID) oder **Access-Reject**.
5. Bei Accept: Port wird autorisiert, Client landet im zugewiesenen VLAN; bei Reject: Port bleibt gesperrt oder Gäste-/Quarantäne-VLAN.

| Rolle | Gerät | Aufgabe |
|---|---|---|
| Supplicant | PC, Notebook, Smartphone | weist sich aus (Software im OS) |
| Authenticator | Switch, Access Point | Türsteher – entscheidet nicht selbst |
| Authentication Server | RADIUS (Microsoft NPS, FreeRADIUS, Cisco ISE) | prüft und autorisiert |

**EAP-Methoden**
| Methode | Nachweis | Bewertung |
|---|---|---|
| **EAP-TLS** | Zertifikat auf Client **und** Server | sehr sicher, braucht PKI |
| PEAP (MSCHAPv2) | Benutzername/Passwort im TLS-Tunnel, Server-Zertifikat | verbreitet, Server-Zertifikat muss geprüft werden |
| EAP-TTLS | ähnlich PEAP | |

**Geräte ohne 802.1X** (Drucker, alte Kameras): **MAB** (MAC Authentication Bypass) – RADIUS prüft die MAC, schwächer.

### Dynamische VLAN-Zuweisung (GA2 Sommer 2023)

RADIUS sendet mit dem Accept Attribute (Tunnel-Type = VLAN, Tunnel-Private-Group-ID = 20) → der Switch legt den Port in VLAN 20. Mitarbeiter, Gäste und unbekannte Geräte landen so automatisch im passenden Segment, egal an welcher Dose.

### AAA

| | Bedeutung | Beispiel |
|---|---|---|
| **Authentication** | Identität prüfen | Zertifikat, AD-Passwort |
| **Authorization** | Rechte festlegen | VLAN, ACL, Zeitfenster |
| **Accounting** | protokollieren | Start/Stop, Dauer, Volumen |

**RADIUS vs. TACACS+:** RADIUS (UDP, verschlüsselt nur das Passwort) für Netzzugang von Benutzern; **TACACS+** (TCP 49, verschlüsselt das ganze Paket, trennt AAA, Befehls-Autorisierung) für die **Administration** von Netzwerkgeräten.

### Weitere Switch-Schutzfunktionen

| Funktion | Schützt vor |
|---|---|
| **DHCP-Snooping** | Rogue-DHCP-Server: Offers nur an trusted Ports |
| **Dynamic ARP Inspection** | ARP-Poisoning: ARP gegen Snooping-Tabelle prüfen |
| **IP Source Guard** | IP-Spoofing am Access-Port |
| **BPDU Guard** | fremde Switches/Schleifen an Endgeräte-Ports |
| **RA Guard** | gefälschte IPv6-Router-Advertisements |
| Ungenutzte Ports **shutdown** + Parkplatz-VLAN | spontanes Anstecken |
| Storm Control | Broadcast-Stürme |
| **SPAN/Mirror-Port** | Analyse: Kopie des Verkehrs für IDS/Wireshark |

## Prüfungsfrage

**Frage (angelehnt an GA2 Sommer 2022 / Sommer 2023):** In den Besprechungsräumen sind Netzwerkdosen frei zugänglich. Es soll verhindert werden, dass Besucher ihre Notebooks anschließen; Mitarbeiter sollen an jeder Dose automatisch in ihr Abteilungs-VLAN kommen. Beschreiben Sie ein geeignetes Verfahren und die beteiligten Komponenten.

**Musterantwort:** Geeignet ist **IEEE 802.1X** mit RADIUS. Das Notebook (Supplicant) muss sich beim Anstecken über EAPoL am Switch (Authenticator) ausweisen, z. B. mit einem Gerätezertifikat (EAP-TLS) oder seinem AD-Konto (PEAP). Der Switch leitet die Anmeldung an den RADIUS-Server weiter, der sie gegen das Active Directory prüft. Bei Erfolg sendet der Server ein Access-Accept mit der VLAN-ID der Abteilung, und der Switch schaltet den Port in diesem VLAN frei. Besuchergeräte ohne gültige Anmeldung erhalten keinen Zugang oder landen in einem Gäste-VLAN mit reinem Internetzugriff. Zusätzlich protokolliert RADIUS die Anmeldungen (Accounting).

## Falle

- „Der Switch prüft das Passwort“ → der Switch ist nur **Authenticator**, prüfen tut der **RADIUS-Server**.
- MAC-Filter/Port-Security als gleichwertig zu 802.1X darstellen → MACs sind fälschbar.
- RADIUS-Ports mit TCP angeben → **UDP 1812/1813** (alt 1645/1646).
- Bei PEAP das Server-Zertifikat nicht prüfen lassen → Angreifer mit eigenem AP kann Zugangsdaten abgreifen.
- Port-Security auf Trunk- oder Uplink-Ports → dort viele MACs, Port geht sofort in Shutdown.

## Merksatz

**„Supplicant fragt, Authenticator vermittelt, RADIUS entscheidet – und Sticky-MAC merkt sich, wer zuerst kam.“**

## Verwandt

- [Netzwerkzugangskontrolle: 802.1X & RADIUS](../Sicherheit/netzwerkzugang-802-1x-radius.md) – dasselbe Thema aus Sicherheits-Sicht
- [WLAN](wlan.md) – WPA2/WPA3-Enterprise nutzt 802.1X
- [VLAN](vlan.md) – dynamische VLAN-Zuweisung
- [ARP](../OSI-IP-mac-bit-byte/arp.md) – ARP-Poisoning und DAI
- [DHCP](../OSI-IP-mac-bit-byte/dhcp.md) – DHCP-Snooping
- [AAA & SIEM](../Sicherheit/aaa-siem.md) – RADIUS, TACACS+, Protokollierung
- [Active Directory](../Sicherheit/active-directory.md) – Benutzerquelle für RADIUS
- [PKI](../Sicherheit/Verschluesselung/pki.md) – Zertifikate für EAP-TLS
- [Switches – Grundlagen](switches-und-vlans.md) – Switch-Härtung
- [Lektion 16 – Firewall (Port-Security)](../../analyse-der-exams/L16_Firewall.md)
