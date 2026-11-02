# Monitoring (Server- und Netzwerküberwachung)

> **Abgrenzung:** Dieses Blatt behandelt **was und wie** überwacht wird (Messgrößen, Schwellwerte, Engpässe). Die Protokolle im Detail (SNMP-Versionen, Syslog-Schweregrade, NTP) → [SNMP, Monitoring, Syslog & NTP](../Netzwerk-Administration/snmp-monitoring-syslog-ntp.md).

## In 30 Sekunden

- Monitoring misst laufend Zustand und Leistung (CPU, RAM, Datenträger, Netz, Dienste, Temperatur, USV, Backups) und **alarmiert**, wenn ein **Schwellenwert** überschritten wird.
- **Polling** heißt: Der Manager fragt regelmäßig ab (SNMP Get). Bei **Push/Trap** meldet der Agent ein Ereignis sofort selbst.
- **SNMP:** UDP **161** (Abfrage) und **162** (Traps). Werte stehen in der MIB und werden über eine **OID** adressiert. v1/v2c schicken den Community-String im Klartext, **v3** bietet Authentifizierung und Verschlüsselung.
- **Logs** zentral sammeln (Syslog UDP 514, Windows-Ereignisanzeige) → Auswertung im SIEM.
- Bottleneck-Analyse: Die Ressource, die am Limit läuft, bremst das System (2025W: 91 % RAM belegt).
- Geprüft: SNMP GA2 2021W, Ressourcenmonitor/Bottleneck GA1 2025W, CPU-/RAM-Überwachungsprogramme GA1 2022W, 2023S, 2023W.

## Kernwissen

### Was wird überwacht?

| Bereich | Messwert | typische Warnschwelle |
|---|---|---|
| CPU | Auslastung in %, Warteschlange | > 80 % über 15 min |
| RAM | belegter Speicher, Auslagerung (Paging) | > 85–90 %, steigende Pagefile-Nutzung |
| Datenträger | freier Platz, IOPS, Latenz, **SMART**-Werte | < 15 % frei, reallokierte Sektoren > 0 |
| Netzwerk | Durchsatz, Fehler, Paketverlust, Latenz, Interface-Status | Fehlerzähler steigt, Link down |
| Dienste | Prozess läuft? Port antwortet? HTTP 200? | Dienst gestoppt |
| Hardware | Temperatur, Lüfter, Netzteil, RAID-Status | RAID „degraded“ |
| Infrastruktur | USV-Ladezustand, Klima, Zutritt | Akku < 50 % |
| Betrieb | Backup-Jobs, Zertifikatsablauf, Updates | Backup fehlgeschlagen, Zertifikat < 30 Tage gültig |

### Polling vs. Push

| | Polling (SNMP Get, Ping-Check) | Push (SNMP Trap/Inform, Agent meldet) |
|---|---|---|
| Ablauf | Manager fragt im Intervall jeden Wert ab | Gerät sendet bei Ereignis sofort |
| Vorteil | einfach, regelmäßige Werte für Graphen | sofortige Meldung, wenig Last |
| Nachteil | Netzlast bei vielen Geräten, **Ereignisse zwischen zwei Abfragen bleiben unbemerkt** | Trap per UDP kann verloren gehen (Inform wird bestätigt) |

### SNMP-Bausteine

| Begriff | Bedeutung |
|---|---|
| **Manager** | Monitoring-Server (z. B. Zabbix, PRTG, Checkmk) |
| **Agent** | Dienst auf dem überwachten Gerät |
| **MIB** | Management Information Base, die Beschreibung aller abfragbaren Werte |
| **OID** | Object Identifier, eindeutige Adresse eines Werts, z. B. `1.3.6.1.2.1.1.3.0` = sysUpTime |
| **Community** | „Passwort“ bei v1/v2c, Standard `public` sofort ändern |
| **Befehle** | `snmpget` (ein Wert), `snmpwalk` (Teilbaum), `snmpset` (Wert setzen) |

### Logs und Werkzeuge

- **Linux:** `top`/`htop`, `free -h`, `df -h`, `iostat`, `journalctl`, `/var/log/`
- **Windows:** Task-Manager, Ressourcenmonitor, Leistungsüberwachung (perfmon), Ereignisanzeige
- **Zentral:** Syslog-Server, SIEM (Korrelation sicherheitsrelevanter Ereignisse), Dashboards (Grafana)
- **Baseline:** Normalwerte erfassen. Eine Abweichung von der Baseline ist oft aussagekräftiger als ein fester Grenzwert.

### Bottleneck-Analyse (2025W)

1. Auslastung aller Ressourcen vergleichen: CPU, RAM, Datenträger, Netz.
2. Die Ressource nahe 100 % ist der Engpass. Beispiel: RAM zu 91 % belegt, das System lagert aus, die Datenträgerlast steigt.
3. Maßnahmen: Ressource erweitern (RAM aufrüsten, Scale-up), Last reduzieren (unnötige Prozesse und Dienste beenden, Speicherleck beheben) oder Last verteilen (Scale-out).

## Prüfungsfrage

**Frage:** Die Auslastung der Server wird mit `snmpget` überwacht. Nennen Sie einen Nachteil dieses Verfahrens und eine Möglichkeit, ihn zu umgehen.

**Musterantwort:** `snmpget` arbeitet per Polling: Der Monitoring-Server fragt jeden Wert in festen Abständen einzeln ab. Kurzzeitige Ereignisse zwischen zwei Abfragen, etwa eine Lastspitze oder ein kurzer Ausfall, bleiben unbemerkt. Außerdem erzeugen viele Abfragen Netzlast. Mit **SNMP-Traps** sendet der Agent beim Überschreiten eines Schwellenwerts sofort selbst eine Meldung an den Manager (ereignisgesteuert).

## Falle

- SNMP v1/v2c als sicher bezeichnen. Der Community-String geht im Klartext übers Netz, für Sicherheit braucht es **v3**.
- Ports vertauschen: 161 = Abfrage am Agent, 162 = Trap-Empfang am Manager.
- Nur alarmieren, ohne Verantwortliche und Eskalation festzulegen. Dann liest niemand den Alarm.
- Zu viele Alarme (Alarmmüdigkeit): Schwellen sinnvoll setzen, Warnung und Kritisch trennen.
- Monitoring-Server ohne eigene Überwachung. Fällt er aus, bleibt es still („Watch the watcher“).

## Merksatz

**„Polling fragt, Trap meldet. 161 fragt, 162 hört.“**

## Verwandt

- [SNMP, Monitoring, Syslog & NTP](../Netzwerk-Administration/snmp-monitoring-syslog-ntp.md) – die Protokolle im Detail
- [Verfügbarkeit & Kennzahlen](verfuegbarkeit-kennzahlen.md) – SMART, Badewannenkurve
- [Skripte: PowerShell, Bash, CMD](skripte-powershell-bash-cmd.md) – Überwachung per Skript und Aufgabenplanung
- [Linux-Grundlagen](linux-grundlagen.md) – Befehle zur Systemanalyse
- [Server-Hardware](../../Komponenten/server-hardware.md) – Engpässe und Serverauswahl
- [AAA & SIEM](../Sicherheit/aaa-siem.md) – Logauswertung
- [Netzwerkanalyse & Troubleshooting](../Netzwerk-Administration/netzwerkanalyse-und-troubleshooting.md)
- [Ports](../OSI-IP-mac-bit-byte/ports.md)
- [Lektion L29 GA2 Praxisthemen](../../analyse-der-exams/L29_GA2_Sonstiges_Praxisthemen.md) – SNMP
