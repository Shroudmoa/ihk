# Active Directory

## In 30 Sekunden

- **Active Directory Domain Services (AD DS)** ist Microsofts **Verzeichnisdienst**: zentrale Verwaltung von Benutzern, Gruppen, Computern und Richtlinien in einer **Domäne**.
- Bausteine: **Domain Controller (DC)** mit der AD-Datenbank, **OUs** zur Strukturierung, **Gruppen** für Berechtigungen, **GPOs** für Einstellungen.
- Protokolle: **Kerberos** (Anmeldung, Port 88), **LDAP** (Verzeichnisabfragen, 389/636), **DNS** (DCs finden, SRV-Einträge) – ohne funktionierendes DNS kein AD.
- Für Ausfallsicherheit mindestens **zwei DCs** pro Domäne; sie replizieren gleichberechtigt (Multi-Master).
- AD ist das **wertvollste Angriffsziel** im Netz: wer Domänen-Admin wird, besitzt alles → getrennte Admin-Konten, MFA, Least Privilege, Härtung.

## Kernwissen

**Logische Struktur**

| Begriff | Bedeutung |
|---|---|
| **Domäne** | Verwaltungs- und Sicherheitsgrenze mit gemeinsamer Datenbank und Richtlinien, DNS-Name z. B. `firma.local` |
| **Tree (Struktur)** | Domänen mit zusammenhängendem Namensraum, z. B. `firma.local` und `berlin.firma.local` |
| **Forest (Gesamtstruktur)** | oberste Ebene, ein oder mehrere Trees mit gemeinsamem Schema; Sicherheitsgrenze des AD |
| **OU (Organisationseinheit)** | Container zur Strukturierung (z. B. nach Abteilung/Standort), Ziel für **GPOs** und delegierte Verwaltung |
| **Objekte** | Benutzer, Gruppen, Computer, Drucker – mit Attributen; eindeutig über den **Distinguished Name** |
| **Schema** | legt fest, welche Objektklassen und Attribute es gibt |
| **Vertrauensstellung (Trust)** | erlaubt Benutzern einer Domäne den Zugriff auf Ressourcen einer anderen |

**Physische Struktur**
- **Domain Controller:** Server mit AD-DS-Rolle und der Datenbank `NTDS.dit`; jeder DC ist beschreibbar (Ausnahme: **RODC**, schreibgeschützter DC für unsichere Außenstellen).
- **Global Catalog:** Teilkopie aller Objekte des Forests für schnelle Suchen (Port 3268/3269).
- **Standorte (Sites):** bilden die physischen Netze ab und steuern die Replikation über langsame WAN-Strecken.
- **FSMO-Rollen** (Betriebsmaster), die nur ein DC hält: forestweit **Schemamaster** und **Domänennamenmaster**; domänenweit **RID-Master**, **PDC-Emulator** (Zeitquelle, Kennwortänderungen) und **Infrastrukturmaster**.

**Gruppen**
- **Typ:** Sicherheitsgruppe (für Berechtigungen) oder Verteilergruppe (nur E-Mail-Verteiler).
- **Bereich:** domänenlokal, global, universal.
- **AGDLP-Prinzip:** **A**ccounts in **G**lobale Gruppen (nach Rolle/Abteilung), diese in **D**omänen**l**okale Gruppen (nach Ressource), und nur diese erhalten die **P**ermissions. So bleibt die Rechtevergabe übersichtlich (siehe [RBAC](../Netzwerk-Administration/rbac.md)).

**Wichtige Ports**

| Dienst | Port |
|---|---|
| DNS | 53 TCP/UDP |
| Kerberos | 88 TCP/UDP |
| LDAP / LDAPS | 389 / 636 TCP |
| Global Catalog | 3268 / 3269 TCP |
| SMB (SYSVOL, GPOs) | 445 TCP |

**Absicherung des AD**

| Maßnahme | Zweck |
|---|---|
| getrennte Admin-Konten, **Tier-Modell** | Domänen-Admin-Konten nie auf normalen Clients verwenden |
| **MFA** und starke Passwortrichtlinie (per GPO, für Admins Fine-Grained Password Policy) | Kontoübernahme erschweren |
| Mitgliedschaft in privilegierten Gruppen minimieren und überwachen | Least Privilege, Alarm bei Änderungen (SIEM) |
| **LAPS** | jedes Gerät hat ein eigenes lokales Admin-Passwort |
| LDAP-Signing/LDAPS, SMB-Signing, alte Protokolle (NTLMv1, SMBv1) abschalten | Abhören und Manipulation verhindern |
| DCs physisch und logisch schützen, nur nötige Rollen | DC = Kronjuwel |
| **Systemstatus-Backup** der DCs, **AD-Papierkorb** aktivieren | gelöschte Objekte und ganze DCs wiederherstellen |
| inaktive Konten regelmäßig deaktivieren | verwaiste Konten sind Einfallstore |

**Abgrenzung Microsoft Entra ID** (früher Azure AD): Cloud-Identitätsdienst für Microsoft 365 und SaaS-Apps; arbeitet mit modernen Protokollen (OAuth 2.0, OpenID Connect, SAML) statt Kerberos/LDAP und kennt keine OUs und GPOs im klassischen Sinn. Über **Entra Connect** werden lokale AD-Konten in die Cloud synchronisiert (Hybrid-Identität).

## Prüfungsfrage

**Frage:** Ein Unternehmen führt Active Directory ein. Nennen Sie drei Vorteile gegenüber lokal verwalteten Benutzerkonten und erläutern Sie, warum mindestens zwei Domain Controller eingesetzt werden.

**Musterantwort:** Erstens werden Benutzer und Passwörter zentral verwaltet, sodass sich ein Mitarbeiter mit einem Konto an allen Rechnern anmelden kann und beim Austritt nur ein Konto gesperrt werden muss. Zweitens werden Berechtigungen über Gruppen vergeben, was die Umsetzung von Least Privilege vereinfacht. Drittens lassen sich Sicherheitseinstellungen wie Passwortrichtlinien oder Bildschirmsperre per Gruppenrichtlinie einheitlich verteilen. Mindestens zwei Domain Controller sind nötig, weil ohne erreichbaren DC keine Anmeldung und keine Kerberos-Authentifizierung möglich ist; der zweite DC repliziert die Datenbank und übernimmt bei Ausfall des ersten.

## Falle

- AD und LDAP gleichsetzen: **LDAP ist ein Protokoll**, AD ein Verzeichnisdienst, der LDAP (und Kerberos, DNS) nutzt.
- GPO auf eine **Gruppe** verknüpfen wollen → verknüpft wird an **Standort, Domäne oder OU**; Gruppen nur über Sicherheitsfilterung.
- „Backup unnötig, wir haben zwei DCs“ → Replikation verteilt auch versehentliches Löschen oder Manipulation sofort.
- DNS vergessen: falscher DNS-Server am Client ist eine der häufigsten Ursachen für Anmeldeprobleme.

## Merksatz

**„Forest – Tree – Domäne – OU: GPO an die OU, Rechte an die Gruppe, nie an den einzelnen User.“**

## Verwandt

- [GPO](../Netzwerk-Administration/gpo.md)
- [Kerberos](../Netzwerk-Administration/kerberos.md)
- [LDAP](../Netzwerk-Administration/ldap.md)
- [RBAC](../Netzwerk-Administration/rbac.md)
- [Least Privilege](least-privilege.md)
- [Passwortrichtlinien](passwortrichtlinien.md)
- [DNS](../OSI-IP-mac-bit-byte/dns.md)
- [AAA und SIEM](aaa-siem.md)
