# RBAC – rollenbasierte Zugriffskontrolle

## In 30 Sekunden

- **RBAC** (Role-Based Access Control): Rechte werden **Rollen** zugewiesen (z. B. „Buchhaltung“, „Helpdesk“), Benutzer erhalten Rollen – **nicht** Einzelrechte.
- Vorteil: Beim Eintritt, Abteilungswechsel oder Austritt ändert man nur die **Rollenzugehörigkeit**; die Rechtevergabe bleibt übersichtlich und prüfbar.
- Im Active Directory wird RBAC über **Sicherheitsgruppen** umgesetzt, nach dem Schema **AGDLP**.
- RBAC ist das Werkzeug für **Least Privilege** und **Need-to-know** und damit eine typische **Zugriffskontrolle** im Sinne der TOM.
- Prüfung 2025W GA1 („Unberechtigten Zugriff verhindern“ → RBAC, MFA), 2021W (Maßnahmen gegen unbefugten DB-Zugriff), 2025S (Umsetzung von Least Privilege).

## Kernwissen

**Grundbegriffe**

| Begriff | Bedeutung |
|---|---|
| Benutzer (Subjekt) | Person oder Dienst, der zugreift |
| **Rolle** | Bündel von Berechtigungen für eine Funktion, z. B. „Vertrieb lesen“ |
| Berechtigung | erlaubte Aktion auf ein Objekt: lesen, schreiben, ausführen, löschen |
| Objekt | Ressource: Ordner, Datenbanktabelle, Anwendung, Drucker |
| **Funktionstrennung** (Separation of Duties) | unvereinbare Rollen nicht derselben Person geben, z. B. „Rechnung anlegen“ und „Zahlung freigeben“ |

**Umsetzung im Active Directory – AGDLP**

| Schritt | Beispiel |
|---|---|
| **A**ccounts (Benutzerkonten) | `m.mueller` |
| in **G**lobale Gruppen nach Rolle/Abteilung | `GG_Vertrieb` |
| in **D**omänen**l**okale Gruppen nach Ressource und Recht | `DL_Kundendaten_Lesen`, `DL_Kundendaten_Aendern` |
| nur diese erhalten **P**ermissions auf der Ressource | NTFS-Recht „Lesen“ auf `\\fs01\Kunden` für `DL_Kundendaten_Lesen` |

**Typisches Berechtigungskonzept (Ausschnitt)**

| Rolle | Kundendaten | Personalakten | Server-Administration |
|---|---|---|---|
| Vertrieb | lesen/ändern | – | – |
| Personalabteilung | – | lesen/ändern | – |
| Helpdesk | lesen | – | Passwörter zurücksetzen |
| Server-Admin (eigenes Admin-Konto) | – | – | voll (nur mit MFA) |

**RBAC im Vergleich zu anderen Modellen**

| Modell | Wer entscheidet über Rechte? | Beispiel |
|---|---|---|
| **DAC** (Discretionary) | der **Besitzer** einer Datei | NTFS-/Linux-Rechte, die der Ersteller vergibt |
| **MAC** (Mandatory) | das **System** nach Sicherheitsstufen | SELinux, Geheimhaltungsstufen beim Militär |
| **RBAC** | die **Rolle** im Unternehmen | AD-Gruppen, Rollen in ERP und Datenbank |
| **ABAC** (Attribute-Based) | Regeln über **Attribute** (Abteilung, Ort, Uhrzeit, Gerät) | Conditional Access in der Cloud |

**Pflege:** regelmäßige **Rezertifizierung** (Abteilungsleiter bestätigt die Rollen seiner Mitarbeiter), Rechte beim Wechsel entziehen, Rollen nicht ausufern lassen („Rollenexplosion“), Änderungen protokollieren.

**Datenschutzbezug:** RBAC setzt die **Zugriffskontrolle** der TOM (Art. 32 DSGVO) und **Privacy by Default** (Art. 25) um – Mitarbeiter sehen nur die Daten, die sie für ihre Aufgabe brauchen.

## Prüfungsfrage

**Frage:** In einem Unternehmen werden Ordnerrechte bisher einzelnen Benutzern zugewiesen. Erläutern Sie das RBAC-Prinzip und zwei Vorteile für die Administration.

**Musterantwort:** Beim rollenbasierten Zugriffsmodell werden Berechtigungen nicht einzelnen Personen, sondern Rollen zugeordnet, die einer Funktion im Unternehmen entsprechen, etwa „Vertrieb“ oder „Personal“. Benutzer werden einer oder mehreren Rollen zugewiesen und erhalten so automatisch die passenden Rechte; im Active Directory geschieht das über Sicherheitsgruppen. Vorteil eins: Bei Eintritt, Abteilungswechsel oder Austritt muss nur die Gruppenmitgliedschaft angepasst werden, statt viele Einzelrechte zu suchen. Vorteil zwei: Die Rechtevergabe ist übersichtlich und prüfbar, sodass Least Privilege eingehalten und bei Audits nachgewiesen werden kann.

## Falle

- Rechte doch wieder direkt an Benutzer vergeben „nur ausnahmsweise“ → genau daraus entsteht der Wildwuchs.
- RBAC und ACL gleichsetzen: die **ACL** ist die Liste am Objekt, **RBAC** das Konzept, wer über welche Rolle in diese Liste kommt.
- MAC (Mandatory Access Control) mit der MAC-Adresse verwechseln.
- Funktionstrennung vergessen, wenn nach Rollenkonzepten in der Buchhaltung gefragt wird.

## Merksatz

**„Rechte an Rollen, Rollen an Menschen – nie Rechte direkt an Menschen.“**

## Verwandt

- [Least Privilege](../Sicherheit/least-privilege.md)
- [Zugriffskontrollmodelle](../../nice-to-know/access-control.md) – DAC, MAC, ACL
- [Active Directory](../Sicherheit/active-directory.md) – Gruppen und AGDLP
- [GPO](gpo.md)
- [LDAP](ldap.md)
- [Zero Trust](../Sicherheit/zero-trust.md)
- [TOM](../../Management/DSGVO/toms.md) – Zugriffskontrolle
- [Lektion 28 GA1-Praxisthemen](../../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md)
