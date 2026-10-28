# HA-Hochverfügbarkeit

### Hochverfügbarkeit (HA – High Availability)

**Definition:** Systeme bleiben trotz einzelner Hardware- oder Softwareausfälle verfügbar.

**Technologien:**
- **Failover-Cluster:** Ersatzserver übernimmt automatisch bei Ausfall.
- **Load Balancer:** Verteilt Anfragen auf mehrere Server.
- **Redundanz:** Doppelte Netzteile, Netzwerkkarten und Netzwerkverbindungen.
- **RAID:** Erhöht die Ausfallsicherheit von Datenträgern, ersetzt kein Backup.
- **USV:** Überbrückt Stromausfälle kurzfristig.
- **Geo-Redundanz:** Zweiter Standort bei Standortausfall.

**Verfügbarkeit:**
99,9 % = ca. 8,76 Stunden Ausfallzeit pro Jahr (bei 365 Tagen).

**SLA:** Vertragliche Vereinbarung über Verfügbarkeit und Serviceleistungen.

**IHK-Verknüpfung:**
- **RAID vs. Backup:** Verfügbarkeit vs. Datensicherung. RAID schützt vor Plattenausfall, Backup schützt vor Datenverlust (Löschen, Ransomware).
- **RTO (Recovery Time Objective):** Maximal tolerierbare Wiederherstellungszeit.
- **RPO (Recovery Point Objective):** Maximal tolerierbarer Datenverlust in Zeit.
- **OSI-Modell:** Redundante Netzwerkverbindungen betreffen insbesondere Schicht 1–3.
- **SPOF (Single Point of Failure):** Einzelne Komponente, deren Ausfall das Gesamtsystem beeinträchtigt.

## Verwandt
- [RAID](raid.md) – Redundanz auf Speicherebene
- [Snapshots-Backups](snapshots-backups.md) – Datensicherung als Teil von HA/DR
- [SLA](sla.md) – Vertragliche Verfügbarkeitsvereinbarung
- [Schutzziele](schutzziele.md) – Verfügbarkeit als eines der CIA-Schutzziele
- [Live-Migration-VMs](live-migration-vms.md) – Ausfallsicherheit durch VM-Migration
- [Netzwerktypen](netzwerktypen.md) – redundante Netzwerke

### Was solltest du noch lernen?
- **RAID 0, 1, 5, 6 und 10:** Leistung, Kapazität und Ausfallsicherheit.
- **Disaster Recovery:** Wiederherstellung nach größeren Ausfällen.
- **SPOF (Single Point of Failure):** Einzelne Komponente, deren Ausfall das Gesamtsystem beeinträchtigt.
