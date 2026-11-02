# Snapshots und VM-Backups

## In 30 Sekunden

- Ein **Snapshot** friert den Zustand einer VM (Platten, optional RAM) zu einem Zeitpunkt ein. Änderungen danach landen in **Delta-Dateien**.
- Einsatz: **kurzfristiger Rollback** vor Updates, Konfigurationsänderungen und Tests.
- **Ein Snapshot ist kein Backup.** Er liegt auf demselben Storage und hängt von der Basisplatte ab. Fällt das Storage aus, sind VM und Snapshot weg.
- Ein **VM-Backup** kopiert die VM auf ein **anderes** Medium. Backup-Software nutzt dafür intern oft kurz einen Snapshot, um einen konsistenten Stand zu lesen.
- Snapshots nicht wochenlang behalten: Die Delta-Dateien wachsen, die Leistung sinkt, und das Zusammenführen dauert lange.
- Backup-Arten, 3-2-1, GVS und Restore-Ketten stehen ausführlich in [Backup-Strategien](../backup/backup-strategien.md).

## Kernwissen

### Snapshot vs. Backup vs. Replikation

| Kriterium | Snapshot | Backup | Replikation |
|---|---|---|---|
| Speicherort | **gleiches** Storage wie die VM | **getrenntes** Medium/Standort | zweites System, oft zweiter Standort |
| unabhängig von der Original-VM | nein (Delta auf Basisplatte) | ja | ja |
| Zweck | Rollback in Minuten | Wiederherstellung nach Verlust, Langzeit | schneller Wiederanlauf (kleines RTO/RPO) |
| Aufbewahrung | Stunden bis wenige Tage | Wochen bis Jahre (Rotation) | nur aktueller Stand plus wenige Wiederherstellungspunkte |
| Schutz vor Storage-Ausfall | nein | ja | ja |
| Schutz vor Ransomware/Löschen | nein (wenn der Angreifer den Hypervisor erreicht) | ja, wenn offline/immutable | nein, Fehler werden mitrepliziert |

### Wie ein Snapshot technisch funktioniert

1. Die Basisplatte (z. B. `.vmdk`, `.vhdx`) wird schreibgeschützt.
2. Neue Schreibzugriffe gehen in eine Delta-Datei (Redirect-on-Write bzw. Copy-on-Write).
3. **Zurücksetzen (Revert):** Delta verwerfen, die VM ist wieder auf dem alten Stand.
4. **Löschen/Konsolidieren:** Delta wird in die Basisplatte eingearbeitet, der neue Stand bleibt.

**Konsistenz:** Ein reiner Absturz-konsistenter Snapshot entspricht einem plötzlichen Stromausfall. Für Datenbanken braucht man **anwendungskonsistente** Snapshots (Windows VSS, Gastwerkzeuge), die Schreibpuffer vorher leeren.

### Best Practices

- Snapshot vor jedem Update, nach erfolgreichem Test zeitnah löschen (Faustregel: höchstens 24–72 h).
- Nicht mehrere Snapshots verketten, Speicherplatz des Datastores überwachen.
- VM-Backups mit einer Backup-Software auf getrenntes Storage, eine Kopie offsite/offline (3-2-1-1-0).
- Restore regelmäßig testen, auch kompletten VM-Restore und Einzeldatei-Restore.

## Prüfungsfrage

**Frage:** Ein Kollege meint, die tägliche Snapshot-Erstellung der VMs ersetze das Backup. Nehmen Sie Stellung.

**Musterantwort:** Die Aussage ist falsch. Ein Snapshot speichert nur die Änderungen seit seiner Erstellung und liegt auf demselben Storage wie die VM. Er hängt von der Basisplatte ab. Fällt das Storage aus oder wird es durch Ransomware verschlüsselt, sind VM und Snapshots gleichzeitig verloren. Viele lang gehaltene Snapshots verschlechtern außerdem die Leistung und füllen den Datastore. Snapshots eignen sich für kurzfristige Rollbacks, etwa vor Updates. Als Datensicherung braucht es zusätzlich ein Backup auf ein getrenntes Medium, mit einer Kopie außer Haus, z. B. nach der 3-2-1-Regel.

## Falle

- „Snapshot = Backup“. Es fehlt das getrennte Medium.
- Snapshot von einem Datenbankserver ohne Anwendungskonsistenz → möglicherweise inkonsistente Datenbank beim Revert.
- Snapshots vergessen. Der Datastore läuft voll, und **alle** VMs darauf bleiben stehen.
- Replikation als Backup verkaufen. Eine gelöschte Datei wird sofort auch im Replikat gelöscht.

## Merksatz

**„Snapshot = Rückgängig-Taste, Backup = Versicherung.“**

## Verwandt

- [Backup-Strategien](../backup/backup-strategien.md) – Voll/Diff/Inkr, GVS, 3-2-1, D2D2T
- [Disaster Recovery & Notfallkonzept](../backup/disaster-recovery-notfallkonzept.md) – RTO/RPO
- [RAID](../raid.md) – RAID ist kein Backup
- [Virtualisierung](virtualisierung.md)
- [Live-Migration-VMs](live-migration-vms.md)
- [Dateisysteme](../../../Komponenten/Storage/dateisysteme.md) – Copy-on-Write-Dateisysteme mit Snapshots
- [Schutzziele](../../Sicherheit/schutzziele.md) – Verfügbarkeit als Schutzziel
