# Auftragsverarbeitung und AVV (Art. 28 DSGVO)

## In 30 Sekunden

- **Auftragsverarbeitung** liegt vor, wenn ein Dienstleister personenbezogene Daten **im Auftrag und nach Weisung** des Verantwortlichen verarbeitet – z. B. Cloud-Hosting, Rechenzentrum, Newsletter-Versand, Fernwartung, Aktenvernichtung.
- Dafür ist ein **Auftragsverarbeitungsvertrag (AVV)** nach **Art. 28 Abs. 3 DSGVO** Pflicht, schriftlich oder elektronisch.
- Der **Auftraggeber bleibt verantwortlich**: Er muss einen Dienstleister mit **ausreichenden Garantien** (TOM) auswählen und ihn kontrollieren.
- Bei Verarbeitung **außerhalb der EU/des EWR** zusätzlich Drittlandregeln: Angemessenheitsbeschluss oder **SCC** plus Prüfung des Schutzniveaus.
- Prüfung 2024S und 2024W (Cloud-Migration, Mailserver in der Cloud), 2026S (Cloud + Datenschutz).

## Kernwissen

**Pflichtinhalte des AVV (Art. 28 Abs. 3)**

| Inhalt | Bedeutung |
|---|---|
| Gegenstand und Dauer | was wird wie lange verarbeitet |
| Art und Zweck | z. B. Hosting der Kundendatenbank |
| Art der Daten, Kategorien Betroffener | z. B. Kontaktdaten von Kunden, Personaldaten |
| **Weisungsgebundenheit** | Verarbeitung nur auf dokumentierte Weisung des Auftraggebers |
| **Vertraulichkeit** | Mitarbeiter des Dienstleisters sind zur Verschwiegenheit verpflichtet |
| **TOM** (Art. 32) | konkret beschriebene Sicherheitsmaßnahmen, meist als Anlage |
| **Unterauftragsverarbeiter** | nur mit vorheriger Genehmigung; gleiche Pflichten weitergeben |
| Unterstützung | bei Betroffenenrechten, Datenpannen, Datenschutz-Folgenabschätzung |
| **Löschung oder Rückgabe** | nach Auftragsende, inkl. Kopien |
| **Nachweise und Kontrollen** | Auskunft, Audits, Inspektionen – auch über Zertifikate (ISO 27001, BSI C5) |
| Meldung von Datenpannen | Dienstleister informiert den Verantwortlichen unverzüglich |

**Abgrenzung: Wann ist es keine Auftragsverarbeitung?**

| Fall | Einordnung |
|---|---|
| Cloud-Speicher, Hosting, externes Rechenzentrum | Auftragsverarbeitung |
| Newsletter-Dienstleister, externes Callcenter | Auftragsverarbeitung |
| IT-Dienstleister mit Fernwartungszugriff auf Systeme mit personenbezogenen Daten | Auftragsverarbeitung |
| Aktenvernichter/Datenträgerentsorger | Auftragsverarbeitung |
| Steuerberater, Rechtsanwalt, Wirtschaftsprüfer | **eigene Verantwortung** (Berufsgeheimnisträger, weisungsfrei) |
| Bank, Post, Paketdienst beim reinen Transport | eigene Verantwortung bzw. keine Auftragsverarbeitung |
| zwei Firmen legen Zwecke gemeinsam fest | **gemeinsame Verantwortlichkeit** (Art. 26) |

**Auswahl eines Cloud-Anbieters – Prüfpunkte**
- Serverstandort in der EU bzw. Drittlandgarantien (SCC, Angemessenheitsbeschluss wie das EU-US Data Privacy Framework für zertifizierte US-Anbieter)
- Zertifizierungen: ISO/IEC 27001, BSI C5, ggf. ISO 27018
- Verschlüsselung **at rest** und **in transit**, Schlüsselverwaltung (am besten beim Kunden)
- Unterauftragsverarbeiter-Liste, Audit-Rechte, Exit-Strategie (Datenexport, Löschnachweis)
- SLA mit Verfügbarkeit, RTO/RPO

## Prüfungsfrage

**Frage:** Ein Unternehmen lagert seinen Mailserver zu einem Cloud-Anbieter aus. Nennen Sie die datenschutzrechtliche Voraussetzung und vier Punkte, die darin geregelt sein müssen.

**Musterantwort:** Da der Anbieter E-Mails und damit personenbezogene Daten im Auftrag verarbeitet, muss ein Auftragsverarbeitungsvertrag nach Art. 28 DSGVO geschlossen werden. Darin sind unter anderem zu regeln: Gegenstand, Dauer, Art und Zweck der Verarbeitung; die Weisungsgebundenheit des Anbieters; die technischen und organisatorischen Maßnahmen wie Verschlüsselung und Zugriffskontrolle; die Zulässigkeit von Unterauftragsverarbeitern nur mit Genehmigung sowie die Löschung oder Rückgabe aller Daten nach Vertragsende. Zusätzlich ist der Serverstandort zu prüfen und bei Drittländern eine Übermittlungsgrundlage wie Standardvertragsklauseln nötig.

## Falle

- „Mit dem AVV geht die Verantwortung auf den Anbieter über“ → nein, der **Auftraggeber bleibt verantwortlich**.
- AVV und SCC gleichsetzen: der AVV regelt die **Auftragsverarbeitung**, die SCC die **Drittlandübermittlung** – bei einem US-Anbieter braucht man oft beides.
- Steuerberater als Auftragsverarbeiter einordnen.
- Mündliche Absprache genügen lassen – der AVV muss **schriftlich oder elektronisch** vorliegen.

## Merksatz

**„Wer fremde Hände an die Daten lässt, braucht einen AVV – die Verantwortung bleibt trotzdem zu Hause.“**

## Verwandt

- [Datenpanne und SCC](scc-datenpanne.md) – Drittlandübermittlung
- [TOM](toms.md) – Auftragskontrolle
- [DSGVO-Grundlagen und Datenschutzkonzept](dsgvo-tom.md)
- [Cloud-Computing](../../FiSi-Administrator/Server/Cloud/cloud-computing.md)
- [SLA](../../FiSi-Administrator/Server/Cloud/sla.md)
- [Lektion 01 Cloud-Computing](../../analyse-der-exams/01_Cloud_Computing.md)
- [Lektion 05 Datenschutz und Informationssicherheit](../../analyse-der-exams/05_Datenschutz_Informationssicherheit.md)
