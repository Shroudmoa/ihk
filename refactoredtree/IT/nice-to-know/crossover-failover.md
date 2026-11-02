# Crossover und Failover

## In 30 Sekunden

- **Crossover-Kabel** = Twisted-Pair-Kabel mit **gekreuzten Sende- und Empfangsadern**; früher nötig, um **gleichartige Geräte direkt** zu verbinden (PC ↔ PC, Switch ↔ Switch). Heute erledigt **Auto-MDI-X** das automatisch.
- In Clustern wird eine solche **direkte Verbindung** oft als **Heartbeat-Link** zwischen zwei Knoten bzw. Controllern genutzt – darüber erkennen sie den Ausfall des Partners.
- **Failover** = der **automatische Umschaltvorgang** auf ein redundantes System, wenn das primäre ausfällt (Ziel: **Verfügbarkeit**).
- Merke die Reihenfolge: **Heartbeat erkennt → Failover schaltet um → (Failback** zurück, wenn das Primärsystem wieder da ist).
- Gefahr bei getrenntem Heartbeat: **Split-Brain** – beide Knoten halten sich für aktiv; Schutz durch **Quorum/Witness** und **Fencing**.

## Kernwissen

### Crossover vs. Patchkabel

| Kabel | Belegung | Verwendung früher | heute |
|---|---|---|---|
| Patchkabel (1:1, „straight“) | beide Enden gleich (T568B–T568B) | PC ↔ Switch | Standard für alles |
| **Crossover** | ein Ende T568A, anderes T568B (Paare 2 und 3 getauscht) | PC ↔ PC, Switch ↔ Switch, Server ↔ Server | dank **Auto-MDI-X** kaum noch nötig – Gigabit-Ports beherrschen das praktisch immer |

### Heartbeat im HA-Cluster

- Zwei Server oder zwei SAN-Controller sind **direkt** (oft per Crossover bzw. dediziertem Link) oder über ein eigenes Netz verbunden.
- Sie senden sich fortlaufend „Ich lebe noch“-Signale (**Heartbeat**).
- Bleibt der Heartbeat aus, startet der Partner das **Failover** und übernimmt Dienste, virtuelle IP und ggf. Speicherzugriff.
- Der Heartbeat-Link sollte **redundant** sein (zweiter Pfad), sonst droht Split-Brain bei einem einfachen Kabeldefekt.

### Failover-Beispiele

| Bereich | Failover-Mechanismus |
|---|---|
| RAID | Hot-Spare springt für ausgefallene Platte ein, Rebuild startet |
| SAN | zweiter Controller / zweiter Pfad (**Multipath**) übernimmt |
| Server-Cluster | Dienst/VM startet auf dem zweiten Knoten (Active/Passive) |
| Netzwerk-Gateway | **VRRP/HSRP**: Backup-Router übernimmt die virtuelle Gateway-IP |
| Internetanschluss | Dual-WAN: Router schaltet auf die Backup-Leitung (LTE/DSL) |
| Firewall | HA-Paar mit Zustandssynchronisation – Verbindungen bleiben erhalten |

| Begriff | Bedeutung |
|---|---|
| **Failback** | Rückschalten auf das Primärsystem nach der Reparatur |
| **Active/Passive** | ein System arbeitet, das andere wartet |
| **Active/Active** | beide arbeiten, Last verteilt; bei Ausfall übernimmt der Rest |
| **Split-Brain** | beide Knoten aktiv, weil die Verbindung getrennt ist → Datenkorruption |
| **Quorum / Witness** | dritte Instanz entscheidet, welcher Knoten aktiv bleiben darf |

## Prüfungsfrage

**Frage:** Zwei Datenbankserver sollen als Active/Passive-Cluster betrieben werden. Erläutern Sie die Begriffe Heartbeat und Failover in diesem Zusammenhang und nennen Sie ein Risiko, wenn nur ein einzelnes Heartbeat-Kabel verwendet wird.

**Musterantwort:** Über den **Heartbeat** senden sich beide Server ständig Lebenszeichen, z. B. über eine direkte Verbindung. Bleibt das Signal des aktiven Servers aus, erkennt der passive Server den Ausfall und führt das **Failover** durch: Er übernimmt die Datenbankdienste und die Cluster-IP-Adresse, sodass die Anwendungen weiterarbeiten können. Wird nur ein einzelnes Heartbeat-Kabel genutzt, kann ein Kabeldefekt dazu führen, dass beide Server den jeweils anderen für ausgefallen halten und gleichzeitig aktiv werden (**Split-Brain**) – mit der Gefahr inkonsistenter Daten. Abhilfe: redundanter Heartbeat und ein Quorum/Witness.

## Falle

- Crossover und Failover als Synonyme behandeln → Crossover ist eine **Verbindung (Kabel)**, Failover ein **Vorgang**.
- „Ohne Crossover-Kabel können Switches nicht verbunden werden“ → heute erledigt **Auto-MDI-X** das.
- Failover als Backup bezeichnen → Failover sichert **Verfügbarkeit**, nicht verlorene oder gelöschte Daten.
- Split-Brain übersehen, wenn nach Risiken eines Clusters gefragt wird.

## Merksatz

**„Crossover ist das Kabel, Heartbeat das Lebenszeichen, Failover die Übernahme.“**

## Verwandt

- [Verkabelung](../FiSi-Administrator/Netzwerk-Administration/verkabelung-kupfer-lwl.md) – Patch- und Crossover-Kabel, Auto-MDI-X
- [FHRP / VRRP](../FiSi-Administrator/Netzwerk-Administration/fhrp-vrrp-hsrp.md) – Failover des Gateways
- [Cluster](../FiSi-Administrator/Server/cluster.md) – Active/Passive, Quorum
- [Hochverfügbarkeit](../FiSi-Administrator/Server/ha-hochverfuegbarkeit.md) – Verfügbarkeitsklassen
- [RAID](../FiSi-Administrator/Server/raid.md) – Hot-Spare als Failover
- [DAS, NAS, SAN](../Komponenten/Storage/das-nas-san.md) – Controller- und Pfad-Redundanz
