# HA – Hochverfügbarkeit (High Availability)

## In 30 Sekunden

- **Hochverfügbarkeit** heißt: Der Dienst läuft weiter, auch wenn einzelne Komponenten ausfallen. Prinzip: **Redundanz ohne Single Point of Failure (SPOF)**.
- Gemessen wird in Prozent Verfügbarkeit (99,9 % ≈ 8,76 h Ausfall/Jahr), vertraglich festgehalten im **SLA**.
- Maßnahmen: redundante Netzteile, ECC-RAM, RAID, NIC-Teaming, USV, Cluster/Failover, Load Balancer, Geo-Redundanz.
- **HA ≠ Backup ≠ Disaster Recovery:** HA überbrückt Komponentenausfälle, Backup schützt vor Datenverlust, DR stellt nach einer Katastrophe wieder her.
- Geprüft: Maßnahmen für hohe Ausfallsicherheit 2021W, 2022W, 2024S. Verfügbarkeit 2025W, 2026S.

## Kernwissen

### Definition

**Hochverfügbarkeit (HA)** bedeutet, dass IT-Systeme trotz Ausfall einzelner Komponenten mit möglichst geringer Unterbrechung weiterhin nutzbar sind. Ziel ist eine hohe **Uptime**.

### Verfügbarkeitsklassen

| Klasse | Verfügbarkeit p.a. | Ausfallzeit p.a. | Einsatz |
|---|---|---|---|
| 99 % (Two Nines) | 99,00 % | ~87,6 h | Unkritische Systeme |
| 99,9 % (Three Nines) | 99,90 % | ~8,76 h | Standardgeschäftsanwendungen |
| 99,99 % (Four Nines) | 99,99 % | ~52,56 min | Kritische Anwendungen |
| 99,999 % (Five Nines) | 99,999 % | ~5,26 min | Hochkritische Systeme (z. B. Banken, Notruf) |

### RTO & RPO

| Begriff | Bedeutung | Fragestellung |
|---|---|---|
| **RTO (Recovery Time Objective)** | Maximal akzeptable **Ausfallzeit** bis zur Wiederherstellung des Dienstes | *Wie lange darf der Dienst ausfallen?* |
| **RPO (Recovery Point Objective)** | Maximal akzeptabler **Datenverlust** in Zeit gemessen | *Wie alt darf das letzte Backup maximal sein?* |

**Beispiel:** RPO = 15 Minuten → Max. 15 Minuten Datenverlust. RTO = 2 Stunden → Dienst muss innerhalb von 2 Stunden wieder laufen.

### USV

Online-USV (VFI) ohne Umschaltzeit für Server, Offline (VFD) und Line-Interactive (VI) für weniger kritische Geräte. Details und Berechnung: [USV](usv.md).

### Maßnahmen zur Hochverfügbarkeit

| Maßnahme | Erklärung |
|---|---|
| **Redundanz** | Doppelte Komponenten (Netzteile, Netzwerkkarten, Festplatten) |
| **RAID** | Redundanz auf Datenträgerebene (ersetzt **kein** Backup) |
| **Failover-Cluster** | Automatischer Übergang auf Ersatzsystem bei Ausfall |
| **Load Balancer** | Lastverteilung auf mehrere Server + Ausfallsicherheit |
| **USV** | Überbrückung von Stromausfällen, kontrolliertes Herunterfahren |
| **Geo-Redundanz** | Zweiter Standort zur Absicherung gegen Standortausfall |
| **Hot Spare** | Reservekomponente zur schnellen Übernahme |

### HA vs. Disaster Recovery (DR)

| Kriterium | **HA (High Availability)** | **DR (Disaster Recovery)** |
|---|---|---|
| **Ziel** | Ausfälle möglichst vermeiden/überbrücken | Wiederherstellung nach Großschaden |
| **Fokus** | **Verfügbarkeit** | **Wiederherstellbarkeit** |
| **Auslöser** | Komponentenausfall | Katastrophe (Brand, Hochwasser, Ransomware) |
| **Zeitrahmen** | Sekunden – Minuten | Stunden – Tage |
| **Maßnahmen** | Cluster, Redundanz, Failover, Load Balancing | Backup/Restore, Notfallstandort, Notfallhandbuch |

### Wichtige Begriffe

- **SLA (Service Level Agreement):** Vertraglich vereinbarte Verfügbarkeitsziele
- **SPOF (Single Point of Failure):** Einziger kritischer Punkt, dessen Ausfall das Gesamtsystem lahmlegt. **Muss vermieden werden**
- **MTBF (Mean Time Between Failures):** Mittlere Zeit zwischen Ausfällen
- **MTTR (Mean Time To Repair/Restore):** Mittlere Reparatur-/Wiederherstellungszeit
- **Verfügbarkeit = MTBF / (MTBF + MTTR)**, Reihen- und Parallelschaltung: [Verfügbarkeit & Kennzahlen](verfuegbarkeit-kennzahlen.md)

## Prüfungsfrage

**Frage:** Nennen und erläutern Sie drei technische Maßnahmen, mit denen ein Datenbankserver hochverfügbar betrieben werden kann. RAID ist bereits vorhanden.

**Musterantwort:** Erstens ein Failover-Cluster aus zwei Knoten mit gemeinsamem Speicher: Fällt der aktive Server aus, startet der zweite den Datenbankdienst automatisch. Zweitens redundante Hot-Swap-Netzteile an zwei getrennten Stromkreisen, abgesichert durch eine Online-USV, sodass weder ein Netzteildefekt noch ein Stromausfall zum Stillstand führt. Drittens eine redundante Netzwerkanbindung per NIC-Teaming an zwei Switches, damit der Ausfall einer Karte, eines Kabels oder Switches die Erreichbarkeit nicht unterbricht.

## Falle

- RAID oder Cluster als Backup bezeichnen. Gelöschte oder verschlüsselte Daten sind sofort auf allen Platten und Knoten weg.
- Redundanz einbauen, aber einen SPOF übersehen: ein Switch, eine USV, ein Storage-Controller, ein Raum.
- RTO und RPO vertauschen. RTO = Zeit bis der Dienst wieder läuft, RPO = maximal tolerierbarer Datenverlust.
- Eine Verfügbarkeitszahl ohne Bezugszeitraum angeben. 99,9 % pro Monat ist etwas anderes als pro Jahr.

## Merksatz

**„Alles doppelt, nichts einzeln. HA hält den Betrieb am Laufen, Backup und DR holen die Daten zurück.“**

## Verwandt

- [RAID](raid.md) – Datenträgerredundanz
- [Snapshots-Backups](virtualisierung/snapshots-backups.md) – Snapshots bei VMs
- [Cluster](cluster.md) – Servercluster
- [Loadbalancer](load-balancer.md) – Lastverteilung
- [Schutzziele](../Sicherheit/schutzziele.md) – Verfügbarkeit (CIA)
- [Skalierbarkeit-Elastizität](virtualisierung/skalierbarkeit-elastizitaet.md) – Skalierungskonzepte
- [Verfügbarkeit & Kennzahlen](verfuegbarkeit-kennzahlen.md) – MTBF, MTTR, Reihe/Parallel
- [USV](usv.md) – VFD, VI, VFI
- [Backup-Strategien](backup/backup-strategien.md) – Datensicherung
- [Disaster Recovery & Notfallkonzept](backup/disaster-recovery-notfallkonzept.md)
- [SLA](Cloud/sla.md)
- [Server-Hardware](../../Komponenten/server-hardware.md) – redundante Komponenten
- [Lektion 03 Verfügbarkeit](../../analyse-der-exams/03_Verfuegbarkeit_Ausfallsicherheit.md)

