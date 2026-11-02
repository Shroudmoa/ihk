# GPO (Group Policy Objects)

## In 30 Sekunden

- **GPO** = Gruppenrichtlinie. Zentral definierte Einstellungen für Benutzer & Computer.
- Wird über **Active Directory** verteilt.
- Gilt für **OU, Domain, Site** – nicht direkt für einzelne User, sondern über Vererbung.
- Ziel: Einheitliche Konfiguration, Sicherheit, weniger Fehler.
- Erstellt/bearbeitet über **Group Policy Management Console (GPMC)**

## Kernwissen

### Wie funktioniert das?

GPOs werden an **Container** (Sites, Domains, Organizational Units) verknüpft. Computer & User, die in diesen Containern sind, wenden sie an.

### Wichtige Begriffe

| Begriff | Erklärung |
|---|---|
| **GPO** | Die eigentliche Richtlinie mit hunderten Einstellungen |
| **GPMC** | Group Policy Management Console zur Verwaltung |
| **OU** | Organizational Unit – meist für die Verknüpfung genutzt |
| **Vererbung** | Untergeordnete OUs erben übergeordnete GPOs |
| **Enforcement / Block Inheritance** | Ermöglicht gezieltes Überschreiben bzw. Sperren der Vererbung |
| **gpupdate /force** | Erzwingt sofortiges Anwenden der Richtlinien |
| **RSoP (Resultant Set of Policy)** | Zeigt das Endergebnis aller angewandten GPOs |

### Anwendungsreihenfolge (wichtig für Prüfung)

Die Verarbeitung läuft grob in folgender Reihenfolge:

1. **Local** – Lokale Richtlinie
2. **Site** – Standortbasiert
3. **Domain** – Für die ganze Domäne
4. **OU** – Für Organisationseinheiten
5. **Child OU** – Noch spezifischer

Merkwort: **LSDOU** (Lokal – Site – Domäne – OU).

**Faustregel:** Später angewendete Richtlinien überschreiben frühere – je näher an der OU, desto höher die Priorität (**last writer wins**). Ausnahmen: **Erzwungen** (Enforced) setzt eine übergeordnete GPO trotzdem durch, **Vererbung deaktivieren** (Block Inheritance) blockiert übergeordnete, nicht erzwungene GPOs.

**Wichtig für Sicherheit:** Die **Kennwort- und Kontosperrungsrichtlinie** für Domänenkonten wirkt nur aus einer GPO, die an die **Domäne** verknüpft ist (Default Domain Policy). Abweichende Regeln für einzelne Gruppen (z. B. Admins) setzt man mit **Fine-Grained Password Policies**, nicht mit einer OU-GPO.

### Typische Einsatzbereiche

| Einsatz | Beispiel |
|---|---|
| **Sicherheit** | Passwort-Richtlinien, Konto-Sperrung, Bildschirmsperre, Firewall-Regeln, Makros sperren, AppLocker |
| **Zertifikate** | Firmen-Root-CA verteilen, z. B. für TLS-Inspection |
| **Softwareverteilung** | MSI-Pakete zentral installieren |
| **Laufwerkszuordnung** | Netzlaufwerke automatisch verbinden |
| **Desktop-Einstellungen** | Startmenü, Desktopsperre, Deaktivieren von USB |
| **Skripte** | Logon-/Logoff-Skripte für User/Computer |

## Prüfungsfrage

**Frage:** Warum nutzt man GPOs statt jede Einstellung manuell auf den Clients vorzunehmen?

**Musterantwort:** GPOs erlauben eine zentrale, einheitliche und nachvollziehbare Konfiguration. Änderungen werden automatisch auf alle betroffenen Clients verteilt. Das spart Zeit, verhindert Konfigurationsfehler und stellt sicher, dass Sicherheitsrichtlinien wie Bildschirmsperre oder Passwortvorgaben überall eingehalten werden.

## Falle

- **GPO wird direkt an User verknüpft** – In der Praxis wird meist an **OU** verknüpft, seltener direkt
- **Vererbung ignorieren** – Widersprüche lösen sich über Priorität, nicht durch „egal“
- **Block Inheritance vs. Enforced (früher No Override)** – Block verhindert die Vererbung nach unten, Enforced setzt sich trotzdem durch
- **Nur User oder nur Computer** – Einige Einstellungen gelten nur für User, andere nur für Computer

## Merksatz

**„LSDOU – der Letzte gewinnt, außer oben ist erzwungen.“** Zentral statt lokal.

## Verwandt

- [RBAC](rbac.md) – Rollenbasierte Berechtigungen
- [LDAP](ldap.md) – Verzeichnisstruktur
- [Betriebssystem-Härtung](../Server/os-haertung.md) – Härtung über GPOs umsetzen
- [Least Privilege](../Sicherheit/least-privilege.md) – Minimale Rechte durch Richtlinien
- [Active Directory](../Sicherheit/active-directory.md) – Domäne, OU, Gruppen
- [Passwortrichtlinien](../Sicherheit/passwortrichtlinien.md) – Kennwortregeln per GPO
- [Lektion 27 TLS-Inspection (CA per GPO verteilen)](../../analyse-der-exams/L27_PKI_TLS_Verschluesselung_VPN.md)
