# E-Mail-Authentifizierung: SPF, DKIM & DMARC

## In 30 Sekunden

- Ziel: **Absender-Spoofing und Phishing** im Namen der eigenen Domain verhindern – alle drei Verfahren werden als **TXT-Records im DNS** veröffentlicht.
- **SPF** legt fest, **welche Server (IP-Adressen)** Mails für die Domain versenden dürfen.
- **DKIM** **signiert** jede Mail mit einem privaten Schlüssel; der öffentliche Schlüssel steht im DNS → Integrität und Herkunft prüfbar.
- **DMARC** verlangt, dass SPF oder DKIM **zur sichtbaren Absenderdomain passen** (Alignment), und legt fest, was bei Fehlschlag passiert (`none`, `quarantine`, `reject`) + Berichte.
- Prüfungsklassiker (GA2 Sommer 2023): Newsletter-Dienstleister ist **nicht im SPF-Record** → Mails landen im Spam; Lösung: `include:` ergänzen.

## Kernwissen

### SPF (Sender Policy Framework)

```
firma.de.  TXT  "v=spf1 mx ip4:203.0.113.0/24 include:newsletterversand.de -all"
```

| Bestandteil | Bedeutung |
|---|---|
| `v=spf1` | Version, Pflicht am Anfang |
| `mx`, `a` | die Server aus MX- bzw. A-Record dürfen senden |
| `ip4:` / `ip6:` | erlaubte Adressen oder Netze |
| `include:` | SPF-Record eines Dienstleisters übernehmen (Newsletter, Microsoft 365) |
| `-all` | alle anderen: **Fail** (ablehnen) |
| `~all` | Softfail (annehmen, markieren) |
| `?all` | neutral |

- Geprüft wird die **Envelope-From**-Adresse (Return-Path, SMTP `MAIL FROM`) gegen die IP des **einliefernden Servers**.
- **Nur ein SPF-Record pro Domain**; höchstens **10 DNS-Lookups** (include, a, mx …), sonst *permerror*.
- Schwäche: **Weiterleitungen** brechen SPF (weiterleitender Server steht nicht im Record); die für Nutzer sichtbare **Header-From**-Adresse wird nicht geprüft.

### DKIM (DomainKeys Identified Mail)

```
sel2026._domainkey.firma.de.  TXT  "v=DKIM1; k=rsa; p=MIIBIjANBgkqh..."
```
1. Der sendende Server bildet einen **Hash** über ausgewählte Header und den Body und **signiert** ihn mit dem **privaten Schlüssel** → Header `DKIM-Signature: d=firma.de; s=sel2026; …`.
2. Der Empfänger holt über **Selektor** (`s=`) und Domain (`d=`) den **öffentlichen Schlüssel** aus dem DNS und prüft die Signatur.
3. Gültig → Mail wurde unterwegs **nicht verändert** und stammt von einem Server, der den Schlüssel der Domain besitzt.

- **Kein CA-Zertifikat** nötig – Vertrauensanker ist das DNS.
- Übersteht Weiterleitungen meist, solange der Inhalt nicht verändert wird (Mailinglisten-Fußzeilen brechen DKIM).
- **DKIM-Fehler** (GA1 Winter 2024): Public Key im DNS passt nicht zum Signaturschlüssel (Schlüsseltausch, Record gelöscht, falscher Selektor) oder Mail wurde unterwegs verändert.

### DMARC

```
_dmarc.firma.de.  TXT  "v=DMARC1; p=reject; rua=mailto:dmarc@firma.de; adkim=s; aspf=r"
```

| Bestandteil | Bedeutung |
|---|---|
| **Alignment** | Domain der **Header-From**-Adresse muss zur per SPF (Envelope) oder DKIM (`d=`) geprüften Domain passen |
| `p=none` | nur beobachten, Berichte sammeln (Einführungsphase) |
| `p=quarantine` | in Spam/Quarantäne |
| `p=reject` | annehmen verweigern |
| `rua=` | Aggregat-Berichte an diese Adresse |
| `adkim` / `aspf` | strenges (`s`) oder lockeres (`r`) Alignment |

DMARC ist bestanden, wenn **SPF oder DKIM** besteht **und** aligned ist.

### Zusammenspiel

| Verfahren | prüft | Record-Ort |
|---|---|---|
| SPF | den **Server** (IP) | `firma.de` TXT |
| DKIM | die **Nachricht** (Signatur) | `<selektor>._domainkey.firma.de` TXT |
| DMARC | die **Domain** im sichtbaren Absender + **Regel** + **Reporting** | `_dmarc.firma.de` TXT |

Ergänzend: **MX-Record** (wohin Mails zugestellt werden – fehlt er, scheitert der Empfang, GA2 Sommer 2023), **PTR/Reverse-DNS** des Mailservers, **TLS** beim Transport (STARTTLS, MTA-STS), Spamfilter und **Sandbox** für Anhänge.

## Prüfungsfrage

**Frage (angelehnt an GA2 Sommer 2023):** Seit die Firma `fahrguth.gmbh` ihren Newsletter über `mail.newsletterversand.de` verschickt, landen die Mails bei vielen Empfängern im Spam. Der SPF-Record lautet `v=spf1 mx -all`. Erklären Sie die Funktion von SPF, die Ursache und die Lösung.

**Musterantwort:** Mit SPF veröffentlicht der Domaininhaber im DNS, welche Server Mails für seine Domain versenden dürfen. Der empfangende Mailserver vergleicht die IP des einliefernden Servers mit diesem Record. Im Record sind nur die eigenen MX-Server erlaubt (`mx`), alle anderen sind per `-all` ausdrücklich verboten. Der Server des Newsletter-Dienstleisters steht nicht darin, die SPF-Prüfung schlägt fehl und die Mails werden abgelehnt oder als Spam markiert. Lösung: den Dienstleister aufnehmen, z. B. `v=spf1 mx include:newsletterversand.de -all`; zusätzlich sollte der Dienstleister mit einem DKIM-Schlüssel der Domain signieren, damit auch DMARC besteht.

## Falle

- SPF als Verschlüsselung oder Signatur beschreiben → SPF ist eine **Absender-IP-Liste**.
- Zwei SPF-Records anlegen → ungültig; Einträge in **einem** Record zusammenführen.
- DKIM braucht ein Zertifikat einer CA → nein, **Schlüssel im DNS**.
- DMARC als eigenes Prüfverfahren verkaufen → DMARC **wertet SPF/DKIM aus** und ergänzt Alignment, Policy und Reporting.
- `p=reject` sofort einführen → erst `p=none` und Berichte auswerten, sonst werden legitime Mails verworfen.

## Merksatz

**„SPF prüft den Server, DKIM die Nachricht, DMARC die Domain – und sagt, was mit Fälschungen passiert.“**

## Verwandt

- [DNS](../FiSi-Administrator/OSI-IP-mac-bit-byte/dns.md) – TXT- und MX-Records
- [Ports](../FiSi-Administrator/OSI-IP-mac-bit-byte/ports.md) – SMTP 25/465/587, IMAP 993
- [Hashfunktionen](../FiSi-Administrator/Sicherheit/Verschluesselung/hash.md) – Grundlage der DKIM-Signatur
- [Symmetrische und asymmetrische Verschlüsselung](../FiSi-Administrator/Sicherheit/Verschluesselung/symmetrische-und-asymmetrische.md) – Schlüsselpaare
- [Hacker-Angriffe](../FiSi-Administrator/Sicherheit/hacking/hackers-can.md) – Phishing und Spoofing
- [Lektion 29 – GA2 Sonstiges (SPF, Spamfilter)](../analyse-der-exams/L29_GA2_Sonstiges_Praxisthemen.md)
