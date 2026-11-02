# MFA (Multi-Faktor-Authentifizierung)

> Abgrenzung: Dieses Blatt vertieft **Faktoren, Verfahren und deren Stärke**. Die Grundlagen **Authentifizierung vs. Autorisierung** (inkl. HTTP 401/403) stehen in [Authentifizierung, Autorisierung und MFA](../Server/authentifizierung-autoritisierung-mfa.md).

## In 30 Sekunden

- **MFA** verlangt mindestens **zwei Faktoren aus verschiedenen Kategorien**: **Wissen**, **Besitz**, **Sein** (Biometrie).
- **2FA** ist der Spezialfall mit genau zwei Faktoren.
- **Passwort + PIN ist keine MFA** – beides ist Wissen. Ein Angreifer, der mitliest, bekommt beides auf einmal.
- Wirkung: Ein **gestohlenes Passwort allein reicht nicht** mehr – stärkste Einzelmaßnahme gegen Phishing-Folgen und Credential Stuffing.
- Prüfung 2025W GA2: „Nennen Sie zwei weitere MFA-Methoden neben dem Wissen“ → z. B. **TOTP-App oder Hardware-Token (Besitz)** und **Fingerabdruck (Sein)**.

## Kernwissen

**Die drei Faktoren**

| Faktor | Frage | Beispiele |
|---|---|---|
| **Wissen** | Was weiß ich? | Passwort, PIN, Passphrase, Sicherheitsfrage |
| **Besitz** | Was habe ich? | Smartphone mit Authenticator-App (TOTP), Hardware-Token (FIDO2/YubiKey), Smartcard mit Zertifikat, Push-Bestätigung |
| **Sein** | Was bin ich? | Fingerabdruck, Gesichtserkennung (Windows Hello, Face ID), Iris, Stimme |

Manchmal ergänzt: **Ort** (Standort, Netz) und **Verhalten** (Tippverhalten) – das sind eher **Kontextsignale** für eine Risikoentscheidung als echte Faktoren.

**Verfahren im Vergleich**

| Verfahren | Faktor | Stärke | Schwäche |
|---|---|---|---|
| SMS-Code | Besitz | einfach, überall verfügbar | SIM-Swapping, Abfangen, Phishing-anfällig – nur noch Notlösung |
| TOTP-App (zeitbasierter Einmalcode, 30 s) | Besitz | offline, verbreitet | Code kann auf Phishing-Seite eingegeben werden |
| Push-Bestätigung | Besitz | bequem | „MFA-Fatigue“: Nutzer bestätigt genervt eine fremde Anfrage → Nummernabgleich aktivieren |
| **FIDO2 / Passkey / Hardware-Token** | Besitz (+ PIN oder Biometrie) | **phishing-resistent**, weil an die echte Domain gebunden | Beschaffung, Ersatz bei Verlust |
| Smartcard mit Zertifikat | Besitz + PIN | hohe Sicherheit, z. B. für Admins, VPN, 802.1X | PKI und Kartenleser nötig |
| Biometrie | Sein | nichts zu merken | nicht änderbar, wenn kompromittiert; DSGVO: besondere Kategorie (Art. 9) |

**Wo MFA Pflicht sein sollte:** Fernzugänge (VPN, RDP-Gateway, Webmail), Cloud-Dienste, Administratorkonten, Zugriffe auf personenbezogene oder Finanzdaten.

**Umsetzung im Unternehmen:** zentral über Identity Provider (AD/Entra ID, RADIUS mit MFA-Erweiterung), Ausnahmen dokumentieren, Wiederherstellungsprozess bei Verlust (Ersatztoken, Identitätsprüfung am Helpdesk) festlegen.

## Prüfungsfrage

**Frage:** Für den VPN-Zugang wird bisher nur ein Passwort verlangt. Erklären Sie das Prinzip der Multi-Faktor-Authentifizierung und nennen Sie zwei Methoden, die neben dem Passwort eingesetzt werden können.

**Musterantwort:** Bei der Multi-Faktor-Authentifizierung muss der Benutzer Nachweise aus mindestens zwei verschiedenen Kategorien erbringen – Wissen, Besitz oder ein biometrisches Merkmal. Ein Angreifer, der das Passwort etwa durch Phishing erbeutet, kann sich damit allein nicht anmelden. Neben dem Passwort eignet sich erstens ein Besitzfaktor wie eine Authenticator-App mit zeitbasiertem Einmalcode oder ein FIDO2-Hardware-Token und zweitens ein biometrischer Faktor wie ein Fingerabdruck am Notebook.

## Falle

- Passwort + Sicherheitsfrage oder Passwort + PIN als MFA bezeichnen → zweimal **Wissen**.
- „Mit MFA ist Phishing erledigt“ → SMS- und TOTP-Codes lassen sich in Echtzeit abphishen; nur FIDO2/Passkeys sind phishing-resistent.
- MFA mit SSO verwechseln: SSO heißt **einmal** anmelden, MFA heißt **mehrere Faktoren** bei der Anmeldung.
- Biometrie ohne Datenschutzhinweis empfehlen – biometrische Daten sind besonders schützenswert.

## Merksatz

**„Wissen, Besitz, Sein – zwei verschiedene müssen es sein.“**

## Verwandt

- [Authentifizierung, Autorisierung und MFA](../Server/authentifizierung-autoritisierung-mfa.md) – Grundlagen
- [Passwortrichtlinien](passwortrichtlinien.md) – der Wissensfaktor
- [Social Engineering und Phishing](hacking/social-engineering-phishing.md)
- [Zero Trust](zero-trust.md) – MFA für jeden Zugriff
- [802.1X und RADIUS](netzwerkzugang-802-1x-radius.md)
- [Lektion 29 GA2-Praxisthemen (Passwort-Policy und MFA)](../../analyse-der-exams/L29_GA2_Sonstiges_Praxisthemen.md)
- [Lektion 27 VPN-Authentifizierung](../../analyse-der-exams/L27_PKI_TLS_Verschluesselung_VPN.md)
