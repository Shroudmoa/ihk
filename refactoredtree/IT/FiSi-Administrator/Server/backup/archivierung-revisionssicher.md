# Archivierung (revisionssicher)

## In 30 Sekunden

- **Backup** = Kopie für die **Wiederherstellung** nach einem Ausfall. Sie ist kurzlebig, wird rotiert und überschrieben.
- **Archiv** = **Langzeitaufbewahrung** des Originals. Es ist unveränderbar, wiederauffindbar und unterliegt gesetzlichen Fristen.
- **Revisionssicher** heißt: vollständig, unveränderbar, nachvollziehbar (protokolliert), wiederauffindbar, über die ganze Frist lesbar.
- Fristen (§ 257 HGB, § 147 AO, GoBD): **10 Jahre** für Bücher, Inventare und Jahresabschlüsse, **8 Jahre** für Buchungsbelege wie Rechnungen (seit 2025, vorher 10), **6 Jahre** für Handels- und Geschäftsbriefe.
- Technik: **WORM**-Medien, Object Lock, Hashwerte, Indizierung.
- Geprüft: 2021W (revisionssicher), 2022W (Speicherbedarf Archiv), 2023W (Backup → Archiv), 2025W (Backup vs. Archiv).

## Kernwissen

### Backup vs. Archiv

| Kriterium | Datensicherung (Backup) | Datenarchivierung |
|---|---|---|
| Ziel | Wiederherstellung nach Datenverlust (RTO/RPO) | Aufbewahrung, Nachweis, Compliance |
| Daten | Kopie aktueller Produktivdaten | oft das **einzige** Original, aus dem Produktivsystem ausgelagert |
| Dauer | Tage bis Monate (Rotation) | Jahre bis Jahrzehnte (gesetzliche Frist) |
| Veränderbarkeit | wird regelmäßig überschrieben | **unveränderbar** (WORM) |
| Zugriff | Restore im Notfall | Recherche über Index/Metadaten |
| Löschen | nach Rotationsplan | erst nach Fristablauf, dann aber **Pflicht** (DSGVO) |

### Anforderungen an revisionssicheres Archiv (GoBD)

| Anforderung | Technische/organisatorische Umsetzung |
|---|---|
| Unveränderbarkeit | WORM-Band (LTO-WORM), Object Lock (S3 Compliance Mode), WORM-Storage |
| Integrität nachweisen | Hashwerte (SHA-256) speichern und regelmäßig prüfen, digitale Signatur/Zeitstempel |
| Vollständigkeit | kein Löschen vor Fristende, lückenlose Übernahme |
| Nachvollziehbarkeit | Audit-Trail: wer hat wann was abgelegt oder gelesen |
| Wiederauffindbarkeit | Indizierung, Metadaten, Volltextsuche |
| Lesbarkeit über die Frist | offene Formate (PDF/A, TIFF), Medienmigration vor Ende der Lebensdauer |
| Verfahrensdokumentation | beschriebener Ablauf, Zuständigkeiten, Zugriffsrechte |

### Rechenweg: Speicherbedarf eines Archivs (Muster 2022W)

Gegeben: 30 GiB neue Daten pro Monat, Komprimierungsrate 60 %, Aufbewahrung 10 Jahre, 1 TiB reserviert.

1. „Komprimierungsrate 60 %“ heißt hier: 60 % werden eingespart, 40 % bleiben → 30 GiB · 0,4 = **12 GiB/Monat**.
2. 10 Jahre: 12 GiB · 12 Monate · 10 = **1.440 GiB ≈ 1,41 TiB**.
3. Reichweite von 1 TiB: 1.024 GiB / 12 GiB ≈ 85,3 Monate ≈ **7 volle Jahre**.

Steht in der Aufgabe „auf 60 % komprimiert“, bleiben 60 % übrig (18 GiB/Monat). Die Formulierung entscheidet, deshalb die eigene Annahme immer hinschreiben.

## Prüfungsfrage

**Frage:** Die Backup-Bänder sollen künftig auch zur Archivierung von Rechnungen dienen. Nennen Sie zwei Maßnahmen, die dafür nötig sind.

**Musterantwort:** Erstens müssen die archivierten Daten unveränderbar gespeichert werden, z. B. auf WORM-Bändern, die nicht überschrieben werden können. Die Bänder fallen dafür aus der normalen Rotation heraus. Zweitens muss die Aufbewahrung über die gesetzliche Frist gesichert sein (Rechnungen als Buchungsbelege 8 Jahre, ältere Musterlösungen nennen noch 10 Jahre). Dafür werden die Daten mit Index/Metadaten katalogisiert, ihre Integrität wird regelmäßig per Hashwert geprüft, und die Bänder werden vor Ende ihrer Lebensdauer auf neue Medien migriert.

## Falle

- „Backup = Archiv“. Ein rotiertes Backup wird nach Wochen überschrieben, ein Archiv darf das nie.
- Archiv ohne Index: Die Daten sind vorhanden, aber nicht auffindbar, also nicht revisionssicher.
- Fristen nur als „lange aufbewahren“ beschreiben. Nach Ablauf muss nach DSGVO (Speicherbegrenzung) **gelöscht** werden.
- Proprietäre Dateiformate, die in 10 Jahren keine Software mehr öffnet.

## Merksatz

**„Backup ist für den Notfall, Archiv ist für den Prüfer.“**

## Verwandt

- [Backup-Strategien](backup-strategien.md)
- [Disaster Recovery & Notfallkonzept](disaster-recovery-notfallkonzept.md)
- [Deduplizierung & Komprimierung](../../../Komponenten/Storage/deduplizierung-komprimierung.md)
- [Kapazitätsplanung](../../../Komponenten/Storage/kapazitaetsplanung.md)
- [Hash](../../Sicherheit/Verschluesselung/hash.md) – Integritätsprüfung
- [DSGVO-TOM](../../../Management/DSGVO/dsgvo-tom.md)
- [Lektion 02 Speicher, Backup, RAID](../../../analyse-der-exams/02_Speicher_Backup_RAID.md)
