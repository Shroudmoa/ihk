# hackers-can

## IT-Sicherheitsbedrohungen (IHK)

| Angriff | Definition | Schutzmaßnahmen |
|---|---|---|
| DoS | Ein Angreifer überlastet einen Dienst, sodass legitime Anfragen nicht mehr verarbeitet werden können. | Rate Limiting, Firewall, Ressourcenlimits |
| DDoS | Viele kompromittierte Systeme überlasten gleichzeitig einen Dienst. | DDoS-Schutz, CDN, Traffic-Filterung |
| Brute Force | Zugangsdaten werden durch zahlreiche systematische Versuche erraten. | MFA, Rate Limiting, Kontosperre |
| Phishing | Gefälschte Nachrichten oder Webseiten sollen Benutzer zur Preisgabe sensibler Daten verleiten. | Mitarbeiterschulungen, MFA, E-Mail-Filter |
| Man-in-the-Middle (MitM) | Ein Angreifer fängt Kommunikation zwischen zwei Parteien ab und kann sie manipulieren. | TLS, Zertifikatsprüfung, VPN in geeigneten Szenarien |
| SQL Injection | Unsichere Eingaben verändern die Struktur einer SQL-Abfrage. | Prepared Statements, Eingabevalidierung, minimale DB-Rechte |
| Cross-Site Scripting (XSS) | Schadcode wird in Webseiten eingeschleust und im Browser anderer Benutzer ausgeführt. | Output Encoding, sichere Frameworks, CSP |
| Malware | Sammelbegriff für Schadsoftware, z. B. Trojaner, Würmer und Ransomware. | Virenschutz/EDR, Updates, Backups, Least Privilege |
| ARP Spoofing / Poisoning | Gefälschte ARP-Nachrichten manipulieren die Zuordnung von IPv4-Adressen zu MAC-Adressen. | Dynamic ARP Inspection, DHCP Snooping, Netzsegmentierung |
| Replay Attack | Eine aufgezeichnete, gültige Nachricht wird erneut übertragen, um eine Aktion zu wiederholen. | Nonces, Zeitstempel, Sequenznummern, Replay-Schutz |
| Webshell | Ein eingeschleustes Skript ermöglicht die Ausführung von Befehlen auf einem Webserver. | Sichere Uploads, Dateirechte, Patchmanagement, Webserver-Härtung |

## IHK-Prüfungswissen

### 1. DoS vs. DDoS
- **DoS:** Überlastungsangriff aus einer einzelnen Quelle.
- **DDoS:** Verteilte Quellen greifen gleichzeitig an, häufig über ein Botnet.
- **Schutzziel:** Verfügbarkeit.

### 2. ARP Spoofing
- ARP ordnet im lokalen IPv4-Netz IP-Adressen den MAC-Adressen zu.
- Der Angreifer sendet gefälschte ARP-Nachrichten, z. B. mit der Behauptung, die MAC-Adresse des Gateways zu besitzen.
- Daten können dadurch über den Angreifer laufen; ein MitM-Angriff wird möglich.

**Wichtig:** Zwei IP-Adressen können auf dieselbe MAC-Adresse zeigen. Das allein beweist keinen Angriff, da legitime Konfigurationen ebenfalls vorkommen.

**OSI:** Schicht 2, Sicherungsschicht.  
**Protokoll:** ARP, kein TCP-/UDP-Port.  
**Gerät:** Switch; Schutz beispielsweise durch Dynamic ARP Inspection.

### 3. Replay Attack
- Eine gültige Nachricht wird aufgezeichnet und erneut gesendet.
- Der Empfänger erkennt ohne geeignete Schutzmechanismen möglicherweise nicht, dass die Nachricht bereits verarbeitet wurde.
- Nonces, Zeitstempel und Sequenznummern ermöglichen die Erkennung oder Ablehnung wiederholter Nachrichten.

**Abgrenzung:** MitM beschreibt das Abfangen oder Manipulieren der Kommunikation; Replay beschreibt das erneute Senden aufgezeichneter Nachrichten.

### 4. Webshell
- Eine Webshell ist ein eingeschleustes Skript auf einem Webserver.
- Sie ermöglicht abhängig von den Rechten des Webserverprozesses die Remote-Ausführung von Befehlen.
- Schutz: Upload-Verzeichnisse ohne Skriptausführung, minimale Dateisystemrechte, Eingabevalidierung und aktuelle Software.

**Risiko:** Vertraulichkeit, Integrität und Verfügbarkeit können betroffen sein.

### 5. Zuordnung zu IHK-Themen

| Thema | Relevanter Zusammenhang |
|---|---|
| OSI-Modell | ARP: Schicht 2; IP: Schicht 3; TCP/UDP: Schicht 4; HTTP(S): Schicht 7 |
| Ports | HTTPS: TCP 443; SSH: TCP 22; DNS: UDP/TCP 53 |
| DSGVO Art. 32 | Angemessene technische und organisatorische Maßnahmen (TOM) |
| CIA-Schutzziele | Vertraulichkeit, Integrität, Verfügbarkeit |
| Authentizität | Zertifikate, digitale Signaturen und sichere Authentifizierung |
| Backup | Unterstützt die Wiederherstellung nach Malware- oder Ransomware-Angriffen |

### Merksätze
- DoS/DDoS → Verfügbarkeit.
- SQL Injection → Datenbankabfragen.
- XSS → Browser des Opfers.
- ARP Spoofing → falsche IP-MAC-Zuordnung im lokalen IPv4-Netz.
- Replay → gültige Nachricht erneut senden.
- Webshell → Befehlsausführung über einen kompromittierten Webserver.
