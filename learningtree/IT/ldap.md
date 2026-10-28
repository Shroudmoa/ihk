# LDAP

## Was ist LDAP?

LDAP steht für **Lightweight Directory Access Protocol**. Es ist ein Netzwerkprotokoll, mit dem Clients auf Verzeichnisdienste zugreifen können, um Informationen zu suchen, auszulesen und – abhängig von den Berechtigungen – zu verändern.

Ein Verzeichnisdienst speichert strukturierte Informationen über Objekte, beispielsweise:
- Benutzer und Gruppen
- Computer und Server
- Organisationseinheiten
- Drucker und Netzwerkressourcen

LDAP ist ein **Protokoll**, kein eigenständiger Verzeichnisdienst und kein Betriebssystem.

## LDAP vs. Active Directory vs. OpenLDAP

| Begriff | Bedeutung |
|---|---|
| LDAP | Protokoll für den Zugriff auf Verzeichnisdienste |
| Active Directory (AD DS) | Microsoft-Verzeichnisdienst, der unter anderem LDAP unterstützt |
| OpenLDAP | Freie Implementierung eines LDAP-Verzeichnisdienstes |
| Microsoft Entra ID | Microsoft-Cloud-Identitätsdienst; nicht einfach ein klassischer LDAP-Server |

**Wichtig:** LDAP ist nicht dasselbe wie Active Directory. Active Directory unterstützt LDAP, verwendet aber auch weitere Protokolle und Funktionen, beispielsweise Kerberos für die Authentifizierung und DNS zur Dienstauffindung.

Linux-Systeme, Anwendungen und viele Netzwerkdrucker können LDAP verwenden, um Benutzerinformationen aus einem Active Directory oder einem anderen LDAP-Verzeichnis abzurufen. Ob das funktioniert, hängt von der jeweiligen Anwendung und ihrer Konfiguration ab.

## Hierarchischer Aufbau: DIT

LDAP-Verzeichnisse sind typischerweise hierarchisch aufgebaut. Diese Struktur heißt **DIT – Directory Information Tree**.

Beispiel:

```text
dc=firma,dc=de
├── ou=IT
│   ├── cn=Max Mustermann
│   └── cn=IT-Admins
├── ou=Personal
│   └── cn=Erika Beispiel
└── ou=Geraete
    └── cn=Drucker-01
```

Die wichtigsten Begriffe:

| Kürzel | Bedeutung | Beispiel |
|---|---|---|
| `dc` | Domain Component | `dc=firma` |
| `ou` | Organizational Unit | `ou=IT` |
| `cn` | Common Name | `cn=Max Mustermann` |
| `uid` | User ID | `uid=max` |
| `dn` | Distinguished Name | Vollständiger Pfad eines Eintrags |

Die genaue Objektstruktur hängt vom Verzeichnis und seinem Schema ab. Benutzer und Gruppen sind nicht zwingend immer unter einer Organisationseinheit angeordnet.

## Distinguished Name (DN)

Der **Distinguished Name** identifiziert einen Eintrag eindeutig innerhalb des Verzeichnisses. Er besteht aus einer Folge von Attributen, die den Eintrag bis zur Wurzel des Verzeichnisses einordnen.

Beispiel:

```text
cn=Max Mustermann,ou=IT,dc=firma,dc=de
```

Von links nach rechts:
- `cn=Max Mustermann`: konkreter Eintrag
- `ou=IT`: Organisationseinheit
- `dc=firma,dc=de`: Verzeichnis-Domain

Ein DN ist vergleichbar mit einem vollständigen Pfad zu einem Objekt. Er ist jedoch kein Dateisystempfad.

**RDN – Relative Distinguished Name:** Der erste Bestandteil eines DN ist in diesem Beispiel `cn=Max Mustermann`. Er identifiziert den Eintrag relativ zu seinem übergeordneten Eintrag.

## Wie funktioniert LDAP?

Typischer Ablauf:

1. Ein Client verbindet sich mit dem LDAP-Server.
2. Der Client authentifiziert sich gegebenenfalls per Bind.
3. Er führt eine Suche oder eine andere erlaubte Operation aus.
4. Der Server prüft die Berechtigungen und verarbeitet die Anfrage.
5. Der Server liefert das Ergebnis oder eine Fehlermeldung zurück.

Wichtige Operationen:
- **Bind:** Verbindung authentifizieren oder eine LDAP-Sitzung mit einer Identität herstellen.
- **Search:** Einträge suchen.
- **Compare:** Prüfen, ob ein Attribut einen bestimmten Wert besitzt.
- **Add:** Eintrag hinzufügen.
- **Modify:** Eintrag verändern.
- **Delete:** Eintrag löschen.
- **Unbind:** LDAP-Sitzung ordnungsgemäß beenden.

Ob ein Client Einträge ändern oder löschen darf, hängt von den Berechtigungen des Verzeichnisdienstes ab.

## Ports und Sicherheit

| Port | Verwendung |
|---|---|
| TCP 389 | LDAP; Verschlüsselung kann beispielsweise per StartTLS aktiviert werden |
| TCP 636 | LDAPS – LDAP über TLS |
| TCP 3268 | Active Directory Global Catalog |
| TCP 3269 | Global Catalog über TLS |

**Wichtig:** Port 389 bedeutet nicht automatisch, dass Daten unverschlüsselt übertragen werden. LDAP kann auf diesem Port mit StartTLS abgesichert werden. Ohne TLS-Schutz können sensible Informationen bei der Übertragung offengelegt werden.

Bei LDAPS wird TLS bereits beim Verbindungsaufbau verwendet. Zertifikate müssen korrekt geprüft werden.

Weitere Sicherheitsmaßnahmen:
- Least-Privilege-Prinzip und restriktive Zugriffsrechte
- Sichere Authentifizierung
- TLS und gültige Zertifikate
- Keine unnötigen anonymen Zugriffe
- Protokollierung und Überwachung
- Schutz von Benutzerpasswörtern und Verzeichnisdaten

## Praxisbeispiel: Linux und Netzwerkdrucker

Ein Unternehmen verwendet Microsoft Active Directory für seine Benutzerkonten.

Ein Linux-Server oder ein Netzwerkdrucker kann LDAP nutzen, um beispielsweise:
- Benutzer anhand ihres Namens zu finden
- Gruppenmitgliedschaften abzufragen
- Benutzerinformationen für einen Dienst abzurufen

Der Client verbindet sich mit dem LDAP-Dienst des Active Directory und verwendet einen konfigurierten Suchbereich, zum Beispiel:

```text
dc=firma,dc=de
```

Ein Suchfilter könnte lauten:

```text
(&(objectClass=user)(sAMAccountName=max))
```

Dieser Filter sucht in Active Directory nach einem passenden Benutzerobjekt mit dem Anmeldenamen `max`. Der konkrete Filter muss zur Verzeichnisstruktur und Anwendung passen.

**Achtung:** Eine LDAP-Suche nach einem Benutzer ist nicht automatisch eine vollständige Authentifizierung. Anwendungen können LDAP für die Benutzerprüfung verwenden; andere Authentifizierungsverfahren wie Kerberos erfüllen zusätzliche Aufgaben.

## LDAP und Berechtigungen

LDAP kann Informationen zu Benutzern und Gruppen bereitstellen. Ob ein Benutzer tatsächlich auf eine Datei, Anwendung oder Ressource zugreifen darf, entscheidet jedoch die jeweilige Anwendung beziehungsweise das zugrunde liegende Berechtigungssystem.

Beispiel:
- LDAP liefert die Gruppenmitgliedschaft eines Benutzers.
- Ein Fileserver prüft die Gruppenmitgliedschaft gegen seine Zugriffsregeln.
- Der Fileserver erlaubt oder verweigert den Zugriff.

LDAP ist daher nicht automatisch ein vollständiges Berechtigungsmanagement für alle Systeme.

## Vorteile und Nachteile

| Vorteile | Nachteile |
|---|---|
| Zentrale Verwaltung von Verzeichnisinformationen | Komplexe Einrichtung und Administration möglich |
| Einheitliche Abfragen für verschiedene Clients | Fehlerhafte Rechte können viele Benutzer betreffen |
| Unterstützt hierarchische Strukturen | Schema und DN-Struktur müssen verstanden werden |
| Von vielen Betriebssystemen und Anwendungen unterstützt | Ohne TLS können Daten gefährdet sein |
| Kann Benutzer- und Gruppeninformationen zentral bereitstellen | Nicht jede Anwendung unterstützt LDAP direkt |

## IHK-Merksätze

- LDAP ist ein Protokoll für den Zugriff auf Verzeichnisdienste.
- DIT bezeichnet die hierarchische Struktur eines Verzeichnisses.
- DN identifiziert einen Eintrag innerhalb des Verzeichnisses.
- Active Directory ist ein Verzeichnisdienst, der LDAP unterstützt.
- OpenLDAP ist eine freie LDAP-Implementierung.
- LDAP kann Benutzer- und Gruppeninformationen liefern, ersetzt aber nicht automatisch alle Authentifizierungs- und Berechtigungsmechanismen.
- TCP 389 ist der typische LDAP-Port; TCP 636 wird für LDAPS verwendet.
- Für vertrauliche Daten sollte LDAP durch TLS abgesichert werden.
