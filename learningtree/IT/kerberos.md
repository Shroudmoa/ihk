# Kerberos

## In 30 Sekunden

- **Kerberos** = Authentifizierungsprotokoll, das Tickets statt Klartext-Passwörter nutzt.
- Läuft im Active Directory fast überall für die Anmeldung.
- Nutzt **symmetrische Verschlüsselung** und das Prinzip „Trusted Third Party“.
- Port **88** (TCP/UDP)
- Ermöglicht **Single Sign-On (SSO)** – einmal anmelden, auf viele Dienste zugreifen.

## Kernwissen

### Aufbau

Im Kerberos gibt es einen zentralen **KDC (Key Distribution Center)**. Dieser besteht aus zwei Komponenten:

| Komponente | Aufgabe |
|---|---|
| **AS (Authentication Server)** | Prüft beim ersten Login Benutzername + Passwort. Stellt bei Erfolg ein **TGT** aus. |
| **TGS (Ticket Granting Server)** | Stellt auf Basis des TGT ein **Service-Ticket** für den gewünschten Dienst aus. |

### Ablauf in Kurzform

1. **AS-Request** – Client fragt beim AS nach einem TGT
2. **AS-Response** – Client erhält TGT + Session-Key (verschlüsselt)
3. **TGS-Request** – Client zeigt TGT dem TGS und fordert ein Ticket für z. B. einen Fileserver
4. **TGS-Response** – Client erhält das Service-Ticket
5. **AP-Request** – Client präsentiert das Service-Ticket dem Zielserver und wird authentifiziert

### Wichtige Begriffe

| Begriff | Erklärung |
|---|---|
| **TGT (Ticket Granting Ticket)** | „Eintrittskarte“ für den Kerberos-Verkehr. Gilt für eine begrenzte Zeit. |
| **Service-Ticket** | Ticket für einen ganz bestimmten Dienst (z. B. CIFS, HTTP, LDAP) |
| **Session-Key** | Temporärer Schlüssel für die verschlüsselte Kommunikation zwischen Client und Server |
| **KDC** | Key Distribution Center – befindet sich auf jedem Domain Controller im AD |
| **Realm** | Der Kerberos-Geltungsbereich (z. B. FIRMA.LOCAL) |

## Prüfungsfrage

**Frage:** Warum ist Kerberos sicherer als eine einfache Passwortabfrage über das Netzwerk?

**Musterantwort:** Kerberos überträgt niemals das eigentliche Passwort im Klartext. Stattdessen wird mit Tickets gearbeitet. Der Client authentifiziert sich gegenüber dem KDC, erhält ein TGT und fordert danach nur noch Service-Tickets an. Die eigentliche Authentifizierung gegenüber dem Zielserver läuft über diese Tickets, wodurch das Risiko von Credential-Sniffing stark reduziert wird.

## Falle

- **Port 88 vergessen** – Kerberos läuft über TCP/UDP **88**, nicht über 389
- **Kerberos ≠ LDAP** – LDAP liefert Verzeichnisinformationen, Kerberos dient der **Authentifizierung**
- **TGT wird nicht zum Zielserver geschickt** – Nur das **Service-Ticket** geht zum eigentlichen Dienst
- **„Kerberos verschlüsselt alles“** – Kerberos authentifiziert, ob Daten danach verschlüsselt werden hängt vom Dienst ab

## Merksatz

**„TGT für das Tor, Service-Ticket für den Raum.“**

## Verwandt

- [LDAP](ldap.md) – Verzeichniszugriff
- [Authentifizierung-Autoritisierung-MFA](authentifizierung-autoritisierung-mfa.md) – Grundlagen der Authentifizierung
- [RBAC](rbac.md) – Berechtigungen nach Rolle
- [PKI-Zertifikate](pki-zertifikate.md) – Kryptografie-Grundlagen
