# E-Mail-Sicherheit

## In 30 Sekunden

- E-Mail ist ursprünglich **ungeschützt**: Absender fälschbar, Inhalt im Klartext, Anhänge können Schadcode enthalten.
- **Transportverschlüsselung** (SMTP mit STARTTLS, IMAPS/POP3S) schützt nur den Weg zwischen den Servern; **Ende-zu-Ende** schützen erst **S/MIME** oder **PGP**.
- Gegen gefälschte Absender: **SPF** (welche Server dürfen senden), **DKIM** (Signatur der Mail), **DMARC** (was mit Fehlschlägen passiert) – alles DNS-Einträge.
- Gegen Schadcode und Phishing: **Spam-/Phishing-Filter**, **Virenscanner**, **Sandbox**, Anhangsfilter und **Awareness**.
- Prüfung 2024W: **sechs TOM zum Schutz von E-Mails** sowie Fragen zu DKIM- und SPF-Einträgen; 2023S/2022S: SPF und Sandbox.

## Kernwissen

**Maßnahmen im Überblick**

| Ziel | Maßnahme | Hinweis |
|---|---|---|
| Vertraulichkeit unterwegs | TLS zwischen Mailservern (STARTTLS, Port 25) und zum Client (Submission 587/465, IMAPS 993) | Server können den Inhalt weiterhin lesen |
| Vertraulichkeit Ende-zu-Ende | **S/MIME** (X.509-Zertifikate) oder **PGP** | nur der Empfänger kann entschlüsseln |
| Echtheit des Absenders | **SPF**, **DKIM**, **DMARC**; S/MIME-Signatur | verhindert Spoofing der Domain |
| Schutz vor Schadcode | Virenscanner, **Sandbox**, Sperre ausführbarer Anhänge und Makros | Sandbox öffnet Anhänge isoliert und beobachtet sie |
| Schutz vor Phishing | Phishing-Filter, Link-Prüfung, Kennzeichnung externer Mails, Schulung | Mensch bleibt letzte Instanz |
| Zugang zum Postfach | **MFA**, starke Passwörter, Least Privilege für Shared Mailboxes | gestohlene Passwörter allein reichen nicht |
| Verfügbarkeit | Backup und Archivierung der Postfächer | revisionssichere Archivierung beachten |

**SPF, DKIM, DMARC**

| Verfahren | Wo | Was es tut |
|---|---|---|
| **SPF** | DNS-TXT-Eintrag der Domain, z. B. `v=spf1 ip4:203.0.113.0/24 include:newsletter.example -all` | listet die IP-Adressen/Server, die für die Domain senden dürfen; der Empfänger vergleicht die Absender-IP. **Nur ein SPF-Eintrag pro Domain.** |
| **DKIM** | Signatur im Header `DKIM-Signature:` (mit `d=` Domain und `s=` Selector); öffentlicher Schlüssel im DNS unter `selector._domainkey.domain` | der sendende Server signiert die Mail mit seinem privaten Schlüssel, der Empfänger prüft mit dem öffentlichen Schlüssel aus dem DNS – **keine CA nötig** |
| **DMARC** | DNS-TXT unter `_dmarc.domain` | legt fest, was bei SPF/DKIM-Fehlschlag passiert (`none`, `quarantine`, `reject`) und wohin Berichte gehen |

**Typische Fehlerbilder**
- Externer Newsletter-Dienst landet im Spam → dessen Server fehlt im SPF-Eintrag; Lösung: `include:` des Dienstleisters ergänzen (2023S).
- „DKIM verification failed“ → öffentlicher Schlüssel im DNS passt nicht (Selector falsch, Eintrag gelöscht, Schlüssel gewechselt) oder die Mail wurde unterwegs verändert (2024W).
- SMTP-Fehler `554 5.7.1` mit Blocklist-Hinweis → Absender-Domain oder -IP steht auf einer Sperrliste (DNSBL), z. B. nach Spam-Missbrauch.

**Rechtlicher Hinweis (2022S):** Ist die private Nutzung des dienstlichen Postfachs erlaubt, darf der Arbeitgeber Inhalte nicht ohne Weiteres kontrollieren; Mailfilter und Kontrollen gehören in eine Richtlinie bzw. Betriebsvereinbarung.

## Prüfungsfrage

**Frage:** Nennen Sie vier technische oder organisatorische Maßnahmen, mit denen ein Unternehmen seinen E-Mail-Verkehr absichert, und erläutern Sie die Aufgabe des SPF-Eintrags.

**Musterantwort:** Erstens TLS-Verschlüsselung zwischen Client und Mailserver sowie zwischen den Mailservern, damit Mails unterwegs nicht mitgelesen werden. Zweitens ein Spam- und Phishing-Filter mit Virenscanner und Sandbox für Anhänge. Drittens SPF, DKIM und DMARC, damit gefälschte Absender der eigenen Domain erkannt werden. Viertens regelmäßige Awareness-Schulungen der Mitarbeiter. Der SPF-Eintrag ist ein TXT-Eintrag im DNS der Domain und legt fest, welche Server E-Mails im Namen dieser Domain versenden dürfen; der empfangende Server prüft, ob die einliefernde IP-Adresse darin enthalten ist.

## Falle

- „TLS verschlüsselt die Mail Ende-zu-Ende“ → nein, nur den Transport; auf den Servern liegt sie im Klartext.
- SPF als Schutz gegen Phishing allgemein verkaufen → SPF prüft nur, ob der sendende Server für die **Domain** berechtigt ist; eine ähnlich klingende Fremddomain fällt nicht auf.
- DKIM-Schlüssel bei einer CA suchen → der öffentliche Schlüssel steht im **DNS**.
- Mehrere SPF-Einträge pro Domain anlegen → ungültig, alles gehört in **einen** Eintrag.

## Merksatz

**„SPF sagt wer senden darf, DKIM beweist dass die Mail echt ist, DMARC sagt was mit Fälschungen passiert.“**

## Verwandt

- [SPF, DKIM und DMARC (Kurzblatt)](../../nice-to-know/email-auth.md)
- [Social Engineering und Phishing](hacking/social-engineering-phishing.md)
- [Digitale Signatur](Verschluesselung/digitale-signatur.md) – S/MIME
- [PGP](Verschluesselung/pgp.md)
- [TLS und HTTPS](Verschluesselung/tls-https.md)
- [DNS](../OSI-IP-mac-bit-byte/dns.md) – TXT-Einträge
- [Lektion 29 GA2-Praxisthemen (E-Mail-Sicherheit)](../../analyse-der-exams/L29_GA2_Sonstiges_Praxisthemen.md)
