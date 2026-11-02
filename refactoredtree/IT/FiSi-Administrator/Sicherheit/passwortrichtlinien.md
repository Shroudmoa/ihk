# Passwortrichtlinien und sichere Passwortspeicherung

## In 30 Sekunden

- Eine **Passwort-Policy** legt fest, wie Passwörter aussehen, wie oft Fehlversuche erlaubt sind und wie sie gespeichert werden (Prüfung 2025W: zwei weitere Regeln nennen).
- Wichtigstes Kriterium ist die **Länge** (mindestens 12 Zeichen, für Admins länger), danach Zeichenvielfalt und **keine Wörterbuch- oder geleakten Passwörter**.
- **Kontosperre** bzw. Verzögerung nach Fehlversuchen bremst Brute-Force-Angriffe.
- Regelmäßiger Zwangswechsel ist **nicht mehr Stand der Technik** (BSI, NIST) – gewechselt wird **anlassbezogen**, z. B. nach einem Leak.
- Passwörter werden nie im Klartext gespeichert, sondern als **Passwort-Hash mit Salt** (Argon2, bcrypt). Ein Passwort allein reicht für kritische Zugänge nicht → **MFA**.

## Kernwissen

**Typische Regeln einer Passwort-Policy**

| Regel | Beispiel | Wirkung |
|---|---|---|
| Mindestlänge | ≥ 12 Zeichen, Admin-/Dienstkonten ≥ 16–20 | Hauptfaktor gegen Brute Force |
| Komplexität | Groß-/Kleinbuchstaben, Ziffern, Sonderzeichen – oder lange Passphrase aus mehreren Wörtern | vergrößert den Suchraum |
| Sperrliste | keine Wörterbuchwörter, Tastaturmuster („qwertz“, „123456“), Firmenname, bekannte geleakte Passwörter | verhindert Wörterbuchangriffe |
| Historie | die letzten n Passwörter nicht wiederverwenden | verhindert Zurückwechseln |
| Kontosperre | z. B. nach 5 Fehlversuchen 15 min Sperre | bremst Online-Raten |
| Keine Mehrfachnutzung | dienstliches Passwort nirgendwo sonst verwenden | schützt vor Credential Stuffing |
| Initialpasswort | individuell, muss bei erster Anmeldung geändert werden | kein Standardpasswort im Umlauf |
| Wechsel | **anlassbezogen** (Verdacht, Leak, Personalwechsel) | statt erzwungenem 90-Tage-Wechsel |
| Werkzeug | Passwortmanager erlauben und bereitstellen | ermöglicht lange, einzigartige Passwörter |

**Umsetzung im Windows-Netz:** per **GPO** (Kontorichtlinien: Kennwortrichtlinie, Kontosperrungsrichtlinie) für die ganze Domäne; abweichende Regeln für Admin-Gruppen über **Fine-Grained Password Policies**. Lokale Admin-Kennwörter mit **LAPS** pro Gerät unterschiedlich und automatisch rotieren.

**Rechnen mit dem Suchraum:** Anzahl Kombinationen = Zeichenvorrat ^ Länge. Beispiel: 8 Zeichen aus 94 druckbaren Zeichen ≈ 6 · 10¹⁵; 12 Zeichen ≈ 4,8 · 10²³ – die Länge wirkt exponentiell.

**Sichere Speicherung**
- nie Klartext, nie umkehrbar verschlüsselt, nie schneller Hash wie MD5 oder einfaches SHA-256
- **langsamer Passwort-Hash**: Argon2id, bcrypt, scrypt oder PBKDF2 mit vielen Iterationen
- **Salt**: zufällig und pro Benutzer verschieden – gleiche Passwörter ergeben verschiedene Hashes, Rainbow Tables werden nutzlos
- optional **Pepper**: zusätzliches Geheimnis, getrennt von der Datenbank gespeichert

**Angriffe, gegen die die Policy wirkt:** Brute Force, Wörterbuchangriff, Rainbow Tables (offline), Credential Stuffing (Zugangsdaten aus fremden Leaks), Password Spraying (ein häufiges Passwort gegen viele Konten).

## Prüfungsfrage

**Frage:** Für den VPN-Zugang eines Unternehmens gilt bereits die Regel „mindestens ein Großbuchstabe und eine Ziffer“. Nennen Sie zwei weitere sinnvolle Regeln der Passwort-Policy und eine ergänzende Maßnahme, die über Passwörter hinausgeht.

**Musterantwort:** Erstens eine Mindestlänge von 12 Zeichen, weil die Länge den Aufwand für Brute-Force-Angriffe am stärksten erhöht. Zweitens ein Verbot von Wörterbuchwörtern, Tastaturmustern und bekannten geleakten Passwörtern, damit Wörterbuchangriffe ins Leere laufen; ergänzend eine Kontosperre nach mehreren Fehlversuchen. Über Passwörter hinaus sollte für den VPN-Zugang eine Multi-Faktor-Authentifizierung, etwa mit TOTP-App oder Hardware-Token, eingeführt werden, damit ein erbeutetes Passwort allein nicht genügt.

## Falle

- Nur „Sonderzeichen“ nennen und die **Länge** vergessen → die Länge ist der wichtigste Faktor.
- „Passwort alle 30 Tage ändern“ als beste Maßnahme verkaufen → führt zu Mustern wie „Sommer2026!“; aktuell gilt anlassbezogener Wechsel. In älteren IHK-Lösungen steht der 90-Tage-Wechsel noch – nennen kann man ihn, aber die moderne Sicht kurz begründen.
- Hash und Verschlüsselung verwechseln: Passwörter werden **gehasht**, nicht verschlüsselt.
- Passwort + PIN als MFA bezeichnen → beides ist **Wissen**.

## Merksatz

**„Lang schlägt kompliziert, einzigartig schlägt oft gewechselt – und gespeichert wird nur gesalzen gehasht.“**

## Verwandt

- [MFA](MFA.md) – der zweite Faktor
- [Hash, Salt und HMAC](Verschluesselung/hash.md) – Passwort-Hashing im Detail
- [GPO](../Netzwerk-Administration/gpo.md) – Kennwort- und Kontosperrungsrichtlinie verteilen
- [Active Directory](active-directory.md)
- [OS-Härtung](../Server/os-haertung.md)
- [Angriffsarten](hacking/hackers-can.md) – Brute Force, Credential Stuffing
- [Lektion 29 GA2-Praxisthemen (Passwort-Policy und MFA)](../../analyse-der-exams/L29_GA2_Sonstiges_Praxisthemen.md)
