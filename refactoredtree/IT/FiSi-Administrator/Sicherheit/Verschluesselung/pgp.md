# PGP (Pretty Good Privacy)

## In 30 Sekunden

- **PGP** (bzw. der offene Standard **OpenPGP**, z. B. GnuPG) verschlüsselt und signiert E-Mails und Dateien **Ende-zu-Ende**.
- Hybridverfahren: Nachricht **symmetrisch** mit zufälligem Sitzungsschlüssel, Sitzungsschlüssel **asymmetrisch** mit dem öffentlichen Schlüssel des Empfängers.
- Vertrauen in Schlüssel über das **Web of Trust** bzw. Fingerprint-Vergleich – keine zentrale CA wie bei **S/MIME**.
- Signatur: Hash mit dem **eigenen privaten** Schlüssel, Prüfung mit dem öffentlichen Schlüssel des Absenders.

## Kernwissen

### Definition
**PGP** ist ein Verschlüsselungsverfahren, das zur sicheren Kommunikation und zum Schutz von Dateien und E-Mails verwendet wird. Es kombiniert **symmetrische und asymmetrische Verschlüsselung**.

## Funktionsweise

PGP verwendet ein Schlüsselpaar:
- **Public Key (öffentlicher Schlüssel):** Darf weitergegeben werden.
- **Private Key (privater Schlüssel):** Muss geheim bleiben.

### 1. Verschlüsseln einer Nachricht
1. Der Absender erstellt einen zufälligen Sitzungsschlüssel.
2. Die Nachricht wird mit dem Sitzungsschlüssel symmetrisch verschlüsselt.
3. Der Sitzungsschlüssel wird mit dem öffentlichen Schlüssel des Empfängers asymmetrisch verschlüsselt.
4. Die verschlüsselte Nachricht und der verschlüsselte Sitzungsschlüssel werden an den Empfänger gesendet.

### 2. Entschlüsseln einer Nachricht
1. Der Empfänger entschlüsselt den Sitzungsschlüssel mit seinem privaten Schlüssel.
2. Mit dem Sitzungsschlüssel entschlüsselt er die Nachricht.

## Digitale Signatur

PGP kann Nachrichten zusätzlich digital signieren.

1. Der Absender erstellt einen Hashwert der Nachricht.
2. Dieser Hashwert wird mit dem privaten Schlüssel des Absenders signiert.
3. Der Empfänger prüft die Signatur mithilfe des öffentlichen Schlüssels des Absenders.

Die digitale Signatur ermöglicht:
- **Authentizität:** Prüfung der Identität des Absenders, sofern der öffentliche Schlüssel zuverlässig zugeordnet ist.
- **Integrität:** Erkennung nachträglicher Veränderungen der Nachricht.
- **Nichtabstreitbarkeit:** Kann die Zuordnung einer Signatur unterstützen, ist aber rechtlich und technisch nicht uneingeschränkt garantiert.

**Wichtig:** Eine digitale Signatur verschlüsselt nicht automatisch den Nachrichteninhalt.

## Vorteile
- Vertrauliche Kommunikation
- Schutz vor unbefugtem Lesen
- Erkennung von Manipulationen durch digitale Signaturen
- Effiziente Verschlüsselung durch Kombination beider Verfahren

## Nachteile
- Schlüsselverwaltung kann aufwendig sein.
- Der öffentliche Schlüssel muss zuverlässig dem richtigen Empfänger zugeordnet werden.
- Geht der private Schlüssel verloren, können verschlüsselte Nachrichten unter Umständen nicht mehr entschlüsselt werden.
- Ein kompromittierter privater Schlüssel kann die Sicherheit gefährden.

## PGP vs. symmetrische und asymmetrische Verschlüsselung

| Verfahren | Funktionsweise | Beispiel |
|---|---|---|
| Symmetrisch | Derselbe Schlüssel verschlüsselt und entschlüsselt. | AES |
| Asymmetrisch | Öffentliches und privates Schlüsselpaar | RSA |
| PGP | Kombiniert symmetrische und asymmetrische Verfahren | E-Mail-Verschlüsselung |

## PGP vs. S/MIME

| Merkmal | PGP / OpenPGP | S/MIME |
|---|---|---|
| Vertrauensmodell | Web of Trust, Fingerprint-Prüfung | hierarchische PKI mit CA (X.509) |
| Schlüssel/Zertifikat | selbst erzeugt, über Keyserver oder persönlich verteilt | Zertifikat von einer CA |
| Verbreitung | technikaffine Nutzer, Open Source | Unternehmen, in Outlook & Co. integriert |

## Prüfungsfrage

**Frage:** Erläutern Sie, warum PGP sowohl symmetrische als auch asymmetrische Verfahren verwendet.

**Musterantwort:** Die Nachricht selbst wird mit einem zufälligen Sitzungsschlüssel symmetrisch verschlüsselt, weil das auch bei großen Anhängen schnell ist. Damit der Empfänger diesen Sitzungsschlüssel erhält, ohne dass vorher ein Geheimnis ausgetauscht werden muss, wird er mit dem öffentlichen Schlüssel des Empfängers asymmetrisch verschlüsselt und mitgeschickt. So verbindet PGP die Geschwindigkeit symmetrischer mit dem einfachen Schlüsselaustausch asymmetrischer Verfahren.

## Falle

- PGP mit TLS verwechseln: TLS schützt nur den **Transportweg** zwischen Servern, PGP schützt die Nachricht **Ende-zu-Ende**, auch im Postfach.
- Verlust des privaten Schlüssels unterschätzen: ohne ihn sind alte verschlüsselte Mails nicht mehr lesbar.

## Merksatz
PGP kombiniert **symmetrische Verschlüsselung für die Nachricht** mit **asymmetrischer Verschlüsselung für den Sitzungsschlüssel**. Digitale Signaturen können zusätzlich die Authentizität und Integrität prüfen.

## Verwandt

- [Digitale Signatur](digitale-signatur.md)
- [Symmetrische und asymmetrische Verschlüsselung](symmetrische-und-asymmetrische.md)
- [PKI und Zertifikate](pki.md) – CA-Modell von S/MIME
- [E-Mail-Sicherheit](../email-sicherheit.md)
- [Lektion 27 PKI, TLS, VPN](../../../analyse-der-exams/L27_PKI_TLS_Verschluesselung_VPN.md)
