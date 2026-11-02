## 1. Aufgabe (Cloud & Mailserver)

### aa) Organisatorische Vorteile Cloud (Mailserver)
- **Kein Betriebsaufwand:** Provider übernimmt Wartung, Patches, Updates und Hardware; IT-Personal wird entlastet.
- **Skalierbarkeit:** Neue Mailkonten werden per Self-Service oder API in Minuten bereitgestellt; keine Hardware-Beschaffung.
- **Ortsunabhängigkeit/Homeoffice:** Zugriff über Internet von überall; keine VPN-Bindung an ein Rechenzentrum.
- **Kostenmodell:** CAPEX → OPEX; keine Investition in Server-Hardware, Storage und Kühlung.

### ab) Aspekte vor Cloud-Auslagerung
- **DSGVO/Compliance:** Auftragsverarbeitungsvertrag (AVV) nach Art. 28 DSGVO; Serverstandort (EU/Drittland), Zertifizierungen (ISO 27001).
- **Latenz/Performance:** Netzwerkanbindung zum Provider (Bandbreite, RTT); kritisch für synchrone Dienste (z.B. Mail-Gateway).
- **Vendor Lock-in/Exit-Strategie:** Datenexport, Portabilität, SLA mit RTO/RPO.

### ba) Server-Auswahl für Mailserver
- **Hapé Proviant L5603 CPU:**
  - *Begründung:* Höchste Skalierbarkeit für wachsende Mailkonten. 64 GB RAM (mehr als ausreichend für Mail-Dienste), 4 × 1 GbE + 2 × 10 GbE (redundante Netzanbindung, hoher Durchsatz), max. 12 Festplatten (großer Storage-Pool). 36 Kerne bei 2,3 GHz bieten mehr Reserven für zukünftige Last. Preis höher, aber langfristig zukunftssicher.
- **Renovo:** 64 GB RAM, 6 × 1 GbE (keine 10 GbE), max. 8 Platten. Gute CPU, aber Netzwerk nicht redundant genug.
- **Dall PowerEcke:** 32 GB RAM, nur 2 × 1 GbE (Single Point of Failure), max. 8 Platten. Zu schwach für Wachstum.

### bb) TOM zum Schutz von E-Mails (6)
- **SPF/DKIM/DMARC:** Verhindert Spoofing und Phishing; SPF im DNS, DKIM signiert Mails, DMARC-Policy.
- **TLS-Verschlüsselung (SMTP over TLS, Port 587):** Schützt Vertraulichkeit und Integrität auf Transportweg.
- **Spam-/Phishing-Filter:** Gateway-Filter (z.B. Mail-SeCure) mit Blacklists, Heuristik, Sandboxing.
- **Virenscanner/Attachment-Filter:** Blockiert ausführbare Anhänge; scannt MIME-Parts.
- **Backup der Mailboxen:** Regelmäßige Sicherung (z.B. täglich) und Restore-Tests.
- **MFA für Mail-Zugang:** Zusätzliche Authentifizierung (TOTP, FIDO2) für Webmail/IMAP.
- **Least Privilege:** Nur notwendige Rechte für Postfächer; keine Shared-Mailboxen mit Vollzugriff.

### bc) Lichtwellenleiter vs. Kupfer
- **Aspekt:** Höhere Bandbreite und geringere Dämpfung über große Distanzen; immun gegen EMI/RFI (elektromagnetische Interferenzen). Zwischen RZ-Stockwerken/Standorten (bis km) ist Glasfaser unverzichtbar; Kupfer limitiert auf ~100 m (Twisted Pair).

---

## 2. Aufgabe (Mail-Security & Zertifikate)

### a) Spam vs. Phishing
- **Spam:** Unerwünschte Massen-E-Mails (Werbung, Betrug); Ziel ist Verbreitung, nicht gezielte Schädigung.
- **Phishing:** Gezielte Täuschung, um Zugangsdaten/Finanzdaten zu stehlen. Imitiert vertrauenswürdige Absender; enthält Links zu Fake-Seiten.

### ba) Stelle der Signaturidentität
- **DKIM-Signatur-Header:** `DKIM-Signature:` im Mail-Header. Enthält `d=` (Domain) und `s=` (Selector).

### bb) Mechanismus für öffentlichen Schlüssel
- **DNS-TXT-Record:** `selector._domainkey.domain.tld` TXT-Record enthält den Public Key. Kein CA erforderlich.

### bc) Stelle des SPF-Eintrags
- **DNS-TXT-Record:** `domain.tld` TXT-Record mit `v=spf1 ...`. Nur ein SPF-Record pro Domain erlaubt.

### bd) Aufgabe des SPF-Eintrags
- **Autorisierung:** Definiert, welche Hosts/IPs E-Mails für die Domain versenden dürfen (HELO/MAIL FROM). Verhindert Spoofing; Empfänger prüft, ob Sender-IP im SPF-Record enthalten ist.

### a) DKIM Verification Failed – Grund
- **Grund:** Öffentlicher Schlüssel im DNS stimmt nicht mit dem privaten Schlüssel überein, mit dem die Mail signiert wurde. Ursachen: DNS-Record geändert/gelöscht, Selektor falsch, Mail wurde unterwegs verändert (Header-Manipulation).

### b) SMTP 554 5.7.1 BLOCKLIST_SENDER_DOMAIN – Grund
- **Grund:** Absender-Domain steht auf einer Blocklist (DNSBL/RBL) des Empfängers. Der Mailserver des Empfängers lehnt die Mail dauerhaft ab (permanent error). Ursache: Domain wurde für Spam/Phishing missbraucht.

### da) Eigenschaften persönlicher X.509-Zertifikate (2)
- **Bindung an Identität:** Enthält Subject (Name, E-Mail), Public Key und Signatur der CA; bindet öffentlichen Schlüssel an Person.
- **Gültigkeitsdauer/Chain of Trust:** Ausgestellt von vertrauenswürdiger CA; enthält Valid From/To; prüfbar über Zertifikatskette.
- **Verwendung:** Signatur (Authentication, Integrity) und Verschlüsselung (Confidentiality) von E-Mails (S/MIME).

### d) E-Mail-Verschlüsselung/Signatur – Schlüsselzuordnung
- **Schritt 2 (Signieren):** Herr Meier signiert mit **seinem privaten Schlüssel**.
- **Schritt 3 (Verschlüsseln):** Herr Meier verschlüsselt mit **Herrn Antons öffentlichem Schlüssel**.
- **Schritt 5 (Entschlüsseln):** Herr Anton entschlüsselt mit **seinem privaten Schlüssel**.
- **Schritt 6 (Signatur prüfen):** Herr Anton prüft mit **Herrn Meiers öffentlichem Schlüssel**.

---

## 3. Aufgabe (RAID & Deduplizierung)

### aa) RAID 15 – Nettokapazität
- **Konfiguration:** RAID 15 = RAID 5 über mehrere RAID-1-Mirrors.
- **Gegeben:** 4 × 2,5 TiB + 2 × 3,5 TiB.
- **Mirrors:** Zwei 3,5-TiB-Platten → Mirror mit 3,5 TiB netto. Vier 2,5-TiB-Platten → zwei Mirrors à 2,5 TiB = 5 TiB netto.
- **RAID 5 über drei Mirrors:** (3 − 1) × kleinste Mirror-Kapazität = 2 × 2,5 TiB = **5 TiB netto**.
- **Rechenweg:** Mirror 1: 3,5 TiB; Mirror 2: 2,5 TiB; Mirror 3: 2,5 TiB. RAID 5 mit Parity über 3 Member: Netto = (3−1) × 2,5 TiB = 5 TiB.

### ab) Maximale Nettokapazität mit Ausfallsicherheit
- **RAID 5 über alle 6 Platten:** Netto = (6−1) × 2,5 TiB = 12,5 TiB (aber gemischte Größen → kleinste Platte 2,5 TiB limitiert).
- **Besser:** RAID 5 über die vier 2,5-TiB-Platten: (4−1) × 2,5 = 7,5 TiB. Plus RAID 1 über die zwei 3,5-TiB-Platten: 3,5 TiB. Gesamt: 7,5 + 3,5 = **11 TiB** (kein gemeinsamer RAID-Verbund).
- **RAID 6 über alle 6:** (6−2) × 2,5 = 10 TiB.
- **Maximal:** RAID 5 über 4 × 2,5 TiB (7,5 TiB) + RAID 1 über 2 × 3,5 TiB (3,5 TiB) = **11 TiB**. In GiB: 11 × 1024 = **11.264 GiB**.

### ac) Hot-Spare-Festplatte
- **Funktion:** Reserve-Platte im RAID-Verbund, die im Normalbetrieb nicht genutzt wird. Bei Ausfall einer Platte übernimmt sie automatisch den Rebuild; kein manueller Eingriff. Reduziert RTO und Risiko eines zweiten Ausfalls während des Rebuilds.

### ad) JBOD – 3 Aspekte gegen Einsatz
- **Keine Redundanz:** Kein Ausfallschutz; eine defekte Platte = Datenverlust auf dieser Platte.
- **Keine Performance-Vorteile:** Kein Striping; sequenzielle Zugriffe nur auf eine Platte limitiert.
- **Verwaltungsaufwand:** Jede Platte muss separat überwacht, gesichert und verwaltet werden; kein gemeinsamer Verbund.

### b) Deduplizierung – 2 Vorteile
- **Speichereinsparung:** Identische Datenblöcke werden nur einmal gespeichert; 10–30× Reduktion bei Backups.
- **Kosteneffizienz:** Weniger physischer Speicher nötig; geringere Hardware- und Energiekosten.

### c) SAN-Nettokapazität (3 Jahre)
- **Altdaten:** 12 TiB × 90% = 10,8 TiB.
- **Zuwachs:** 650 GiB/Jahr × 3 = 1950 GiB = 1950/1024 ≈ 1,904 TiB.
- **Gesamt:** 10,8 + 1,904 = **12,704 TiB ≈ 12,7 TiB**.

---

## 4. Aufgabe (Programmierung – Mittelwerte)

### aa) Quellcode-Analyse – Ist-Werte
- **Code:** `for (int i = 1; i < 6; i += 3)` → i = 1, 4.
- **i=1:** temp = (40 + 42 + 47) / 3 = 129/3 = 43. `PerfCPU[1]=0; PerfCPU[0]=43; PerfCPU[2]=0;` → Array: 43, 0, 0, 52, 50, 60, 76, 77, 81.
- **i=4:** temp = (52 + 50 + 60) / 3 = 162/3 = 54. `PerfCPU[4]=0; PerfCPU[3]=54; PerfCPU[5]=0;` → Array: 43, 0, 0, 54, 0, 0, 76, 77, 81.
- **Ergebnis:** **43, 0, 0, 54, 0, 0, 76, 77, 81**.

### ab) Zwei korrekte Anweisungen
- **Fehler 1 – Schleifengrenze:** `i < 6` läuft nur bis i=4; das letzte Triple (Index 6,7,8) wird nicht verarbeitet. Korrekt: `i < PerfCPU.Length - 1` oder `i <= 7`.
- **Fehler 2 – Falsche Zuweisung:** `PerfCPU[i] = 0; PerfCPU[i-1] = temp; PerfCPU[i+1] = 0;` – Der Mittelwert muss in die **mittlere** Position (Index i), nicht in i-1. Korrekt: `PerfCPU[i-1] = 0; PerfCPU[i] = temp; PerfCPU[i+1] = 0;`

### ba) CSV-Vorteile (2)
- **Einfachheit/Kompatibilität:** Reiner Text, von Excel, LibreOffice und Skripten lesbar; keine spezielle Software nötig.
- **Geringe Dateigröße:** Kein Overhead durch Metadaten/Formatierung; effizient für große Datenmengen.

### bb) CSV-Nachteil
- **Kein Standard für Sonderzeichen/Escaping:** Kommas, Anführungszeichen und Zeilenumbrüche in Feldern erfordern Quoting; unterschiedliche Implementierungen führen zu Parsing-Fehlern. Keine Typinformationen (alles Text).

---

**Merksatz:** IHK-Aufgaben sind **Kausalitätsketten** – wer RAID-15-Netto, DKIM/SPF-DNS-Records und Array-Traces nicht als Hard-Facts sitzen hat, verliert Punkte an der Oberfläche. Lerne Standards als Vokabeln, nicht als Prosa.
