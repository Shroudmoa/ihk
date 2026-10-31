# IT-Sicherheit im Arbeitsalltag

> **IT-Sicherheit ist Arbeitsschutz für Daten.** Prüfungsrelevant: Phishing erkennen, öffentliches WLAN, Passwörter, Updates, Backup und Meldewege.

## Schutzziele

| Ziel | Bedeutung | Verletzung |
|---|---|---|
| **Vertraulichkeit** | nur Berechtigte lesen Daten | Datenleck, fremder Zugriff |
| **Integrität** | Daten sind unverändert und korrekt | Manipulation, Fälschung |
| **Verfügbarkeit** | Systeme und Daten sind nutzbar | Ausfall, Ransomware |
| **Authentizität** | Absender/Quelle ist echt | gefälschte E-Mail |
| **Verbindlichkeit** | Handlung ist nachweisbar | abstreitbare Bestellung |

**Merke:** Sicherheit ist nicht nur „Passwort“. Auch Backup, Rechte und Notfallplan gehören dazu.

**Kontrollsumme/Hash:** Eine Prüfsumme erkennt Veränderungen und schützt die **Integrität**. Sie verschlüsselt nicht und schützt weder Vertraulichkeit noch Verfügbarkeit.

## Phishing und Social Engineering

**Phishing:** gefälschte Nachricht soll zur Preisgabe von Daten oder zur Ausführung einer Datei verleiten.

Erkennungsmerkmale:
- Absenderadresse stimmt nicht mit Anzeigename überein.
- Druck, Frist, Drohung („Konto wird gesperrt“).
- Rechtschreibfehler, ungewöhnliche Anrede.
- Link führt zu fremder Domain; vor dem Klick prüfen.
- unerwarteter Anhang (.zip, .exe, .html, Makro-Dokument).
- Login-Seite fragt Passwort ab, obwohl man schon angemeldet ist.

**Richtig:** nicht klicken, nicht antworten, Anhang nicht öffnen, **IT/Melde­stelle informieren**, E-Mail löschen bzw. in Quarantäne verschieben.

**Falsch:** Link öffnen „nur zum Prüfen“, Daten eingeben, Anhang an Kollegen weiterleiten, Absender mit „sieht echt aus“ entschuldigen.

**Social Engineering:** Ausnutzung von Vertrauen, Autorität, Zeitdruck oder Hilfsbereitschaft. Auch Anrufe („CEO-Fraud“) und Besuche am Empfang gehören dazu.

## Passwörter und Anmeldung

- Lang und einzigartig, mindestens 12–16 Zeichen, Passphrase.
- **Kein** Passwort für mehrere Dienste.
- Passwortmanager statt Zettel.
- **2FA/MFA:** Wissen + Besitz (Token, App) oder Biometrie.
- Passwort niemals per E-Mail oder Telefon weitergeben.
- Bei Verdacht: sofort ändern, IT informieren.

**Prüfungsfalle:** Regelmäßiges Zwangswechseln ist heute nicht mehr pauschal empfohlen; entscheidend sind Länge, Einzigartigkeit und MFA. Ein starkes Passwort allein ersetzt keine MFA.

## Öffentliches WLAN

- Offene Netze sind nicht vertraulich; Mitlesen möglich.
- **Kein** Online-Banking, keine Firmenzugänge, keine vertraulichen Daten.
- Nur mit **VPN** des Unternehmens nutzen.
- HTTPS prüfen, aber HTTPS schützt nicht vor falschem Netzbetreiber.
- Automatische Verbindung zu offenen Netzen deaktivieren.
- Keine unbekannten USB-Ladegeräte/Ports („Juice Jacking“).

## Updates und Schadsoftware

- Updates zeitnah installieren; Sicherheitsupdates nicht aufschieben.
- Nur Software aus offiziellen Quellen.
- Makros in Office-Dokumenten standardmäßig deaktivieren.
- Ransomware: Dateien verschlüsselt, Erpressung; **nicht** zahlen, System isolieren, IT informieren.
- Virenschutz und Firewall aktiv lassen.

## Backup

**3-2-1-Regel:**
- **3** Kopien der Daten,
- **2** verschiedene Medien,
- **1** Kopie an einem anderen Ort (offline/offsite).

Backup regelmäßig **testen**. Ein ungetestetes Backup ist kein Backup. Ransomware verschlüsselt auch angeschlossene Netzlaufwerke; daher Offline-Kopie.

## Datenschutz im Alltag

- Nur notwendige personenbezogene Daten verarbeiten.
- Bildschirm sperren, Unterlagen wegräumen (Clean Desk).
- Keine Kundendaten auf privaten USB-Sticks oder Privat-Mail.
- Papier mit personenbezogenen Daten schreddern.
- Zugriffsrechte nach **Least Privilege**.
- Datenschutzvorfall melden; DSGVO-Fristen beachten.

## 3-Zeiler für die Prüfung

**Frage:** Ein Kollege erhält eine E-Mail „Ihr Konto wird gesperrt, jetzt Passwort bestätigen“. Was tun?

**Antwort:** Es handelt sich um einen Phishing-Versuch. Link nicht öffnen, keine Daten eingeben und die IT bzw. den Sicherheitsbeauftragten informieren. Die E-Mail sollte gelöscht oder in Quarantäne verschoben werden.

## Prüfungsfallen

- HTTPS allein bedeutet nicht „vertrauenswürdige Seite“.
- Öffentliches WLAN ohne VPN ist unsicher.
- Backup ohne Test ist wertlos.
- Bei Ransomware nicht zahlen und nicht neu starten.
- Sicherheitsvorfall intern melden, nicht „selbst lösen“.

## Verwandt

- [Schutzziele (IT)](../IT/schutzziele.md) – CIA und erweiterte Ziele
- [Patchmanagement (IT)](../IT/patchmanagment-tu-su-pu.md) – Updates
- [Backup & Snapshots (IT)](../IT/snapshots-backups.md) – Sicherung
- [MFA & Authentifizierung (IT)](../IT/authentifizierung-autoritisierung-mfa.md) – 2FA
- [DSGVO & TOM (IT)](../IT/dsgvo-tom.md) – Datenschutz
- [Urheberrecht & Compliance](urheberrecht-compliance.md) – Regeln im Betrieb
