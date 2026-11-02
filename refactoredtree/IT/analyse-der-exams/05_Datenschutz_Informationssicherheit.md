# 5. Datenschutz & Informationssicherheit

*Quelle: GA1 Aufgabe 2 (2024 Schulung), Aufgabe 4 (2021), 2024/2025*

> ✅ **Verifiziert am echten Prüfungskorpus (10 Jahrgänge 2021–2026):**
> Datenschutz/DSGVO ist mit **8 von 10 Jahrgängen** eines der häufigsten Themen
> (Meldepflicht Datenpanne: 2021W, 2024S, 2025S; TOM-Zuordnung: 2022W, 2024S;
> Anonymisierung vs. Pseudonymisierung: 2021W; Cloud+Datenschutz/AV-Vertrag:
> 2026S). **Asymmetrische Verschlüsselung/PKI ist KEIN Randthema**, sondern
> kam in **5 von 10 Jahrgängen** vor (2023S, 2023W, 2024W, 2026S; 2021W nur
> symmetrisch) – siehe neuer Abschnitt 5.6b unten. Die Verschlüsselungstabelle
> in 5.6 stammt konkret aus **2021W** und enthielt dort **kein RSA**
> (RSA/asymmetrisch war erst ab 2023 Thema).

## 5.2 TOM – Technische und organisatorische Maßnahmen

Zuordnung üben (2024 Muster):

| Situation | TOM |
|-----------|-----|
| RFID-Karte fürs Gebäude | **Zutrittskontrolle** (physisch rein) |
| Biometrie am PC | **Zugangskontrolle** (an System anmelden) |
| Berechtigungskonzept Dateiebene | **Zugriffskontrolle** (was darf genutzt werden) |
| Datenträger verschlüsseln | Zugriffskontrolle / Datenschutz |
| Alarmanlage außerhalb Geschäftszeit | Zutrittskontrolle |

## 5.3 Phishing erkennen (E-Mail-Analyse)

Typische Merkmale (nenne 3):
- Unspezifische Anrede („Lieber Kunde")
- Absender-Adresse privat (z. B. `@gmail.de`) statt Firmenadresse
- Empfänger nur im **CC** / „undisclosed recipients"
- Link-Text ≠ tatsächliche URL (Hover zeigt `yahoo.ru`)
- Dringlichkeit/Zeitdruck („12 Stunden!")
- Rechtschreib- / Grammatikfehler, keine Signatur

**Folgen eines Klicks** (nenne 3):
- Hohe CPU-Auslastung
- Häufige Abstürze
- Unbekannte Prozesse/Programme
- Geänderte/gelöschte Daten
- Eigenmächtig versendete E-Mails

## 5.4 Malware-Typen

| Typ | Merkmal |
|-----|---------|
| **Virus** | Hängt sich an andere Programme, braucht Wirt |
| **Wurm** | Repliziert sich selbst über Netzwerk |
| **Trojaner** | Gibt sich nützlich, agiert im Hintergrund |
| **Ransomware** | Verschlüsselt Daten, fordert Lösegeld |

## 5.5 White-Hat vs. Black-Hat

- **White-Hat:** Dringt im Auftrag des Eigentümers ein, um Schwachstellen zu
  finden und zu schließen.
- **Black-Hat:** Nutzt Schwachstellen für eigenen Vorteil (Diebstahl, Schaden).

## 5.6 Verschlüsselungsalgorithmen

> ⚠️ **Korrektur:** Dies ist die **originale Tabelle aus 2021W** – dort ging
> es **ausschließlich um symmetrische Verschlüsselung und Hashfunktionen**.
> **RSA war in dieser 2021er-Tabelle NICHT enthalten** (kein asymmetrisches
> Verfahren dabei). Verwechsle das nicht mit späteren Jahrgängen, in denen
> RSA/asymmetrische Verfahren separat drankamen (siehe 5.6b).

| Algorithmus | Art | Einschätzung (2021W) |
|-------------|-----|--------------|
| **3DES** | symmetrisch | unsicher |
| **AES-128** | symmetrisch | bedingt geeignet |
| **AES-256** | symmetrisch | gut geeignet |
| **MD5** | Hash | nicht geeignet/veraltet |
| **SHA-256** | Hash | sicher, aber **keine Verschlüsselung** (Hashfunktion, nicht umkehrbar) |

Symmetrisch = gleicher Schlüssel zum Ver-/Entschlüsseln.
Hash = Einweg, Prüfsumme (kein Entschlüsseln).

## 5.6b Asymmetrische Verschlüsselung & PKI (wichtig!)

> Dieses Thema fehlte bisher komplett, kam aber in **5 von 10 Jahrgängen**
> vor: **2023S, 2023W, 2024W, 2026S** (2021W war noch rein symmetrisch).

**Prinzip:** Es gibt ein Schlüsselpaar – einen **öffentlichen Schlüssel**
(public key, wird verteilt) und einen **privaten Schlüssel** (private key,
bleibt geheim). Was mit dem einen verschlüsselt wird, kann nur mit dem
anderen entschlüsselt werden.

**Nachteil ggü. symmetrischer Verschlüsselung:** deutlich **langsamer**
(höherer Rechenaufwand) – deshalb in der Praxis oft nur für
Schlüsselaustausch/Signatur, nicht für große Datenmengen verwendet.

### 2023S – Zertifikats-Einrichtung (4 Schritte)
1. Zertifikat bei einer **Certificate Authority (CA)** beantragen
2. Privaten Schlüssel/Zertifikat **schützen** (sicher aufbewahren)
3. Zertifikat auf dem System **installieren**
4. **Übergeordnete Zertifikate** (Zertifikatskette/Root-CA) einbinden

### 2023W – DES/AES/RSA im Vergleich
| Verfahren | Einschätzung |
|---|---|
| **DES** | unsicher (nur 56-Bit-Schlüssel) |
| **AES** | empfohlen (symmetrisch, aktueller Standard) |
| **RSA** | **nicht** für Datenträgerverschlüsselung geeignet – eher für **Schlüsselaustausch** und **digitale Signatur** |

### 2024W – Asymmetrischer E-Mail-Workflow (S/MIME-artig, X.509-Zertifikate)
1. **Signieren** der E-Mail mit dem **eigenen privaten** Schlüssel
2. **Verschlüsseln** mit dem **öffentlichen** Schlüssel des Empfängers
3. Empfänger **entschlüsselt** mit seinem **eigenen privaten** Schlüssel
4. Empfänger prüft die **Signatur** mit dem **öffentlichen** Schlüssel des Senders

### 2026S – TLS-Handshake & CA-Aufgaben
- Ablauf: **ClientHello** → **ServerHello** → Zertifikatsprüfung →
  Aushandlung eines symmetrischen **Session-Key** für die eigentliche
  Datenübertragung (Hybridverfahren: asymmetrisch für den Schlüsselaustausch,
  symmetrisch für die Nutzdaten).
- Aufgaben einer **CA (Certificate Authority)**: Identitätsprüfung des
  Antragstellers, **Signierung** des Zertifikats, Sperrlisten-/Statusprüfung
  über **CRL** (Certificate Revocation List) bzw. **OCSP**.

## 5.7 DSGVO – Datenschutzkonzept & Meldepflicht

**Inhalt eines Datenschutzkonzepts** (nenne 4, 2024):
- Verantwortliche Stelle / Datenschutzbeauftragter
- Verzeichnis der Verarbeitungstätigkeiten
- IT-Sicherheitsmaßnahmen (Updates, Firewall, AV, Verschlüsselung)
- Notfallplan bei Verletzungen
- Löschfristen

**Bei Daten-Leck an falschen Empfänger** (nenne 3, DSGVO):
- Meldung an Datenschutzbehörde
- Kontakt zum Empfänger → Löschung fordern
- Betroffene Kunden informieren
- Dokumentation des Vorfalls
- Interne Maßnahmen (Schulung etc.)

**Meldefrist (2021W):** Datenpannen müssen binnen **72 Stunden** sowohl an
die **zuständige Behörde** als auch an die **Betroffenen** gemeldet werden.
Dieses Thema kam auch 2024S vor; **2025S** wurden dazu **3 Kriterien**
abgefragt (wann/wie/an wen eine Datenpanne zu melden ist).

## 5.8 Anonymisierung vs. Pseudonymisierung

- **Pseudonymisierung:** Personenbezug wird durch Kennung ersetzt,
  **reversibel** (kann zurückgeführt werden). Beispiel: Kundennummer statt Name.
- **Anonymisierung:** Daten unwiderruflich vom Personenbezug gelöst,
  **nicht** rekonstruierbar.

## 5.9 Datenträger-Entsorgung

- Formatieren/Löschen reicht **nicht** (restaurierbar)!
- Maßnahmen: **Überschreiben** (mehrfach), **physische Zerstörung**
  (Schreddern), **Degaussen** (Magnetfeld löschen).

## Typische Prüfungsaufgabe

> „Nennen Sie drei Kriterien, die aus Datenschutzsicht bei der Speicherung
> von Daten zu beachten sind."

→ z. B. Datenminimierung, Zweckbindung, Löschfristen, Einwilligung.
