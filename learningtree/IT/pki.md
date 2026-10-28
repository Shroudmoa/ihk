# PKI

# PKI / CA / TLS-Infrastruktur – IHK-Referenz

---

## 1. Kryptografisches Fundament
Asymmetrie: Ein Schlüsselpaar (Public Key `PK`, Private Key `SK`).
`PK` verschlüsselt / verifiziert, `SK` entschlüsselt / signiert.
**Voraussetzung für Vertrauen:** Authentizität des `PK`. Ohne Bindung an Identität ist `PK` wertlos (Man-in-the-Middle trivial).

---

## 2. Zertifikat (X.509 v3) – Datenstruktur
Bindet `PK` an Distinguished Name (DN: CN, O, OU, C, …).
Felder (Auszug):
| Feld | Inhalt | Relevanz |
|------|--------|----------|
| **Subject** | Identität (CN = FQDN / UPN) | Validierung Host/User |
| **Issuer** | DN der signierenden CA | Kettenaufbau |
| **Validity** | NotBefore / NotAfter | Ablauf = Vertrauensverlust |
| **Subject Public Key Info** | Algorithmus + `PK` | Krypto-Operationen |
| **Extensions** | SAN, KeyUsage, EKU, AIA, CDP, OCSP | Policy-Enforcement |
| **SignatureAlgorithm + Signature** | Hash (SHA-256+) + CA-Signatur (`SK_CA`) | Integrität & Authentizität |

**Signaturprüfung:** `Verify(PK_CA, TBSCertificate, Signature) == true` → Zertifikat unverfälscht & vom Aussteller autorisiert.

---

## 3. Certificate Authority (CA) – Rollen & Hierarchie
| Rolle | Aufgabe | Schlüsselmanagement |
|-------|---------|---------------------|
| **Root CA** | Trust Anchor, self-signed, offline | `SK_Root` in HSM, air-gapped, Lebenszeit 15–25 J. |
| **Intermediate CA** | Issuing CA, signiert End-Entity-Certs | `SK_Int` in HSM, online, Lebenszeit 5–10 J., Cross-Signing möglich. |
| **End-Entity (Leaf)** | Server (TLS), Client (Auth/Mail), Code Signing | `SK_Leaf` auf Host / Smartcard / Token. |

**Policy:** Root signiert nur Intermediates. Intermediates signieren Leaves. Root nie direkt Leaves (Key Compromise Impact Minimierung).

---

## 4. Vertrauenskette (Chain of Trust) – Validation Algorithm
1. **Trust Anchor** lokal im OS/Browser Root Store (ca. 150 Roots).
2. **Chain Building:** Leaf → Intermediate(s) → Root (via Authority Key Identifier / Subject Key Identifier Matching).
3. **Path Validation (RFC 5280):**
   - Signaturprüfung jedes Hops (`PK_Issuer` verifiziert `Sig_Child`).
   - Zeitprüfung (`Now ∈ Validity`).
   - **Revocation Check:** CRL (Port 80/443 HTTP) **oder** OCSP (Port 80/443 HTTP) **oder** OCSP Stapling (TLS Extension).
   - **Policy Constraints:** KeyUsage (digitalSignature, keyEncipherment, keyCertSign…), ExtendedKeyUsage (serverAuth, clientAuth, codeSigning…), Name Constraints, Basic Constraints (CA:true/false, PathLen).
4. **Ergebnis:** `Valid` / `Invalid` / `Unknown`. Fehler = TLS-Alert `certificate_unknown` / `bad_certificate` / `certificate_revoked`.

---

## 5. TLS-Handshake (TLS 1.2 / 1.3) – PKI-Integration
| Schritt | Krypto | PKI-Bezug |
|---------|--------|-----------|
| **ClientHello** | – | Sendet Supported Groups, Signature Algorithms, `server_name` (SNI). |
| **ServerHello** | – | Wählt Cipher Suite (z. B. `TLS_AES_256_GCM_SHA384`). |
| **Certificate** | – | Sendet **Leaf + Intermediates** (kein Root). |
| **CertificateVerify** (TLS 1.3) | `Sign(SK_Leaf, Transcript-Hash)` | Beweist Besitz von `SK_Leaf`. |
| **Finished** | HMAC/ AEAD | Bestätigt Integrität des Handshakes. |

**Port:** 443 (HTTPS), 587 (SMTP Submission), 993 (IMAPS), 995 (POP3S), 636 (LDAPS), 8443 (Alt-HTTPS), 21 (FTPES), 990 (FTPS).

---

## 6. Widerruf & Lebenszyklus – DSGVO-Relevanz
| Mechanismus | Port/Protokoll | Latency | DSGVO-Aspekt |
|-------------|----------------|---------|--------------|
| **CRL** | HTTP (80/443) | Hoch (volle Liste) | Art. 32 – Verfügbarkeit der Prüfdaten. |
| **OCSP** | HTTP (80/443) | Mittel (Request/Response) | Responder loggt SerialNumber → personenbezogen (IP + Cert). |
| **OCSP Stapling** | TLS Extension | Niedrig (Server cached Response) | Kein Client→CA Traffic, Metadaten-Schutz. |
| **CRLite / CT Logs** | HTTPS (443) | Client-seitig | Transparenz, Missbrauchserkennung (Art. 25 Privacy by Design). |

**Lifecycle:** Key Gen → CSR (PKCS#10) → Validation (DV/OV/EV) → Issuance → Deployment → Monitoring (CT, Expiry) → Renewal / Revocation → Destruction (`SK` Zeroize).

---

## 7. OSI-Modell Mapping
| Layer | PKI/TLS Bezug |
|-------|---------------|
| **L7 Application** | HTTPS, SMTPS, IMAPS, LDAPS – Protokolle nutzen TLS. |
| **L6 Presentation** | **TLS/SSL** – Verschlüsselung, Kompression, Serialisierung (ASN.1/DER). Zertifikatsvalidierung passiert hier. |
| **L5 Session** | TLS-Session-ID / Session-Ticket (Resumption), PSK. |
| **L4 Transport** | TCP (Port 443, 587, …) – zuverlässiger Byte-Stream für TLS Records. |
| **L3 Network** | IPsec (ESP/AH) nutzt eigene PKI (IKEv2 + X.509), Port 500/4500 UDP. |
| **L2 Data Link** | 802.1X (EAP-TLS) – Client/Server Certs für Port-Auth. |
| **L1 Physical** | Kein direkter Bezug. |

---

## 8. Key & Zertifikatsformate (Interoperabilität)
| Format | Kodierung | Inhalt | Typischer Use-Case |
|--------|-----------|--------|-------------------|
| **PEM** | Base64 + Header/Footer (`-----BEGIN CERTIFICATE-----`) | Cert, Key, Chain, CSR | Linux/Apache/Nginx, OpenSSL CLI. |
| **DER** | Binär (ASN.1) | Cert, Key | Java/Windows, Smartcards. |
| **PKCS#7 / .p7b** | DER/PEM | Cert Chain (ohne Key) | IIS, Java Keystore Import. |
| **PKCS#12 / .pfx / .p12** | DER, Passwort-verschlüsselt (PBES2) | **Privater Key + Cert Chain** | Windows, IIS, Load Balancer, Client-Zert-Export. |
| **JKS / PKCS12 (Java)** | Proprietär / Standard | KeyStore / TrustStore | Tomcat, Jetty, Keytool. |

**OpenSSL Cheats (Prüfungswissen):**
```bash
# CSR + Key erzeugen
openssl req -newkey rsa:4096 -nodes -keyout server.key -out server.csr -subj "/CN=example.de/O=Firma/OU=IT/C=DE"

# Self-Signed (nur Lab!)
openssl x509 -req -in server.csr -signkey server.key -out server.crt -days 365 -sha256 -extfile <(echo "subjectAltName=DNS:example.de,DNS:www.example.de\nkeyUsage=digitalSignature,keyEncipherment\nextendedKeyUsage=serverAuth")

# PKCS#12 Export
openssl pkcs12 -export -inkey server.key -in server.crt -certfile chain.crt -out server.pfx -passout pass:Geheim123

# Chain Validierung
openssl verify -CAfile rootCA.crt -untrusted intermediate.crt server.crt
```

---

## 9. Häufige Prüfungsfallen (Savage Checklist)
1. **Root CA im Vertrauensspeicher ≠ Sicherheit.** Wenn `SK_Root` kompromittiert → komplette PKI wertlos.
2. **Fehlende SAN (Subject Alternative Name).** Chrome/Edge lehnen CN-only Certs ab (seit 2017).
3. **KeyUsage / EKU Mismatch.** Server-Zert ohne `serverAuth` EKU = Validierungsfehler.
4. **Incomplete Chain.** Server sendet nur Leaf → Client kann nicht zum Root bauen → `certificate_unknown`.
5. **OCSP Stapling nicht konfiguriert.** Client leakht Metadaten an CA, Latenz, DSGVO-Verstoß (Art. 25).
6. **Algorithmen-Schwäche.** SHA-1, RSA < 2048 Bit, ECDSA P-256 nur mit RFC 8422 Kurven. BSI TR-02102-1 beachten.
7. **Private Key auf Webserver lesbar (world-readable).** `chmod 600`, User separation, ideal HSM / TPM / Cloud KMS.
8. **Wildcard (*.domain.de) auf Public-Suffix (z. B. *.de).** Verboten (CA/B Forum Baseline Requirements).
9. **CT Log Monitoring fehlt.** Rogue Certs werden nicht erkannt → Identitätsdiebstahl.
10. **Automatisierung (ACME/Certbot) ohne Rate-Limit-Handling.** Produktionsausfall durch Blockierung.

---

## 10. DSGVO & Compliance Mapping
| Anforderung | Technische Umsetzung PKI |
|-------------|--------------------------|
| **Art. 5(1)(f) Integrität & Vertraulichkeit** | TLS 1.2+ mit AEAD (GCM/ChaCha20-Poly1305), Forward Secrecy (ECDHE). |
| **Art. 25 Privacy by Design** | OCSP Stapling, Certificate Transparency Monitoring, Minimale Cert-Lifetime (90 Tage via ACME). |
| **Art. 32 Sicherheit der Verarbeitung** | HSM für CA/Leaf Keys, Key Rotation Policy, Revocation Process < 24h. |
| **Art. 33/34 Meldung Verletzungen** | CT-Log Alerting → unverzügliche Revocation → Neubestellung. |
| **Art. 28 Auftragsverarbeitung** | Externe CA = Auftragsverarbeiter → AV-Vertrag, Audit-Recht (WebTrust/ETSI EN 319 411). |

---

## 11. Kurze Definitionsliste für 3-Zeilen-Antworten
- **PKI:** Hierarchisches System aus CAs, RAs, Verzeichnissen und Policies zur Ausstellung, Verteilung, Validierung und Widerruf von X.509-Zertifikaten.
- **Vertrauensanker (Trust Anchor):** Selbstsigniertes Root-CA-Zertifikat, das implizit vertraut wird (im OS/Browser Root Store).
- **Chain of Trust:** Kryptografische Verkettung Leaf → Intermediate(s) → Root, validiert durch Signaturen, Zeit, Revocation, Policy.
- **OCSP Stapling:** Server liefert zeitlich signierte OCSP-Antwort im TLS-Handshake mit, verhindert Client→CA-Requests (Performance, Privacy).
- **Certificate Transparency (CT):** Öffentliche, append-only Logs aller ausgestellten Zertifikate; ermöglicht Monitoring & Missbrauchserkennung.
- **HSM (Hardware Security Module):** FIPS 140-2/3 Level 3+ Gerät für Schlüsselerzeugung, -speicherung und Signieroperationen; Export von `SK` unmöglich.
- **ACME (RFC 8555):** Automatisiertes Protokoll (HTTP-01, DNS-01, TLS-ALPN-01) für Zertifikat-Lifecycle-Management (Certbot, Windows ACME).

---

acasts
