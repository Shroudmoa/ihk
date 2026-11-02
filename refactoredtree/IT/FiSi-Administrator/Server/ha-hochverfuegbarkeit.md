# HA - Hochverfügbarkeit (High Availability)

## Definition

**Hochverfügbarkeit (HA)** bedeutet, dass IT-Systeme trotz Ausfall einzelner Komponenten mit möglichst geringer Unterbrechung weiterhin nutzbar sind. Ziel ist eine hohe **Uptime**.

## Verfügbarkeitsklassen

| Klasse | Verfügbarkeit p.a. | Ausfallzeit p.a. | Einsatz |
|---|---|---|---|
| 99 % (Two Nines) | 99,00 % | ~87,6 h | Unkritische Systeme |
| 99,9 % (Three Nines) | 99,90 % | ~8,76 h | Standardgeschäftsanwendungen |
| 99,99 % (Four Nines) | 99,99 % | ~52,56 min | Kritische Anwendungen |
| 99,999 % (Five Nines) | 99,999 % | ~5,26 min | Hochkritische Systeme (z. B. Banken, Notruf) |

## RTO & RPO

| Begriff | Bedeutung | Fragestellung |
|---|---|---|
| **RTO (Recovery Time Objective)** | Maximal akzeptable **Ausfallzeit** bis zur Wiederherstellung des Dienstes | *Wie lange darf der Dienst ausfallen?* |
| **RPO (Recovery Point Objective)** | Maximal akzeptabler **Datenverlust** in Zeit gemessen | *Wie alt darf das letzte Backup maximal sein?* |

**Beispiel:** RPO = 15 Minuten → Max. 15 Minuten Datenverlust. RTO = 2 Stunden → Dienst muss innerhalb von 2 Stunden wieder laufen.

## USV-Typen

| Typ | Bezeichnung | Funktionsweise | Einsatz |
|---|---|---|---|
| **Offline/Standby** | VFD (Voltage and Frequency Dependent) | Direkt am Netz, schaltet erst bei Ausfall um. Geringste Umschaltzeit spürbar. | Einzelarbeitsplätze, kleine Geräte |
| **Line-Interactive** | VI (Voltage Independent) | Netzspannungskorrektur (AVR), bei Ausfall Umschaltung auf Batterie. | Kleine Server, Netzwerkgeräte, Drucker |
| **Online/Doppelwandler** | VFI (Voltage and Frequency Independent) | Dauerhafter Gleich-/Wechselrichterbetrieb. **Nahezu unterbrechungsfrei**. | Kritische Server, Speichersysteme, Rechenzentren |

## Maßnahmen zur Hochverfügbarkeit

| Maßnahme | Erklärung |
|---|---|
| **Redundanz** | Doppelte Komponenten (Netzteile, Netzwerkkarten, Festplatten) |
| **RAID** | Redundanz auf Datenträgerebene (ersetzt **kein** Backup) |
| **Failover-Cluster** | Automatischer Übergang auf Ersatzsystem bei Ausfall |
| **Load Balancer** | Lastverteilung auf mehrere Server + Ausfallsicherheit |
| **USV** | Überbrückung von Stromausfällen, kontrolliertes Herunterfahren |
| **Geo-Redundanz** | Zweiter Standort zur Absicherung gegen Standortausfall |
| **Hot Spare** | Reservekomponente zur schnellen Übernahme |

## HA vs. Disaster Recovery (DR)

| Kriterium | **HA (High Availability)** | **DR (Disaster Recovery)** |
|---|---|---|
| **Ziel** | Ausfälle möglichst vermeiden/überbrücken | Wiederherstellung nach Großschaden |
| **Fokus** | **Verfügbarkeit** | **Wiederherstellbarkeit** |
| **Auslöser** | Komponentenausfall | Katastrophe (Brand, Hochwasser, Ransomware) |
| **Zeitrahmen** | Sekunden – Minuten | Stunden – Tage |
| **Maßnahmen** | Cluster, Redundanz, Failover, Load Balancing | Backup/Restore, Notfallstandort, Notfallhandbuch |

## Wichtige Begriffe

- **SLA (Service Level Agreement):** Vertraglich vereinbarte Verfügbarkeitsziele
- **SPOF (Single Point of Failure):** Einziger kritischer Punkt, dessen Ausfall das Gesamtsystem lahmlegt. **Muss vermieden werden**
- **MTBF (Mean Time Between Failures):** Mittlere Zeit zwischen Ausfällen
- **MTTR (Mean Time To Repair/Restore):** Mittlere Reparatur-/Wiederherstellungszeit

## IHK-Merksätze

- **RAID vs. Backup:** RAID = Verfügbarkeit, Backup = Wiederherstellung & Schutz vor Löschung/Ransomware
- **Online-USV (VFI)** hat **keine merkliche Umschaltzeit**
- **RTO < RPO?** Nein – RTO ist die Zeit bis wieder da, RPO ist der maximal erlaubte Datenverlust
- **Hochverfügbarkeit ≠ Katastrophenschutz** – dafür braucht es Disaster Recovery

## Verwandt

- [RAID](raid.md) – Datenträgerredundanz
- [Snapshots-Backups](virtualisierung/snapshots-backups.md) – Datensicherung
- [Cluster](cluster.md) – Servercluster
- [Loadbalancer](load-balancer.md) – Lastverteilung
- [Schutzziele](../Sicherheit/schutzziele.md) – Verfügbarkeit (CIA)
- [Skalierbarkeit-Elastizität](virtualisierung/skalierbarkeit-elastizitaet.md) – Skalierungskonzepte

