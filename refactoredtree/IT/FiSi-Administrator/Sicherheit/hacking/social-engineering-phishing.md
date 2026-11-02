# Social Engineering und Phishing

## In 30 Sekunden

- **Social Engineering** greift den **Menschen** an, nicht die Technik: Vertrauen, Hilfsbereitschaft, Angst und Zeitdruck werden ausgenutzt.
- **Phishing** ist die häufigste Form: gefälschte E-Mails oder Webseiten sollen Zugangsdaten abgreifen oder Malware einschleusen.
- Die IHK fragt typischerweise **Phishing-Merkmale in einer Beispiel-Mail** (2024S) und **Spam vs. Phishing** (2024W).
- Eine Firewall hilft hier kaum – wirksam sind **Awareness-Schulungen, MFA, Mailfilter und SPF/DKIM/DMARC**.
- Wer geklickt hat, meldet es **sofort** der IT – Schweigen verschlimmert den Schaden.

## Kernwissen

**Spielarten**

| Begriff | Beschreibung |
|---|---|
| **Phishing** | Massen-Mail mit gefälschtem Absender und Link auf eine nachgebaute Login-Seite |
| **Spear-Phishing** | gezielt auf eine Person/Abteilung zugeschnitten, mit echten Namen und Insiderwissen |
| **Whaling / CEO-Fraud** | angeblicher Chef fordert eilige Überweisung („bitte diskret behandeln“) |
| **Smishing / Vishing** | Phishing per SMS bzw. per Telefonanruf (z. B. falscher IT-Support) |
| **Quishing** | Phishing-Link versteckt in einem QR-Code |
| **Pretexting** | Angreifer erfindet eine Rolle und Geschichte, um Informationen zu erhalten |
| **Baiting** | präparierter USB-Stick wird „verloren“, Neugier führt zum Einstecken |
| **Tailgating** | Angreifer läuft hinter einem Mitarbeiter durch die gesicherte Tür |
| **Shoulder Surfing / Dumpster Diving** | Passwort über die Schulter ablesen bzw. Papiermüll durchsuchen |

**Phishing-Merkmale (nenne 3 – Prüfung 2024S)**
- Absenderadresse passt nicht zur Firma (z. B. Freemail-Adresse statt Firmendomain)
- unpersönliche Anrede („Lieber Kunde“), viele Empfänger oder „undisclosed recipients“
- **Zeitdruck oder Drohung** („Ihr Konto wird in 12 Stunden gesperrt“)
- Linktext und tatsächliche URL stimmen nicht überein (Mauszeiger darüber halten)
- Aufforderung, Passwort, TAN oder Kontodaten einzugeben
- unerwarteter Anhang (Office-Datei mit Makros, ZIP, ausführbare Datei)
- Rechtschreibfehler, fehlende Signatur oder fehlendes Impressum

**Spam vs. Phishing (Prüfung 2024W)**

| | Spam | Phishing |
|---|---|---|
| Ziel | Werbung/Verbreitung an möglichst viele | **Täuschung**, um Zugangsdaten oder Geld zu erbeuten |
| Absender | oft offen werblich | imitiert vertrauenswürdigen Absender (Bank, IT, Chef) |
| Schaden | lästig, kostet Zeit | Kontoübernahme, Malware, finanzieller Schaden |

**Gegenmaßnahmen**

| Art | Maßnahme |
|---|---|
| Organisatorisch | regelmäßige **Awareness-Schulungen** und Phishing-Simulationen, klarer Meldeweg („Phishing melden“-Button), Rückruf-Regel bei Zahlungsanweisungen (Vier-Augen-Prinzip), Clean-Desk-Policy, Besucherregelung |
| Technisch | **MFA** (gestohlenes Passwort allein reicht nicht), Spam-/Phishing-Filter, Sandbox für Anhänge, **SPF/DKIM/DMARC** gegen gefälschte Absender, Makros per GPO sperren, Webfilter, Kennzeichnung externer Mails |

**Wenn jemand geklickt hat:** Gerät vom Netz trennen, IT/Security sofort informieren, betroffenes Passwort ändern und Sitzungen beenden, Vorfall dokumentieren; bei personenbezogenen Daten den Datenschutzbeauftragten einbinden.

## Prüfungsfrage

**Frage:** Ein Mitarbeiter erhält eine Mail vom „IT-Service“ mit der Aufforderung, sein Postfach innerhalb von 12 Stunden über einen Link zu „aktualisieren“. Nennen Sie drei Merkmale, an denen er Phishing erkennt, und zwei Maßnahmen, die das Unternehmen dagegen ergreifen sollte.

**Musterantwort:** Merkmale: Der künstliche Zeitdruck von 12 Stunden, eine Absenderadresse, die nicht zur Firmendomain gehört, und ein Link, dessen tatsächliche Zieladresse beim Darüberfahren auf eine fremde Domain zeigt. Maßnahmen: Erstens regelmäßige Awareness-Schulungen mit Phishing-Simulationen und einem klaren Meldeweg, damit Mitarbeiter solche Mails erkennen und melden. Zweitens MFA für alle Logins, damit erbeutete Passwörter allein keinen Zugang ermöglichen.

## Falle

- „Die Firewall verhindert Phishing“ → nein, die Mail kommt über einen erlaubten Weg (Prüfung 2026S GA2: Bedrohungen ohne Firewall-Schutz).
- HTTPS-Schloss als Echtheitsbeweis werten → auch Phishing-Seiten haben gültige Zertifikate.
- Nur technische Maßnahmen nennen – Social Engineering verlangt **immer auch** organisatorische Maßnahmen.
- Spam und Phishing gleichsetzen: der Unterschied liegt in der **Täuschungsabsicht**.

## Merksatz

**„Druck, Drohung, fremde Domain – erst prüfen, dann klicken. Und im Zweifel melden.“**

## Verwandt

- [E-Mail-Sicherheit](../email-sicherheit.md) – SPF, DKIM, DMARC, Spamfilter, Sandbox
- [MFA](../MFA.md) – warum gestohlene Passwörter dann nicht reichen
- [Malware und Ransomware](malware-ransomware.md) – was ein Klick auslösen kann
- [Angriffsarten im Überblick](hackers-can.md)
- [IT-Sicherheit im Arbeitsalltag](../../../../Wirtschaft/it-sicherheit-im-arbeitsalltag.md)
- [Lektion 05 Datenschutz und Informationssicherheit](../../../analyse-der-exams/05_Datenschutz_Informationssicherheit.md)
