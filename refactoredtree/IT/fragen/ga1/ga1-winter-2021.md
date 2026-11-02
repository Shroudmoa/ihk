```markdown
## 1. Aufgabe (Cloud-Dienste)

### a) Cloud-Vorteile
- **Zeiteinsparung:** Keine Hardware-Beschaffung und Installation; Ressourcen werden per API/Self-Service in Minuten provisioniert statt Wochen.
- **Skalierbarkeit:** Ressourcen (CPU, RAM, Storage) werden elastisch an Lastspitzen angepasst (Scale out/up). Keine Überprovisionierung.
- **Kostenreduktion:** Umwandlung von CAPEX (Investition) in OPEX (Betriebskosten); Pay-as-you-go, keine Kosten für Leerlauf und Wartung eigener Hardware.

### b) Public vs. Private Cloud
- **Public Cloud:** Hohe Skalierbarkeit und Elastizität, geringe Einstiegskosten, global verfügbare Ressourcen. Ideal für variable Lasten.
- **Private Cloud:** Volle Kontrolle über Daten, Standort und Compliance (DSGVO); hohe Sicherheit durch Isolation. Ideal für sensible Daten.

### c) Servicemodelle
- **SaaS:** Fertige Anwendung über das Internet. Kunde nutzt Software, verwaltet nichts (kein OS, keine Middleware). Beispiel: Microsoft 365, Salesforce.
- **PaaS:** Bereitstellung einer Laufzeitumgebung (Runtime, Middleware, DB). Kunde deployt nur die Anwendung; OS/Infrastruktur verwaltet der Provider. Beispiel: Azure App Service, Google App Engine.
- **IaaS:** Bereitstellung virtueller Infrastruktur (VMs, Storage, Netzwerk). Kunde verwaltet OS und Anwendungen. Beispiel: AWS EC2, Azure VMs.

### d) Migration E-Mail und Datenbank
- **E-Mail:** **SaaS** sinnvoll (z.B. Microsoft 365, Google Workspace). Standardisierte Anwendung, kein Betriebsaufwand, hohe Verfügbarkeit.
- **Datenbank:** **PaaS** sinnvoll (z.B. Azure SQL, AWS RDS). Provider übernimmt Patches, Backups, HA; Kunde konfiguriert nur Schema und Zugriffe. IaaS nur bei voller Kontrolle nötig.

### e) Nutzungsgerechte Abrechnung (Measured Service)
- **Prinzip:** Abrechnung nach tatsächlicher Nutzung (Pay-per-Use). Beispiel: VM wird stunden-/sekundengenau abgerechnet; Storage nach belegtem GB; Traffic nach GB. Keine Fixkosten für ungenutzte Ressourcen.

---

## 2. Aufgabe (Speicher & Backup)

### aa) Maximale Betriebszeit SAN (20 TiB, <70% Belegung)
- **Aktueller Datenbestand:** 9 TiB × 90% = 8,1 TiB.
- **Maximale Belegung:** 20 TiB × 70% = 14 TiB.
- **Verfügbarer Zuwachs:** 14 − 8,1 = 5,9 TiB.
- **Jährlicher Zuwachs:** 750 GiB = 750/1024 ≈ 0,7324 TiB.
- **Jahre:** 5,9 / 0,7324 ≈ 8,05 → **8 ganze Jahre** (bei 9 Jahren würde 70% überschritten).

### ab) Cluster-Size – Anwendungsfälle
- **Kleine Cluster (z.B. 512 B/4 KiB):** Viele kleine Dateien (z.B. Log-Dateien, Dokumente). Reduziert internen Fragmentierungsverlust (Slack Space).
- **Große Cluster (z.B. 64 KiB):** Große Dateien (z.B. Videos, Datenbank-Dumps, Images). Weniger Verwaltungsaufwand, höhere sequenzielle Performance, weniger Fragmentierung.

### b) Deduplizierung vs. Komprimierung
- **Deduplizierung:** Identische Datenblöcke werden nur einmal gespeichert und per Referenz verlinkt. Beispiel: Backup mehrerer VMs mit gleichem OS → OS-Blöcke nur einmal; drastische Einsparung.
- **Komprimierung:** Redundante Informationen innerhalb einer Datei werden mittels Algorithmus (z.B. LZ4, ZIP) reduziert. Beispiel: Text-/Log-Dateien oder Datenbank-Dumps werden gepackt; 50–80% Einsparung je nach Datenart.

### d) Revisionssichere Archivierung
- **Prinzip:** Daten werden unveränderbar, vollständig und nachvollziehbar archiviert. Änderungen/Löschungen sind protokolliert; Integrität (Hash/Signatur) und Aufbewahrungsfristen (z.B. 10 Jahre, § 257 HGB, § 147 AO) sind gewährleistet. Zugriff nur lesend; Audit-Trail.

---

## 3. Aufgabe (Verfügbarkeit & Programmierung)

### a) Zuverlässigkeit vs. Verfügbarkeit (Kassensystem)
- **Zuverlässigkeit (Reliability):** System funktioniert über einen definierten Zeitraum ohne Ausfall. Beispiel: Kassensystem darf während der Öffnungszeiten nicht abstürzen (MTBF hoch).
- **Verfügbarkeit (Availability):** System ist zu einem definierten Zeitpunkt erreichbar/nutzbar. Beispiel: 99,9% Uptime während der Geschäftszeiten; bei Ausfall schnelle Wiederherstellung (RTO klein).

### ba) Zwei weitere Maßnahmen für hohe Ausfallsicherheit
- **Redundante Netzteile:** Zwei Netzteile im Server (Hot-Swap); bei Ausfall eines Netzteils übernimmt das andere.
- **ECC-RAM:** Fehlerkorrigierender Speicher; verhindert Datenkorruption durch Bitfehler (Integrität).
- **Redundante Netzwerkanbindung:** Zwei NICs (Teaming/Bonding) an zwei Switches; verhindert Single Point of Failure.

### bb) MTBF für 16-Platten-Verbund
- **Formel:** MTBF_Verbund = MTBF_Einzel / Anzahl Platten = 800.000 h / 16 = 50.000 h.
- **In Jahren (24/7):** 50.000 h / (24 × 365) = 50.000 / 8.760 ≈ **5,71 Jahre**.

### ca) Bits für KiB-Wert (16 TiB)
- **16 TiB in KiB:** 16 × 1024^3 = 16 × 1.073.741.824 = 17.179.869.184 KiB.
- **Bits:** 2^n ≥ 17.179.869.184 → 2^34 = 17.179.869.184. Also **34 Bit** mindestens.

### cb) Zugriff auf Struct-Property
- **Code:** `Disk_2[0].wert` – Zugriff auf das erste Array-Element (Index 0) und dessen Property `wert`. Beispiel: `int belegung = Disk_2[0].wert;`

### cd) SQL: Anzahl Einträge am 21.11.2021
```sql
SELECT COUNT(*) FROM tblDiskM WHERE Datum = '2021-11-21';
```

### cd) UML-Diagramme
- **Statische Sicht:** Klassendiagramm, Objektdiagramm, Komponentendiagramm, Paketdiagramm.
- **Dynamische Sicht:** Aktivitätsdiagramm, Sequenzdiagramm, Kommunikationsdiagramm, Zustandsdiagramm.

---

## 4. Aufgabe (Datenschutz & Verschlüsselung)

### aa) Drei Datenschutz-Kriterien bei Speicherung
- **Zweckbindung:** Daten nur für den festgelegten Zweck speichern (Art. 5 DSGVO).
- **Datenminimierung:** Nur notwendige Daten erheben/speichern.
- **Speicherbegrenzung/Löschfristen:** Definierte Aufbewahrungsdauer, danach löschen.
- **Rechtsgrundlage:** Einwilligung oder gesetzliche Erlaubnis (Art. 6 DSGVO).
- **Sicherheit (TOMs):** Verschlüsselung, Zugriffskontrolle.

### ab) Vorschriftsmäßige Entsorgung von Festplatten
- **Physische Zerstörung:** Schreddern oder Degaussing (Entmagnetisierung) – Daten sind unwiederbringlich.
- **Software-basiertes sicheres Löschen:** Mehrfaches Überschreiben mit Zufallsdaten (z.B. nach BSI TR-02102 oder NIST 800-88). Bei SSDs: Secure Erase (ATA/NVMe).
- **Nachweis:** Zertifikat über Datenvernichtung durch zertifizierten Dienstleister (ISO 27001).

### ac) Warum Formatieren/Löschen nicht ausreicht
- **Grund:** Formatieren oder Löschen entfernt nur die Dateisystem-Referenzen; die Rohdaten bleiben auf dem Datenträger und können mit Forensik-Tools (z.B. TestDisk, PhotoRec) wiederhergestellt werden. Datenschutzverstoß (Art. 32 DSGVO).

### ba) Informationspflichten bei unbefugtem Zugriff
- **Meldung an Aufsichtsbehörde:** Innerhalb 72h (Art. 33 DSGVO), wenn Risiko besteht.
- **Benachrichtigung der Betroffenen:** Unverzüglich, wenn hohes Risiko (Art. 34 DSGVO).
- **Dokumentation:** Vorfall im Verzeichnis dokumentieren (Art. 33 Abs. 5).

### bb) Maßnahmen gegen unbefugten Zugriff (Datenbank)
- **Zugriffskontrolle (RBAC):** Nur notwendige Rechte; Least Privilege.
- **Verschlüsselung:** Datenbankverschlüsselung (TDE) und TLS für Verbindungen.
- **Monitoring/Audit:** Logging von Zugriffen, IDS/IPS, Alarm bei Anomalien.
- **Patch-Management:** Regelmäßige Updates schließen Schwachstellen.

### c) Verschlüsselungsalgorithmen
| Algorithmus | Bedeutung |
|---|---|
| **AES-128** | Symmetrisch, 128-Bit-Schlüssel. Sicher, schnell; Standard für viele Anwendungen. |
| **AES-256** | Symmetrisch, 256-Bit-Schlüssel. Höchste Sicherheit; empfohlen für sensible Daten. |
| **MD5** | Hashfunktion, 128 Bit. Kryptografisch gebrochen (Kollisionen); nicht für Sicherheit geeignet, nur für Integritätschecks. |
| **SHA256** | Hashfunktion, 256 Bit. Sicher; Standard für Signaturen, Zertifikate, Integrität. |

### d) Anonymisierung vs. Pseudonymisierung
- **Anonymisierung:** Personenbezug wird unwiederbringlich entfernt. Re-Identifikation ist ausgeschlossen (auch mit Zusatzwissen). Daten fallen aus dem Anwendungsbereich der DSGVO.
- **Pseudonymisierung:** Personenbezug wird durch ein Pseudonym ersetzt; mit Zusatzwissen (Mapping-Tabelle) ist Re-Identifikation möglich. Daten bleiben personenbezogen (DSGVO gilt weiter).
- **Unterschied:** Anonymisierung = irreversibel, Pseudonymisierung = reversibel.

---

**Merksatz:** IHK-Aufgaben sind **Kausalitätsketten** – wer Cloud-Servicemodelle (IaaS/PaaS/SaaS), RAID-Formeln, MTBF und DSGVO-Fristen (72h!) nicht als Hard-Facts sitzen hat, verliert Punkte an der Oberfläche. Lerne Standards als Vokabeln, nicht als Prosa.
```
