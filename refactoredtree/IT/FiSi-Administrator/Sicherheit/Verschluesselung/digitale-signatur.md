# Digitale Signatur

## In 30 Sekunden

- Eine **digitale Signatur** beweist, **von wem** ein Dokument stammt (**Authentizität**) und dass es **unverändert** ist (**Integrität**); zusätzlich stützt sie die **Nichtabstreitbarkeit**.
- Prinzip: **Hash** der Nachricht bilden → mit dem **eigenen privaten Schlüssel** signieren → Empfänger prüft mit dem **öffentlichen Schlüssel des Absenders**.
- Eine Signatur **verschlüsselt nichts** – der Inhalt bleibt lesbar, solange nicht zusätzlich verschlüsselt wird.
- Damit der öffentliche Schlüssel wirklich zum Absender gehört, braucht es ein **Zertifikat** einer vertrauenswürdigen CA (PKI) oder ein Web of Trust (PGP).
- Prüfungsklassiker 2024W: **Signieren und Verschlüsseln einer E-Mail – wer nimmt welchen Schlüssel?**

## Kernwissen

**Ablauf**

| Schritt | Absender (Anna) | Empfänger (Ben) |
|---|---|---|
| 1 | bildet den **Hashwert** der Nachricht (z. B. SHA-256) | – |
| 2 | signiert den Hash mit **Annas privatem Schlüssel** → Signatur | – |
| 3 | sendet Nachricht + Signatur (+ Zertifikat) | empfängt beides |
| 4 | – | berechnet selbst den Hash der empfangenen Nachricht |
| 5 | – | prüft die Signatur mit **Annas öffentlichem Schlüssel** und vergleicht mit dem eigenen Hash |
| 6 | – | gleich → echt und unverändert; ungleich → manipuliert oder falscher Absender |

**Signieren + Verschlüsseln einer E-Mail (S/MIME oder PGP, Prüfung 2024W, 2023S)**

| Aktion | Schlüssel |
|---|---|
| Signieren | **eigener privater** Schlüssel des Absenders |
| Verschlüsseln | **öffentlicher** Schlüssel des Empfängers |
| Entschlüsseln | **eigener privater** Schlüssel des Empfängers |
| Signatur prüfen | **öffentlicher** Schlüssel des Absenders |

**Abgrenzung**

| Verfahren | Schützt | Schlüssel | Nichtabstreitbarkeit |
|---|---|---|---|
| Hash allein | Integrität (nur, wenn der Referenzhash vertrauenswürdig ist) | keiner | nein |
| **HMAC** | Integrität + Authentizität | **gemeinsamer geheimer** Schlüssel | nein – beide Seiten könnten den MAC erzeugt haben |
| **Digitale Signatur** | Integrität + Authentizität | asymmetrisches Schlüsselpaar | **ja**, nur der Inhaber kennt den privaten Schlüssel |
| Verschlüsselung | Vertraulichkeit | symmetrisch oder Public Key des Empfängers | nein |

**Einsatzbereiche:** E-Mail (S/MIME, PGP), Zertifikate selbst (die CA signiert das Zertifikat), TLS-Handshake (CertificateVerify), Code-Signing von Software und Treibern, Secure Boot (signierte Bootloader), DNSSEC, DKIM bei E-Mails, elektronische Rechnungen und Verträge.

**Rechtlich (eIDAS-Verordnung):** einfache, fortgeschrittene und **qualifizierte elektronische Signatur**; nur die qualifizierte (mit qualifiziertem Zertifikat und sicherer Signaturerstellungseinheit, z. B. Signaturkarte) ist der handschriftlichen Unterschrift gleichgestellt.

**Was die Signatur wertlos macht:** gestohlener oder ungeschützter privater Schlüssel, schwacher Hash (MD5/SHA-1 – Kollisionen ermöglichen gefälschte Signaturen), abgelaufenes oder gesperrtes Zertifikat.

## Prüfungsfrage

**Frage:** Herr Meier will Herrn Anton eine E-Mail senden, die verschlüsselt und digital signiert ist. Geben Sie für jeden Schritt an, welcher Schlüssel verwendet wird, und nennen Sie die Schutzziele, die die Signatur erfüllt.

**Musterantwort:** Herr Meier signiert die E-Mail mit seinem eigenen privaten Schlüssel und verschlüsselt sie mit dem öffentlichen Schlüssel von Herrn Anton. Herr Anton entschlüsselt mit seinem eigenen privaten Schlüssel und prüft die Signatur mit dem öffentlichen Schlüssel von Herrn Meier. Die Signatur gewährleistet Integrität, weil jede Änderung den Hashvergleich scheitern lässt, und Authentizität sowie Nichtabstreitbarkeit, weil nur Herr Meier seinen privaten Schlüssel besitzt.

## Falle

- „Signieren mit dem öffentlichen Schlüssel“ → falsch, **signiert wird immer privat**, geprüft öffentlich.
- „Die Signatur macht die Mail vertraulich“ → nein, dafür braucht es zusätzlich Verschlüsselung.
- Verschlüsseln mit dem **eigenen** öffentlichen Schlüssel → dann kann nur man selbst entschlüsseln.
- Hash und Signatur gleichsetzen: ein Hash hat keinen Schlüssel und beweist keine Herkunft.

## Merksatz

**„Signieren privat, prüfen öffentlich – verschlüsseln öffentlich (vom Empfänger), entschlüsseln privat.“**

## Verwandt

- [Hash](hash.md) – Hashfunktionen, HMAC, Salt
- [PKI und Zertifikate](pki.md) – wer bestätigt, dass der Schlüssel echt ist
- [PGP](pgp.md) – Signatur und Verschlüsselung im Web of Trust
- [Symmetrische und asymmetrische Verschlüsselung](symmetrische-und-asymmetrische.md)
- [TLS und HTTPS](tls-https.md)
- [E-Mail-Sicherheit](../email-sicherheit.md) – S/MIME und DKIM
- [Schutzziele](../schutzziele.md) – Integrität, Authentizität, Verbindlichkeit
- [Lektion 27 PKI, TLS, VPN](../../../analyse-der-exams/L27_PKI_TLS_Verschluesselung_VPN.md)
