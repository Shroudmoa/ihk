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

**„5 minus 1, 6 minus 2, 10 halb.“** Erst Platten zählen, dann rechnen – nie TB raten.

## Verwandt

- [Snapshots-Backups](snapshots-backups.md) – warum RAID kein Backup ersetzt
- [Schutzziele](schutzziele.md) – Verfügbarkeit als Schutzziel
- [DAS-NAS-SAN](das-nas-san.md) – wo das Array im Speichernetz hängt
