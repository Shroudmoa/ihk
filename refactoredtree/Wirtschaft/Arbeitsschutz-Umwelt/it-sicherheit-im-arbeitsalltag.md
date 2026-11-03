# IT-Sicherheit & Datenschutz im Arbeitsalltag

## In 30 Sekunden

- Schutzziele: **Vertraulichkeit, Integrität, Verfügbarkeit** (+ Authentizität, Verbindlichkeit). Eine **Kontrollsumme/Hash** sichert die **Integrität**.
- **Phishing** erkennen: falscher Absender hinter dem Anzeigenamen, Zeitdruck/Drohung, Link auf fremde Domain, unerwarteter Anhang, Abfrage von Zugangsdaten. Richtig: **nicht klicken, nicht weiterleiten, IT-Abteilung informieren, löschen**.
- Bekannte oder häufig besuchte Websites können **kompromittiert** sein; HTTPS heißt nur „verschlüsselt“, nicht „vertrauenswürdig“.
- **Öffentliches WLAN:** Daten können mitgelesen werden → nur mit **VPN**; Mailprogramme verschlüsseln nicht automatisch.
- Datenschutz am Arbeitsplatz: DSGVO/BDSG, **Clean Desk**, Bildschirmsperre, Least Privilege, Datenpannen sofort melden; Überwachungstechnik nur mit Betriebsrat (§ 87 Abs. 1 Nr. 6 BetrVG).

## Kernwissen

### Schutzziele

| Ziel | Bedeutung | Maßnahme | Verletzung |
|---|---|---|---|
| **Vertraulichkeit** | nur Berechtigte sehen Daten | Verschlüsselung, Zugriffsrechte, VPN | Datenleck, Mitlesen im WLAN |
| **Integrität** | Daten sind vollständig und unverändert | **Hash/Prüfsumme**, digitale Signatur, Versionierung | Manipulation |
| **Verfügbarkeit** | Systeme und Daten nutzbar, wenn benötigt | Backup, RAID, USV, Redundanz | Ausfall, Ransomware, DDoS |
| Authentizität | Absender/Quelle ist echt | Zertifikate, Signatur, MFA | gefälschte E-Mail |
| Verbindlichkeit | Handlung nicht abstreitbar | Signatur, Protokollierung | abgestrittene Bestellung |

Eine Kontrollsumme schützt **nicht** Vertraulichkeit, Verfügbarkeit, Lesbarkeit oder Verständlichkeit.

### Phishing und Social Engineering

**Merkmale einer Phishing-Mail:**
- Absenderadresse passt nicht zum Anzeigenamen; Tippfehler-Domains (paypa1.com)
- Zeitdruck oder Drohung („Konto wird in 24 Std. gesperrt“)
- Link führt auf eine fremde Domain (Mouse-over prüfen) und fordert **Zugangsdaten** an
- unerwartete Anhänge (ZIP, EXE, Office mit Makros, HTML)
- **nicht** immer schlecht gemacht: Phishing kann persönliche Anrede, Signatur, Logo und HTTPS-Seite haben

**Richtiges Verhalten:** Link nicht öffnen, keine Daten eingeben, nicht weiterleiten, nicht ausdrucken und archivieren → **IT-Abteilung/IT-Sicherheit benachrichtigen**, E-Mail löschen oder über den Melde-Button melden. Wurden Daten eingegeben: sofort Passwort ändern und melden.

**Social Engineering:** Ausnutzen von Hilfsbereitschaft, Autorität, Neugier oder Zeitdruck – z. B. CEO-Fraud (gefälschte Zahlungsanweisung vom „Chef“), falscher Support-Anruf, liegengelassener USB-Stick (Baiting), Tailgating an der Zutrittstür.

### Sicheres Surfen und mobiles Arbeiten

- Auch **bekannte** und vertrauenswürdige Seiten können manipuliert sein (Drive-by-Download, Malvertising) → Updates, Browser-Schutz, Werbeblocker.
- **Öffentliches WLAN:** Betreiber kümmern sich oft nicht um Sicherheit, Daten können mitgelesen werden (Man-in-the-Middle, Evil Twin) → **Firmen-VPN** nutzen, automatische WLAN-Verbindung deaktivieren, kein Login ohne VPN; die Eingabe der E-Mail-Adresse im Hotspot-Portal macht das WLAN nicht sicher.
- **Homeoffice:** Bildschirm vor Blicken schützen, Gerät sperren, keine Familienmitglieder an Firmengeräten, Ausdrucke sicher entsorgen.

### Passwörter und Anmeldung

- Lange Passphrasen (≥ 12–16 Zeichen), für jeden Dienst einzigartig, **Passwortmanager**.
- **MFA/2FA** (Wissen + Besitz/Biometrie) für alle wichtigen Zugänge.
- Passwörter nie per Mail/Telefon herausgeben – die echte IT fragt nicht danach.
- Regelmäßiges Zwangswechseln empfiehlt das BSI nicht mehr pauschal – nur bei Verdacht auf Kompromittierung.

### Malware, Updates, Backup

- Updates und Patches zeitnah installieren; Software nur aus freigegebenen Quellen; Makros deaktiviert lassen.
- **Ransomware:** Gerät vom Netz trennen, **nicht zahlen**, IT informieren.
- **3-2-1-Backup:** 3 Kopien, 2 Medien, 1 Kopie extern/offline; Wiederherstellung regelmäßig testen.

### Datenschutz am Arbeitsplatz (DSGVO/BDSG)

| Regel | Inhalt |
|---|---|
| Grundsätze (Art. 5 DSGVO) | Rechtmäßigkeit, Zweckbindung, **Datenminimierung**, Richtigkeit, Speicherbegrenzung, Integrität/Vertraulichkeit |
| Beschäftigtendaten (§ 26 BDSG) | nur verarbeiten, was für das Arbeitsverhältnis erforderlich ist; Bewerberdaten nach Absage löschen (ca. 6 Monate wegen AGG-Fristen) |
| Datenschutzbeauftragter | Pflicht ab **20 Personen**, die ständig personenbezogene Daten automatisiert verarbeiten (§ 38 BDSG) |
| Datenpanne | intern sofort melden; Meldung an die Aufsichtsbehörde binnen **72 Stunden** (Art. 33 DSGVO) |
| Überwachung | Videoüberwachung, Mitarbeiter-Tracking, Software mit Leistungskontrolle → **Mitbestimmung des Betriebsrats** und Datenschutzprüfung |
| Alltag | Clean Desk, Bildschirmsperre (Win + L), keine Kundendaten auf privaten Sticks/Mail, Akten schreddern, **Verschwiegenheit** über Betriebs- und Kundendaten |

## Prüfungsfrage

**Frage (Multiple Choice):** Ein Mitarbeiter erhält eine E-Mail seiner „Bank“ mit der Aufforderung, über einen Link seine Zugangsdaten zu bestätigen. Wie sollte er reagieren?

1. den Link öffnen und die Daten eingeben, um eine Sperrung zu vermeiden
2. das Mailprogramm beenden und den Rechner neu starten
3. die E-Mail an alle Kollegen weiterleiten, um sie zu warnen
4. die E-Mail löschen und die IT-Abteilung benachrichtigen
5. die E-Mail ausdrucken und archivieren

**Musterantwort:** **4.** Es handelt sich um einen Phishing-Versuch. Die IT kann andere Empfänger schützen und den Absender sperren; Weiterleiten verbreitet die Gefahr, Neustart oder Ausdrucken lösen nichts.

## Falle

- HTTPS-Schloss ≠ seriöse Seite – auch Phishing-Seiten haben Zertifikate.
- Phishing-Mails haben nicht immer Fehler; sie können eine korrekte Anrede und Signatur haben.
- Hash/Kontrollsumme → **Integrität**, nicht Vertraulichkeit.
- Öffentliches WLAN ohne VPN ist unsicher, auch mit Passwort im Portal.
- Sicherheitsvorfälle **melden**, nicht selbst „lösen“ oder verschweigen.

## Merksatz

**„Erst denken, dann klicken – im Zweifel die IT fragen.“**

## Verwandt

- [Urheberrecht & Compliance](../Recht/urheberrecht-compliance.md) – Regeln im Betrieb
- [Betriebsrat & JAV](../Mitbestimmung/betriebsrat-jav.md) – Mitbestimmung bei Überwachungstechnik
- [Arbeitsschutz & Arbeitssicherheit](arbeitsschutz-arbeitssicherheit.md) – Bildschirmarbeit
- [Schutzziele (IT)](../../IT/FiSi-Administrator/Sicherheit/schutzziele.md)
- [DSGVO & TOM (IT)](../../IT/Management/DSGVO/dsgvo-tom.md)
- [MFA & Authentifizierung (IT)](../../IT/FiSi-Administrator/Server/authentifizierung-autoritisierung-mfa.md)
- [Backup & Snapshots (IT)](../../IT/FiSi-Administrator/Server/virtualisierung/snapshots-backups.md)
- [Lektion 05 – Datenschutz & Informationssicherheit](../../IT/analyse-der-exams/05_Datenschutz_Informationssicherheit.md)
- Echte Aufgaben: [Winter 2021](../fragen/Winter-2021.md) (25–30)
