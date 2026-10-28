# RAID


## In 30 Sekunden

- RAID bündelt mehrere Platten: **Verfügbarkeit, Performance oder beides**.
- Nutzbare Kapazität: RAID 0 = N·S, RAID 1 = S, RAID 5 = (N−1)·S, RAID 6 = (N−2)·S, RAID 10 = N/2·S.
- Ausfalltoleranz: RAID 0 = 0, RAID 1/10 = mindestens 1, RAID 5 = 1, RAID 6 = 2.
- **RAID ist kein Backup.** Löschen, Ransomware und Controller-Ausfall treffen das ganze Array.
- Der Rebuild ist die Risikophase – bei großen RAID-5-Arrays kann dabei die nächste Platte sterben.

## Kernwissen

| Level | Min. Platten | Kapazität | Ausfälle | Besonderheit |
|---|---:|---|---:|---|
| 0 | 2 | N·S | 0 | Striping, schnell, keine Redundanz |
| 1 | 2 | S | 1 | Mirroring, teuer, simpel |
| 5 | 3 | (N−1)·S | 1 | Parity, gute Balance |
| 6 | 4 | (N−2)·S | 2 | zwei Parity, große Arrays |
| 10 | 4 | N/2·S | ≥ 1 | Mirror + Stripe, schnell und sicher |

**Rechenbeispiel: 6 Platten à 4 TB**
- RAID 5: (6−1)·4 = **20 TB** nutzbar, 1 Ausfall.
- RAID 6: (6−2)·4 = **16 TB** nutzbar, 2 Ausfälle.
- RAID 10: 3·4 = **12 TB** nutzbar, mindestens 1 Ausfall.

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

## Merksatz


Erst Platten zählen, dann rechnen – nie TB raten.


---

## RAID 50 – Striping aus mehreren RAID-5-Sets
### Konzept
Mehrere unabhängige RAID-5-Sets werden untereinander **gestripet** (verknüpft wie bei RAID 0).
Beispiel: 3× RAID-5 mit je 3 Platten = 9 Platten insgesamt → ein großes RAID-50.

| Merkmal | Wert |
|---------|------|
| **Mindestplatten** | 6 (z. B. 2 Sets à 3 Platten) |
| **Redundanz** | Einfache Parität (XOR) pro RAID-5-Set |
| **Ausfalltoleranz** | 1 Platte **pro Set** darf ausfallen (mehrere Platten insgesamt möglich, solange nicht im selben Set) |
| **Lesen** | Exzellent (Striping über alle Sets) |
| **Schreiben** | Mittel (Paritätsberechnung pro Set) |
| **Nutzkapazität** | (Anzahl Sets) × (n−1) × Plattengröße |

> **Drei-Zeilen-Explain**
> 1️⃣ RAID 50 fasst mehrere RAID-5-Gruppen zu einem großen logischen Laufwerk zusammen. 2️⃣ Pro RAID-5-Gruppe darf eine Platte sterben, zwei im selben Set bedeuten Datenverlust. 3️⃣ Es liefert hohe Lese-Performance und gute Kapazität, ist aber beim Schreiben langsamer als RAID 10.

### Typisches Einsatzgebiet
- Große Dateiserver, Video-Archive, Backup-Repository, Medienbibliotheken
- Wo **viel Speicherplatz** und **gutes sequentielles Lesen** wichtiger sind als maximale Schreib-IOPS

---

## RAID 60 – Striping aus mehreren RAID-6-Sets
### Konzept
Wie RAID 50, aber die Basis sind **RAID-6-Sets** mit **Doppelparität** (z. B. Reed-Solomon).
Beispiel: 2× RAID-6 mit je 4 Platten = 8 Platten insgesamt → RAID 60.

| Merkmal | Wert |
|---------|------|
| **Mindestplatten** | 8 (z. B. 2 Sets à 4 Platten) |
| **Redundanz** | Doppelparität pro RAID-6-Set |
| **Ausfalltoleranz** | 2 Platten **pro Set** dürfen gleichzeitig ausfallen |
| **Lesen** | Exzellent (Striping) |
| **Schreiben** | Schwächer (2× Paritätsberechnung pro Set) |
| **Nutzkapazität** | (Anzahl Sets) × (n−2) × Plattengröße |

> **Drei-Zeilen-Explain**
> 1️⃣ RAID 60 stripet mehrere RAID-6-Gruppen, dadurch ist es noch ausfallsicherer als RAID 50. 2️⃣ Jedes Set verträgt zwei gleichzeitige Plattenausfälle ohne Datenverlust. 3️⃣ Der Preis ist mehr Schreib-Overhead und weniger nutzbare Kapazität durch die Doppelparität.

### Typisches Einsatzgebiet
- Virtualisierungskluster (VMware, Hyper-V, Proxmox)
- Produktionsdatenbanken, große SAN/NAS-Systeme
- Kritische Unternehmensdaten, wo **Ausfallsicherheit** vor reiner Kapazität geht

---

## Tabelle 1: RAID-Level im Überblick (AP2-Prüfungsrelevant)

| RAID | Min. Platten | Redundanz / Technik | Ausfalltoleranz | Lesen | Schreiben | Kapazität | Kosten | Typischer Einsatz |
|------|-------------|---------------------|-----------------|-------|-----------|-----------|--------|-------------------|
| **0** | 2 | Keine (Striping) | 0 | Sehr gut | Sehr gut | 100 % | Günstig | Temp-Daten, Cache, nicht-kritisch |
| **1** | 2 | Spiegelung | 1 pro Paar | Gut | Mittel | 50 % | Teuer | OS, Boot, kleine kritische Volumes |
| **5** | 3 | Einfache Parität (XOR) | 1 Platte | Gut | Mittel | (n−1)/n | Mittel | Dateiserver, Archiving |
| **6** | 4 | Doppelparität | 2 Platten | Gut | Schwächer | (n−2)/n | Mittel | Großspeicher, NAS, SAN |
| **10** (1+0) | 4 | Spiegel + Stripe | 1 pro Spiegelpaar | Sehr gut | Gut | 50 % | Teuer | Datenbanken, Virtualisierung, Hochverfügbarkeit |
| **50** | 6 | Striping von RAID-5-Sets | 1 pro RAID-5-Set | Sehr gut | Mittel | Variabel | Mittel–hoch | Video-Archive, Backup-Storage, große Dateiserver |
| **60** | 8 | Striping von RAID-6-Sets | 2 pro RAID-6-Set | Sehr gut | Schwächer–mittel | Variabel | Hoch | Virtualisierung, kritische DBs, Big Data |

---

## Tabelle 2: RAID 50 vs. RAID 60 – Vorteile, Nachteile & Ideal-Einsatz

| Kriterium | RAID 50 | RAID 60 |
|-----------|---------|---------|
| **Aufbau** | Mehrere RAID-5-Sets gestripet | Mehrere RAID-6-Sets gestripet |
| **Mindestplatten** | 6 | 8 |
| **Ausfalltoleranz** | 1 Platte pro Set (mehrere Sets möglich) | 2 Platten pro Set |
| **Les performance** | Exzellent (Striping) | Exzellent (Striping) |
| **Schreib performance** | Mittel (Parität pro Set) | Schwächer (2× Parität pro Set) |
| **Nutzkapazität** | Höher (nur 1 Parity-Platte pro Set) | Niedriger (2 Parity-Platten pro Set) |
| **Rebuild-Zeit** | Kürzer | Länger (mehr Parity-Daten) |
| **Kosten** | Mittel | Hoch (mehr Platten für Redundanz nötig) |
| **Risiko beim Rebuild** | URE-Risiko bei großen SATA-Platten vorhanden | Geringer durch Doppelparität |
| **Ideal für** | Große Dateien, Video-Editing, Backup-Repository, Dateiserver mit viel Lesen | Virtualisierung, Datenbanken, kritische SAN/NAS, Hochverfügbarkeit |

---

## AP2-Zusatzwissen: Was du noch wissen solltest

| Thema | Kurzfassung für die Prüfung |
|-------|-----------------------------|
| **Hot Spare** | Ersatzplatte, die im Hintergrund bereit liegt und automatisch bei einem Ausfall in den RAID-Verbund übernommen wird. Verkürzt die Ausfallzeit. |
| **Hardware- vs. Software-RAID** | HW-RAID: Dedizierter Controller mit Cache und BBU/FBU (schont CPU, teurer). SW-RAID: z. B. Linux `mdadm`, ZFS (`raidz1/2/3`), Windows Storage Spaces (günstig, CPU-lastiger). |
| **URE (Unrecoverable Read Error)** | Bei großen SATA-HDDs kann beim Rebuild ein Lesefehler auftreten. RAID 5 ist hier riskant, RAID 6 robuster. |
| **Write Hole** | Bei Stromausfall während Paritätsberechnung kann die Parität inkonsistent werden. Lösung: BBU/FBU beim Controller oder Copy-on-Write (z. B. ZFS). |
| **SSD vs. HDD in RAID** | SSDs haben kaum URE-Probleme, aber Schreibverstärkung (Write Amplification) und gleichzeitiger Verschleiß (alle Platten alt werden zur selben Zeit). |
| **RAID ist kein Backup** | Schutz vor Plattenausfall, aber nicht vor Ransomware, versehentlichem Löschen, Feuer, Diebstahl. Immer zusätzliches Backup (3-2-1-Regel) einplanen. |

---



## Verwandt

- [Snapshots-Backups](snapshots-backups.md) – warum RAID kein Backup ersetzt
- [Schutzziele](schutzziele.md) – Verfügbarkeit als Schutzziel
- [DAS-NAS-SAN](das-nas-san.md) – wo das Array im Speichernetz hängt
