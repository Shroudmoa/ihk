# Link Aggregation (LACP)

## In 30 Sekunden

- **Link Aggregation** bündelt mehrere physische Ethernet-Leitungen zu **einem logischen Link** – Standard **IEEE 802.3ad** (heute 802.1AX), Steuerprotokoll **LACP**.
- Zwei Vorteile (GA2 Sommer 2025): **höhere Gesamtbandbreite** und **Redundanz** – fällt ein Link aus, läuft der Rest mit reduzierter Bandbreite weiter, ohne Umschaltpause.
- Alle Mitglieder brauchen **gleiche Geschwindigkeit, Duplex und VLAN-Konfiguration** und enden auf **denselben zwei Geräten** (oder einem Stack/MLAG).
- Verteilung per **Hash** (MAC/IP/Port) → **eine einzelne Verbindung nutzt nur einen Link**; nur viele parallele Flows profitieren von der vollen Bandbreite.
- Namen je Hersteller: **EtherChannel/Port-Channel** (Cisco), **Trunk** (HPE/Aruba), **Bonding** (Linux), **NIC-Teaming** (Windows).

## Kernwissen

### Betriebsarten

| Modus | Verhalten | Bewertung |
|---|---|---|
| LACP **active** | sendet aktiv LACP-Pakete | empfohlen (mind. eine Seite active) |
| LACP **passive** | antwortet nur | passive + passive = kein Bündel! |
| **statisch** („on“) | Bündel ohne Protokoll | keine Fehlererkennung bei Fehlverkabelung → Schleifengefahr |

LACP erkennt Fehlverkabelung und ausgefallene Links selbstständig und nimmt sie aus dem Bündel. Üblich sind bis zu **8 aktive Links** pro Bündel.

### Rechnen

| Bündel | Aggregierte Bandbreite | Bei Ausfall eines Links | Einzelner Download |
|---|---:|---:|---:|
| 2 × 1 Gbit/s | 2 Gbit/s | 1 Gbit/s | max. 1 Gbit/s |
| 4 × 1 Gbit/s | 4 Gbit/s | 3 Gbit/s | max. 1 Gbit/s |
| 2 × 10 Gbit/s | 20 Gbit/s | 10 Gbit/s | max. 10 Gbit/s |

### Einsatzorte

- **Switch ↔ Switch** (Uplink Etage → Core), oft als Trunk mit mehreren VLANs.
- **Server/Hypervisor ↔ Switch** (NIC-Teaming/Bonding) – idealerweise auf zwei Switches eines **Stacks/MLAG**, damit auch ein Switch-Ausfall abgefangen wird (GA1 Winter 2021/2022: „redundante Netzwerkanbindung“).
- **NAS/Storage ↔ Switch** für mehrere gleichzeitige Clients.

### Konfiguration (Cisco)

```
interface range gi1/0/23 - 24
 channel-group 1 mode active
interface port-channel 1
 switchport mode trunk
 switchport trunk allowed vlan 10,20,30
```

### Link Aggregation vs. STP

STP blockiert eine redundante Leitung, Link Aggregation nutzt **alle** gleichzeitig. Für STP ist das Bündel **ein** Link, es wird also nichts blockiert. Details: [STP](stp-spanning-tree.md).

## Prüfungsfrage

**Frage (angelehnt an GA2 Sommer 2025):** Zwischen den Switches MLS1 und MLS2 werden zwei 1-Gbit/s-Leitungen per Link Aggregation verbunden. Erläutern Sie den Begriff und nennen Sie zwei Vorteile. Ein Mitarbeiter wundert sich, dass seine einzelne Dateikopie trotzdem nur mit ca. 110 MB/s läuft. Erklären Sie das.

**Musterantwort:** Link Aggregation fasst mehrere physische Leitungen nach IEEE 802.3ad/LACP zu einer logischen Verbindung zusammen. Vorteile: (1) Die Bandbreite addiert sich auf bis zu 2 Gbit/s, (2) fällt eine Leitung oder ein Port aus, bleibt die Verbindung ohne Unterbrechung mit 1 Gbit/s bestehen (Redundanz). Die Verteilung auf die Links erfolgt per Hash über Adressen bzw. Ports – eine einzelne Verbindung wird immer über **denselben** Link geschickt, damit die Reihenfolge der Frames erhalten bleibt. Eine einzelne Kopie ist daher auf 1 Gbit/s ≈ 125 MB/s brutto (ca. 110 MB/s netto) begrenzt; erst mehrere gleichzeitige Verbindungen nutzen die 2 Gbit/s aus.

## Falle

- „2 × 1 Gbit/s = jede Übertragung mit 2 Gbit/s“ → nur in Summe über viele Flows.
- Links mit **unterschiedlicher Geschwindigkeit** oder unterschiedlichen VLANs bündeln → Bündel kommt nicht zustande.
- Beide Seiten **passive** → kein LACP-Bündel.
- Bündel über **zwei unabhängige** Switches ohne Stack/MLAG → funktioniert nicht.
- Link Aggregation mit STP verwechseln (STP blockiert, LACP bündelt).
- Den HPE-Begriff „Trunk“ (= Bündel) mit dem 802.1Q-Trunk (= VLAN-Tagging) verwechseln.

## Merksatz

**„Bündeln statt blockieren – mehr Summe, nicht mehr Einzeltempo.“**

## Verwandt

- [STP](stp-spanning-tree.md) – blockieren statt bündeln
- [Switches – Grundlagen](switches-und-vlans.md) – Stacking, Uplinks
- [VLAN](vlan.md) – Bündel als 802.1Q-Trunk
- [Verkabelung](verkabelung-kupfer-lwl.md) – SFP/SFP+-Uplinks
- [Hochverfügbarkeit](../Server/ha-hochverfuegbarkeit.md) – redundante Anbindung von Servern
- [Lektion 15 – VLAN & Routing](../../analyse-der-exams/L15_VLAN_Routing.md)
