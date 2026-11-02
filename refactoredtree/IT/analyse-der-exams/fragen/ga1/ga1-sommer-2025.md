## 1. Aufgabe (Virtualisierung, RAID & USV)

### aa) Virtualisierung – 3 Vor- / 3 Nachteile
- **Vorteil 1 – Konsolidierung:** Mehrere VMs auf einem Host erhöhen die Hardware-Auslastung (CPU/RAM/Storage); senkt Strom- und Kühlkosten (Green IT).
- **Vorteil 2 – Flexibilität/Provisioning:** VMs lassen sich in Minuten klonen, snapshotten und migrieren (z.B. Live-Migration). Disaster Recovery via VM-Images.
- **Vorteil 3 – Isolation:** Fehler oder Kompromittierung einer VM bleiben im Idealfall auf diese beschränkt; Host-Trennung erhöht Security.
- **Nachteil 1 – Single Point of Failure:** Fällt der Host aus, fallen alle VMs aus. Erfordert HA-Cluster (Redundanz) – kostet mehr.
- **Nachteil 2 – Performance-Overhead:** Hypervisor verbraucht Ressourcen; I/O-Latenz und CPU-Overhead bei dichter Konsolidierung (Noisy Neighbor).
- **Nachteil 3 – Komplexität/Lizenzkosten:** Mehr Management-Aufwand (Hypervisor, Backup, Monitoring); Lizenzmodelle (vCPU, Core) können teuer werden.

### ab) Hypervisor-Typen
- **Typ-1 (Bare-Metal):**
  - *Erläuterung:* Läuft direkt auf der Hardware, kein Host-OS. Dünne Schicht, hohe Performance.
  - *Einsatzbeispiel:* Rechenzentrum, Server-Virtualisierung, Cloud-Infrastruktur.
  - *Produkt:* VMware ESXi, Microsoft Hyper-V, Citrix XenServer.
- **Typ-2 (Hosted):**
  - *Erläuterung:* Läuft als Anwendung auf einem Host-Betriebssystem; nutzt dessen Treiber und Dienste.
  - *Einsatzbeispiel:* Entwickler-Arbeitsplatz, Test- und Schulungsumgebungen.
  - *Produkt:* Oracle VirtualBox, VMware Workstation, Parallels Desktop.

### ba) RAID-6 Erweiterung auf 160 TiB (8 TiB/Platte)
- **Formel RAID 6:** Netto = (N − 2) × 8 TiB. Benötigt: 160 TiB.
- **Rechnung:** (N − 2) × 8 ≥ 160 → N − 2 ≥ 20 → N ≥ 22 → **22 Platten**.
- **Ergebnis:** 22 Festplatten (20 Daten + 2 Parity). Netto exakt 160 TiB.

### bb) RAID 10 vs. RAID 6 bei 2 Plattenausfällen
- **RAID 6:** Übersteht den gleichzeitigen Ausfall von **2 Platten** (Dual-Parity). Daten bleiben verfügbar; Rebuild möglich.
- **RAID 10:** Übersteht den Ausfall von 2 Platten **nur**, wenn sie in **verschiedenen Mirrors** liegen. Fallen beide Platten eines Mirror-Paares aus, ist der Verbund verloren (Datenverlust).
- **Fazit:** RAID 6 ist bei 2 Ausfällen sicherer; RAID 10 bietet bessere Performance, aber geringere Ausfalltoleranz bei ungünstiger Verteilung.

### bc) RAID 10 mit 22 Platten (8 TiB)
- **Formel RAID 10:** Netto = 50% der Rohkapazität = (N / 2) × 8 TiB.
- **Rechnung:** (22 / 2) × 8 = 11 × 8 = **88 TiB**.
- **Hinweis:** Alternative mit 12 Platten: (12/2) × 8 = 48 TiB.

### c) USV – prozentualer Ladungsstand für Notstromstart
- **USV-Ausgangsleistung:** 2400 VA; Serverlast: 600 VA → Auslastung = 25%.
- **Laufzeit bei 100% Ladung:** 60 Minuten (bei 600 VA Last rechnerisch).
- **Berechnung:** 100% Ladung ≙ 60 min Laufzeit. 30% Restladung ≙ 18 min. Notstrom braucht 3 min → Start bei 18 + 3 = **21 min** Restlaufzeit.
- **Prozent:** (21 / 60) × 100 = **35%**.
- **Ergebnis:** Start bei **35%** Ladungsstand.

---

## 2. Aufgabe (IT-Sicherheitsvorfall & DSGVO)

### a) DSGVO – 3 Kriterien für Kundeninformation
- **Risiko für Rechte und Freiheiten:** Hohes Risiko für Betroffene (Art. 34 DSGVO), z.B. Identitätsdiebstahl, finanzielle Schäden.
- **Art der Daten:** Besonders sensible Daten (Art. 9 DSGVO) wie Gesundheits-, Finanz- oder Authentifizierungsdaten.
- **Wahrscheinlichkeit und Schwere:** Eintrittswahrscheinlichkeit und Schwere des Schadens; Anzahl betroffener Personen.
- **Ergänzend:** Meldung an Aufsichtsbehörde innerhalb 72h (Art. 33) ist unabhängig von der Kundeninformation.

### b) Backup-Optimierung
- **Verkürzung der Wiederherstellungszeit (RTO):** Einsatz von **Snapshots** und **deduplizierten Disk-basierten Backups** statt LTO-Band. Restore von Disk/SSD ist um Größenordnungen schneller als Band. Zusätzlich **Continuous Data Protection (CDP)** oder Replikation.
- **Vermeidung von Datenverlust (RPO):** **Häufigere inkrementelle/differenzielle Backups** (z.B. stündlich) oder **CDP/Replikation** in Echtzeit. Reduziert RPO auf Minuten. Zusätzlich **Write-Ahead-Log/Transaktionslogs** der Datenbank sichern.

### c) 3-2-1-Regel – Vorschlag
- **3 Kopien:** Produktivdaten + 2 Backups.
- **2 verschiedene Medien:** z.B. NAS (Disk) + LTO-Band (Tape) oder Cloud-Storage.
- **1 Kopie offsite:** Georedundant, z.B. Rechenzentrum eines anderen Standorts oder Cloud (EU-Region).
- **Beispiel:** Produktiv-SAN → tägliches Backup auf NAS (Medien 1) → wöchentliches Tape-Backup ins Offsite-Lager (Medium 2, offsite). Optional Cloud-Replikation als dritte Kopie.

### d) System-Härtung – 2 Maßnahmen
- **Deaktivierung unnötiger Dienste/Ports:** Reduziert Angriffsfläche (Attack Surface). Z.B. SMBv1, Telnet, RDP nach außen deaktivieren.
- **Least-Privilege & Account-Härtung:** Keine Standard-/Admin-Konten für Alltag; starke Passwort-Policy, MFA, Account-Lockout.
- **Weitere:** AppLocker/Whitelisting, Entfernen nicht benötigter Software, Protokollierung/Audit aktivieren.

### e) Least-Privilege vs. Zero-Trust
- **Least-Privilege:** Benutzer/Prozesse erhalten nur die minimal notwendigen Rechte. Maßnahme: **RBAC (Role-Based Access Control)** mit granularen Berechtigungen; Admin-Konten nur für Admin-Tätigkeiten (Tier-Modell).
- **Zero-Trust:** Kein implizites Vertrauen – auch intern nicht. Maßnahme: **MFA für alle Zugriffe**, Micro-Segmentation (Netzwerkzonen), kontinuierliche Verifikation von Identität und Gerätezustand (Device Posture).

### f) Zusätzliche sicherheitsrelevante Tests
- **Penetrationstest (Black-Box/White-Box):** Simulierte Angriffe auf externe und interne Systeme; prüft Ausnutzbarkeit von Schwachstellen.
- **Social-Engineering-Test (Phishing-Simulation):** Prüft Awareness der Mitarbeiter; identifiziert Schulungsbedarf.
- **Red-Team-Exercise:** Ganzheitliche Angriffssimulation über mehrere Wochen.
- **Schwachstellen-Scan (automatisiert):** Regelmäßiger Vulnerability-Scan (z.B. Nessus, OpenVAS).

---

## 3. Aufgabe (MDM-Systeme)

### aa) Bester Schutz bei Verlust/Diebstahl
- **Anbieter A:** Unterstützt **Wipe Mobile Device** (Remote-Löschung) und **Lockdown Mobile Device** (Fern-Sperrung). Anbieter B hat kein Wipe; Anbieter C hat kein Lockdown.
- **Begründung:** Nur mit Wipe und Lockdown kann bei Diebstahl der Zugriff auf Unternehmensdaten sofort unterbunden werden. Anbieter A bietet beides.

### ab) Schutz privater Daten bei BYOD
- **Containerisierung / Workspace-Trennung:** Private und dienstliche Daten werden getrennt (z.B. Android Work Profile, iOS Managed Apps). Das MDM verwaltet nur den dienstlichen Container.
- **Selektives Wipe:** Bei Verlust wird nur der Firmen-Container gelöscht; private Fotos/Daten bleiben unberührt.
- **Datenschutz:** MDM darf private Daten nicht einsehen; nur Firmen-Container wird überwacht.

### ac) 5 weitere Sachverhalte für Betriebsvereinbarung
- **Zulässige Endgeräte/OS-Versionen:** Mindestanforderungen an Hardware und Software (z.B. Android ≥ 12, iOS ≥ 16).
- **Nutzungsgrenzen:** Private Nutzung erlaubt/verboten; Kostenübernahme (z.B. Datenvolumen).
- **MDM-Rechte:** Welche Aktionen darf das MDM ausführen (Wipe, Ortung, App-Installation)?
- **Datenschutz/Überwachung:** Verbot der Leistungs-/Verhaltenskontrolle; Hinweis auf § 87 BetrVG (Mitbestimmung).
- **Verhalten bei Verlust/Diebstahl:** Meldepflicht, Sperrung, selektives Wipe.
- **Exit-Szenario:** Was passiert bei Ausscheiden des Mitarbeiters (Datenlöschung, Rückgabe)?

### b) CAL-Einsatzfälle (MDM-Server)
- **User CAL:** Empfohlen, wenn **mehr Geräte als Benutzer** existieren oder Mitarbeiter **mehrere Geräte** nutzen (roaming). Lizenz pro Person; Geräteanzahl irrelevant.
- **Device CAL:** Empfohlen, wenn **mehr Benutzer als Geräte** existieren (Schichtbetrieb, geteilte Geräte). Lizenz pro Gerät; Benutzeranzahl irrelevant.

### c) Update vs. Upgrade (MDM)
- **Update:** Patch/Fehlerbehebung innerhalb der gleichen Major-Version (z.B. 1.1 → 1.2). Meist **kostenlos** (oder Wartungsvertrag). Funktionsumfang bleibt gleich; Fokus auf Sicherheit/Stabilität.
- **Upgrade:** Wechsel auf eine höhere Major-Version (z.B. 1.x → 2.x). Oft **kostenpflichtig** (neue Lizenz). Neue Funktionen, ggf. Migrationsaufwand und Inkompatibilitäten.

---

## 4. Aufgabe (Programmierung – Array-Kopie)

### a) Werte in den ersten neun Feldern von RAM2
- **Code:** `RAM2[i*2+1] = i; RAM2[i*2] = RAM1[i];`
- **Trace:**
  - i=0: RAM2[1]=0, RAM2[0]=40
  - i=1: RAM2[3]=1, RAM2[2]=42
  - i=2: RAM2[5]=2, RAM2[4]=47
  - i=3: RAM2[7]=3, RAM2[6]=52
  - i=4: RAM2[9]=4, RAM2[8]=50 (RAM2[9] liegt außerhalb der ersten neun Felder)
- **Erste neun Felder (Index 0–8):** **40, 0, 42, 1, 47, 2, 52, 3, 50**

### b) Zwei Fehler + korrekter Code
- **Fehler 1 – Schleifenbedingung:** `for (int i = 0; i < 9; i++)` läuft nur bis i=8; das letzte Element RAM1[8]=71 wird nicht korrekt verarbeitet. Zudem ist die Schleife für 9 Elemente korrekt, aber der Fehler liegt in der Index-Berechnung.
- **Fehler 2 – Falscher Zielindex für die Füllzahlen:** `RAM2[i*2+1] = i;` schreibt 0,1,2,… statt der geforderten Zahlen 1,2,3,… (Schema zeigt 1,2,3,…). Korrekt: `RAM2[i*2+1] = i + 1;`
- **Korrekter Code 1:** `RAM2[i * 2 + 1] = i + 1;`
- **Korrekter Code 2:** Die Schleife muss bis `i < RAM1.Length` laufen (nicht hartkodiert 9), um alle Elemente zu erfassen. `for (int i = 0; i < RAM1.Length; i++)`

### b) Syntaxfehler vs. Semantikfehler
- **Syntaxfehler:** Verstoß gegen die Grammatik der Programmiersprache; Compiler meldet Fehler. Beispiel: `int x = "Hallo";` (Typkonflikt) oder fehlendes Semikolon `int x = 5`.
- **Semantikfehler:** Code ist syntaktisch korrekt, aber die Logik ist falsch; Compiler meldet nichts, Laufzeitverhalten ist falsch. Beispiel: `RAM2[i*2+1] = i;` statt `i+1` – Programm läuft, liefert aber falsche Werte.

---

**Merksatz:** IHK-Aufgaben sind **Kausalitätsketten** – wer RAID-6-Formel `(N-2)*K`, USV-Laufzeit als lineare Funktion und Syntax-vs-Semantik nicht als Hard-Facts sitzen hat, verliert Punkte an der Oberfläche. Lerne Standards als Vokabeln, nicht als Prosa.
