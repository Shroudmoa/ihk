# Lektion 15 – VLANs & Routing zwischen VLANs

**Bereich:** Netzwerktechnik (Layer 2 / Layer 3)
**Relevanz:** Hoch – wiederkehrendes Prüfungsthema, oft mit Konfigurations-/Fehlersuche-Szenarien
**Lernzeit:** ca. 2h

> ✅ **Verifiziert anhand der echten AP2-Prüfungen 2021–2026:** VLAN ist **kein Randthema**, sondern kam in **6 von 10 Jahrgängen** vor: 2022S, 2023S, 2023W, 2024W, 2025S, 2025W. Alle Beispiele, Begriffe und Konfigurationsausschnitte unten stammen aus diesen realen Prüfungen (Quellenangabe in eckigen Klammern).

## 1. Warum das wichtig ist
- Netze wachsen: eine einzige Broadcast-Domäne wird langsam, unsicher und unübersichtlich.
- VLANs trennen logisch (nicht physisch) – spart Hardware und erhöht Sicherheit.
- Ohne korrektes Routing zwischen VLANs können Segmente nicht (oder nicht gewollt) kommunizieren.
- Prüfung fragt gerne: „Wie verbinde ich zwei VLANs?" und „Warum kommt kein Ping?"

## 2. Grundlagen

### VLAN-Zweck (mehrfach als Prüfungsfrage: "Nennen Sie Gründe für VLANs" [2022S, 2023S, 2023W])
| Ziel | Beispiel |
|------|----------|
| Broadcast-Domains trennen | Gast-WLAN vs. Produktiv |
| Sicherheit / Segmentierung | VoIP, Management, Buchhaltung getrennt |
| Flexibilität | Ports logisch umgruppierbar statt umzupatchen |

### VLAN-Zuordnung: statisch (portbasiert) vs. dynamisch
| Typ | Funktionsweise | Quelle |
|-----|-----------------|--------|
| **Portbasiert / statisch** | Port wird fest einem VLAN zugewiesen (`switchport access vlan X`) – Gerät bekommt VLAN allein durch den Port, an dem es hängt | [2023S, 2024W] |
| **802.1X-dynamisch** | VLAN-Zuweisung erfolgt erst nach erfolgreicher Authentifizierung (z. B. per RADIUS-Server), unabhängig vom physischen Port – Gerät "nimmt sein VLAN mit" | [2023S] |

### Port-Typen & 802.1Q-Tag
| Typ | Eigenschaft | Tagging |
|-----|-------------|---------|
| **Access-Port** | genau ein VLAN | untagged (eine VLAN-ID) |
| **Trunk-Port** | mehrere VLANs gleichzeitig, Uplink-Konfiguration oft explizit als **"tagged"** bezeichnet [2023W] | 802.1Q-Tag mit VLAN-ID, Ports die getaggten Traffic führen werden auf dem Switch entsprechend **markiert/gekennzeichnet** [2024W] |

**802.1Q-Tag im Detail [2023S]:** Das VLAN-Tag ist **32 Bit** groß und wird zwischen Quell-MAC und Ethertype in den Frame eingefügt (TPID 16 Bit + PCP 3 Bit + DEI/CFI 1 Bit + **VLAN-ID 12 Bit**). Dadurch ändert sich der Frame-Aufbau auf **OSI-Schicht 2** – die MAC-Adressen selbst bleiben unverändert, aber der Frame wird um das Tag-Feld größer [2023W].

**Natives VLAN [2024W]:**
- Das native VLAN ist das VLAN, dessen Frames auf einem Trunk-Port **untagged** übertragen werden (Default meist VLAN 1).
- **Grund, warum das native VLAN nicht das Default-VLAN bleiben sollte:** Ein Angreifer kann durch **VLAN-Hopping** (z. B. Double-Tagging-Angriff) aus dem nativen/Default-VLAN in andere VLANs gelangen, wenn native VLAN = Default-VLAN ist. Deshalb wird empfohlen, ein ungenutztes VLAN als natives VLAN festzulegen und es vom Default-VLAN zu trennen.

### Routing zwischen VLANs
- **Router-on-a-Stick / One-armed-Router** [explizit als Begriff in 2025W]: ein physischer Router-Port, mehrere Subinterfaces (z. B. `g0/0.10`, `g0/0.20` bei Cisco oder `eth1.11`, `eth1.12` bei Linux-Notation [2025S]), jeweils `encapsulation dot1Q <VLAN-ID>`.
- **Layer-3-Switch:** Switch Virtual Interface (SVI), z. B. `interface vlan 10`, dient als Gateway für das VLAN.
- **VLAN-IDs sind mit Subnetzen verknüpft** [2025S]: jedes VLAN bekommt sein eigenes IP-Subnetz, das Subinterface trägt die Gateway-IP dieses Subnetzes.

### STP vs. Link Aggregation im Vergleich [2023S – explizite Vergleichstabelle in der Prüfung gefordert]
| Merkmal | STP (Spanning Tree) | Link Aggregation (LACP) |
|---------|----------------------|--------------------------|
| Prinzip | Redundante Leitung wird **blockiert**, bis die aktive ausfällt | Mehrere Leitungen werden **gebündelt**, alle aktiv gleichzeitig |
| Normalbetrieb | **Eine Leitung bleibt ungenutzt** (Blocking-State) | Volle Bündelbandbreite nutzbar |
| Bei Ausfall einer Leitung | Blockierter Port wird aktiv (Konvergenzzeit nötig) | Bündel läuft mit **reduzierter Datenrate** weiter, kein Umschalten nötig |
| Root Bridge | niedrigste Bridge-Priority / MAC | – |

### Link Aggregation (LACP / 802.3ad)
- Bündelt physische Links (z. B. 2× 1 Gbit → 2 Gbit Nutzlast + Redundanz).
- Konfiguration als **EtherChannel** (`channel-group ... mode active`).
- Prüfungsbezug [2025S]: Vorteile sind höhere **Verfügbarkeit** (Redundanz bei Kabel-/Portausfall) **und** höherer **Durchsatz** (Bündelung der Bandbreite).
- **Spanning Tree als Grund für geblockten Port** [2025S]: Wenn ein Port im STP-Blocking-State ist, liegt das meist an einer redundanten (Schleifen-)Verbindung, die STP bewusst deaktiviert hat.

### Glasfaser-Transceiver: SFP vs. SFP+ [2023S]
| Modul | Geschwindigkeit | Einsatz |
|-------|------------------|---------|
| **SFP** | 1 Gbit/s | Standard-Uplink/Trunk zwischen Switches |
| **SFP+** | 10 Gbit/s | Core-/Backbone-Uplinks mit höherem Bandbreitenbedarf |

### Eigenes VLAN für VoIP [2025W]
Ein separates VLAN für Telefonie (VoIP) wird typischerweise begründet mit:
- **QoS:** Sprachpakete können priorisiert werden, ohne mit Daten-Traffic zu konkurrieren.
- **Sicherheit:** Trennung von Telefonanlage/Endgeräten vom übrigen Datennetz reduziert Angriffsfläche.

### FHRP (First Hop Redundancy Protocol) – VRRP-Beispiel [2026S]
- **VRRP / HSRP:** virtuelles Gateway (eine virtuelle IP), mehrere physische Router.
- Ausfall eines Routers bleibt für Clients transparent (Default-Gateway bleibt erreichbar).
- **Konkretes Prüfungsbeispiel [2026S]:** virtuelle Gateway-IP `10.0.0.99/16`. Funktionsweise: Der **aktive (Master-)Router** beantwortet ARP-Anfragen für diese virtuelle IP mit seiner virtuellen MAC-Adresse. Der **Standby-Router** überwacht den Master per VRRP-Hello-Pakete und **übernimmt die virtuelle IP**, sobald der Master ausfällt – die Clients bemerken davon nichts, da sich ihr Gateway (`10.0.0.99`) nicht ändert.

## 3. Konfigurationsbeispiel (Router-on-a-Stick)

```
Switch:
 vlan 10
  name BUERO
 vlan 20
  name GAST
 interface gi0/1
  switchport mode access
  switchport access vlan 10
 interface gi0/2
  switchport mode trunk
  switchport trunk allowed vlan 10,20

Router:
 interface g0/0.10
  encapsulation dot1Q 10
  ip address 192.168.10.1 255.255.255.0
 interface g0/0.20
  encapsulation dot1Q 20
  ip address 192.168.20.1 255.255.255.0
```

## 4. Typische Fehlerquellen
- Trunk erlaubt VLAN nicht (`allowed vlan`) → Traffic bleibt stecken.
- Subinterface-VLAN-ID im Router stimmt nicht mit Switch überein.
- Native VLAN mismatch (Security-Risiko + Filterung).
- STP blockiert den einzigen Pfad (Schleife vermutet).
- **Client und Router-Interface in unterschiedlichen VLANs** → kein Routing möglich, obwohl beide "im Netz" sind [2023W – reale Fehlerdiagnose-Aufgabe].
- **DHCP-Server in anderem VLAN als Client:** DHCP-Discover-Broadcast überschreitet keine VLAN-Grenze → **DHCP-Relay (ip helper-address)** auf dem SVI/Subinterface nötig [2023S].

## 4b. Übungsaufgabe: Cisco-IOS-Konfigurationsfehler finden [angelehnt an 2025W]

Gegeben ist folgender Konfigurationsausschnitt eines Router-on-a-Stick für VLAN 20 (`192.168.20.0/24`):

```
interface g0/0.20
 encapsulation dot1Q 20
 ip address 192.168.30.1 255.255.255.0
```

Finde den Fehler.

<details>
<summary>Lösung (aufklappen)</summary>

Die konfigurierte Gateway-IP `192.168.30.1` liegt **außerhalb des Subnetzes** von VLAN 20 (`192.168.20.0/24`). Richtig wäre z. B. `192.168.20.1 255.255.255.0`. Mit der falschen IP kann kein Client aus VLAN 20 den Router als Gateway nutzen, da die IP-Adresse nicht im selben Subnetz liegt.

</details>

## 5. Übungsaufgabe (im Prüfungsstil, selbst erstellt)

Ein Client im VLAN 10 (`192.168.10.50/24`, Gateway `192.168.10.1`) kann Rechner im VLAN 20 (`192.168.20.0/24`) nicht pingen. Der Ping an das eigene Gateway funktioniert. Nenne drei mögliche Ursachen und jeweils die Prüfung.

<details>
<summary>Lösung (aufklappen)</summary>

1. **Kein Routing aktiv / falsches Gateway:** Gateway-IP auf Client prüfen (`ipconfig`); Router-Interface `g0/0.10` up? `show ip interface brief`.
2. **Subinterface-VLAN-Mapping falsch:** Am Router `encapsulation dot1Q 20` statt 10? `show run interface g0/0.20`.
3. **Trunk lässt VLAN 20 nicht durch:** `switchport trunk allowed vlan` prüfen – VLAN 20 muss gelistet sein. Ziel-Host evtl. am Access-Port falschen VLAN zugeordnet.
4. (Zusatz) **Ziel-Host Firewall:** ICMP auf Ziel-Rechner blockiert → trotz Routing kein Ping.

</details>

## Selbstcheck – kannst du das?
- [ ] Ich kann erklären, warum VLANs Broadcast-Domänen trennen.
- [ ] Ich unterscheide Access- und Trunk-Port (inkl. 802.1Q, 32-Bit-Tag mit 12-Bit-VLAN-ID).
- [ ] Ich unterscheide statische (portbasierte) und 802.1X-dynamische VLAN-Zuordnung.
- [ ] Ich erkläre, warum das native VLAN nicht dem Default-VLAN entsprechen sollte (VLAN-Hopping).
- [ ] Ich beschreibe Router-on-a-Stick/One-armed-Router und SVI.
- [ ] Ich vergleiche STP und Link Aggregation (Bandbreitennutzung im Normalbetrieb & bei Ausfall).
- [ ] Ich kenne SFP (1 Gbit) vs. SFP+ (10 Gbit).
- [ ] Ich erkläre VRRP anhand einer virtuellen Gateway-IP (z. B. 10.0.0.99/16) inkl. ARP-Verhalten.
- [ ] Ich begründe ein eigenes VLAN für VoIP (QoS/Sicherheit).

## Weiterführende Ressourcen
- Cisco Networking Academy: VLANs & Inter-VLAN Routing
- IHK-Prüfungsaufgaben AP2 (GA1/GA2) zu Switching – reale Jahrgänge 2022S, 2023S, 2023W, 2024W, 2025S, 2025W, 2026S (FHRP)
- RFC 826 (ARP), IEEE 802.1Q, IEEE 802.1D (STP)
