# Symmetrische-und-Asymmetrische

# Symmetrische und asymmetrische Verschlüsselung

## Symmetrische Verschlüsselung

Bei der symmetrischen Verschlüsselung verwenden Sender und Empfänger **denselben geheimen Schlüssel** zum Ver- und Entschlüsseln.

Beispiele:
- AES
- ChaCha20
- 3DES (veraltet, nicht mehr für neue Systeme empfohlen)

### Problem: Schlüsselaustausch

Beide Kommunikationspartner müssen den geheimen Schlüssel sicher austauschen. Wenn sie vorher keinen gemeinsamen Schlüssel besitzen, entsteht das Problem: **Wie überträgt man den Schlüssel sicher über einen unsicheren Kanal?**

Bei vielen Kommunikationspartnern wird die Schlüsselverwaltung aufwendig.

Für `n` Teilnehmer, die jeweils paarweise einen eigenen Schlüssel benötigen, sind insgesamt

`n · (n − 1) / 2`

verschiedene Schlüssel nötig.

Beispiel: 100 Teilnehmer benötigen 4.950 paarweise Schlüssel.

## Asymmetrische Verschlüsselung

Bei der asymmetrischen Verschlüsselung besitzt jeder Teilnehmer ein Schlüsselpaar:

- **Public Key:** öffentlicher Schlüssel, darf verteilt werden.
- **Private Key:** privater Schlüssel, muss geheim bleiben.

Bei der klassischen Verschlüsselung mit Public-Key-Verfahren verschlüsselt der Sender mit dem Public Key des Empfängers. Entschlüsseln kann der Empfänger mit seinem Private Key.

Beispiele:
- RSA
- ElGamal
- ECC-basierte Verfahren, z. B. ECIES

### Vorteil

Der öffentliche Schlüssel muss nicht geheim übermittelt werden. Dadurch entfällt der direkte Austausch eines gemeinsamen symmetrischen Schlüssels.

### Problem: Vertrauen in den Public Key

Ein Angreifer könnte seinen eigenen Public Key als den Schlüssel des Empfängers ausgeben. Deshalb muss überprüft werden, ob ein öffentlicher Schlüssel tatsächlich zur angegebenen Person oder zum Server gehört.

Lösungen:
- **PKI (Public Key Infrastructure):** verwaltet Zertifikate und Vertrauensbeziehungen.
- **Digitale Zertifikate:** binden einen öffentlichen Schlüssel an eine Identität.
- **Zertifizierungsstellen (CA):** stellen Zertifikate aus.
- **Fingerprint-Prüfung:** öffentlicher Schlüssel wird über seinen Fingerprint auf einem vertrauenswürdigen Weg überprüft.

## Warum kombiniert man beide Verfahren?

Asymmetrische Verfahren sind für den Schlüsselaustausch und die Authentifizierung nützlich. Symmetrische Verfahren sind für die eigentliche Datenverschlüsselung meist effizienter.

Beispiel HTTPS/TLS:

1. Der Client baut eine Verbindung zum Webserver auf.
2. Der Server präsentiert ein Zertifikat, dessen Gültigkeit und Identität der Client prüft.
3. Beide Seiten handeln über einen Schlüsselaustausch, beispielsweise ECDHE, gemeinsame Sitzungsschlüssel aus.
4. Die eigentlichen Daten werden mit symmetrischer Verschlüsselung, beispielsweise AES-GCM oder ChaCha20-Poly1305, geschützt.

Moderne TLS-Verbindungen verwenden häufig **ephemeren Diffie-Hellman-Schlüsselaustausch**. Dabei werden gemeinsame Schlüssel abgeleitet; die übertragenen Daten werden nicht einfach mit dem öffentlichen RSA-Schlüssel verschlüsselt.

## Vergleich

| Merkmal | Symmetrisch | Asymmetrisch |
|---|---|---|
| Schlüssel | Gemeinsamer geheimer Schlüssel | Public Key + Private Key |
| Geschwindigkeit | Sehr hoch | Meist langsamer |
| Schlüsselverteilung | Geheimer Schlüssel muss sicher vereinbart werden | Public Key darf öffentlich sein |
| Hauptproblem | Sicherer Schlüsselaustausch und Schlüsselverwaltung | Authentizität des Public Keys |
| Beispiele | AES, ChaCha20 | RSA, ECC |
| Typische Nutzung | Verschlüsselung großer Datenmengen | Schlüsselaustausch, Signaturen, Authentifizierung |

## IHK-Merksatz

Symmetrische Verschlüsselung ist schnell, benötigt aber einen gemeinsamen geheimen Schlüssel. Asymmetrische Kryptografie verwendet ein öffentliches und ein privates Schlüsselpaar und vereinfacht den Schlüsselaustausch, benötigt aber eine vertrauenswürdige Zuordnung des Public Keys zur Identität. In der Praxis kombinieren Protokolle wie TLS beide Ansätze.
