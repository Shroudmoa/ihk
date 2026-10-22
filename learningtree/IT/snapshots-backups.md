# Snapshots-Backups

# VM-Snapshots und Backups

## Snapshots

Ein Snapshot speichert den Zustand einer VM zu einem bestimmten Zeitpunkt. Je nach Hypervisor werden Änderungen danach in zusätzlichen Delta-Dateien gespeichert.

- **Einsatz:** Vor Updates, Konfigurationsänderungen oder Softwaretests.
- **Vorteil:** Schnelles Zurücksetzen auf einen früheren Zustand.
- **Nachteil:** Benötigen zusätzlichen Storage und können bei langer Aufbewahrung die Performance beeinträchtigen.
- **Risiko:** Liegen Snapshot und VM auf demselben Storage, können beide durch einen Storage-Ausfall verloren gehen.

**Wichtig:** Ein Snapshot ist kein unabhängiges Backup. Er eignet sich für kurzfristige Rollbacks, nicht als alleinige Datensicherung.

## Backups

Ein Backup ist eine separate Sicherung zur langfristigen Aufbewahrung und Wiederherstellung von Daten oder vollständigen VMs.

### 3-2-1-1-0-Regel

- **3** Kopien der Daten, inklusive Original.
- **2** unterschiedliche Speichermedien.
- **1** Kopie extern (Offsite).
- **1** Kopie offline oder unveränderbar (Immutable).
- **0** ungeprüfte Fehler: Backups regelmäßig prüfen und Wiederherstellungen testen.

**Ransomware-Schutz:** Getrennte Zugriffsrechte, Offline-Kopien und unveränderbare Backups erschweren die Manipulation von Sicherungen.

## Backup-Arten

### Vollbackup (Full Backup)
Sichert alle ausgewählten Daten vollständig.

- Vorteil: Einfachste Wiederherstellung.
- Nachteil: Hoher Speicherbedarf und längere Sicherungszeit.
- Restore: Ein vollständiges Backup genügt.

### Inkrementelles Backup (Incremental)
Sichert Änderungen seit dem letzten Backup, egal ob Vollbackup oder inkrementelles Backup.

- Vorteil: Geringer Speicherbedarf und kurze Sicherungszeit.
- Nachteil: Restore benötigt das Vollbackup und alle erforderlichen nachfolgenden Inkremente.

### Differentielles Backup (Differential)
Sichert alle Änderungen seit dem letzten Vollbackup.

- Vorteil: Restore benötigt nur das Vollbackup und das letzte differentielle Backup.
- Nachteil: Speicherbedarf und Sicherungsdauer steigen bis zum nächsten Vollbackup.

## Vergleich

| Kriterium | Vollbackup | Inkrementell | Differentiell |
|---|---|---|---|
| Speicherbedarf | Hoch | Niedrig | Steigend |
| Sicherungsdauer | Hoch | Niedrig | Steigend |
| Restore-Aufwand | Gering | Hoch | Mittel |
| Benötigte Sicherungen | Vollbackup | Vollbackup + alle benötigten Inkremente | Vollbackup + letztes Differential |

## IHK-Prüfungswissen

- **RPO (Recovery Point Objective):** Maximal tolerierbarer Datenverlust, ausgedrückt als Zeit.
- **RTO (Recovery Time Objective):** Maximal tolerierbare Zeit bis zur Wiederherstellung.
- **RAID ≠ Backup:** RAID schützt je nach Level vor bestimmten Laufwerksausfällen, aber nicht zuverlässig vor Ransomware oder versehentlichem Löschen.
- **Restore-Test:** Prüft, ob die gesicherten Daten tatsächlich wiederhergestellt werden können.

## 3-Zeilen-IHK-Antwort

Snapshots ermöglichen kurzfristige Rollbacks, sind aber kein Ersatz für unabhängige Backups.
Backups schützen Daten durch separate Sicherungskopien und ermöglichen die Wiederherstellung nach Ausfällen oder Datenverlust.
Die 3-2-1-1-0-Regel kombiniert mehrere Kopien, unterschiedliche Medien, externe und offline beziehungsweise unveränderbare Sicherungen sowie überprüfte Wiederherstellbarkeit.

## Verwandt

- [RAID](raid.md) – RAID ist kein Backup
- [Live-Migration-VMs](live-migration-vms.md) – Alternative zum Herunterfahren
- [Schutzziele](schutzziele.md) – Verfügbarkeit als Schutzziel

