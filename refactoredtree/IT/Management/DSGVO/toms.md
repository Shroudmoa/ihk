# TOMs

# TOM – Technische und organisatorische Maßnahmen

## In 30 Sekunden

- **TOM nach Art. 32 DSGVO:** Maßnahmen zum Schutz personenbezogener Daten vor Verlust, Missbrauch, unbefugtem Zugriff und Veränderung.
- Unternehmen müssen ein angemessenes Schutzniveau gewährleisten. Maßgeblich sind Risiko, Schutzbedarf, Stand der Technik und Umsetzungskosten.
- **Technische TOM:** z. B. Verschlüsselung, Firewall, Zugriffskontrolle und Backups.
- **Organisatorische TOM:** z. B. Schulungen, Berechtigungskonzepte, Sicherheitsrichtlinien und Notfallpläne.
- TOM sind kein einmaliges Projekt. Sie müssen regelmäßig überprüft, bewertet und bei Bedarf angepasst werden.

## 1. Die 8 Kontrollarten

| Kontrollart | Zweck und konkrete Beispiele |
|---|---|
| **Zutrittskontrolle** | Unbefugte vom Gebäude und Serverraum fernhalten: Schließsystem, Chipkarte, Besucherliste |
| **Zugangskontrolle** | Unbefugte Systemanmeldungen verhindern: MFA, sichere Passwörter, Account-Sperre |
| **Zugriffskontrolle** | Datenzugriffe auf Berechtigte begrenzen: RBAC, ACL, Least Privilege |
| **Weitergabekontrolle** | Unbefugte Übertragung verhindern: TLS, VPN, verschlüsselte Datenträger |
| **Eingabekontrolle** | Änderungen nachvollziehbar machen: Audit-Logs, Zeitstempel, Vier-Augen-Prinzip |
| **Auftragskontrolle** | Auftragsverarbeiter kontrollieren: AVV, Weisungen, Nachweise und Audits |
| **Verfügbarkeitskontrolle** | Daten und Systeme verfügbar halten: Backups, USV, RAID, Notfallkonzept |
| **Trennungskontrolle** | Daten und Verarbeitungszwecke trennen: Mandantentrennung, getrennte Test- und Produktivsysteme |

**Prüfungsfalle:** RAID ist kein Backup. RAID erhöht die Ausfallsicherheit bei bestimmten Festplattenfehlern, schützt aber nicht zuverlässig vor versehentlichem Löschen, Malware oder Brand.

## 2. Technische vs. organisatorische TOM

| Technische Maßnahmen | Organisatorische Maßnahmen |
|---|---|
| Verschlüsselung at rest und in transit | Datenschutz- und Security-Schulungen |
| Firewall, IDS/IPS | Sicherheitsrichtlinien und Verantwortlichkeiten |
| MFA und ACL | Rollen- und Berechtigungskonzept |
| Pseudonymisierung | Verfahren für Datenschutzverletzungen |
| Backup-Software und USV | Backup- und Wiederherstellungskonzept |
| Protokollierung und Monitoring | Regelmäßige Berechtigungsprüfungen |
| Endpoint Protection | Notfallplan und Incident-Response-Prozess |

**Wichtig:** Viele Maßnahmen sind technisch und organisatorisch zugleich. Eine Backup-Software ist technisch; die festgelegten Sicherungsintervalle, Verantwortlichkeiten und Wiederherstellungstests sind organisatorische Bestandteile.

## 3. Grundsätze für die Umsetzung

- **Stand der Technik:** Maßnahmen müssen dem aktuellen Risiko und dem technischen Entwicklungsstand angemessen sein. Veraltete Maßnahmen können unzureichend sein.
- **Least Privilege:** Benutzer erhalten nur die Rechte, die sie für ihre Aufgaben benötigen.
- **Need-to-know:** Datenzugriff nur bei tatsächlichem dienstlichem Bedarf.
- **Defense in Depth:** Mehrere unabhängige Schutzschichten kombinieren, z. B. Firewall, MFA, Verschlüsselung und Monitoring.
- **Regelmäßige Überprüfung:** Berechtigungen, Logs, Backups und Sicherheitskonfigurationen kontrollieren.
- **Wirksamkeit testen:** Backups durch Restore-Tests überprüfen; bloße Erfolgsmeldungen reichen nicht.
- **Risikoangemessenheit:** Schutzmaßnahmen nach Eintrittswahrscheinlichkeit und möglichem Schadensausmaß auswählen.

## 4. Schutzziele und DSGVO

| Schutzziel | Passende TOM |
|---|---|
| Vertraulichkeit | Verschlüsselung, MFA, Zugriffskontrolle |
| Integrität | Hashes, digitale Signaturen, Änderungsprotokolle |
| Verfügbarkeit | Backups, USV, redundante Systeme |
| Belastbarkeit | Redundanz, Monitoring, Notfallkonzept |
| Wiederherstellbarkeit | Restore-Tests, Disaster-Recovery-Plan |

Rechtsgrundlagen:
- **Art. 5 DSGVO:** Grundsätze wie Datenminimierung, Integrität und Vertraulichkeit.
- **Art. 25 DSGVO:** Datenschutz durch Technikgestaltung und datenschutzfreundliche Voreinstellungen.
- **Art. 32 DSGVO:** Sicherheit der Verarbeitung und angemessene TOM.
- **Art. 33 DSGVO:** Meldung bestimmter Datenschutzverletzungen an die Aufsichtsbehörde innerhalb von 72 Stunden nach Bekanntwerden.
- **Art. 28 DSGVO:** Auftragsverarbeitung und AVV.

## 5. Prüfungsfrage mit Musterantwort

**Frage:** Ein Unternehmen speichert Kundendaten in einer Cloud. Nenne drei geeignete TOM und erkläre ihren Zweck.

**Musterantwort:**

1. Verschlüsselung schützt Kundendaten vor unbefugter Kenntnisnahme bei Speicherung und Übertragung.
2. MFA und rollenbasierte Zugriffskontrolle verhindern unbefugte Anmeldungen und begrenzen Datenzugriffe.
3. Regelmäßige Backups mit Restore-Tests ermöglichen die Wiederherstellung nach Datenverlust oder Systemausfällen.

## 6. Verwandte IHK-AP2-Themen

- **BSI IT-Grundschutz:** systematisches Sicherheitsmanagement und Risikoanalyse.
- **BDSG:** nationale Ergänzungen zur DSGVO.
- **BfDI und Landesdatenschutzbehörden:** Datenschutzaufsicht; die zuständige Behörde hängt vom Verantwortlichen und Sachverhalt ab.
- **BSI:** Bundesamt für Sicherheit in der Informationstechnik; Empfehlungen und Standards zur IT-Sicherheit.
- **AVV / DPA:** Vertrag zur Auftragsverarbeitung nach Art. 28 DSGVO.
- **ISO 27001 / ISMS:** systematisches Management der Informationssicherheit.
- **IAM, RBAC, ACL:** Identitäts-, Rollen- und Zugriffskontrollmanagement.
- **Backup, RAID, USV, Disaster Recovery:** Verfügbarkeit und Wiederherstellbarkeit.
- **Incident Response:** Erkennen, Eindämmen, Analysieren und Beheben von Sicherheitsvorfällen.
- **OSI-Modell:** TLS schützt die Datenübertragung kryptografisch; Firewalls filtern Netzwerkverkehr abhängig von Typ und Konfiguration auf unterschiedlichen Ebenen.
