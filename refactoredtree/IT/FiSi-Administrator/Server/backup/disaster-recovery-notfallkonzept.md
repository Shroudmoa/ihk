# Disaster Recovery und Notfallkonzept

## In 30 Sekunden

- **Disaster Recovery (DR)** heißt: IT nach einem Großschaden wiederherstellen, z. B. nach Brand, Hochwasser, Ransomware oder Ausfall eines Rechenzentrums.
- **RTO** (Recovery Time Objective): Wie lange darf der Dienst **ausfallen**? → bestimmt, **wie schnell** wiederhergestellt wird.
- **RPO** (Recovery Point Objective): Wie viel **Datenverlust** (in Zeit) ist erlaubt? → bestimmt, **wie oft** gesichert oder repliziert wird.
- Grundlage ist die **Business-Impact-Analyse (BIA)**. Ergebnis ist das **Notfallhandbuch** mit Wiederanlaufplänen.
- HA vermeidet Ausfälle, DR stellt nach einem Ausfall wieder her. Beides wird gebraucht.
- Geprüft: RTO/RPO 2023S, 2025S (Backup-Optimierung), 2026S.

## Kernwissen

### RTO und RPO auf der Zeitachse

```
 letztes Backup        Ausfall                 Dienst wieder da
      |<----- RPO ------->|<-------- RTO --------->|
      (Daten verloren)    (Dienst nicht nutzbar)
```

| Kennzahl | Frage | Beispiel | Technische Stellschraube |
|---|---|---|---|
| **RPO** | Wie alt darf der letzte Datenstand sein? | 15 min | Backup-Intervall, Log-Backups, Replikation, CDP |
| **RTO** | Wie schnell muss der Dienst wieder laufen? | 4 h | Restore von Disk statt Band, Standby-System, Snapshots, Failover |
| **MTPD / MTA** | Ab wann ist der Schaden existenzbedrohend? | 2 Tage | RTO muss deutlich darunter liegen |

**Optimieren (2025S b):**
- **RTO senken:** Restore von Disk/SSD statt LTO, deduplizierte Disk-Backups, VM-Replikation auf einen Standby-Host, Snapshots, vorbereitete Ersatzhardware, geübte Abläufe.
- **RPO senken:** häufigere Sicherungen (stündlich statt nächtlich), Transaktionslog-Sicherung der Datenbank, synchrone oder asynchrone Replikation, Continuous Data Protection (CDP).

### Ablauf: vom Risiko zum Notfallhandbuch

1. **Business-Impact-Analyse:** Welche Geschäftsprozesse sind kritisch? Was kostet eine Stunde Ausfall? → RTO und RPO je Prozess festlegen.
2. **Risikoanalyse:** Bedrohungen (Brand, Stromausfall, Ransomware, Fehlbedienung) mit Eintrittswahrscheinlichkeit und Schaden bewerten.
3. **Strategie wählen:** Backup-Konzept, Redundanz, Ausweichstandort, Verträge (SLA, Ersatzteilservice).
4. **Notfallhandbuch erstellen:** Alarmierungskette, Verantwortliche, Wiederanlaufreihenfolge, Zugangsdaten (sicher hinterlegt), Kontaktlisten, Checklisten.
5. **Testen und üben:** Restore-Tests, Notfallübungen, danach Plan anpassen (PDCA).

Orientierung: **BSI-Standard 200-4** (Business Continuity Management) und ISO 22301.

### Wiederanlaufreihenfolge (Abhängigkeiten beachten)

Strom/Klima → Netzwerk (Core-Switch, Firewall) → Storage → Hypervisor → **DNS/DHCP/Active Directory** → Datenbanken → Fachanwendungen → Clients.
Ohne Namensauflösung und Anmeldung startet keine Fachanwendung sinnvoll.

### Ausweichstandorte

| Typ | Zustand | RTO | Kosten |
|---|---|---|---|
| **Hot Site** | voll ausgestattet, Daten laufend repliziert | Minuten | sehr hoch |
| **Warm Site** | Hardware vorhanden, Daten müssen eingespielt werden | Stunden | mittel |
| **Cold Site** | nur Räume, Strom und Netz | Tage bis Wochen | gering |
| **Cloud-DR (DRaaS)** | Replikation in die Cloud, Start bei Bedarf | Minuten bis Stunden | nutzungsabhängig |

### Rechenweg: Ausfallkosten und RPO-Verlust

- Ausfallkosten = Ausfalldauer · Kosten pro Stunde. RTO 4 h bei 2.500 €/h → bis zu **10.000 €** je Vorfall.
- Datenverlust bei nächtlichem Backup um 22 Uhr und Ausfall um 17 Uhr: **19 h** Arbeit verloren. Das RPO liegt also faktisch bei 24 h. Ist ein RPO von 1 h gefordert, reicht das nächtliche Backup nicht.

## Prüfungsfrage

**Frage:** Erläutern Sie RTO und RPO und nennen Sie je eine Maßnahme, um die Werte für den Datenbankserver eines Webshops zu verbessern.

**Musterantwort:** Das RTO ist die maximal tolerierbare Zeit vom Ausfall bis zur Wiederherstellung des Dienstes, z. B. „Shop nach 2 Stunden wieder online“. Das RPO ist der maximal tolerierbare Datenverlust, gemessen als Zeit seit der letzten Sicherung, z. B. „höchstens 15 Minuten Bestellungen verloren“. Das RTO verbessert ein Standby-Datenbankserver mit Replikation oder ein Restore von Disk statt von Band. Das RPO verbessern häufigere Sicherungen der Transaktionslogs (z. B. alle 15 Minuten) oder eine laufende Replikation.

## Falle

- RTO und RPO vertauschen. **T**ime = Ausfall**dauer**, **P**oint = Daten**stand**.
- RPO für einen Zeitpunkt halten. Es ist eine **Zeitspanne** des Datenverlusts.
- Eine HA-Lösung (Cluster) als DR verkaufen. Ein Cluster im selben Brandabschnitt brennt mit, und Ransomware wird sofort mitrepliziert.
- Ein Notfallhandbuch, das nur auf dem verschlüsselten Fileserver liegt. Es muss auch offline oder in Papierform verfügbar sein.
- Wiederanlauf ohne Abhängigkeiten planen, etwa die Anwendung vor DNS und AD.

## Merksatz

**„RPO schaut zurück (wie viele Daten weg?), RTO schaut nach vorn (wie lange bis wieder da?).“**

## Verwandt

- [Backup-Strategien](backup-strategien.md) – Backup-Arten, 3-2-1, Restore-Kette
- [Archivierung revisionssicher](archivierung-revisionssicher.md)
- [HA-Hochverfügbarkeit](../ha-hochverfuegbarkeit.md) – HA vs. DR
- [Verfügbarkeit & Kennzahlen](../verfuegbarkeit-kennzahlen.md) – MTBF, MTTR, 99,x %
- [SLA](../Cloud/sla.md) – vertragliche Verfügbarkeit
- [Schutzziele](../../Sicherheit/schutzziele.md) – Verfügbarkeit (CIA)
- [Lektion 03 Verfügbarkeit](../../../analyse-der-exams/03_Verfuegbarkeit_Ausfallsicherheit.md)
- [Lektion L12 Backup-Strategien](../../../analyse-der-exams/L12_Backup_Strategien.md)
