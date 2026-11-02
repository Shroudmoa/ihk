## 1. Aufgabe (Cloud-Migration & Datenschutz)

### aa) Cloud Vor-/Nachteile
- **Vorteil 1 – Skalierbarkeit/Elastizität:** Ressourcen (CPU, RAM, Storage) werden bedarfsgerecht skaliert (Scale out/up). Keine Investition in eigene Hardware; Pay-as-you-go. Entspricht QoS im OSI-Modell Layer 7 (Service-Güte).
- **Vorteil 2 – Verfügbarkeit/Redundanz:** Provider bietet georedundante Rechenzentren; Ausfallsicherheit durch Load Balancer und Failover. Erfüllt RTO/RPO-Anforderungen.
- **Nachteil 1 – Datenhoheit/DSGVO:** Daten liegen beim Provider (ggf. Drittland). Auftragsverarbeitungsvertrag (AVV) nach Art. 28 DSGVO zwingend; Kontrollverlust über Speicherort.
- **Nachteil 2 – Abhängigkeit (Vendor Lock-in):** Proprietäre APIs und fehlende Portabilität erschweren Wechsel. Ausfall des Providers = Ausfall des Dienstes (Single Point of Failure).

### ab) Anforderungen an Provider (Webauftritt)
- **SLA mit garantierter Verfügbarkeit:** Mindestens 99,9% Uptime (entspricht ~8,7h Ausfall/Jahr). RTO/RPO vertraglich fixieren.
- **Datenschutz/AVV:** Auftragsverarbeitungsvertrag nach Art. 28 DSGVO; Serverstandort in EU; Verschlüsselung at-rest und in-transit (TLS 1.3, AES-256).
- **Zertifizierungen:** ISO 27001, SOC 2; regelmäßige Audits und Penetrationstests durch Provider.
- **Skalierung/Performance:** CDN-Anbindung, DDoS-Schutz, garantierte Bandbreite (QoS).

### ba) Datenschutzkonzept – 4 inhaltliche Anforderungen
- **Verzeichnis von Verarbeitungstätigkeiten (Art. 30 DSGVO):** Welche Daten, Zweck, Rechtsgrundlage, Speicherdauer, Empfänger.
- **Technische und organisatorische Maßnahmen (TOMs):** Verschlüsselung, Zutritts-/Zugangs-/Zugriffskontrolle, Pseudonymisierung.
- **Löschkonzept/Fristen:** Definierte Aufbewahrungs- und Löschfristen (Art. 5 DSGVO – Speicherbegrenzung).
- **Betroffenenrechte-Prozess:** Auskunft, Berichtigung, Löschung (Art. 15–17 DSGVO); Verfahren bei Datenschutzvorfällen (Art. 33/34).
- **Datenschutzbeauftragter (DSB):** Benennung und Stellung.

### bb) Datenpanne – 3 unverzügliche Maßnahmen (Art. 33/34 DSGVO)
- **Meldung an Aufsichtsbehörde:** Innerhalb von 72 Stunden nach Bekanntwerden (Art. 33 DSGVO), sofern Risiko für Betroffene besteht.
- **Benachrichtigung der Betroffenen:** Unverzüglich, wenn hohes Risiko (Art. 34 DSGVO) – z.B. Identitätsdiebstahl.
- **Interne Dokumentation:** Vorfall im Verarbeitungsverzeichnis dokumentieren (Art. 33 Abs. 5); Ursachenanalyse und Maßnahmen zur Eindämmung (z.B. Zugriff sperren, Passwörter zurücksetzen).

---

## 2. Aufgabe (Informationssicherheit & Awareness)

### aa) TOM-Zuordnung
- **RFID-Karte ins Gebäude:** **Zutrittskontrolle** (physischer Zugang zum Gebäude).
- **Biometrische Identifikation am PC:** **Zugangskontrolle** (wer darf sich am System anmelden).
- **Berechtigungskonzept auf Dateiebene:** **Zugriffskontrolle** (wer darf welche Daten lesen/schreiben).
- **Datenträger verschlüsseln:** **Zugriffskontrolle** (Schutz bei Diebstahl – Daten nicht lesbar).
- **Alarmanlage außerhalb der Geschäftszeiten:** **Zutrittskontrolle** (physische Absicherung).

### ab) Schutzziele (CIA-Triade)
- **Confidentiality (Vertraulichkeit):** Daten nur für autorisierte Personen zugänglich. Maßnahmen: Verschlüsselung (AES, TLS), Zugriffskontrolle (RBAC), Need-to-know.
- **Integrity (Integrität):** Daten unverändert und korrekt. Maßnahmen: Hashwerte (SHA-256), digitale Signaturen, Checksummen, Schreibrechte.
- **Availability (Verfügbarkeit):** Systeme/Daten sind bei Bedarf nutzbar. Maßnahmen: Redundanz (RAID, Cluster), USV, Backup, DDoS-Schutz.

### ba) Phishing-Merkmale (3)
- **Absenderdomain inkonsistent:** `victorsavage@gmail.de` – keine offizielle ZUMM-Domain; Gmail-Adresse für IT-Service.
- **Zeitdruck/Drohung:** "12 Stunden Zeit", "danach funktioniert Mail nicht mehr" – klassische Urgency-Taktik.
- **Unpersönliche Anrede:** "Lieber Kunde" statt Name; `undisclosed recipients` im CC.
- **Verdächtiger Link:** `https://best.it/Update` – nicht ZUMM-Domain; URL auf Hover prüfen.

### bb) Unregelmäßigkeiten nach Klick (3)
- **Performance-Einbruch:** CPU/RAM dauerhaft hoch, Lüfter laut – Cryptominer oder Ransomware.
- **Netzwerk-Traffic:** Ungewöhnliche ausgehende Verbindungen (C2-Server), DNS zu DGA-Domains.
- **Dateiänderungen:** Dateien verschlüsselt (.locked), neue unbekannte Prozesse im Task-Manager, deaktivierter Virenscanner.

### bc) Malware-Zuordnung
- **Repliziert sich über Netzwerk:** **Wurm** (Worm).
- **Installiert sich mit anderen Programmen:** **Trojaner** (Trojan).
- **Verschlüsselt Daten, fordert Zahlung:** **Ransomware**.
- **Nützlich scheint, im Hintergrund Ressourcen freigibt:** **Trojaner** (Backdoor/Botnet-Client).

### c) White-Hat vs. Black-Hat
- **White-Hat:** Ethische Hacker; Ziel ist Schwachstellen finden und melden (Penetrationstest, Bug Bounty) zur Verbesserung der Sicherheit. Handeln mit Erlaubnis (Authorization).
- **Black-Hat:** Kriminelle Hacker; Ziel ist finanzieller Gewinn, Datenklau, Sabotage, Erpressung. Handeln ohne Erlaubnis, illegal (StGB §202a/c).

---

## 3. Aufgabe (Programmierung – MaxMax2)

### a) Tabellenwerte (Trace)
- **i=0, CPULoad=12:** max=12, max2=0. (12 > 0 → max2=0, max=12)
- **i=1, CPULoad=10:** max=12, max2=0. (10 > 12? nein; 10 > 0? ja → **Fehler**: Zeile 17 setzt `max = CPULoad[i]` statt `max2`)
- **i=2, CPULoad=40:** max=40, max2=12. (40 > 12 → max2=12, max=40)
- **i=3, CPULoad=73:** max=73, max2=40. (73 > 40 → max2=40, max=73)
- **i=4, CPULoad=33:** max=73, max2=40. (33 > 73? nein; 33 > 40? nein → keine Änderung)
- **i=5, CPULoad=60:** max=73, max2=40. (60 > 73? nein; 60 > 40? ja → **Fehler**: Zeile 17 setzt `max = 60` statt `max2 = 60`)
- **Endwerte laut Code:** max=60 (durch Fehler), max2=40. Aufgabe sagt max=73, max2=60 → tatsächlich Fehler in Zeile 17.

### ab) Fehlerursache
- **Zeile 17:** `max = CPULoad[i];` ist falsch. Es muss `max2 = CPULoad[i];` heißen.
- **Wirkung:** Wenn ein Wert größer als max2 aber kleiner als max ist, wird fälschlich max überschrieben. Dadurch geht der eigentliche Maximalwert verloren.
- **Beispiel:** Bei i=5 (60) wird max von 73 auf 60 gesetzt; max2 bleibt 40. max2=60 wird nie erreicht.

### a) Fehlerbeseitigung (Vorlage)
| Programmzeile | Anweisung |
|---|---|
| 17 | `max2 = CPULoad[i];` |

### ba) int-Wertebereich
- **int (32-Bit signed):** −2.147.483.648 bis +2.147.483.647 (−2^31 bis 2^31−1).

### bb) byte statt int für max/max2
- **Speichereffizienz:** CPU-Auslastung ist 0–100%; byte (0–255) reicht aus. 4 Byte pro int vs. 1 Byte pro byte → 75% weniger Speicher.
- **Cache-Effizienz:** Kleinere Datentypen passen besser in CPU-Cache; bei großen Arrays messbar schneller.
- **Fachlich:** Wertebereich ist durch Semantik (Prozent) begrenzt; int ist Overkill.

---

## 4. Aufgabe (Lizenzierung, Ausfallsicherheit, RAID)

### a) CAL-Einsatzfälle
- **User CAL:** Empfohlen, wenn **mehr Geräte als Benutzer** existieren oder Benutzer **roaming** mit mehreren Geräten zugreifen. Lizenz pro Person, unabhängig von Geräteanzahl.
- **Device CAL:** Empfohlen, wenn **mehr Benutzer als Geräte** existieren (Schichtbetrieb, geteilte Arbeitsplätze). Lizenz pro Gerät, unabhängig von Benutzeranzahl.

### b) Notebook-Diebstahl – 3 Maßnahmen
- **Full Disk Encryption (BitLocker/LUKS):** Daten auf Platte verschlüsselt; ohne Pre-Boot-Authentifizierung (PIN/TPM) kein Zugriff.
- **Remote Wipe / MDM:** Mobile Device Management ermöglicht Fernlöschung und Sperrung bei Verlust (über Internet).
- **BIOS/UEFI-Passwort + Secure Boot:** Verhindert Boot von externen Medien und unbefugte Änderungen der Boot-Reihenfolge.
- **Zugriffskontrolle:** Starke Anmeldung (MFA), Account-Lockout.

### c) Hohe Ausfallsicherheit – 3 technische Maßnahmen
- **Redundanz:** RAID (5/6/10), redundante Netzteile, Cluster (Active/Passive oder Active/Active), georedundante Rechenzentren.
- **USV (Online/VFI):** Überbrückt Stromausfälle; verhindert Datenverlust und sauberes Herunterfahren.
- **Backup/Disaster Recovery:** Regelmäßige Vollsicherung + inkrementell/differenziell; RTO/RPO definiert; Offsite-Backup.
- **Monitoring/Failover:** Load Balancer, Health Checks, automatisches Failover bei Ausfall.

### da) RAID-Berechnung (20 TiB netto, 4 TiB pro Platte)
- **RAID 10:** Nettokapazität = 50% der Rohkapazität. Benötigt: 20 TiB netto → 40 TiB roh → 40/4 = **10 Platten** (5 Mirrors × 2).
- **RAID 5:** Nettokapazität = (N−1) × 4 TiB. (N−1)×4 ≥ 20 → N−1 ≥ 5 → N ≥ 6 → **6 Platten** (5 Daten + 1 Parity, netto 20 TiB).
- **RAID 6:** Nettokapazität = (N−2) × 4 TiB. (N−2)×4 ≥ 20 → N−2 ≥ 5 → N ≥ 7 → **7 Platten** (5 Daten + 2 Parity, netto 20 TiB).
- **Hinweis:** Bei RAID 5 mit 6 Platten ist netto exakt 20 TiB; bei RAID 6 mit 7 Platten exakt 20 TiB.

---

**Merksatz:** IHK-Aufgaben sind **Kausalitätsketten** – wer CIA-Triade, TOMs, RAID-Kapazitätsformeln und DSGVO-Fristen (72h!) nicht als Hard-Facts sitzen hat, verliert Punkte an der Oberfläche. Lerne Standards als Vokabeln, nicht als Prosa.
