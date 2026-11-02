# TLS und HTTPS

## In 30 Sekunden

- **TLS** (Transport Layer Security) sichert eine Verbindung über TCP ab: **Vertraulichkeit** (Verschlüsselung), **Integrität** (AEAD/MAC) und **Authentizität des Servers** (X.509-Zertifikat).
- **HTTPS** = HTTP über TLS, Standardport **TCP 443**.
- TLS ist ein **Hybridverfahren**: asymmetrisch für Authentifizierung und Schlüsselaustausch, **symmetrisch** (z. B. AES-GCM) für die Nutzdaten.
- Ablauf grob: **ClientHello → ServerHello + Zertifikat → Zertifikatsprüfung → Session-Key → verschlüsselte Daten** (Prüfung 2026S, 2021W).
- Nur **TLS 1.2 und 1.3** sind noch zulässig; SSL und TLS 1.0/1.1 sind abgeschaltet.

## Kernwissen

**TLS-1.3-Handshake (vereinfacht, 1 Round Trip)**

| Schritt | Wer | Inhalt |
|---|---|---|
| 1. **ClientHello** | Client | unterstützte TLS-Versionen und Cipher Suites, Zufallswert, **Key Share** (Diffie-Hellman-Anteil), Servername (SNI) |
| 2. **ServerHello** | Server | gewählte Version und Cipher Suite, eigener **Key Share** – ab hier sind beide in der Lage, denselben Sitzungsschlüssel zu berechnen |
| 3. **Certificate + CertificateVerify** | Server | Zertifikat mit Kette (ohne Root) und Signatur über den Handshake – beweist den Besitz des privaten Schlüssels |
| 4. **Zertifikatsprüfung** | Client | Name (SAN) passt? Gültigkeitszeitraum? Kette bis zu einer vertrauenswürdigen Root-CA? Nicht gesperrt (CRL/OCSP)? |
| 5. **Finished** | beide | Prüfsumme über den gesamten Handshake – Manipulation würde auffallen |
| 6. **Anwendungsdaten** | beide | **symmetrisch** verschlüsselt mit dem ausgehandelten Sitzungsschlüssel |

**Schlüsselaustausch:** Heute per **ECDHE** (ephemeres Diffie-Hellman). Der Sitzungsschlüssel wird nie übertragen, sondern auf beiden Seiten berechnet. Da er nach der Sitzung verworfen wird, bleiben alte Sitzungen geschützt, selbst wenn später der private Server-Schlüssel gestohlen wird – das ist **Perfect Forward Secrecy (PFS)**.

**Warum nur noch TLS 1.3 (Prüfung 2022W, RFC 8446)?**
- unsichere Altverfahren entfernt (RC4, 3DES, SHA-1, CBC-Modus, statischer RSA-Schlüsselaustausch)
- **Forward Secrecy** ist Pflicht
- Handshake ab dem ServerHello **verschlüsselt** (Zertifikat nicht mehr im Klartext)
- schneller: 1-RTT statt 2-RTT

**TLS vs. reines TCP/IP (2022W):** TCP sorgt für zuverlässige Zustellung, schützt aber nichts – jeder auf dem Weg kann mitlesen und verändern. TLS ergänzt Verschlüsselung, Integritätsschutz und Serverauthentifizierung.

**OSI-Einordnung:** TLS liegt **oberhalb von TCP** (Schicht 4) und unterhalb des Anwendungsprotokolls; meist wird es den Schichten 5/6 zugeordnet. Deshalb funktioniert **NAT trotz TLS** (Prüfung 2025W): NAT ändert nur IP-Adressen und Ports in den Schichten 3/4, die verschlüsselten Nutzdaten bleiben unberührt. Anders bei IPsec ESP: dort fehlen Ports, daher NAT-Traversal.

**Weitere Anwendungen von TLS:** SMTPS/Submission (465/587 mit STARTTLS), IMAPS 993, POP3S 995, LDAPS 636, FTPS, VPN (SSL-VPN), EAP-TLS bei 802.1X.

**TLS-Inspection:** Proxy oder NGFW stellt sich als gewollter Man-in-the-Middle zwischen Client und Server und erzeugt für jede Seite ein eigenes, von der Firmen-CA signiertes Zertifikat. Clients müssen dieser CA vertrauen (Verteilung per GPO/MDM), sonst kommt `NET::ERR_CERT_AUTHORITY_INVALID` bzw. `SEC_ERROR_UNKNOWN_ISSUER` (2022S, 2024W).

**Interne oder externe CA? (2022W)**
- **Externe (öffentliche) CA** für Dienste, die Fremde nutzen: Browser vertrauen ihr bereits, Sperrdienste sind vorhanden.
- **Interne CA** für rein interne Dienste: kostenlos und flexibel, aber das Root-Zertifikat muss auf alle Clients verteilt und die PKI selbst betrieben werden.

## Prüfungsfrage

**Frage:** Beschreiben Sie, wie beim Aufruf einer HTTPS-Seite asymmetrische und symmetrische Verfahren zusammenwirken, und begründen Sie, warum man nicht alles asymmetrisch verschlüsselt.

**Musterantwort:** Beim Verbindungsaufbau sendet der Client ein ClientHello, der Server antwortet mit ServerHello und seinem Zertifikat. Der Client prüft das Zertifikat gegen die Vertrauenskette; mit dem öffentlichen Schlüssel des Servers wird dessen Identität bestätigt, und über einen Diffie-Hellman-Schlüsselaustausch vereinbaren beide einen gemeinsamen Sitzungsschlüssel. Die eigentlichen Daten werden anschließend symmetrisch, etwa mit AES, verschlüsselt. Asymmetrische Verfahren sind um ein Vielfaches langsamer und für große Datenmengen ungeeignet, daher nutzt man sie nur für Authentifizierung und Schlüsselaustausch – das ist das Hybridverfahren.

## Falle

- „Die Daten werden mit dem öffentlichen Schlüssel des Servers verschlüsselt“ → falsch, die Nutzdaten laufen **symmetrisch**.
- Das Schloss im Browser als Beweis für eine seriöse Seite werten → es bestätigt nur Verschlüsselung und Domain, nicht die Absicht des Betreibers.
- SSL und TLS gleichsetzen: SSL ist veraltet und abgeschaltet, der Name hält sich nur umgangssprachlich.
- Root-Zertifikat vom Server mitsenden lassen → unnötig, der Client hat die Root im eigenen Trust Store; fehlen aber die **Intermediates**, scheitert die Kettenprüfung.

## Merksatz

**„Asymmetrisch zum Kennenlernen, symmetrisch zum Reden.“**

## Verwandt

- [PKI und Zertifikate](pki.md) – CA, Vertrauenskette, CRL/OCSP
- [Symmetrische und asymmetrische Verschlüsselung](symmetrische-und-asymmetrische.md)
- [Digitale Signatur](digitale-signatur.md) – CertificateVerify und Zertifikatsignatur
- [Hash](hash.md) – Integrität und HMAC
- [Firewall-Generationen und TLS-Inspection](../firewall/firewalls-alle-generationen-sicherheitskonzepte-antivirus.md)
- [VPN](../../../nice-to-know/VPN/readme.md)
- [Ports](../../OSI-IP-mac-bit-byte/ports.md)
- [Lektion 27 PKI, TLS, VPN](../../../analyse-der-exams/L27_PKI_TLS_Verschluesselung_VPN.md)
