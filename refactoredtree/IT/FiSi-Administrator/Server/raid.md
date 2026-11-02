# RAID

## In 30 Sekunden

- RAID bündelt mehrere Platten: **Verfügbarkeit, Performance oder beides**.
- Nutzbare Kapazität: RAID 0 = N·S, RAID 1 = S, RAID 5 = (N−1)·S, RAID 6 = (N−2)·S, RAID 10 = N/2·S. (Für 50/60 siehe Kernwissen).
- Ausfalltoleranz: RAID 0 = 0, RAID 1/10 = mindestens 1, RAID 5 = 1, RAID 6 = 2, RAID 50/60 = 1/2 pro Sub-Array.
- **RAID ist kein Backup.** Löschen, Ransomware und Controller-Ausfall treffen das ganze Array.
- Der Rebuild ist die Risikophase – bei großen RAID-5/6-Arrays kann dabei die nächste Platte sterben.

## Kernwissen

| Level | Min. Platten | Kapazität | Ausfälle | Besonderheit |
|---|---:|---|---:|---|
| 0 | 2 | N·S | 0 | Striping, schnell, keine Redundanz |
| 1 | 2 | S | 1 | Mirroring, teuer, simpel |
| 5 | 3 | (N−1)·S | 1 | Parity, gute Balance |
| 6 | 4 | (N−2)·S | 2 | zwei Parity, große Arrays |
| 10 | 4 | N/2·S | ≥ 1 | Mirror + Stripe, schnell und sicher |
| **50** | **6** | **(N−M)·S** | **1 pro Sub-Array** | **Stripe über RAID 5 (Performance + 1 Ausfall)** |
| **60** | **8** | **(N−2M)·S** | **2 pro Sub-Array** | **Stripe über RAID 6 (Performance + 2 Ausfälle)** |

*(Hinweis: M = Anzahl der RAID-5/6-Gruppen. Ein RAID 50 mit 6 Platten hat z.B. 2 Gruppen à 3 Platten).*

**Rechenbeispiel: 6 Platten à 4 TB**
- RAID 5: (6−1)·4 = **20 TB** nutzbar, 1 Ausfall.
- RAID 6: (6−2)·4 = **16 TB** nutzbar, 2 Ausfälle.
- RAID 10: 3·4 = **12 TB** nutzbar, mindestens 1 Ausfall.
- **RAID 50 (2x RAID 5):** (3−1)·4 + (3−1)·4 = **16 TB** nutzbar, 1 Ausfall pro Gruppe.
- **RAID 60 (2x RAID 6):** (4−2)·4 + (4−2)·4 = **16 TB** nutzbar (bei 8 Platten), 2 Ausfälle pro Gruppe.

**Die vier Bausteine**
- **Striping** – Daten verteilen: Geschwindigkeit, keine Sicherheit.
- **Mirroring** – Daten kopieren: Sicherheit, halbe Kapazität.
- **Parity** – Daten errechnen: Sicherheit mit weniger Overhead als Mirroring.
- **Hot Spare** – Reserveplatte, springt automatisch ein.

**Hardware vs. Software**
- Hardware-RAID: eigener Controller, oft Cache mit Batterie- oder Flash-Schutz.
- Software-RAID: nutzt CPU und OS (`mdadm`, Storage Spaces), günstiger, hostabhängig.

## Prüfungsfrage

**Frage:** Ein Server hat 4× 8-TB-Platten. Vergleiche RAID 5 und RAID 10 nach nutzbarer Kapazität, Ausfalltoleranz und Eignung für eine Datenbank.

**Musterantwort:** RAID 5 ergibt (4−1)·8 = 24 TB und toleriert einen Plattenausfall; Schreibzugriffe erzeugen Parity-Overhead und der Rebuild ist riskant. RAID 10 ergibt 4/2·8 = 16 TB, toleriert mindestens einen Ausfall und ist bei Schreiblast schneller, kostet aber 8 TB Kapazität. Für eine schreibintensive Datenbank ist RAID 10 die bessere Wahl.

## Falle

- Brutto und netto verwechseln: 6× 4 TB ist nicht 24 TB nutzbar.
- RAID 5 als backupähnlich verkaufen → sofort Punktabzug.
- RAID 0 als „schnell und sicher“ beschreiben → RAID 0 hat keine Redundanz.
- Bei RAID 10 die Ausfalltoleranz überschätzen: fällt pro Mirror die falsche Platte aus, ist es vorbei.
- **Bei RAID 50/60 die Ausfalltoleranz falsch berechnen:** Ein RAID 50 mit zwei Sub-Arrays überlebt **nicht** zwei Ausfälle, wenn beide in derselben Gruppe auftreten. Die Formel (N−M) gilt nur, wenn pro Gruppe maximal eine Platte ausfällt.
- **RAID 50/60 als Wundermittel sehen:** Es ist ein Kompromiss. Du brauchst viele Platten (min. 6 bzw. 8), was teuer und platzintensiv ist.

## Merksatz

**„5 minus 1, 6 minus 2, 10 halb. 50/60 sind verschachtelt – zähl die Gruppen, nicht das Ganze.“** Erst Platten zählen, dann rechnen – nie TB raten.

## Verwandt

- [Snapshots-Backups](virtualisierung/snapshots-backups.md) – warum RAID kein Backup ersetzt
- [Schutzziele](../Sicherheit/schutzziele.md) – Verfügbarkeit als Schutzziel
- [DAS-NAS-SAN](../../Komponenten/Storage/das-nas-san.md) – wo das Array im Speichernetz hängt





---------------------------------------------------
# RAID-Rebuild & Wiederherstellungszeiten

## Grundlagen des Rebuilds
*   **Definition:** Der Prozess, bei dem ein RAID-Controller die Daten einer ausgefallenen Festplatte auf eine neue (oder Hot-Spare) Festplatte rekonstruiert.
*   **Degraded Mode:** Der Zustand des RAIDs während des Rebuilds. Die Redundanz ist reduziert (z.B. RAID 5 hat keine mehr, RAID 6 hat nur noch eine).
*   **Einflussfaktoren auf die Dauer:**
    *   **Festplattengröße:** Größere Platten = längere Rebuild-Zeit.
    *   **Controller-Leistung (ROC/CPU):** Schwache Controller brauchen länger für die Paritätsberechnung.
    *   **Systemlast:** Während des Rebuilds muss das System weiterarbeiten (I/O-Konflikte).
    *   **Rebuild-Rate:** Eine künstliche Drosselung (damit das System nicht überlastet wird) verlängert den Prozess.

## Rebuild-Zeiten nach RAID-Level (Relativ)

| RAID-Level | Rebuild-Geschwindigkeit | Grund / Berechnung |
| :--- | :--- | :--- |
| **RAID 1 / 10** | **Sehr schnell** (Stunden) | Reine 1:1-Kopie der intakten Spiegelplatte. Keine Paritätsberechnung nötig. |
| **RAID 5** | **Mittel** (Stunden bis Tage) | Fehlende Daten werden per XOR aus den verbleibenden Daten- und Paritätsplatten berechnet. |
| **RAID 6** | **Langsam** (Tage) | Höchste Rechenlast, da zwei unabhängige Paritätsblöcke (P und Q) berechnet werden müssen. |
| **RAID 50 / 60** | **Abhängig von der Sub-Gruppe** | Der Rebuild betrifft nur die eine fehlerhafte RAID-5/6-Gruppe. Die Zeit entspricht ungefähr der eines einzelnen RAID 5 oder RAID 6 derselben Größe. |

## Risiko im Degraded Mode
*   **RAID 5:** Fällt während des Rebuilds eine zweite Platte aus, ist das Array **komplett zerstört** (Datenverlust). Deshalb ist ein schneller Rebuild (z.B. durch Hot-Spare) hier kritisch.
*   **RAID 6:** Verträgt den Ausfall von zwei Platten. Fällt während des Rebuilds eine zweite aus, ist das System noch geschützt, aber der Rebuild startet von vorn. Fällt eine dritte aus, ist das Array zerstört.
*   **RAID 10:** Sehr geringes Risiko. Selbst wenn während des Rebuilds eine Platte aus dem *anderen* Mirror-Paar ausfällt, ist das Array noch funktionsfähig.

## IHK-Merksatz
> „Der Rebuild dauert immer so lange, wie es dauert, eine einzelne Festplatte komplett zu beschreiben. RAID 10 ist am schnellsten (reine Kopie), RAID 6 am langsamsten (doppelte Paritätsberechnung). Während des Rebuilds (Degraded Mode) ist das System anfällig für weitere Ausfälle.“
