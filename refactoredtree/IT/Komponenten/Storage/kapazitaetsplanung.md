# Kapazitätsplanung (Speicherbedarf berechnen)

## In 30 Sekunden

- Typische GA1-Rechnung: **Bestand + Zuwachs · Jahre ≤ Kapazität · Füllgrenze**.
- Erst **Einheiten angleichen**: 1 TiB = 1.024 GiB, 1 TB = 1.000 GB. Eine 4-TB-Platte zeigt nur ≈ 3,64 TiB an.
- Ergebnisse mit „Jahren“ **abrunden** (die Grenze darf nicht überschritten werden), Plattenzahlen **aufrunden** (es muss reichen).
- Danach geht es oft weiter: benötigte Nettokapazität → Plattenanzahl im RAID → [RAID](../../FiSi-Administrator/Server/raid.md).
- Geprüft: 2021W (Betriebszeit SAN), 2022W (RAID 6 + Hot Spare, Archiv), 2023S (Nettokapazität 5 Jahre), 2024W (SAN 3 Jahre), 2025S (RAID-Erweiterung).

## Kernwissen

### Einheiten (Dezimal vs. Binär)

| Dezimal (SI, Herstellerangabe) | Binär (IEC, Betriebssystem) | Verhältnis |
|---|---|---|
| 1 KB = 1.000 Byte | 1 KiB = 1.024 Byte | 1 KiB = 1,024 KB |
| 1 GB = 10⁹ Byte | 1 GiB = 2³⁰ Byte | 1 GB ≈ 0,931 GiB |
| 1 TB = 10¹² Byte | 1 TiB = 2⁴⁰ Byte | 1 TB ≈ 0,909 TiB |

### Rechenweg A: Wie lange reicht der Speicher?

Gegeben: neues SAN mit 20 TiB, das alte NAS mit 10 TiB ist zu 85 % belegt (Daten werden übernommen), Zuwachs 600 GiB pro Jahr. Die Belegung soll 70 % nicht überschreiten.

1. Bestand: 10 TiB · 0,85 = **8,5 TiB**
2. Grenze: 20 TiB · 0,70 = **14 TiB**
3. Puffer: 14 − 8,5 = **5,5 TiB** = 5,5 · 1.024 = 5.632 GiB
4. Jahre: 5.632 GiB / 600 GiB ≈ 9,39 → **9 volle Jahre** (im 10. Jahr wäre die Grenze überschritten)

### Rechenweg B: Welche Nettokapazität brauche ich? (Muster 2023S/2024W)

Gegeben: Altbestand 8 TiB zu 90 % belegt, Zuwachs 450 GiB/Jahr, Planung für 5 Jahre.

1. Bestand: 8 · 0,9 = 7,2 TiB
2. Zuwachs: 5 · 450 GiB = 2.250 GiB = 2.250 / 1.024 ≈ 2,197 TiB
3. Bedarf: 7,2 + 2,197 ≈ **9,4 TiB netto**

### Rechenweg C: Wie viele Platten? (weiter mit B)

RAID 6 mit 1-TiB-Platten: (N − 2) · 1 TiB ≥ 9,4 TiB → N − 2 ≥ 9,4 → N ≥ 11,4 → **12 Platten** (10 Daten + 2 Parität).
Mit Hot Spare: **13 Platten**.

**Muster 2022W:** 13 TiB netto, 2-TiB-Platten, RAID 6 + 1 Hot Spare → (N − 2) · 2 ≥ 13 → N ≥ 8,5 → 9 Platten im RAID + 1 Hot Spare = **10 Platten**.

**Muster 2025S:** RAID 6 mit 8-TiB-Platten soll von 80 auf 160 TiB netto wachsen → (N − 2) · 8 ≥ 160 → N = 22. Vorher (80 TiB) waren es 12 → **10 zusätzliche Platten**.

### Rechenweg D: Komprimierung und Aufbewahrung

30 GiB pro Monat, Einsparung durch Komprimierung 60 % → 12 GiB/Monat → 10 Jahre = 12 · 12 · 10 = **1.440 GiB**. Details unter [Archivierung](../../FiSi-Administrator/Server/backup/archivierung-revisionssicher.md).

### Planungsreserve

- Füllgrenze 70–80 %: Volle Datenträger werden langsam, Snapshots und Rebuilds brauchen Platz, und es bleibt Zeit für die Beschaffung.
- Wachstum oft prozentual (z. B. +20 %/Jahr). Dann zinseszinsartig rechnen: Bestand · 1,2ⁿ.
- Backups brauchen zusätzlichen Platz (→ [Backup-Strategien](../../FiSi-Administrator/Server/backup/backup-strategien.md)).

## Prüfungsfrage

**Frage:** Ein Fileserver mit 12 TiB ist zu 90 % belegt. Die Daten sollen auf ein SAN umziehen, der Zuwachs beträgt 650 GiB pro Jahr. Welche Nettokapazität muss das SAN für 3 Jahre bereitstellen?

**Musterantwort:** Bestand 12 TiB · 0,9 = 10,8 TiB. Zuwachs 3 · 650 GiB = 1.950 GiB = 1.950 / 1.024 ≈ 1,904 TiB. Benötigte Nettokapazität: 10,8 + 1,904 ≈ **12,7 TiB**.

## Falle

- GiB und TiB mischen, ohne durch 1.024 zu teilen.
- Die **Rohkapazität** des alten Systems als Bestand nehmen statt der **belegten** Menge.
- Jahre aufrunden. 9,39 Jahre heißt: 9 Jahre sicher, im 10. Jahr wird die Grenze überschritten.
- Plattenzahl abrunden. 11,4 Platten → 12.
- Netto und brutto verwechseln: Nach der Kapazitätsrechnung kommt noch der RAID-Verlust dazu.

## Merksatz

**„Einheiten angleichen, Bestand + Zuwachs, Jahre ab- und Platten aufrunden.“**

## Verwandt

- [RAID](../../FiSi-Administrator/Server/raid.md) – Netto-Formeln
- [DAS-NAS-SAN](das-nas-san.md)
- [Dateisysteme](dateisysteme.md) – Clustergröße und Verschnitt
- [Deduplizierung & Komprimierung](deduplizierung-komprimierung.md)
- [Bit-Byte](../../FiSi-Administrator/OSI-IP-mac-bit-byte/bit-byte.md) – Einheiten
- [Lektion 02 Speicher, Backup, RAID](../../analyse-der-exams/02_Speicher_Backup_RAID.md)
