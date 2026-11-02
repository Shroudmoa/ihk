# Failover vs. Crossover (im Kontext von Speichersystemen)

Die Begriffe stammen ursprünglich aus dem Netzwerkbereich und dem High Availability (HA), werden in der IHK aber auch auf Speichersysteme (SAN, RAID) angewendet. Wichtig: Sie beschreiben **zwei völlig unterschiedliche Dinge**. 

## 1. Crossover (im HA-Kontext / Clustering)
*   **Definition:** Ein „Crossover“ (auch *Failover-Kabel* oder *Heartbeat-Link*) ist eine **direkte physische Verbindung** zwischen zwei Systemen (z.B. zwei Servern oder zwei SAN-Controllern).
*   **Zweck:** Diese Leitung dient oft als **Heartbeat** (Lebenszeichen). Die beiden Systeme senden sich permanent „Ich lebe noch“-Signale. Fällt eines aus, erkennt das andere sofort den Ausfall und übernimmt dessen Aufgaben (Failover).
*   **Im Speicherkontext:** Zwei SAN-Controller sind über ein Crossover-Kabel verbunden. Fällt Controller A aus, merkt Controller B das über dieses Kabel und übernimmt alle Datenzugriffe ohne Unterbrechung.

## 2. Failover (im Speicherkontext)
*   **Definition:** Failover ist der **automatische Umschaltprozess** auf ein redundantes System (Ersatzsystem), wenn das Primärsystem ausfällt.
*   **Zweck:** Aufrechterhaltung der **Verfügbarkeit** (High Availability). Der Benutzer merkt im Idealfall nichts vom Ausfall.
*   **Umsetzung bei RAID:** Fällt eine Festplatte aus, übernimmt die Hot-Spare-Platte (automatischer Failover).
*   **Umsetzung bei SAN:** Fällt ein SAN-Controller oder ein Switch (Fabric) aus, übernimmt der zweite Controller/der zweite Pfad (Multipath) die Datenübertragung.

## IHK-Merksatz
> „**Crossover** ist die physische Direktverbindung (Herzschlag/Heartbeat) zwischen zwei Systemen, um den Ausfall des anderen sofort zu erkennen. **Failover** ist der eigentliche Umschaltvorgang auf das Ersatzsystem, wenn der Fehler erkannt wurde.“
