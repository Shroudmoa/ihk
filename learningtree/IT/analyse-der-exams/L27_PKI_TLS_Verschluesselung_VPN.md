# Lektion 27 – Asymmetrische Verschlüsselung, PKI, TLS & VPN

**Bereich:** GA1 + GA2 (Querschnittsthema, sehr häufig in beiden Teilen)
**Relevanz:** Sehr hoch – Zertifikate, Schlüsselaustausch und VPN kommen fast in jedem Jahrgang vor
**Lernzeit:** ca. 2h

## 1. Warum das wichtig ist
- Verschlüsselung ist das am häufigsten geprüfte Sicherheitsthema in GA1 **und** GA2.
- Die Prüfung verlangt nicht nur „AES" oder „RSA" als Stichwort, sondern den **korrekten Ablauf** (wer verschlüsselt/signiert womit).
- PKI (Public-Key-Infrastruktur), TLS-Handshake und VPN-Arten sind Klassiker – wer den Ablauf runterbeten kann, holt sichere Punkte.
- Typischer Fehler: symmetrisch und asymmetrisch verwechseln, oder öffentlichen/privaten Schlüssel vertauschen.

## 2. Grundlagen – Symmetrisch vs. Asymmetrisch
| Merkmal | Symmetrisch | Asymmetrisch |
|---------|-------------|---------------|
| Schlüssel | **ein** gemeinsamer Schlüssel für Ver- und Entschlüsselung | **Schlüsselpaar**: öffentlich (public) + privat (private) |
| Beispiele | AES-128/256 (aktuell), DES/3DES (veraltet) | RSA |
| Geschwindigkeit | schnell, für **große Datenmengen** geeignet | **deutlich langsamer**, nicht für Bulk-Verschlüsselung |
| Problem | Schlüsselaustausch (wie bekommt der Empfänger den Schlüssel sicher?) | löst das Schlüsselaustauschproblem |
| Typischer Einsatz | Datenträger-/Festplattenverschlüsselung, TLS-Nutzdaten | Schlüsselaustausch, digitale Signatur, Zertifikate |

**Wichtig (2023W-Aufgabe):** RSA ist **nicht** für die Verschlüsselung ganzer Datenträger geeignet – dafür ist es zu langsam. Für Datenträgerverschlüsselung wird **symmetrisch** (z. B. AES) verschlüsselt; asymmetrisch wird nur der **Session-/Bulk-Key** ausgetauscht oder signiert. Das nennt man **Hybridverfahren** und ist auch das Prinzip hinter TLS (siehe Abschnitt 4).

## 3. PKI – Aufbau und Zertifikats-Einrichtung
Eine **Public-Key-Infrastruktur (PKI)** verwaltet digitale Zertifikate und bindet öffentliche Schlüssel an Identitäten (Personen, Server, Geräte).

**Zertifikats-Einrichtung in 4 Schritten (2023S-Prüfungsablauf):**
1. **Zertifikat bei einer CA (Certificate Authority) beantragen** – z. B. über einen Certificate Signing Request (CSR).
2. **Zertifikat/privaten Schlüssel schützen** – sicher speichern, z. B. mit Passwort oder Hardware-Token.
3. **Zertifikat installieren** – auf Server/Client einspielen.
4. **Übergeordnete Zertifikate (Vertrauenskette) einbinden** – Root-CA- und Intermediate-CA-Zertifikate in den Zertifikatsspeicher aufnehmen, damit die **Vertrauenskette (Chain of Trust)** vollständig geprüft werden kann.

**Aufgaben einer CA (2026S):**
- **Identitätsprüfung** des Antragstellers vor Ausstellung.
- **Signierung** des Zertifikats mit dem privaten Schlüssel der CA.
- **Sperrprüfung** über **CRL (Certificate Revocation List)** – Liste widerrufener Zertifikate – oder **OCSP (Online Certificate Status Protocol)** – Online-Echtzeitabfrage des Sperrstatus.

Alles basiert auf dem Standard **X.509** für digitale Zertifikate.

## 4. TLS-Handshake (2026S)
Ablauf beim Verbindungsaufbau (z. B. HTTPS):

| Schritt | Beschreibung |
|---------|--------------|
| 1. **ClientHello** | Client schickt unterstützte TLS-Versionen/Cipher Suites |
| 2. **ServerHello** | Server wählt Version/Cipher, sendet sein **Zertifikat** |
| 3. **Zertifikatsprüfung** | Client prüft Zertifikat gegen Vertrauenskette (Root-/Intermediate-CA) |
| 4. **Session-Key-Aushandlung** | asymmetrisch wird ein **symmetrischer Session-Key** ausgehandelt (z. B. per Diffie-Hellman) |
| 5. **Verschlüsselte Kommunikation** | die eigentlichen Nutzdaten werden **symmetrisch** (z. B. AES) verschlüsselt übertragen |

Das ist das **Hybridverfahren**: Asymmetrisch nur für Handshake/Schlüsselaustausch, symmetrisch für die eigentliche (schnelle) Datenübertragung.

## 5. Asymmetrischer E-Mail-Workflow (2024W – wichtig, genau so abgefragt)
Beim signierten **und** verschlüsselten Versand einer E-Mail (Basis: X.509-Zertifikate) gilt eine feste Reihenfolge:

**Senden (durch den Sender):**
1. **Signieren** mit dem **eigenen privaten** Schlüssel des Senders.
2. **Verschlüsseln** mit dem **öffentlichen** Schlüssel des **Empfängers**.

**Empfangen (durch den Empfänger):**
3. **Entschlüsseln** mit dem **eigenen privaten** Schlüssel des Empfängers.
4. **Signaturprüfung** mit dem **öffentlichen** Schlüssel des **Senders**.

**Merkregel:**
- Verschlüsseln = immer mit dem **öffentlichen** Schlüssel des **Empfängers** (nur er kann mit seinem privaten Schlüssel entschlüsseln).
- Signieren = immer mit dem **eigenen privaten** Schlüssel (jeder kann mit dem öffentlichen Schlüssel des Senders die Signatur prüfen → Authentizität + Integrität).

## 6. Weitere Grundlagen (Diffie-Hellman, MD5-Schwachstellen)
- **Diffie-Hellman-Schlüsselaustausch:** ermöglicht zwei Parteien, über einen unsicheren Kanal einen gemeinsamen geheimen (symmetrischen) Schlüssel zu vereinbaren, ohne ihn zu übertragen – Basis vieler TLS-/VPN-Handshakes.
- **MD5-Kollisionsangriffe:** MD5 gilt als **kryptographisch gebrochen**, da Kollisionen (zwei unterschiedliche Eingaben mit gleichem Hash) erzeugbar sind. Wird MD5 zum Signieren von Zertifikaten verwendet, können Angreifer gefälschte, aber gültig signierte Zertifikate erzeugen. Deshalb werden moderne Zertifikate mit **SHA-256** oder stärker signiert. (Dieses Thema kam in GA2 mehrfach vor: 2021W, 2022W, 2023W, 2024W.)

## 7. TLS-Inspection als Man-in-the-Middle-Prinzip (NGFW, 2024W GA2)
Next-Generation-Firewalls (NGFW) können verschlüsselten TLS-Traffic prüfen (**TLS-Inspection**):
- Die Firewall agiert als **Intermediate-CA** und baut selbst eine TLS-Verbindung zum Client auf (mit einem von ihr signierten Zertifikat) sowie eine zweite TLS-Verbindung zum eigentlichen Zielserver.
- Für den Client sieht das aus wie eine normale TLS-Verbindung – technisch ist es aber ein **Man-in-the-Middle**-Aufbau (gewollt, zur Inhaltsprüfung).
- Voraussetzung: Das Zertifikat der Intermediate-CA (Firewall) muss dem Client/Browser als **vertrauenswürdig** bekannt sein (z. B. per Gruppenrichtlinie verteilt).
- Fehlt dieses Vertrauen, erscheint ein Zertifikatsfehler wie z. B. `SEC_ERROR_UNKNOWN_ISSUER` – der Browser kennt die ausstellende (Intermediate-)CA nicht.

## 8. VPN-Grundlagen
| VPN-Typ | Beschreibung | Typischer Einsatz |
|---------|--------------|--------------------|
| **End-to-Site** | Einzelner Client verbindet sich über VPN mit einem Firmennetz | Homeoffice-Mitarbeiter, mobiler Außendienst |
| **Site-to-Site** | Zwei Netzwerke (z. B. Filiale ↔ Zentrale) werden dauerhaft per VPN gekoppelt | Standortvernetzung |

**IPsec-Schutzziele (2025W):**
- **Vertraulichkeit** – Daten sind verschlüsselt, nicht mitlesbar.
- **Integrität** – Daten wurden nicht unbemerkt verändert.
- **Authentizität** – Kommunikationspartner ist wirklich der, für den er sich ausgibt.

**Client-Authentifizierungsmethoden (2025W):**
- Zertifikat (Client-Zertifikat auf dem Endgerät)
- Smartcard (Hardware-Token)
- OTP-App (One-Time-Password, z. B. TOTP-Authenticator)

## 9. Typische VPN-Fehlerursachen (2023S, 2025W)
| Ursache | Erklärung |
|---------|-----------|
| **MTU-Probleme** | VPN-Overhead (Verschlüsselung, zusätzliche Header) verkleinert die nutzbare Paketgröße; wird die MTU nicht angepasst, kommt es zu Fragmentierung oder Paketverlust |
| **NAT-Traversal** | IPsec ESP (Encapsulating Security Payload) hat keine Portnummer wie TCP/UDP – klassisches NAT kann ESP-Pakete nicht korrekt umsetzen; Lösung: **NAT-T (NAT-Traversal)**, das ESP in UDP kapselt |
| **IPv6-Breakout** | Wenn der Client parallel eine native IPv6-Verbindung hat, kann Traffic am VPN-Tunnel (der ggf. nur IPv4 tunnelt) vorbeigehen ("Leck") |
| **Hotspot-NAT** | Öffentliche Hotspots (z. B. Hotel-WLAN) nutzen oft mehrfaches/restriktives NAT, das VPN-Handshakes oder bestimmte Ports blockiert |

## 10. Übungsaufgabe (im Prüfungsstil, selbst erstellt)

Anna möchte Ben eine vertrauliche E-Mail senden. Beide besitzen ein eigenes Schlüsselpaar (öffentlich/privat) auf Basis von X.509-Zertifikaten. Die E-Mail soll sowohl **verschlüsselt** als auch **digital signiert** werden.

a) Nennen Sie die vier Schritte in der richtigen Reihenfolge, die beim Senden durch Anna und beim Empfangen durch Ben ausgeführt werden.
b) Erklären Sie, warum RSA für diesen Zweck genutzt wird, aber nicht zur Verschlüsselung der gesamten Festplatte von Bens Laptop eingesetzt würde.

<details>
<summary>Lösung (aufklappen)</summary>

**a) Reihenfolge:**
1. Anna **signiert** die E-Mail mit ihrem **eigenen privaten** Schlüssel.
2. Anna **verschlüsselt** die E-Mail mit dem **öffentlichen** Schlüssel von **Ben** (dem Empfänger).
3. Ben **entschlüsselt** die E-Mail mit seinem **eigenen privaten** Schlüssel.
4. Ben **prüft die Signatur** mit dem **öffentlichen** Schlüssel von **Anna** (dem Sender).

**b) Begründung:**
RSA (asymmetrisch) eignet sich gut für Signatur und Schlüsselaustausch, weil hier nur kleine Datenmengen (Hashwert der Nachricht, Session-Key) verarbeitet werden müssen und die Sicherheit auf dem privaten/öffentlichen Schlüsselpaar beruht. Für die Verschlüsselung einer **gesamten Festplatte** wäre RSA jedoch viel zu **langsam**, da asymmetrische Verfahren deutlich mehr Rechenaufwand pro Byte benötigen als symmetrische Verfahren. Für Datenträgerverschlüsselung wird daher ein **symmetrisches Verfahren** wie AES-256 verwendet.

</details>

## Selbstcheck – kannst du das?
- [ ] Ich kenne den Unterschied zwischen symmetrischer und asymmetrischer Verschlüsselung inkl. Vor-/Nachteilen.
- [ ] Ich kann die 4 Schritte der Zertifikats-Einrichtung nennen.
- [ ] Ich kann den asymmetrischen E-Mail-Workflow (Signieren/Verschlüsseln, wessen Schlüssel) fehlerfrei aufsagen.
- [ ] Ich kann den TLS-Handshake in Grundzügen beschreiben (inkl. Hybridverfahren).
- [ ] Ich kenne die Aufgaben einer CA (Identitätsprüfung, Signierung, CRL/OCSP).
- [ ] Ich kann End-to-Site von Site-to-Site-VPN unterscheiden und die IPsec-Schutzziele nennen.
- [ ] Ich kenne typische VPN-Fehlerursachen (MTU, NAT-Traversal, IPv6-Breakout, Hotspot-NAT).
- [ ] Ich verstehe, warum TLS-Inspection technisch ein Man-in-the-Middle-Prinzip ist.

## Weiterführende Ressourcen
- IHK-Prüfungsaufgaben AP2 GA1/GA2 zu Verschlüsselung, PKI, TLS und VPN (2021–2026)
- RFC 5280 (X.509-Zertifikate), RFC 4301 (IPsec)
- BSI-Grundschutz: Kryptokonzepte und PKI
