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
- **RAID vs. Backup:** Verfügbarkeit vs. Datensicherung.
- **RTO:** Maximal tolerierbare Wiederherstellungszeit.
- **RPO:** Maximal tolerierbarer Datenverlust in Zeit.
- **OSI-Modell:** Redundante Netzwerkverbindungen betreffen insbesondere Schicht 1–3.

### Was solltest du noch lernen?
- **RAID 0, 1, 5, 6 und 10:** Leistung, Kapazität und Ausfallsicherheit.
- **Disaster Recovery:** Wiederherstellung nach größeren Ausfällen.
- **SPOF (Single Point of Failure):** Einzelne Komponente, deren Ausfall das Gesamtsystem beeinträchtigt.
