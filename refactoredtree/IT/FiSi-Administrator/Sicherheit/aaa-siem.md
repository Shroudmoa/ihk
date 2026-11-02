# AAA und SIEM

## In 30 Sekunden

- **AAA** steht für **Authentication** (Wer bist du?), **Authorization** (Was darfst du?) und **Accounting** (Was hast du wann getan?).
- Zentrale AAA-Server: **RADIUS** (WLAN, VPN, 802.1X) und **TACACS+** (Administration von Netzwerkgeräten), meist mit **Active Directory/LDAP** als Benutzerquelle.
- **SIEM** (Security Information and Event Management) sammelt Logs aller Systeme zentral, **korreliert** sie und schlägt bei verdächtigen Mustern Alarm.
- Prüfung: RADIUS-AAA-Komponenten und Vorteile gegenüber PSK (2021W, 2022S GA2), Nachvollziehbarkeit von Änderungen per Audit-Logging (2025W GA1).
- Logs enthalten personenbezogene Daten → **Zweckbindung, Zugriffsschutz und Löschfristen** nach DSGVO beachten.

## Kernwissen

**AAA im Überblick**

| Komponente | Frage | Beispiele |
|---|---|---|
| **Authentication** | Wer bist du? | Passwort + MFA, Zertifikat (EAP-TLS), Kerberos-Ticket |
| **Authorization** | Was darfst du? | VLAN-Zuweisung, AD-Gruppen, Berechtigungsstufen auf Switch/Router |
| **Accounting** | Was hast du getan? | Sitzungsbeginn/-ende, übertragene Datenmenge, ausgeführte Befehle, Windows-Ereignisse 4624/4625 (An-/Fehlanmeldung) |

**RADIUS vs. TACACS+**

| Merkmal | RADIUS | TACACS+ |
|---|---|---|
| Transport | **UDP 1812/1813** | **TCP 49** |
| Verschlüsselung | nur das Passwort-Attribut verschleiert | gesamter Nachrichteninhalt verschlüsselt (Shared Secret) |
| AAA-Trennung | Authentifizierung und Autorisierung gekoppelt | alle drei getrennt konfigurierbar |
| Typischer Einsatz | Netzzugang: WLAN-Enterprise, 802.1X, VPN | Geräteadministration: Befehle je Admin erlauben und protokollieren |
| Produkte | Microsoft NPS, FreeRADIUS, Cisco ISE | Cisco ISE, tac_plus |

**SIEM – vom Log zum Alarm**
1. **Sammeln:** Logs von Servern, Clients, Firewalls, Switches, AAA-Servern, Anwendungen (Syslog, Windows Event Forwarding, Agenten)
2. **Normalisieren:** einheitliches Format, Zeit synchronisiert (NTP)
3. **Anreichern:** Benutzer, Gruppen, Standorte, Schwachstellendaten zuordnen
4. **Korrelieren:** Regeln verknüpfen Ereignisse verschiedener Quellen
5. **Alarmieren und auswerten:** Tickets, Bereitschaft, ggf. Security Operations Center (SOC)
6. **Archivieren:** manipulationsgeschützt (z. B. WORM-Speicher), mit festgelegter Aufbewahrungsfrist

**Beispiel-Korrelationsregeln**
- mehr als 5 Fehlanmeldungen in 5 Minuten, danach Erfolg → möglicher Brute-Force-Erfolg
- Anmeldung aus zwei weit entfernten Ländern innerhalb kurzer Zeit („unmögliche Reise“)
- neues Konto in der Gruppe „Domänen-Admins“ außerhalb eines Change-Fensters
- massenhaftes Umbenennen von Dateien auf einem Fileserver → Verdacht auf Ransomware

**Datenschutz bei Logs:** Protokolle enthalten Benutzernamen und IP-Adressen. Die DSGVO schreibt **keine feste Aufbewahrungsdauer** vor – das Unternehmen legt eine Frist nach Zweck fest (Sicherheitslogs oft wenige Monate, gesetzliche Aufbewahrungspflichten können länger sein), beschränkt den Zugriff (Least Privilege, Vier-Augen-Prinzip) und beteiligt bei Mitarbeiterüberwachung den Betriebsrat (§ 87 Abs. 1 Nr. 6 BetrVG).

## Prüfungsfrage

**Frage:** Im Firmen-WLAN soll statt eines gemeinsamen Schlüssels ein RADIUS-Server eingesetzt werden. Erläutern Sie die drei A des AAA-Prinzips an diesem Beispiel.

**Musterantwort:** Bei der Authentifizierung prüft der RADIUS-Server, ob der Benutzer mit seinen eigenen Zugangsdaten oder seinem Zertifikat tatsächlich der ist, der er vorgibt zu sein, zum Beispiel gegen das Active Directory. Bei der Autorisierung legt er fest, was der Benutzer darf, etwa welches VLAN ihm zugewiesen wird. Beim Accounting protokolliert er, wann sich wer wie lange verbunden hat, sodass Zugriffe nachvollziehbar sind. Gegenüber einem gemeinsamen Schlüssel kann so jeder Zugang einzeln gesperrt und zugeordnet werden.

## Falle

- Accounting mit „Abrechnung“ übersetzen und nur an Kosten denken → gemeint ist vor allem die **Protokollierung**.
- RADIUS und TACACS+ Ports vertauschen: RADIUS UDP 1812/1813, TACACS+ TCP 49.
- „SIEM verhindert Angriffe“ → es **erkennt und meldet**; blockieren müssen andere Systeme oder Menschen.
- „DSGVO verlangt 12 Monate Log-Aufbewahrung“ → falsch, die DSGVO verlangt **Speicherbegrenzung**; die Frist legt das Unternehmen begründet fest.

## Merksatz

**„Wer bist du – was darfst du – was hast du getan.“**

## Verwandt

- [802.1X, RADIUS und Port-Security](netzwerkzugang-802-1x-radius.md)
- [Zugriffskontrollmodelle](../../nice-to-know/access-control.md)
- [Active Directory](active-directory.md)
- [IDS und IPS](firewall/ids-ips.md)
- [Sicherheitsvorfall – Incident Response](hacking/wurde-ich-gehackt.md)
- [Kerberos](../Netzwerk-Administration/kerberos.md) und [LDAP](../Netzwerk-Administration/ldap.md)
- [Lektion 29 GA2-Praxisthemen (802.1X und RADIUS)](../../analyse-der-exams/L29_GA2_Sonstiges_Praxisthemen.md)
