# Authentifizierung, Autorisierung und MFA

> Abgrenzung: Dieses Blatt erklärt die Grundlagen **Authentifizierung vs. Autorisierung** (inkl. HTTP 401/403) und ordnet MFA ein. Faktoren, Verfahren (TOTP, FIDO2, SMS) und deren Stärke vertieft [MFA](../Sicherheit/MFA.md).

## In 30 Sekunden

- **Identifizierung:** Wer **behauptest** du zu sein? (Benutzername)
- **Authentifizierung (AuthN):** **Beweise**, dass du es bist (Passwort, Token, Fingerabdruck).
- **Autorisierung (AuthZ):** **Was darfst** du? Rechte werden erst **nach** erfolgreicher Authentifizierung geprüft.
- **Accounting:** **Was hast** du getan? Protokollierung (das dritte A von AAA).
- **MFA:** mindestens zwei Faktoren aus **verschiedenen** Kategorien: Wissen, Besitz, Sein.
- HTTP: **401** = nicht (gültig) authentifiziert, **403** = authentifiziert, aber nicht berechtigt.

## Kernwissen

### Vergleich

| Merkmal | Authentifizierung | Autorisierung |
|---|---|---|
| Frage | Wer bist du (wirklich)? | Was darfst du? |
| Zweck | Identität überprüfen | Berechtigungen prüfen und durchsetzen |
| Zeitpunkt | zuerst | danach |
| Ergebnis | Identität bestätigt oder Anmeldung abgelehnt | Zugriff erlaubt oder verweigert |
| Technik | Passwort, Zertifikat, Smartcard, Biometrie, Kerberos, RADIUS | ACLs, NTFS-Rechte, Gruppen, RBAC, OAuth-Scopes |
| Beispiel | Login mit Passwort + Authenticator-App | darf Personalakten lesen, aber nicht löschen |

### Die Faktoren (Kurzfassung)

| Faktor | Beispiele |
|---|---|
| **Wissen** | Passwort, PIN |
| **Besitz** | Smartphone mit TOTP-App, FIDO2-Schlüssel, Smartcard |
| **Sein** | Fingerabdruck, Gesichtserkennung |

**Passwort + PIN ist keine MFA**, beides ist Wissen. Details und Verfahrensvergleich: [MFA](../Sicherheit/MFA.md).

### Zusammenspiel im Unternehmensnetz

1. Benutzer meldet sich am Client an → **Authentifizierung** gegen Active Directory (Kerberos) bzw. am VPN/WLAN über RADIUS.
2. Er erhält ein Ticket bzw. Token mit seinen Gruppenmitgliedschaften.
3. Beim Zugriff auf `\\srv\finanzen` prüft der Server die **Autorisierung**: Freigabe- und NTFS-Rechte der Gruppen.
4. Zugriffe und Fehlversuche werden protokolliert (**Accounting**, Auswertung im SIEM).

### HTTP-Statuscodes

- **401 Unauthorized:** Anfrage ohne gültige Anmeldedaten. Der Name ist irreführend, gemeint ist „nicht authentifiziert“.
- **403 Forbidden:** Der Server kennt den Benutzer, verweigert aber den Zugriff (fehlende Berechtigung).

## Prüfungsfrage

**Frage:** Ein Mitarbeiter meldet sich erfolgreich am Intranet an, erhält beim Aufruf der Personalakten aber eine Fehlermeldung. Erläutern Sie anhand dieses Falls den Unterschied zwischen Authentifizierung und Autorisierung.

**Musterantwort:** Bei der Anmeldung hat der Mitarbeiter mit Benutzername und Passwort (ggf. plus zweitem Faktor) nachgewiesen, dass er der ist, der er vorgibt zu sein. Die Authentifizierung war also erfolgreich. Beim Aufruf der Personalakten prüft das System anschließend, ob seine Identität bzw. Rolle dafür berechtigt ist. Diese Autorisierung schlägt fehl, weil nur die Personalabteilung Zugriff hat. Der Server antwortet daher z. B. mit HTTP 403. Eine erfolgreiche Anmeldung bedeutet also nicht automatisch Zugriff auf alle Ressourcen.

## Falle

- Authentifizierung und Autorisierung vertauschen oder gleichsetzen.
- „Autoritisierung“ schreiben. Der Fachbegriff heißt **Autorisierung**.
- Zwei Passwörter oder Passwort + Sicherheitsfrage als MFA bezeichnen. Beides ist Wissen.
- 401 als „keine Berechtigung“ deuten. Das ist 403.
- Autorisierung nur einmal beim Login prüfen. Sie wird **bei jedem Zugriff** auf eine Ressource ausgewertet.

## Merksatz

**„Authentifizierung = Ausweis zeigen, Autorisierung = Zutrittsliste prüfen, Accounting = Logbuch führen.“**

## Verwandt

- [MFA](../Sicherheit/MFA.md) – Faktoren und Verfahren im Detail
- [AAA & SIEM](../Sicherheit/aaa-siem.md) – Authentication, Authorization, Accounting
- [NTFS- und Freigabeberechtigungen](ntfs-freigabeberechtigungen.md) – Autorisierung im Dateisystem
- [RBAC](../Netzwerk-Administration/rbac.md) – rollenbasierte Rechte
- [Kerberos](../Netzwerk-Administration/kerberos.md)
- [Active Directory](../Sicherheit/active-directory.md)
- [Least Privilege](../Sicherheit/least-privilege.md)
