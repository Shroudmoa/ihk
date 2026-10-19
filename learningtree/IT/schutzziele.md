# Schutzziele

Vertraulichkeit 🔒 – Nur berechtigte Personen dürfen die Daten sehen.
Integrität 🛡 – Daten dürfen nicht unbemerkt verändert oder manipuliert werden.
Verfügbarkeit ⚡️ – Daten und Systeme müssen zugänglich sein, wenn sie gebraucht werden.
Authentizität 👤 – Es muss sichergestellt sein, dass jemand/das System wirklich derjenige ist, für den es sich ausgibt.
Verbindlichkeit - Eine Handlung darf später nicht glaubhaft abgestritten werden.

## Schutzziele der Informationssicherheit (IHK)

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

### Merksätze für die Prüfung

- Vertraulichkeit: Wer darf lesen?
- Integrität: Wurden Daten verändert?
- Verfügbarkeit: Funktioniert das System bei Bedarf?
- Authentizität: Ist der Kommunikationspartner echt?
- Verbindlichkeit: Kann eine Handlung glaubhaft abgestritten werden?
