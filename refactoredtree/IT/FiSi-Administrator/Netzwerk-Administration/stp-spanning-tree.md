# STP (Spanning Tree Protocol)

## In 30 Sekunden

- Redundante Verbindungen zwischen Switches erzeugen **Layer-2-Schleifen** → **Broadcast-Sturm**, instabile MAC-Tabellen, doppelte Frames; Ethernet-Frames haben **keine TTL**, kreisen also endlos.
- STP (IEEE **802.1D**, schneller: **RSTP 802.1w**) berechnet einen schleifenfreien Baum und **blockiert redundante Ports logisch**.
- Fällt der aktive Weg aus, schaltet STP den blockierten Port frei – **Konvergenzzeit**: klassisch 30–50 s, RSTP wenige Sekunden.
- Wurzel des Baums ist die **Root Bridge** = Switch mit der **niedrigsten Bridge-ID** (Priorität, dann MAC).
- GA2-Bezug: **„Warum ist dieser Port geblockt?“** (2025S) und **STP vs. Link Aggregation** (2023S).

## Kernwissen

### Warum Schleifen tödlich sind

1. Ein Broadcast (z. B. ARP) wird von jedem Switch an alle Ports geflutet.
2. Über die Schleife kommt er zurück und wird erneut geflutet – in beide Richtungen.
3. Nach Sekunden ist die Leitung mit Kopien voll (**Broadcast-Sturm**), CPU-Last steigt, MAC-Tabellen springen ständig um (**MAC-Flapping**) → Netz steht.

### Ablauf

| Schritt | Was passiert |
|---:|---|
| 1 | Switches tauschen **BPDUs** (Bridge Protocol Data Units) aus |
| 2 | **Root-Bridge-Wahl:** niedrigste Bridge-ID = Priorität (Standard 32768, in 4096er-Schritten) + MAC |
| 3 | Jeder andere Switch wählt seinen **Root Port** (günstigster Weg zur Root Bridge) |
| 4 | Pro Segment wird ein **Designated Port** bestimmt (weiterleitend) |
| 5 | Alle übrigen Ports werden **blockiert** (Alternate/Backup) |

**Pfadkosten** (802.1D-1998): 10 Mbit/s = 100, 100 Mbit/s = 19, 1 Gbit/s = 4, 10 Gbit/s = 2 → schnellere Leitungen werden bevorzugt.

**Praxis:** Root Bridge **bewusst festlegen** (niedrige Priorität am Core-Switch), sonst wird zufällig der Switch mit der ältesten (niedrigsten) MAC Root – ggf. ein Etagen-Switch.

### Portzustände

| STP (802.1D) | RSTP (802.1w) | Leitet Daten weiter? | Lernt MACs? |
|---|---|---|---|
| Blocking | Discarding | nein | nein |
| Listening | Discarding | nein | nein |
| Learning | Learning | nein | ja |
| Forwarding | Forwarding | **ja** | ja |
| Disabled | Discarding | nein | nein |

Klassisch dauert der Wechsel Blocking → Forwarding bis zu **50 s** (Max Age 20 s + 2 × Forward Delay 15 s), bei RSTP meist **unter 1–2 s**.

### Varianten und Schutzfunktionen

| Begriff | Bedeutung |
|---|---|
| **RSTP** (802.1w) | schnelle Konvergenz, heute Standard |
| **MSTP** (802.1s) | mehrere Spanning-Tree-Instanzen für VLAN-Gruppen → Lastverteilung |
| **PVST+ / Rapid-PVST+** | Cisco: ein Baum pro VLAN |
| **PortFast / Edge Port** | Access-Port geht sofort in Forwarding (Endgerät, keine Schleife möglich) |
| **BPDU Guard** | Port wird abgeschaltet, wenn an einem Edge-Port BPDUs ankommen (jemand steckt einen Switch an) |
| **Root Guard** | verhindert, dass ein fremder Switch Root Bridge wird |

### STP vs. Link Aggregation (GA2 Sommer 2023)

| Merkmal | STP | Link Aggregation (LACP) |
|---|---|---|
| Normalbetrieb | redundante Leitung **blockiert, ungenutzt** | **alle Leitungen aktiv**, Bandbreite gebündelt |
| Leitungsausfall | blockierter Port wird nach Konvergenzzeit aktiv | Verkehr läuft sofort über die übrigen Links, **reduzierte Bandbreite** |
| Topologie | beliebige Wege, auch über mehrere Switches | parallele Leitungen zwischen **denselben zwei** Geräten (bzw. Stack/MLAG) |
| Ziel | Schleifenfreiheit | Bandbreite + Redundanz |

Beide ergänzen sich: Ein LACP-Bündel zählt für STP als **ein** logischer Link.

## Prüfungsfrage

**Frage (angelehnt an GA2 Sommer 2025):** Die vier Multilayer-Switches MLS1–MLS4 sind ringförmig verkabelt. Ein Kollege bemerkt, dass der Port von MLS1 zu MLS2 als „blocking“ angezeigt wird, obwohl Kabel und Port in Ordnung sind. Erklären Sie den Grund und was beim Ausfall der Leitung MLS1–MLS3 passiert.

**Musterantwort:** Durch die ringförmige Verkabelung entsteht eine Layer-2-Schleife. Das Spanning Tree Protocol erkennt die redundante Verbindung über den BPDU-Austausch und versetzt einen Port – hier MLS1 → MLS2 – bewusst in den Blocking-Zustand, um Broadcast-Stürme und doppelte Frames zu verhindern. Fällt die aktive Leitung MLS1–MLS3 aus, bleiben die BPDUs auf diesem Weg aus; STP berechnet den Baum neu und schaltet den blockierten Port nach der Konvergenzzeit (RSTP: wenige Sekunden, klassisches STP: bis 50 s) in den Forwarding-Zustand. Die Verbindung bleibt damit über den Ersatzweg erhalten.

## Falle

- „STP verteilt Last auf beide Leitungen“ → nein, die redundante Leitung ist **ungenutzt** (außer bei MSTP/PVST je VLAN).
- Blockierten Port für einen Defekt halten → er ist **gewollt** blockiert.
- „Root Bridge = Switch mit höchster Priorität“ → **niedrigste** Bridge-ID gewinnt.
- STP auf Schicht 3 einordnen → STP ist ein **Layer-2**-Protokoll.
- STP komplett abschalten, „weil es stört“ → beim nächsten Fehlstecken steht das Netz.

## Merksatz

**„Schleife erkannt, Port verbannt – fällt der Weg, kommt er zurück.“**

## Verwandt

- [Link Aggregation (LACP)](link-aggregation-lacp.md) – die Alternative, die alle Leitungen nutzt
- [Switches – Grundlagen](switches-und-vlans.md) – Broadcast-Domänen, BPDU Guard
- [VLAN](vlan.md) – STP pro VLAN (PVST, MSTP)
- [Netzwerkanalyse & Troubleshooting](netzwerkanalyse-und-troubleshooting.md) – Broadcast-Sturm erkennen
- [Hochverfügbarkeit](../Server/ha-hochverfuegbarkeit.md) – Redundanz als Prinzip
- [Lektion 15 – VLAN & Routing](../../analyse-der-exams/L15_VLAN_Routing.md)
