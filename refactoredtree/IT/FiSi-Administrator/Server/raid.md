# RAID

## In 30 Sekunden

- RAID bündelt mehrere Platten: **Verfügbarkeit, Performance oder beides**.
- Nutzbare Kapazität: RAID 0 = N·S, RAID 1 = S, RAID 5 = (N−1)·S, RAID 6 = (N−2)·S, RAID 10 = N/2·S. (Für 50/60 siehe Kernwissen.)
- Ausfalltoleranz: RAID 0 = 0, RAID 1/10 = mindestens 1, RAID 5 = 1, RAID 6 = 2, RAID 50/60 = 1/2 pro Sub-Array.
- **RAID ist kein Backup.** Löschen, Ransomware und Controller-Ausfall treffen das ganze Array → [Backup-Strategien](backup/backup-strategien.md).
- Der Rebuild ist die Risikophase: Bei großen RAID-5/6-Arrays kann dabei die nächste Platte ausfallen.
- Geprüft in 7 von 10 Jahrgängen, fast immer mit Rechnung (Kapazität oder Plattenanzahl).

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
| JBOD | 1 | Summe aller Platten | 0 | Platten nur aneinandergehängt, kein Striping, keine Redundanz |

*(M = Anzahl der RAID-5/6-Gruppen. Ein RAID 50 mit 6 Platten hat z. B. 2 Gruppen à 3 Platten.)*

**Rechenbeispiel: 6 Platten à 4 TB**
- RAID 5: (6−1)·4 = **20 TB** nutzbar, 1 Ausfall.
- RAID 6: (6−2)·4 = **16 TB** nutzbar, 2 Ausfälle.
- RAID 10: 3·4 = **12 TB** nutzbar, mindestens 1 Ausfall.
- **RAID 50 (2× RAID 5):** (3−1)·4 + (3−1)·4 = **16 TB** nutzbar, 1 Ausfall pro Gruppe.
- **RAID 60 (2× RAID 6, 8 Platten):** (4−2)·4 + (4−2)·4 = **16 TB** nutzbar, 2 Ausfälle pro Gruppe.

### Rückwärts rechnen: Wie viele Platten brauche ich?

Formel nach N auflösen und **aufrunden**.

| Gegeben | Rechnung | Ergebnis |
|---|---|---|
| 20 TiB netto, 4-TiB-Platten, RAID 5 (2024S) | (N−1)·4 ≥ 20 → N ≥ 6 | **6 Platten** |
| dasselbe, RAID 6 | (N−2)·4 ≥ 20 → N ≥ 7 | **7 Platten** |
| dasselbe, RAID 10 | N/2·4 ≥ 20 → N ≥ 10 | **10 Platten** |
| 9,4 TiB netto, 1-TiB-Platten, RAID 6 (2023S) | N−2 ≥ 9,4 → N ≥ 11,4 | **12 Platten** |
| 13 TiB netto, 2-TiB-Platten, RAID 6 + Hot Spare (2022W) | (N−2)·2 ≥ 13 → N ≥ 8,5 → 9, plus 1 Hot Spare | **10 Platten** |
| 160 TiB netto, 8-TiB-Platten, RAID 6 (2025S) | (N−2)·8 ≥ 160 → N ≥ 22 | **22 Platten** |

Woher kommt der Nettobedarf? → [Kapazitätsplanung](../../Komponenten/Storage/kapazitaetsplanung.md).

### Unterschiedlich große Platten und verschachtelte Level

- In einem RAID-Verbund zählt pro Platte nur die Kapazität der **kleinsten** Platte. 4 × 2,5 TiB + 2 × 3,5 TiB als RAID 5 über alle sechs: (6−1) · 2,5 = **12,5 TiB**, je 1 TiB der großen Platten bleibt ungenutzt.
- **RAID 15** (RAID 5 über RAID-1-Paare, 2024W): drei Spiegelpaare (2,5 | 2,5 | 3,5 TiB) → RAID 5 über drei Mitglieder: (3−1) · 2,5 = **5 TiB**. Sehr hohe Ausfallsicherheit bei hohem Plattenaufwand.
- **JBOD** (2024W): kein Schutz, keine Beschleunigung, fällt eine Platte aus, sind deren Daten verloren.

**Die Bausteine**
- **Striping:** Daten verteilen. Bringt Geschwindigkeit, keine Sicherheit.
- **Mirroring:** Daten kopieren. Bringt Sicherheit, kostet die halbe Kapazität.
- **Parity:** Daten errechnen (XOR). Bringt Sicherheit mit weniger Overhead als Mirroring, kostet Schreibleistung.
- **Hot Spare:** Reserveplatte im Leerlauf, ersetzt eine ausgefallene Platte **automatisch** und startet den Rebuild sofort. Das verkürzt die Zeit im ungeschützten Zustand.

**Welches Level wofür? (2022S)**
- Höchste **Schreibrate** mit Fehlertoleranz: **RAID 10** (kein Parity-Overhead).
- Größte **Nettokapazität** mit Fehlertoleranz: **RAID 5**.
- Größte Sicherheit bei großen Arrays: **RAID 6**, denn es übersteht 2 Ausfälle in **beliebiger** Kombination. RAID 10 übersteht 2 Ausfälle nur, wenn sie in verschiedenen Spiegelpaaren liegen (2025S).

**Hardware vs. Software**
- Hardware-RAID: eigener Controller, Cache mit Batterie- oder Flash-Schutz (BBU) für Write-Back. Merkmale → [Übung RAID-Controller](../../analyse-der-exams/uebung-raid-controller.md).
- Software-RAID: nutzt CPU und Betriebssystem (`mdadm`, Storage Spaces, ZFS). Günstiger, aber an den Host gebunden.

### Rebuild und Degraded Mode

- **Degraded Mode:** Eine Platte ist ausgefallen, das Array läuft weiter, aber mit reduzierter Redundanz (RAID 5: keine mehr, RAID 6: noch eine).
- **Rebuild:** Daten der ausgefallenen Platte werden auf die Ersatzplatte (Hot Spare) zurückgerechnet oder kopiert. Dauer hängt ab von Plattengröße, Controller, Systemlast und Rebuild-Priorität.

| Level | Rebuild | Grund | Risiko währenddessen |
|---|---|---|---|
| RAID 1 / 10 | schnell (Stunden) | 1:1-Kopie der Spiegelplatte | gering, nur der Ausfall des Spiegelpartners ist fatal |
| RAID 5 | mittel (Stunden bis Tage) | XOR aus allen übrigen Platten | **zweiter Ausfall = Totalverlust**, alle Platten werden voll gelesen |
| RAID 6 | langsam | zwei Paritäten (P und Q) | erst der dritte Ausfall ist fatal |
| RAID 50 / 60 | wie die betroffene Sub-Gruppe | nur eine Gruppe wird neu aufgebaut | wie RAID 5/6 dieser Gruppe |

Gegenmaßnahmen: Hot Spare, RAID 6 statt 5 bei großen Platten, Platten verschiedener Chargen (Mix-and-Match), Monitoring (SMART, RAID-Status), und vor allem ein aktuelles Backup.

## Prüfungsfrage

**Frage:** Ein Server hat 4× 8-TB-Platten. Vergleichen Sie RAID 5 und RAID 10 nach nutzbarer Kapazität, Ausfalltoleranz und Eignung für eine Datenbank.

**Musterantwort:** RAID 5 ergibt (4−1)·8 = 24 TB und toleriert einen Plattenausfall. Schreibzugriffe erzeugen Parity-Overhead, und der Rebuild ist riskant. RAID 10 ergibt 4/2·8 = 16 TB, toleriert mindestens einen Ausfall und ist bei Schreiblast schneller, kostet aber 8 TB Kapazität. Für eine schreibintensive Datenbank ist RAID 10 die bessere Wahl.

## Falle

- Brutto und netto verwechseln: 6× 4 TB sind nicht 24 TB nutzbar.
- Plattenanzahl abrunden: 11,4 Platten sind **12**.
- Den Hot Spare in die Kapazität einrechnen. Er trägt keine Daten.
- RAID 5 als backupähnlich verkaufen → sofort Punktabzug.
- RAID 0 als „schnell und sicher“ beschreiben. RAID 0 hat keine Redundanz.
- Bei RAID 10 die Ausfalltoleranz überschätzen: Fallen beide Platten desselben Spiegelpaars aus, ist das Array verloren.
- **Bei RAID 50/60 die Ausfalltoleranz falsch berechnen:** Ein RAID 50 mit zwei Sub-Arrays überlebt **nicht** zwei Ausfälle, wenn beide in derselben Gruppe auftreten.
- **RAID 50/60 als Wundermittel sehen:** Es ist ein Kompromiss und braucht viele Platten (min. 6 bzw. 8).
- Bei gemischten Plattengrößen mit der größten statt der kleinsten Platte rechnen.

## Merksatz

**„5 minus 1, 6 minus 2, 10 halb. 50/60 sind verschachtelt, also die Gruppen zählen, nicht das Ganze.“** Erst Platten zählen, dann rechnen, nie TB raten.

## Verwandt

- [Backup-Strategien](backup/backup-strategien.md) – warum RAID kein Backup ersetzt
- [Snapshots-Backups](virtualisierung/snapshots-backups.md) – Snapshots sind auch kein Backup
- [Kapazitätsplanung](../../Komponenten/Storage/kapazitaetsplanung.md) – Nettobedarf berechnen
- [Verfügbarkeit & Kennzahlen](verfuegbarkeit-kennzahlen.md) – MTBF, Badewannenkurve, Mix-and-Match
- [Server-Hardware](../../Komponenten/server-hardware.md) – RAID-Controller, Hot-Swap
- [DAS-NAS-SAN](../../Komponenten/Storage/das-nas-san.md) – wo das Array im Speichernetz hängt
- [Schutzziele](../Sicherheit/schutzziele.md) – Verfügbarkeit als Schutzziel
- [Lektion 02 Speicher, Backup, RAID](../../analyse-der-exams/02_Speicher_Backup_RAID.md)
- [Übung RAID-Controller](../../analyse-der-exams/uebung-raid-controller.md)
