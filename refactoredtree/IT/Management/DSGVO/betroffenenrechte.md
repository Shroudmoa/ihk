# DSGVO-Betroffenenrechte

## In 30 Sekunden

- Betroffene Personen haben nach **Art. 12–22 DSGVO** Rechte gegenüber dem Verantwortlichen: **Information, Auskunft, Berichtigung, Löschung, Einschränkung, Datenübertragbarkeit, Widerspruch** und Schutz vor rein automatisierten Entscheidungen.
- Anfragen müssen **unverzüglich, spätestens innerhalb eines Monats** beantwortet werden (verlängerbar um zwei Monate bei Komplexität, mit Begründung) – grundsätzlich **kostenlos**.
- Vor der Auskunft muss die **Identität** des Anfragenden geprüft werden – sonst entsteht selbst eine Datenpanne.
- Das **Recht auf Löschung** gilt nicht grenzenlos: gesetzliche Aufbewahrungspflichten (z. B. Rechnungen) gehen vor – dann **sperren statt löschen**.
- Für die IT bedeutet das: Daten müssen **auffindbar, exportierbar und gezielt löschbar** sein (Löschkonzept, Datenmodell).

## Kernwissen

| Artikel | Recht | Inhalt |
|---|---|---|
| 13/14 | **Informationspflicht** | bei Erhebung informieren: Verantwortlicher, Zweck, Rechtsgrundlage, Empfänger, Speicherdauer, Rechte |
| 15 | **Auskunft** | ob und welche Daten verarbeitet werden, Zwecke, Empfänger, Dauer, Herkunft; Anspruch auf eine **Kopie** |
| 16 | **Berichtigung** | falsche Daten korrigieren, unvollständige ergänzen |
| 17 | **Löschung** („Recht auf Vergessenwerden“) | wenn Zweck entfallen, Einwilligung widerrufen, Widerspruch erfolgreich oder Verarbeitung unrechtmäßig – Ausnahmen u. a. bei gesetzlichen Aufbewahrungspflichten |
| 18 | **Einschränkung** | Daten sperren statt löschen, z. B. solange die Richtigkeit bestritten ist |
| 19 | **Mitteilungspflicht** | Empfänger über Berichtigung/Löschung informieren |
| 20 | **Datenübertragbarkeit** | Daten in strukturiertem, gängigem, maschinenlesbarem Format erhalten (z. B. CSV, JSON) oder direkt an einen anderen Anbieter übermitteln lassen – bei Einwilligung oder Vertrag und automatisierter Verarbeitung |
| 21 | **Widerspruch** | gegen Verarbeitung auf Basis berechtigter/öffentlicher Interessen; gegen **Direktwerbung** immer |
| 22 | **automatisierte Einzelentscheidung** | Recht, keiner rein automatisierten Entscheidung mit erheblicher Wirkung unterworfen zu werden (z. B. vollautomatische Kreditablehnung) |
| 7 Abs. 3 | **Widerruf der Einwilligung** | jederzeit, so einfach wie die Erteilung, wirkt für die Zukunft |
| 77 | **Beschwerde** | bei der Datenschutz-Aufsichtsbehörde |
| 82 | **Schadensersatz** | materieller und immaterieller Schaden |

**Ablauf einer Auskunftsanfrage**
1. Eingang dokumentieren, Frist notieren (1 Monat)
2. **Identität prüfen** (z. B. Abgleich mit hinterlegten Kontaktdaten)
3. Daten in allen Systemen suchen (CRM, ERP, Mail, Backups, Dienstleister)
4. Auskunft zusammenstellen, Rechte Dritter schwärzen
5. **sicher** übermitteln (verschlüsselt, nicht an eine unbestätigte Adresse)
6. Vorgang dokumentieren (Rechenschaftspflicht)

**Technische Konsequenzen:** zentrales Verarbeitungsverzeichnis, eindeutige Kunden-IDs, Exportfunktionen, Löschroutinen mit Fristen, Sperrkennzeichen für Daten in Aufbewahrungsfrist; Backups werden nicht einzeln bereinigt, sondern Löschungen beim Rückspielen erneut angewandt und Backups nach Rotationsfrist überschrieben.

## Prüfungsfrage

**Frage:** Eine ehemalige Kundin verlangt die Löschung all ihrer Daten. Ihre Rechnungen sind sieben Jahre alt, ihre Newsletter-Einwilligung hat sie widerrufen. Wie gehen Sie vor?

**Musterantwort:** Zuerst wird die Identität der Kundin geprüft und die Monatsfrist notiert. Die Daten für den Newsletter werden gelöscht, weil die Einwilligung als Rechtsgrundlage durch den Widerruf entfallen ist. Für Rechnungsdaten gilt eine gesetzliche Aufbewahrungspflicht (bei Buchungsbelegen in der Regel acht Jahre, bei Handelsbüchern und Abschlüssen zehn Jahre); diese Daten werden bis zum Fristende gesperrt, also nur noch für die Aufbewahrung vorgehalten, und danach gelöscht. Die Kundin wird über die Löschung und den Grund für die eingeschränkte Aufbewahrung informiert, der Vorgang wird dokumentiert.

## Falle

- „Löschung ist immer sofort vollständig durchzuführen“ → gesetzliche Aufbewahrungspflichten gehen vor, dann **Einschränkung/Sperrung**.
- Auskunft ohne Identitätsprüfung per Mail verschicken → mögliche **Datenpanne**.
- Frist mit 72 Stunden verwechseln – die gilt für Meldungen von Datenpannen, Betroffenenanfragen haben **einen Monat**.
- Datenübertragbarkeit als PDF-Ausdruck erfüllen wollen → verlangt ist ein **maschinenlesbares** Format.

## Merksatz

**„Auskunft, Berichtigung, Löschung, Einschränkung, Übertragung, Widerspruch – binnen eines Monats, nach Identitätsprüfung.“**

## Verwandt

- [Rechtsgrundlagen und Grundsätze](rechtsgrundlagen-grundsaetze.md)
- [DSGVO-Grundlagen und Datenschutzkonzept](dsgvo-tom.md)
- [Datenschutzbeauftragter](dbs-datenschutzbeauftragten.md)
- [Sichere Datenlöschung](../../FiSi-Administrator/Sicherheit/datentraeger-entsorgung.md)
- [Lektion 05 Datenschutz und Informationssicherheit](../../analyse-der-exams/05_Datenschutz_Informationssicherheit.md)
