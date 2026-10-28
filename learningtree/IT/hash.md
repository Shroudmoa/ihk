# Hash

# Hashing und verwandte Themen – IHK AP2

## 1. Was ist ein Hash?

Eine **Hashfunktion** wandelt Daten beliebiger Länge in einen Hashwert fester Länge um.

Beispiel mit SHA-256:

```text
Eingabe:  Hallo
Hash:     185f8db32271fe25f561a6fc938b2e26...
```

Bereits eine kleine Änderung der Eingabe erzeugt normalerweise einen völlig anderen Hashwert.

**Eigenschaften einer kryptografischen Hashfunktion:**
- Deterministisch: Gleiche Eingabe → gleicher Hashwert.
- Feste Ausgabelänge: SHA-256 erzeugt immer 256 Bit.
- Einweg-Eigenschaft: Aus dem Hash soll die ursprüngliche Eingabe praktisch nicht berechenbar sein.
- Kollisionsresistenz: Es soll praktisch unmöglich sein, gezielt zwei unterschiedliche Eingaben mit demselben Hash zu finden.
- Lawineneffekt: Kleine Eingabeänderungen führen zu stark veränderten Hashwerten.

**Wichtig:** Hashing ist keine Verschlüsselung. Ein Hash besitzt keinen Schlüssel, mit dem man ihn einfach entschlüsseln kann.

## 2. Hashing vs. Verschlüsselung vs. Encoding

| Verfahren | Umkehrbar? | Schlüssel? | Zweck | Beispiel |
|---|---|---|---|---|
| Hashing | Nicht praktisch umkehrbar | Nein | Integritätsprüfung, Passwortspeicherung | SHA-256 |
| Symmetrische Verschlüsselung | Ja, mit richtigem Schlüssel | Ja | Daten vertraulich speichern/übertragen | AES |
| Asymmetrische Verschlüsselung | Ja, mit passendem Schlüssel | Ja, Schlüsselpaar | Vertrauliche Kommunikation | RSA-OAEP |
| Encoding | Ja, ohne Geheimnis | Nein | Daten in anderes Format umwandeln | Base64 |

### Beispiele

**Hashing:**
```text
Datei → SHA-256 → Hashwert
```

**Verschlüsselung:**
```text
Klartext + Schlüssel → AES → Chiffretext
Chiffretext + Schlüssel → AES → Klartext
```

**Encoding:**
```text
Hallo → Base64 → SGFsbG8=
```

Base64 ist keine Verschlüsselung und bietet keinen Schutz vor unbefugtem Lesen.

## 3. Hashing und Integrität

Ein Hash kann verwendet werden, um festzustellen, ob sich Daten verändert haben.

Beispiel:
1. Hersteller veröffentlicht eine Datei und ihren SHA-256-Hash.
2. Du berechnest den Hash der heruntergeladenen Datei.
3. Du vergleichst beide Werte.
4. Stimmen sie überein, ist das ein Hinweis darauf, dass die Datei unverändert ist.

**Einschränkung:** Ein Hash allein beweist nicht, von wem die Datei stammt. Ein Angreifer könnte Datei und Hash gemeinsam ersetzen.

Für einen vertrauenswürdigen Herkunftsnachweis eignen sich beispielsweise digitale Signaturen.

## 4. Hashing bei Passwörtern

Passwörter sollten nicht im Klartext in einer Datenbank gespeichert werden.

### Sicherer Ablauf

Bei der Registrierung:

```text
Passwort
   ↓
Passwort-Hashing mit Salt
   ↓
Hashwert + benötigte Parameter speichern
```

Bei der Anmeldung:

```text
Eingegebenes Passwort
   ↓
Mit gespeicherten Parametern erneut berechnen
   ↓
Ergebnis vergleichen
   ↓
Übereinstimmung → Passwort korrekt
```

Das ursprüngliche Passwort muss dafür nicht wiederhergestellt werden.

### Warum nicht einfach SHA-256?

SHA-256 ist für viele allgemeine Hash-Anwendungen geeignet, aber für die Passwortspeicherung allein ungeeignet.

Der Grund: SHA-256 ist sehr schnell. Ein Angreifer kann bei einem gestohlenen Passwort-Hash sehr viele Passwortkandidaten ausprobieren.

Für die Passwortspeicherung verwendet man deshalb spezielle, absichtlich aufwendigere Verfahren:

| Verfahren | Einordnung |
|---|---|
| Argon2id | Bevorzugte Wahl für neue Anwendungen |
| scrypt | Speicherintensives Passwort-Hashing |
| bcrypt | Bewährtes Verfahren, häufig in älteren Systemen |
| PBKDF2 | Iteratives Verfahren, unter anderem bei bestimmten Compliance-Anforderungen |

Die Parameter müssen passend konfiguriert werden. Die konkrete Wahl hängt von System, Sicherheitsanforderungen und verfügbarer Rechenleistung ab.

Quelle: [OWASP – Password Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html)

## 5. Salt und Pepper

### Salt

Ein **Salt** ist ein zufälliger Wert, der beim Passwort-Hashing zusätzlich verwendet wird.

```text
Passwort + individueller Salt
              ↓
       Passwort-Hashing
              ↓
           Hashwert
```

Eigenschaften:
- Für jedes Passwort individuell.
- Wird zusammen mit dem Hash gespeichert.
- Muss nicht geheim sein.
- Erschwert vorberechnete Angriffe und den Vergleich gleicher Passwörter anhand ihrer Hashwerte.

Haben zwei Benutzer dasselbe Passwort, entstehen mit unterschiedlichen Salts normalerweise unterschiedliche Hashwerte.

### Pepper

Ein **Pepper** ist ein zusätzliches Geheimnis, das nicht zusammen mit den Passwort-Hashes in der Datenbank gespeichert wird.

| Salt | Pepper |
|---|---|
| Individuell pro Passwort | Üblicherweise für mehrere Passwörter gemeinsam |
| Darf öffentlich sein | Muss geheim bleiben |
| Wird beim Hashing verwendet | Wird zusätzlich in das Schutzverfahren eingebunden |
| Wird meist mit dem Hash gespeichert | Wird getrennt geschützt gespeichert |

Ein Pepper kann zusätzlichen Schutz bieten, wenn nur die Passwortdatenbank kompromittiert wird. Er ersetzt weder Salt noch ein geeignetes Passwort-Hashing-Verfahren.

## 6. Typische Angriffe auf Passwort-Hashes

### Brute Force
Alle möglichen Passwortkombinationen werden ausprobiert.

### Wörterbuchangriff
Häufige Passwörter und bekannte Passwortmuster werden ausprobiert.

### Rainbow Tables
Vorberechnete Zuordnungen von Eingaben zu Hashwerten werden verwendet. Individuelle Salts erschweren diese Angriffe erheblich.

### Credential Stuffing
Gestohlene Zugangsdaten werden bei anderen Diensten ausprobiert. Das funktioniert besonders häufig, wenn Passwörter wiederverwendet werden.

**Schutzmaßnahmen:**
- Lange, einzigartige Passwörter oder Passphrasen.
- MFA.
- Argon2id oder ein geeignet konfiguriertes alternatives Passwort-Hashing-Verfahren.
- Individuelle Salts.
- Rate-Limiting gegen automatisierte Login-Versuche.
- Keine Passwort-Wiederverwendung.

## 7. Hashfunktionen und wichtige Algorithmen

| Algorithmus | Hashlänge | Bewertung / Einsatz |
|---|---:|---|
| MD5 | 128 Bit | Kryptografisch unsicher; nicht für Sicherheitsprüfungen verwenden |
| SHA-1 | 160 Bit | Kollisionsunsicher; für neue Sicherheitsanwendungen vermeiden |
| SHA-256 | 256 Bit | Geeignet für viele allgemeine kryptografische Hash-Anwendungen |
| SHA-512 | 512 Bit | SHA-2-Familie, geeignet für allgemeine kryptografische Hash-Anwendungen |
| SHA-3 | Je nach Variante | Alternative Hashfamilie mit anderer Konstruktion |
| Argon2id | Konfigurationsabhängig | Passwortspeicherung |

**Achtung:** Die Hashlänge allein entscheidet nicht über die Sicherheit. Für Passwortspeicherung ist beispielsweise Argon2id geeigneter als ein einzelner schneller SHA-256-Aufruf.

## 8. HMAC – Hash mit geheimem Schlüssel

**HMAC** steht für *Hash-based Message Authentication Code*.

HMAC kombiniert eine kryptografische Hashfunktion mit einem geheimen Schlüssel.

Zweck:
- Integrität überprüfen.
- Authentizität der Nachricht anhand eines gemeinsamen Geheimnisses prüfen.

```text
Nachricht + geheimer Schlüssel
             ↓
            HMAC
             ↓
       Authentifizierungscode
```

Der Empfänger berechnet den HMAC mit demselben geheimen Schlüssel und vergleicht das Ergebnis.

**Unterschied zum normalen Hash:**
- Hash: kein geheimer Schlüssel notwendig.
- HMAC: gemeinsamer geheimer Schlüssel erforderlich.

HMAC ist keine Verschlüsselung: Die Nachricht bleibt lesbar, sofern sie nicht zusätzlich verschlüsselt wird.

## 9. Digitale Signatur

Eine digitale Signatur ermöglicht es, die Herkunft und Integrität von Daten zu prüfen.

Vereinfacht:

1. Der Absender berechnet einen Hash der Nachricht.
2. Das Signaturverfahren verwendet den privaten Schlüssel des Absenders.
3. Der Empfänger prüft die Signatur mit dem zugehörigen öffentlichen Schlüssel.
4. Eine erfolgreiche Prüfung bestätigt die Signatur unter den Voraussetzungen des Verfahrens und der vertrauenswürdigen Schlüsselzuordnung.

Eigenschaften:
- Integritätsprüfung.
- Authentizitätsnachweis.
- Unterstützt Nichtabstreitbarkeit, wobei die tatsächliche rechtliche Wirkung vom Verfahren und Kontext abhängt.

Eine digitale Signatur verschlüsselt die Nachricht nicht automatisch.

## 10. Hash, HMAC und digitale Signatur im Vergleich

| Merkmal | Hash | HMAC | Digitale Signatur |
|---|---|---|---|
| Geheimer Schlüssel | Nein | Ja, gemeinsamer Schlüssel | Privater Schlüssel beim Signieren |
| Integritätsprüfung | Ja, bei vertrauenswürdigem Vergleichswert | Ja | Ja |
| Herkunftsprüfung | Nicht allein | Bei geheim gehaltenem Schlüssel begrenzt möglich | Mit vertrauenswürdiger Schlüsselzuordnung |
| Verschlüsselung | Nein | Nein | Nein |
| Beispiel | SHA-256 | HMAC-SHA-256 | RSA-PSS, ECDSA |

## 11. Typische IHK-Prüfungsfallen

- Hashing ist keine Verschlüsselung.
- Base64 ist Encoding, keine Verschlüsselung.
- SHA-256 ist für allgemeine Hash-Anwendungen geeignet, aber allein nicht für Passwortspeicherung.
- Salt ist nicht geheim und wird pro Passwort individuell verwendet.
- Pepper ist geheim und wird getrennt von der Passwortdatenbank aufbewahrt.
- Ein Hash allein beweist nicht die Herkunft einer Datei.
- HMAC verwendet einen gemeinsamen geheimen Schlüssel.
- Digitale Signaturen verwenden ein asymmetrisches Schlüsselpaar.
- Hashing ist nicht automatisch sicher: Algorithmus und Einsatzzweck müssen passen.

## Merksatz

**Hash = Fingerabdruck von Daten.  
Salt = individueller Zusatz beim Passwort-Hashing.  
Pepper = zusätzliches Geheimnis.  
HMAC = Integrität und Authentizitätsprüfung mit gemeinsamem Schlüssel.  
Digitale Signatur = Integrität und Herkunftsnachweis mit asymmetrischen Schlüsseln.  
Verschlüsselung = Vertraulichkeit durch Schlüssel.**
