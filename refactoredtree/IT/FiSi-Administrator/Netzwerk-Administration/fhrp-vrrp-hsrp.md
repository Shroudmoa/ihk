# FHRP – VRRP, HSRP, GLBP

## In 30 Sekunden

- Clients haben **ein** Default-Gateway. Fällt dieser Router aus, ist das Netz abgeschnitten → **Single Point of Failure**.
- **FHRP** (First Hop Redundancy Protocol) lässt mehrere Router eine **gemeinsame virtuelle IP- und MAC-Adresse** bereitstellen – diese virtuelle IP ist das Gateway der Clients.
- **Master/Active** beantwortet ARP für die virtuelle IP und leitet weiter; **Backup/Standby** überwacht per Hello-Paketen und **übernimmt bei Ausfall** – für die Clients transparent.
- Protokolle: **VRRP** (offener Standard, RFC 5798), **HSRP** (Cisco), **GLBP** (Cisco, mit Lastverteilung).
- GA2 Sommer 2026: VRRP-Funktionsweise erklären und als Gateway der Server die **virtuelle IP `10.0.0.99`** eintragen – nicht die physische Router-IP.

## Kernwissen

### Aufbau (Beispiel GA2 Sommer 2026)

```
                Provider 1 (LWL)       Provider 2 (DSL)
                     |                        |
              Router A 10.0.0.1         Router B 10.0.0.2
              (Master, Prio 200)        (Backup, Prio 100)
                     \                        /
                      ---- virtuelle IP 10.0.0.99/16 ----
                                   |
                     Server: Default-Gateway 10.0.0.99
```

### Funktionsweise VRRP

1. Router A und B bilden eine **VRRP-Gruppe** (VRID) mit gemeinsamer virtueller IP und virtueller MAC `00:00:5E:00:01:<VRID>`.
2. Der Router mit der **höchsten Priorität** wird **Master**, beantwortet ARP-Anfragen nach `10.0.0.99` mit der virtuellen MAC und leitet die Pakete weiter.
3. Der Master sendet periodisch (Standard **1 s**) **Advertisements** an Multicast `224.0.0.18`.
4. Bleiben sie aus (ca. 3 × Intervall), wird der **Backup zum Master**, übernimmt virtuelle IP und MAC und sendet ein **Gratuitous ARP**, damit Switches ihre MAC-Tabellen aktualisieren.
5. Clients behalten ihr Gateway `10.0.0.99` und ihren ARP-Eintrag – sie bemerken höchstens einen kurzen Aussetzer.
6. **Preemption:** Kehrt Router A zurück, kann er (wenn konfiguriert) die Master-Rolle zurückholen. **Tracking:** Fällt nur die WAN-Leitung von A aus, senkt A seine Priorität, damit B übernimmt.

### Vergleich

| | VRRP | HSRP | GLBP |
|---|---|---|---|
| Standard | offen (RFC 5798) | Cisco | Cisco |
| Rollen | Master / Backup | Active / Standby | AVG / mehrere AVF |
| Virtuelle MAC | `00:00:5E:00:01:xx` | `00:00:0C:07:AC:xx` (v1) | mehrere |
| Lastverteilung | nur über mehrere Gruppen | nur über mehrere Gruppen | **ja**, eingebaut |
| Multicast | 224.0.0.18 | 224.0.0.2 (v1) / 224.0.0.102 (v2) | 224.0.0.102 |
| Virtuelle IP = physische IP erlaubt | ja (dann „Owner“) | nein | nein |

### Abgrenzung

| Technik | Schützt vor | Schicht |
|---|---|---|
| FHRP | Ausfall des **Gateway-Routers** | 3 |
| STP | Schleifen bei redundanten **Switch**-Wegen | 2 |
| Link Aggregation | Ausfall einer **Leitung** zwischen zwei Geräten | 2 |
| Zwei Provider (Dual-WAN) | Ausfall des **Internetanschlusses** | WAN |
| Cluster / Load Balancer | Ausfall eines **Servers** | 4–7 |

**Verfügbarkeit zweier paralleler Anschlüsse:** A = 1 − (1 − A1) · (1 − A2), z. B. 99 % und 99 % → 1 − 0,01 · 0,01 = **99,99 %** (GA2 Winter 2025).

## Prüfungsfrage

**Frage (angelehnt an GA2 Sommer 2026):** Das Rechenzentrum ist über Router A (`10.0.0.1`, LWL) und Router B (`10.0.0.2`, DSL) angebunden; beide nutzen VRRP mit der virtuellen IP `10.0.0.99/16`. Erläutern Sie Aufgabe und Funktionsweise von VRRP und geben Sie an, welches Standard-Gateway die Server erhalten.

**Musterantwort:** VRRP stellt ein **ausfallsicheres Default-Gateway** bereit. Beide Router bilden eine VRRP-Gruppe mit der virtuellen IP `10.0.0.99` und einer virtuellen MAC. Der Router mit der höheren Priorität ist Master: Er beantwortet ARP-Anfragen für `10.0.0.99` und leitet den Verkehr weiter. Er sendet regelmäßig Advertisements; bleiben diese aus, übernimmt der Backup-Router die virtuelle IP und MAC und gibt dies per Gratuitous ARP bekannt. Für die Server ändert sich nichts. Als Standard-Gateway erhalten die Server daher **`10.0.0.99`** – nicht `10.0.0.1` oder `10.0.0.2`, weil diese Adressen beim Ausfall des jeweiligen Routers nicht mehr erreichbar wären.

## Falle

- Physische Router-IP als Gateway eintragen → bei Ausfall dieses Routers ist das Netz trotz Redundanz weg.
- FHRP mit Load Balancing gleichsetzen → VRRP/HSRP haben **einen** aktiven Router pro Gruppe.
- „Clients müssen bei Ausfall neu konfiguriert werden“ → genau das verhindert FHRP.
- FHRP schützt **nicht** vor dem Ausfall des Switches, an dem beide Router hängen → Switch-Redundanz zusätzlich planen.
- Ohne **Tracking** bleibt ein Router Master, obwohl seine WAN-Leitung tot ist.

## Merksatz

**„Ein Gateway für die Clients, zwei Router dahinter – die virtuelle IP wandert, der Client merkt’s nicht.“**

## Verwandt

- [Routing](routing.md) – Default-Route und Gateway
- [ARP](../OSI-IP-mac-bit-byte/arp.md) – Gratuitous ARP beim Rollenwechsel
- [STP](stp-spanning-tree.md) – Redundanz auf Schicht 2
- [Link Aggregation (LACP)](link-aggregation-lacp.md) – Leitungsredundanz
- [WAN-Anschlussarten](wan-anschlussarten.md) – zweiter Provider als Backup
- [Hochverfügbarkeit](../Server/ha-hochverfuegbarkeit.md) – Verfügbarkeitsklassen
- [Cluster](../Server/cluster.md) – Redundanz auf Serverebene
- [Lektion 15 – VLAN & Routing](../../analyse-der-exams/L15_VLAN_Routing.md)
- [Lektion 18 – Netzwerk-Berechnungen](../../analyse-der-exams/L18_Netzwerk_Berechnungen.md)
