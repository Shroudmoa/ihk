# Authentifizierung-Autoritisierung-MFA

# Authentifizierung, Autorisierung und MFA – IHK AP2

## 1. Authentifizierung vs. Autorisierung

**Authentifizierung (Authentication): Wer bist du?**

Das System überprüft, ob ein Benutzer tatsächlich die behauptete Identität besitzt.

Beispiele:
- Benutzername und Passwort eingeben
- Anmeldung mit einer Smartcard
- Fingerabdruck scannen

Ergebnis: Die Identität wird erfolgreich bestätigt oder die Anmeldung wird abgelehnt.

**Autorisierung (Authorization): Was darfst du?**

Nachdem die Identität authentifiziert wurde, prüft das System, welche Aktionen und Ressourcen der Benutzer verwenden darf.

Beispiele:
- Darf der Benutzer Dateien lesen?
- Darf er Dateien löschen?
- Darf er Benutzerkonten verwalten?

| Merkmal | Authentifizierung | Autorisierung |
|---|---|---|
| Frage | Wer bist du? | Was darfst du? |
| Zweck | Identität überprüfen | Berechtigungen überprüfen |
| Zeitpunkt | Normalerweise vor der Autorisierung | Nach erfolgreicher Authentifizierung |
| Ergebnis | Identität bestätigt oder abgelehnt | Zugriff erlaubt oder verweigert |
| Beispiel | Login mit Passwort und MFA | Benutzer darf Dateien lesen, aber nicht löschen |

### HTTP-Statuscodes

- **401 Unauthorized:** Die Anfrage ist nicht erfolgreich authentifiziert. Beispielsweise fehlen gültige Anmeldedaten.
- **403 Forbidden:** Die Anfrage wurde verstanden, aber der Zugriff ist nicht erlaubt. Beispielsweise fehlen die erforderlichen Berechtigungen.

Wichtig: In der HTTP-Spezifikation bedeutet `401` fehlende oder ungültige Authentifizierungsdaten. `403` bedeutet, dass der Server die Anfrage ablehnt. Die konkrete Implementierung kann variieren.

### Prüfungsbeispiel

Ein Mitarbeiter meldet sich erfolgreich an. Anschließend versucht er, die Personalakten aufzurufen.

- Login erfolgreich: Authentifizierung erfolgreich.
- Zugriff auf Personalakten nicht erlaubt: Autorisierung verweigert.
- Ein korrekt angemeldeter Mitarbeiter kann also trotzdem keinen Zugriff auf jede Ressource haben.

**Achtung:** Eine fehlerhafte Konfiguration kann dazu führen, dass Berechtigungen falsch vergeben werden. Eine erfolgreiche Authentifizierung bedeutet nicht automatisch, dass der Zugriff erlaubt sein sollte.
