# Cluster

## In 30 Sekunden

- Ein **Cluster** ist ein Verbund mehrerer Server (**Knoten**), die nach außen wie **ein** System wirken.
- Arten: **HA-/Failover-Cluster** (Verfügbarkeit), **Load-Balancing-Cluster** (Lastverteilung, Skalierung), **HPC-Cluster** (Rechenleistung).
- **Active/Passive:** Ein Knoten arbeitet, der zweite wartet und übernimmt bei Ausfall (Failover). **Active/Active:** Alle Knoten arbeiten und teilen die Last.
- Knoten überwachen sich per **Heartbeat**. **Quorum** (Mehrheitsentscheid) verhindert **Split-Brain**.
- Typisch: gemeinsamer Speicher (SAN, Cluster Shared Volume) und **N+1**-Reserve, damit ein Knoten die Last eines ausgefallenen mittragen kann.
- Geprüft: Cluster als HA-Maßnahme 2021W/2022W/2024S, Server-Cluster planen 2023W.

## Kernwissen

### Cluster-Arten

| Art | Ziel | Funktionsweise | Beispiel |
|---|---|---|---|
| **HA-/Failover-Cluster** | Verfügbarkeit | Dienst wechselt bei Ausfall automatisch auf einen anderen Knoten | Windows-Failovercluster für SQL Server, Hyper-V-/vSphere-HA |
| **Load-Balancing-Cluster** | Leistung, Skalierung | Load Balancer verteilt Anfragen auf gleichwertige Knoten | Webserver-Farm |
| **HPC-Cluster** | Rechenleistung | Aufgabe wird in Teile zerlegt und parallel berechnet | Simulation, KI-Training |
| **Storage-Cluster** | Kapazität, Verfügbarkeit | Daten verteilt und repliziert über Knoten | Ceph, vSAN, Storage Spaces Direct |

### Active/Passive vs. Active/Active

| | Active/Passive | Active/Active |
|---|---|---|
| Normalbetrieb | ein Knoten aktiv, einer im Standby | alle Knoten aktiv |
| Auslastung | Standby-Hardware bleibt ungenutzt | gute Ausnutzung |
| Failover | kurze Unterbrechung beim Umschalten (Dienst startet neu) | verbleibende Knoten übernehmen sofort |
| Dimensionierung | Standby so stark wie der aktive Knoten | jeder Knoten mit Reserve (z. B. max. 50 % Last bei 2 Knoten) |
| Einsatz | Datenbanken, Dateiserver | Webserver, Anwendungsserver |

### Bausteine eines HA-Clusters

| Baustein | Aufgabe |
|---|---|
| **Heartbeat** | Knoten senden sich laufend Lebenszeichen, über ein eigenes Netz oder eine Direktverbindung |
| **Quorum** | Nur die Mehrheit der Stimmen darf weiterarbeiten. Bei gerader Knotenzahl entscheidet ein Zeuge (Witness-Disk oder Cloud-Witness) |
| **Split-Brain** | Die Heartbeat-Verbindung bricht ab, beide Knoten halten sich für aktiv und schreiben gleichzeitig → Datenkorruption. Das Quorum verhindert es |
| **Shared Storage** | Alle Knoten sehen dieselben Daten (SAN/iSCSI, Cluster Shared Volume) |
| **virtuelle IP / Clustername** | Clients sprechen den Cluster an, nicht einen einzelnen Knoten |
| **Fencing / STONITH** | ein als fehlerhaft erkannter Knoten wird hart abgeschaltet, bevor ein anderer übernimmt |

### Planung (2023W: zwei Sachverhalte bei der Cluster-Planung)

- **Redundanz ohne SPOF:** zwei Switches, redundante Netzteile, Shared Storage selbst hochverfügbar (zwei Controller, Multipath).
- **Ressourcen nach N+1:** Fällt ein Knoten aus, müssen die übrigen seine VMs aufnehmen. Bei 3 Hosts sollte jeder höchstens ≈ 66 % ausgelastet sein.
- **Gleiche Konfiguration:** gleiche CPU-Generation (für Live-Migration), gleiche Netzwerk- und Storage-Anbindung, gleiche Patchstände.
- **Lizenzen** für alle Knoten (z. B. Windows Server Datacenter bei vielen VMs).
- **Standort:** Ein Cluster in einem Raum schützt nicht vor Brand. Dafür Stretched Cluster oder DR-Standort.

### Rechenweg: N+1-Kapazität

Es laufen 40 VMs mit insgesamt 960 GB RAM-Bedarf, ein Host hat 512 GB RAM.
- Ohne Reserve: 960 / 512 = 1,875 → 2 Hosts
- Mit N+1: Auch nach dem Ausfall eines Hosts müssen 960 GB Platz finden → 2 verbleibende Hosts nötig → **3 Hosts**

## Prüfungsfrage

**Frage:** Die Virtualisierungsumgebung soll als Server-Cluster betrieben werden. Erläutern Sie zwei Sachverhalte, die bei der Planung zu berücksichtigen sind.

**Musterantwort:** Erstens muss das Cluster ohne Single Point of Failure aufgebaut sein. Die Knoten brauchen gemeinsamen, selbst redundanten Speicher (SAN mit zwei Controllern und Multipath) sowie eine redundante Netzanbindung über zwei Switches, sonst hängt das ganze Cluster an einer Komponente. Zweitens müssen die Ressourcen nach dem N+1-Prinzip geplant werden: Fällt ein Knoten aus, müssen die übrigen genug CPU und RAM haben, um dessen VMs per Failover zu übernehmen. Die Knoten sollten zudem gleich ausgestattet sein (gleiche CPU-Generation), damit Live-Migration funktioniert.

## Falle

- Cluster und Backup gleichsetzen. Löscht jemand Daten oder verschlüsselt Ransomware sie, ist das auf allen Knoten sofort gleich.
- Zwei Knoten ohne Zeugen planen. Bei Verbindungsabbruch droht Split-Brain, oder beide stoppen.
- Active/Active-Knoten zu 90 % auslasten. Beim Ausfall eines Knotens reicht die Kapazität nicht.
- Load Balancer und HA-Cluster gleichsetzen. Ein Load Balancer verteilt, ein HA-Cluster übernimmt. Oft werden beide kombiniert.
- Shared Storage als einzelnes NAS ohne Redundanz. Das ist wieder ein SPOF.

## Merksatz

**„Heartbeat merkt den Ausfall, Quorum verhindert Streit, N+1 sorgt für Platz.“**

## Verwandt

- [HA-Hochverfügbarkeit](ha-hochverfuegbarkeit.md)
- [Load Balancer](load-balancer.md)
- [Verfügbarkeit & Kennzahlen](verfuegbarkeit-kennzahlen.md) – Parallelschaltung rechnen
- [Live-Migration-VMs](virtualisierung/live-migration-vms.md)
- [Crossover & Failover](../../nice-to-know/crossover-failover.md) – Heartbeat-Verbindung
- [DAS-NAS-SAN](../../Komponenten/Storage/das-nas-san.md) – Shared Storage
- [Skalierbarkeit-Elastizität](virtualisierung/skalierbarkeit-elastizitaet.md)
- [Lektion 03 Verfügbarkeit](../../analyse-der-exams/03_Verfuegbarkeit_Ausfallsicherheit.md)
