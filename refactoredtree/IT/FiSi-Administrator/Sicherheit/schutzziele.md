# Schutzziele der Informationssicherheit

## In 30 Sekunden

- Die drei **Grundwerte (CIA-Triade)**: **Vertraulichkeit** (Confidentiality), **Integrität** (Integrity), **Verfügbarkeit** (Availability).
- Erweiterte Schutzziele: **Authentizität** (Echtheit von Person/System/Daten) und **Verbindlichkeit/Nichtabstreitbarkeit**.
- Prüfungsaufgaben verlangen fast immer: **Schutzziel nennen + passende Maßnahme** (2024S, 2023W, 2025W bei VPN/IPsec).
- Merkhilfe: Vertraulichkeit = wer darf lesen, Integrität = unverändert, Verfügbarkeit = nutzbar, wenn gebraucht.

## Kernwissen

### Übersicht

| Schutzziel | Definition | Technische Maßnahmen |
|---|---|---|
| Vertraulichkeit | Nur autorisierte Personen dürfen Daten lesen. | Verschlüsselung (TLS), Zugriffskontrolle (ACL), MFA |
| Integrität | Daten dürfen nicht unbemerkt verändert werden. | Hashwerte (SHA-256), digitale Signaturen, Schreibrechte |
| Verfügbarkeit | Systeme und Daten müssen bei Bedarf nutzbar sein. | RAID, Backups, Redundanz, DDoS-Schutz |
| Authentizität | Die Identität eines Benutzers oder Systems muss nachweisbar sein. | Zertifikate, digitale Signaturen, MFA |
| Verbindlichkeit | Eine Handlung darf später nicht glaubhaft abgestritten werden können (Nichtabstreitbarkeit). | Digitale Signaturen, manipulationsgeschützte Protokollierung |

### IHK-Prüfungswissen

**Vertraulichkeit**
- Bedrohung: Unbefugter Zugriff auf personenbezogene Daten.
- Maßnahme: TLS verschlüsselt die Übertragung; ACLs beschränken Zugriffe.
- DSGVO: Art. 32 verlangt geeignete technische und organisatorische Maßnahmen (TOM).

**Integrität**
- Bedrohung: Manipulation von Daten oder Konfigurationsdateien.
- Maßnahme: SHA-256 erkennt Änderungen, sofern der Referenzhash vertrauenswürdig ist.
- Abgrenzung: Ein Hash allein beweist weder den Absender noch die Authentizität.

**Verfügbarkeit**
- Bedrohung: Hardwareausfall, Ransomware oder DDoS-Angriff.
- Maßnahme: Redundanz, getestete Backups und Notfallkonzepte.
- Kennzahlen: RTO = maximale Wiederherstellungszeit; RPO = maximal tolerierter Datenverlust in Zeit.

**Authentizität**
- Bedrohung: Identitätsdiebstahl oder ein gefälschter Server.
- Maßnahme: Zertifikate, PKI, digitale Signaturen und MFA.
- Abgrenzung: Authentifizierung prüft die Identität; Autorisierung legt die erlaubten Rechte fest.

**Verbindlichkeit**
- Bedrohung: Ein Benutzer bestreitet eine digital ausgelöste Transaktion.
- Maßnahme: Digitale Signaturen und manipulationsgeschützte Protokolle.
- Abgrenzung: Ein normales Logfile allein gewährleistet keine Nichtabstreitbarkeit.

### Verknüpfung mit anderen IHK-Themen

| Thema | Zusammenhang |
|---|---|
| OSI-Modell | TLS arbeitet oberhalb von TCP; IPsec schützt auf der Netzwerkschicht (OSI 3). |
| Protokolle und Ports | HTTPS: TCP 443; SSH: TCP 22; DNS: UDP/TCP 53. |
| Netzwerkgeräte | Firewall filtert Verbindungen; Switch segmentiert LANs über VLANs; Router verbindet Netze. |
| Kryptografie | Symmetrische Verschlüsselung schützt Vertraulichkeit; Hashfunktionen unterstützen Integritätsprüfungen; digitale Signaturen unterstützen Integrität, Authentizität und Verbindlichkeit. |
| DSGVO Art. 5 | Personenbezogene Daten müssen unter anderem rechtmäßig, zweckgebunden und sicher verarbeitet werden. |
| DSGVO Art. 32 | Angemessenes Sicherheitsniveau durch risikobasierte TOM. |
| Backup | Unterstützt Verfügbarkeit und Wiederherstellbarkeit, verhindert aber nicht automatisch Datenmanipulation. |

### Kurzfragen

- Vertraulichkeit: Wer darf lesen?
- Integrität: Wurden Daten verändert?
- Verfügbarkeit: Funktioniert das System bei Bedarf?
- Authentizität: Ist der Kommunikationspartner echt?
- Verbindlichkeit: Kann eine Handlung glaubhaft abgestritten werden?

## Prüfungsfrage

**Frage:** Nennen Sie die drei Schutzziele der Informationssicherheit und ordnen Sie jedem eine technische Maßnahme zu. Welches Schutzziel verletzt ein DDoS-Angriff, welches ein Ransomware-Angriff mit Datenabfluss?

**Musterantwort:** Vertraulichkeit bedeutet, dass nur Berechtigte Daten lesen können – Maßnahme: Verschlüsselung, etwa AES für Datenträger und TLS für Übertragungen. Integrität bedeutet, dass Daten nicht unbemerkt verändert werden – Maßnahme: Hashwerte und digitale Signaturen. Verfügbarkeit bedeutet, dass Systeme und Daten bei Bedarf nutzbar sind – Maßnahme: Redundanz wie RAID und Cluster sowie getestete Backups. Ein DDoS-Angriff verletzt die Verfügbarkeit; Ransomware mit Datenabfluss verletzt Verfügbarkeit und Vertraulichkeit.

## Falle

- „Backup sichert die Integrität“ → ein Backup dient der **Verfügbarkeit** bzw. Wiederherstellbarkeit.
- Verschlüsselung als Schutz der Integrität nennen → sie schützt primär die **Vertraulichkeit**; Integrität sichern Hashes, MACs und Signaturen.
- Authentizität und Autorisierung verwechseln: Authentizität fragt „echt?“, Autorisierung „darf er?“.

## Merksatz

**„C-I-A: Geheim – Unverändert – Erreichbar.“**

## Verwandt

- [ISMS und IT-Grundschutz](isms-bsi-grundschutz.md) – Schutzbedarf je Grundwert
- [TOM nach DSGVO](../../Management/DSGVO/toms.md) – Maßnahmen nach Art. 32 DSGVO
- [Angriffsarten](hacking/hackers-can.md) – Bedrohungen: DoS, Phishing, MitM
- [Digitale Signatur](Verschluesselung/digitale-signatur.md) – Integrität, Authentizität, Verbindlichkeit
- [Lektion 03 Verfügbarkeit und Ausfallsicherheit](../../analyse-der-exams/03_Verfuegbarkeit_Ausfallsicherheit.md)
- [Lektion 05 Datenschutz und Informationssicherheit](../../analyse-der-exams/05_Datenschutz_Informationssicherheit.md)
- [RAID](../Server/raid.md) – Verfügbarkeit durch Redundanz
- [Snapshots-Backups](../Server/virtualisierung/snapshots-backups.md) – Verfügbarkeit durch Sicherungen

