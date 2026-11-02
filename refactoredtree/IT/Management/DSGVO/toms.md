# TOM – technische und organisatorische Maßnahmen

> Abgrenzung: Dieses Blatt ist das **Detailblatt zu den TOM** (Art. 32 DSGVO) mit Kontrollarten und Zuordnungsaufgaben. Den DSGVO-Überblick und das Datenschutzkonzept findest du in [DSGVO-Grundlagen und Datenschutzkonzept](dsgvo-tom.md).

## In 30 Sekunden

- **TOM** sind die Maßnahmen, mit denen personenbezogene Daten **sicher** verarbeitet werden – Pflicht nach **Art. 32 DSGVO**.
- Maßstab: **Risiko**, **Stand der Technik**, Implementierungskosten, Art und Zweck der Verarbeitung – „angemessen“, nicht „maximal“.
- **Technisch** = Technik (Verschlüsselung, MFA, Backup, Firewall), **organisatorisch** = Regeln und Abläufe (Schulung, Berechtigungskonzept, Besucherregelung, Notfallplan).
- In der Prüfung zählt die **Zuordnung zu den Kontrollarten**: Zutritt (Gebäude) – Zugang (System) – Zugriff (Daten) – Weitergabe – Eingabe – Auftrag – Verfügbarkeit – Trennung (2022W, 2024S).
- TOM müssen **regelmäßig überprüft** und angepasst werden (Art. 32 Abs. 1 lit. d).

## Kernwissen

**Was Art. 32 DSGVO ausdrücklich nennt**
- Pseudonymisierung und Verschlüsselung
- dauerhafte Vertraulichkeit, Integrität, Verfügbarkeit und **Belastbarkeit** der Systeme
- rasche **Wiederherstellbarkeit** nach einem Zwischenfall
- Verfahren zur **regelmäßigen Überprüfung** der Wirksamkeit

**Die acht Kontrollarten (prüfungsübliche Gliederung)**

| Kontrollart | Leitfrage | Technische Beispiele | Organisatorische Beispiele |
|---|---|---|---|
| **Zutrittskontrolle** | Wer kommt **in den Raum/das Gebäude**? | RFID-Karte, Biometrie an der Tür, Alarmanlage, Videoüberwachung, Sicherheitsschloss | Besucherregelung mit Begleitung, Schlüsselverwaltung, Wachdienst |
| **Zugangskontrolle** | Wer kann sich **am System anmelden**? | Passwort + **MFA**, Kontosperre, automatische Bildschirmsperre, 802.1X | Passwortrichtlinie, Verfahren für neue/ausscheidende Mitarbeiter |
| **Zugriffskontrolle** | Wer darf **welche Daten** lesen/ändern? | Rollen und ACL, Verschlüsselung von Datenträgern, Protokollierung von Zugriffen | Berechtigungskonzept nach **Least Privilege/Need-to-know**, Rezertifizierung |
| **Weitergabekontrolle** | Wie sind Daten **bei Übertragung und Transport** geschützt? | **TLS, VPN**, verschlüsselte E-Mail (S/MIME), verschlüsselte USB-Sticks | Regeln für Datenträgertransport, Versandprotokoll |
| **Eingabekontrolle** | Wer hat **was wann geändert**? | Audit-Logs, Änderungsprotokolle, Zeitstempel | Vier-Augen-Prinzip, Aufbewahrung der Protokolle |
| **Auftragskontrolle** | Halten **Dienstleister** sich an Weisungen? | – | **AVV nach Art. 28**, Auswahl nach Zertifizierungen, Kontrollen/Audits |
| **Verfügbarkeitskontrolle** | Sind Daten gegen **Verlust und Ausfall** geschützt? | Backup (3-2-1), RAID, USV, Virenschutz, Brandmelder, Klimatisierung | Backup- und Notfallkonzept, **Restore-Tests** |
| **Trennungskontrolle** | Werden Daten **zweckgetrennt** verarbeitet? | Mandantentrennung, getrennte Datenbanken, Test- und Produktivsystem getrennt | Berechtigungen je Zweck, Verbot von Echtdaten im Test |

**Zuordnungsaufgabe aus der Prüfung (2024S GA1)**

| Situation | Kontrollart |
|---|---|
| RFID-Karte für das Gebäude | **Zutrittskontrolle** |
| biometrische Anmeldung am PC | **Zugangskontrolle** |
| Berechtigungskonzept auf Dateiebene | **Zugriffskontrolle** |
| Verschlüsselung der Notebook-Festplatte | **Zugriffskontrolle** (Daten bei Diebstahl unlesbar); beim Transport von Datenträgern auch Weitergabekontrolle |
| Alarmanlage außerhalb der Geschäftszeit | **Zutrittskontrolle** |
| E-Mails per TLS übertragen | **Weitergabekontrolle** |
| Änderungsprotokoll in der Kundendatenbank | **Eingabekontrolle** |
| tägliches Backup auf ein Offline-Medium | **Verfügbarkeitskontrolle** |

**Zutrittskontrolle im Detail (Serverraum)**
- bauliche Sicherung: Stahltür, keine Fenster bzw. gesicherte Fenster, Sicherheitsschloss
- elektronische Zutrittskontrolle mit Karte oder PIN, möglichst mit **Protokollierung**
- Einbruchmeldeanlage, Türkontakte, Videoüberwachung des Eingangs (Datenschutzhinweis!)
- Besucher nur in Begleitung, Besucherbuch, Ausweispflicht
- Brandschutz und Klimatisierung gehören zur **Verfügbarkeitskontrolle**

**TOM zum Schutz von E-Mails (2024W GA1, nenne 6):** TLS-Transportverschlüsselung, S/MIME oder PGP Ende-zu-Ende, SPF/DKIM/DMARC, Spam-/Phishing-Filter mit Sandbox, Virenscanner und Anhangsfilter, MFA für Postfachzugriff, Backup/Archivierung, Awareness-Schulungen.

**Grundsätze für gute TOM:** Defense in Depth (mehrere Schichten), Least Privilege, Stand der Technik, Wirksamkeit testen (Restore-Tests, Pentests), Dokumentation als Nachweis (Rechenschaftspflicht).

## Prüfungsfrage

**Frage:** Ein Unternehmen speichert Kundendaten auf einem Server im eigenen Serverraum. Nennen Sie je eine Maßnahme zur Zutrittskontrolle, Zugangskontrolle und Zugriffskontrolle und ordnen Sie sie als technisch oder organisatorisch ein.

**Musterantwort:** Zutrittskontrolle: Der Serverraum wird mit einer elektronischen Schließanlage mit RFID-Karte und Zutrittsprotokoll gesichert – technisch; ergänzend regelt eine Besucherordnung die Begleitpflicht – organisatorisch. Zugangskontrolle: Die Anmeldung am Server erfordert ein starkes Passwort und einen zweiten Faktor (MFA) – technisch. Zugriffskontrolle: Ein Berechtigungskonzept legt fest, dass nur der Vertrieb Kundendaten lesen und ändern darf; umgesetzt wird es über AD-Gruppen und Dateisystemrechte – organisatorisch geplant, technisch umgesetzt.

## Falle

- **Zutritt, Zugang, Zugriff** verwechseln: Zutritt = Raum, Zugang = System, Zugriff = Daten.
- Nur technische Maßnahmen nennen, wenn „technisch **und** organisatorisch“ gefragt ist.
- „RAID ist unsere Backup-Lösung“ → RAID schützt nicht vor Löschen, Malware oder Brand.
- Die acht Kontrollarten als Gesetzestext ausgeben: Sie stammen aus dem alten BDSG (Anlage zu § 9 a. F.) und sind heute eine **Gliederungshilfe**; Art. 32 DSGVO formuliert die Ziele (Vertraulichkeit, Integrität, Verfügbarkeit, Belastbarkeit, Wiederherstellbarkeit, Überprüfung).

## Merksatz

**„Zutritt zum Raum, Zugang zum System, Zugriff auf die Daten.“**

## Verwandt

- [DSGVO-Grundlagen und Datenschutzkonzept](dsgvo-tom.md)
- [Auftragsverarbeitung (AVV)](auftragsverarbeitung-avv.md) – Auftragskontrolle
- [Anonymisierung und Pseudonymisierung](anonymisierung-pseudonymisierung.md)
- [Schutzziele](../../FiSi-Administrator/Sicherheit/schutzziele.md)
- [Least Privilege](../../FiSi-Administrator/Sicherheit/least-privilege.md)
- [MFA](../../FiSi-Administrator/Sicherheit/MFA.md)
- [E-Mail-Sicherheit](../../FiSi-Administrator/Sicherheit/email-sicherheit.md)
- [ISMS und IT-Grundschutz](../../FiSi-Administrator/Sicherheit/isms-bsi-grundschutz.md)
- [Sichere Datenlöschung](../../FiSi-Administrator/Sicherheit/datentraeger-entsorgung.md)
- [Lektion 05 Datenschutz und Informationssicherheit](../../analyse-der-exams/05_Datenschutz_Informationssicherheit.md)
