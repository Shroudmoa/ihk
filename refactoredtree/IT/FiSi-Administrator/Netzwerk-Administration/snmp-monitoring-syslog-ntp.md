# SNMP, Monitoring, Syslog & NTP

## In 30 Sekunden

- **SNMP** fragt Werte von Netzwerkgeräten ab (CPU, Temperatur, Port-Status, Traffic): **Manager ↔ Agent**, Werte adressiert über **OIDs** aus der **MIB**; Ports **UDP 161** (Abfrage) und **UDP 162** (Traps).
- **Polling** (`snmpget`) fragt periodisch – Ereignisse dazwischen gehen verloren; **Traps/Informs** meldet der Agent **sofort selbst** (GA2 Winter 2021).
- **SNMPv1/v2c** nutzen eine **Community** im Klartext; **SNMPv3** bietet **Authentifizierung und Verschlüsselung** pro Benutzer.
- **Syslog** (UDP 514, TLS: TCP 6514) sammelt Logmeldungen zentral, 8 Schweregrade **0 Emergency – 7 Debug**.
- **NTP** (UDP 123) synchronisiert die Zeit – Voraussetzung für **Kerberos** (max. 5 min Abweichung), korrelierbare Logs und gültige Zertifikate.

## Kernwissen

### SNMP-Begriffe

| Begriff | Bedeutung |
|---|---|
| **Manager** (NMS) | Monitoring-Server, der abfragt und Traps empfängt (PRTG, Zabbix, Checkmk, LibreNMS) |
| **Agent** | Dienst auf Switch, Router, Server, USV, Drucker |
| **MIB** | Management Information Base – „Wörterbuch“ der abfragbaren Objekte |
| **OID** | Object Identifier, Pfad im MIB-Baum, z. B. `1.3.6.1.2.1.1.3.0` |
| **Community** | v1/v2c-„Passwort“ (`public` = lesen, `private` = schreiben) – unbedingt ändern |

**Bekannte OIDs (System-Gruppe `1.3.6.1.2.1.1`)**
| OID | Objekt | Datentyp | Beispielwert |
|---|---|---|---|
| `1.3.6.1.2.1.1.1.0` | sysDescr | OCTET STRING | „Cisco IOS Software …“ |
| `1.3.6.1.2.1.1.3.0` | sysUpTime | **TimeTicks** (1/100 s) | 123456789 |
| `1.3.6.1.2.1.1.5.0` | sysName | OCTET STRING | „core-sw01“ |
| `1.3.6.1.2.1.2.2.1.8.x` | ifOperStatus | INTEGER | 1 = up, 2 = down |
| `1.3.6.1.2.1.2.2.1.10.x` | ifInOctets | **Counter32** | empfangene Bytes |

**Datentypen:** INTEGER, OCTET STRING, **Counter** (steigt nur, läuft über), **Gauge** (aktueller Wert, z. B. Temperatur, CPU %), TimeTicks, IpAddress.

### Operationen

| Operation | Richtung | Zweck |
|---|---|---|
| Get | Manager → Agent | einzelnen Wert lesen (`snmpget`) |
| GetNext / GetBulk | Manager → Agent | Baum durchlaufen (`snmpwalk`) |
| Set | Manager → Agent | Wert schreiben (z. B. Port abschalten) |
| **Trap** | Agent → Manager (UDP 162) | ungefragte Ereignismeldung, **ohne** Bestätigung |
| **Inform** | Agent → Manager | wie Trap, aber **mit** Bestätigung (v2c/v3) |

```
snmpget -v2c -c Geheim123 192.168.66.10 1.3.6.1.2.1.1.3.0
snmpwalk -v3 -l authPriv -u monitor -a SHA -A <auth> -x AES -X <priv> 192.168.66.10 1.3.6.1.2.1.2
```

### Versionen

| Version | Sicherheit | Bewertung |
|---|---|---|
| v1 | Community im Klartext | veraltet |
| v2c | Community im Klartext, GetBulk, Informs | verbreitet, nur im Management-VLAN |
| **v3** | **USM**: Benutzer, Authentifizierung (SHA), Verschlüsselung (AES) – Stufen noAuthNoPriv / authNoPriv / **authPriv** | empfohlen |

### Polling vs. Trap (GA2 Winter 2021)

| | Polling (`snmpget`) | Trap |
|---|---|---|
| Auslöser | Manager fragt im Intervall (z. B. 5 min) | Agent meldet bei Ereignis |
| Nachteil | Spitzen/Kurzausfälle zwischen zwei Abfragen bleiben unentdeckt; Last durch viele Abfragen | UDP ohne Bestätigung – Trap kann verloren gehen (→ Inform) |
| Vorteil | Verlaufsdaten, Trends, Graphen | sofortige Alarmierung |
Am besten **kombinieren**: Polling für Trends, Traps für Alarme.

### Monitoring-Konzept

- **Was:** Erreichbarkeit (ICMP), Dienste (Port-/HTTP-Checks), Ressourcen (CPU, RAM, Disk), Interfaces (Auslastung, Fehler), Umgebung (Temperatur, USV), Zertifikatslaufzeiten.
- **Schwellwerte** (Warnung/kritisch), **Alarmierung** (E-Mail, SMS, Ticket), **Eskalation**, **Dashboards**.
- **Flow-Daten** (NetFlow, sFlow, IPFIX): *wer spricht mit wem, wie viel* – für Kapazitätsplanung und Anomalien.
- Monitoring-Verkehr in ein **Management-VLAN**, Zugriff per ACL beschränken.

### Syslog

| Schweregrad | Name | Beispiel |
|---:|---|---|
| 0 | Emergency | System unbenutzbar |
| 1 | Alert | sofort handeln |
| 2 | Critical | Hardwaredefekt |
| 3 | Error | Fehler |
| 4 | Warning | Schwellwert erreicht |
| 5 | Notice | normal, aber bemerkenswert (Link up/down) |
| 6 | Informational | Info |
| 7 | Debug | Fehlersuche |

Zentraler Syslog-Server (rsyslog, Graylog, ELK) bzw. **SIEM**: Logs manipulationssicher speichern, korrelieren, Alarme auslösen. Klassisch **UDP 514** (unzuverlässig, unverschlüsselt), sicher über **TCP 6514 mit TLS**. **DSGVO:** Logs enthalten oft personenbezogene Daten (IP, Benutzer) → Zweckbindung, Zugriffsschutz, Löschfristen.

### NTP

| Begriff | Bedeutung |
|---|---|
| **Stratum 0** | Referenzuhr (GPS, DCF77, Atomuhr) |
| **Stratum 1** | Server direkt an der Referenzuhr |
| Stratum 2–15 | jeweils eine Ebene weiter weg; 16 = nicht synchronisiert |
| Port | **UDP 123** |
| AD | Domänencontroller mit PDC-Emulator-Rolle ist Zeitquelle der Domäne |

**Warum Zeit wichtig ist:** Kerberos lehnt Tickets bei mehr als **5 Minuten** Abweichung ab (Anmeldung scheitert), Logs mehrerer Geräte lassen sich nur mit gleicher Zeit **korrelieren** (Forensik), Zertifikate und TOTP-Codes sind zeitabhängig, Backups und Cronjobs laufen zur richtigen Zeit.

## Prüfungsfrage

**Frage (angelehnt an GA2 Winter 2021):** Das Monitoring fragt alle 10 Minuten per `snmpget` die Temperatur der Switches ab. Ein Switch fiel wegen Überhitzung aus, ohne dass vorher ein Alarm kam. Erläutern Sie den Nachteil des Verfahrens, wie er umgangen werden kann, und warum SNMPv3 eingesetzt werden sollte.

**Musterantwort:** `snmpget` ist **Polling**: Der Manager fragt nur im festen Intervall. Steigt die Temperatur zwischen zwei Abfragen schnell an, bleibt das unentdeckt; zudem erzeugen viele Abfragen Last. Abhilfe: Auf dem Switch einen Schwellwert konfigurieren, bei dessen Überschreitung der Agent sofort einen **SNMP-Trap** (bzw. ein bestätigtes **Inform**) an den Manager auf UDP 162 sendet; zusätzlich kann das Intervall verkürzt werden. **SNMPv3** sollte eingesetzt werden, weil v1/v2c die Community im Klartext übertragen – wer sie mitliest, kann Geräte auslesen oder per Set umkonfigurieren. v3 authentifiziert Benutzer und verschlüsselt die Kommunikation.

## Falle

- Traps an Port 161 → Traps gehen an **162**, Abfragen an 161.
- SNMP als TCP-Protokoll → **UDP**.
- „Community = sicheres Passwort“ → bei v1/v2c **Klartext**.
- Syslog-Schweregrade falsch herum: **0 ist das Schlimmste**, 7 Debug.
- NTP nur als Komfortfunktion sehen → ohne Zeit **keine Kerberos-Anmeldung** und keine verwertbaren Logs.

## Merksatz

**„SNMP fragt (161) oder wird angerufen (162), v3 flüstert verschlüsselt; Syslog zählt von 0 (Panik) bis 7 (Plaudern); ohne NTP kein Kerberos.“**

## Verwandt

- [Netzwerkanalyse & Troubleshooting](netzwerkanalyse-und-troubleshooting.md) – Monitoring als Frühwarnsystem
- [Ports](../OSI-IP-mac-bit-byte/ports.md) – 161/162, 514, 123
- [AAA & SIEM](../Sicherheit/aaa-siem.md) – zentrale Log-Auswertung
- [Kerberos](kerberos.md) – Zeitabhängigkeit der Tickets
- [Switches – Grundlagen](switches-und-vlans.md) – Management-VLAN
- [USV](../Server/usv.md) – USV per SNMP überwachen
- [Lektion 29 – GA2 Sonstiges (SNMP)](../../analyse-der-exams/L29_GA2_Sonstiges_Praxisthemen.md)
