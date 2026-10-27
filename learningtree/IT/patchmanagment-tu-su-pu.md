# Patchmanagment-TU-SU-PU

# Patchmanagement – IHK AP2 FISI 2026 (Technisch & Prüfer-relevant)

---

### Was du geschrieben hast (Korrektur & Ergänzung)

**Deine Punkte sind inhaltlich richtig, aber teils formlos/mit Tippfehlern. Hier die technisch-polierte Version für die Prüfung:**

1. **Update verfügbar ≠ installieren**
   Updates können instabil, fehlerhaft, inkompatibel mit bestehender Software/Diensten sein.

2. **Mehrstufiger Prozess: TU → SU → PU**
   - **TU (Testumgebung)**: Isoliert (oft VMs), wenig Aufwand. Wichtige Systeme abgebildet. Fehler hier = keine Wirkung auf PU.
   - **SU (Staging)**: Abbildung realer Bedingungen (same Hardware, Config, Datenmenge). Prüfung von Performance, Schnittstellen, kleineren Gruppen intern, bevor ausgerollt wird.
   - **PU (Production)**: Gestaffelt (Rolling Update), nur Teil der Server/ Gruppen. Beobachtungsphase. ROLLBACK ist einfach und möglich, wenn unentdeckte Probleme auftauchen.

3. **Snapshots & Backups**
   Vor jedem Update Pflicht. Basis für Rollback bei kritischen Folgen.

4. **Zeitfaktor: Kritikalität vs. Sorgfalt**
   Nicht jedes Update muss dringend installiert werden. Security-Patches (z. B. PHP) schneller durchlaufen, Feature-Updates priorisiert prüfen.

5. **Zwei Faktoren beim Testen**
   Sorgfalt und Geschwindigkeit gegeneinander abwägen. Ungetestete Updates = Ausfälle & Inkompatibilität.

---

### IHK-prüfungsrelevante Details (Die "Sache")

| Begriff | IHK-relevante Definition |
|---------|--------------------------|
| **Änderungsmanagement (Change Management)** | Strukturierter Prozess: Änderungsantrag → Bewertung → Terminplanung → Umsetzung → Überprüfung → ROLLBACK. Dokumentationspflicht (§ 11 BSIG, DSGVO-Art. 32). |
| **Kritikalitätsstufen** | *Critical*: Sofort (Security-Patches, CVE > 9.0). *High*: Woche. *Medium*: Monat. *Low*: Quartal/Feature. Priorisierung anhand von Geschäftsrisiko. |
| **Rolling Update** | Updates werden nacheinander auf Knoten/Servern gespielt, Gesamtverfügbarkeit bleibt gewahrt. ROLLBACK je Knoten möglich. |
| **Snapshot / Image** | Exakte Kopie von VM/Disk vor Update. Wiederherstellung in Minuten. Prüfung: "Ist Snapshot aktuell und bootfähig?" |
| **Logging & Monitoring** | Jedes Patch-Vorhaben muss im Log (z. B. WSUS, SCCM, apt-log) mit User, Zeitpunkt, Ergebnis stehen. DSGVO-konform: Keine sensiblen Daten in Clear-Text, Verschlüsselung bei Transport (TLS). |
| **Kompatibilitätstest** | Prüfung auf: DLL-Hijacking, Dienst-Abhängigkeiten, Treiber-Kompatibilität, Datenbank-Schema-Updates. |

---

### 3-Linien-Exam-Antwort (Passe genau auf deine Formatvorgabe)

1. Patchmanagement folgt dem Zyklus **TU (Test) → SU (Staging) → PU (Production)** mit schrittweiser Annäherung und dokumentiertem **ROLLBACK**-Plan bei Fehlschlag.
2. **Kritikalität bestimmt Tempo**: Security-Patches (Critical) schneller durchlassen, Feature-Updates priorisiert auf Kompatibilität und Performance prüfen. Snapshots/Backups sind vor jedem Schritt Pflicht.
3. Ungetestete Updates bergen **Ausfallrisiko** und Inkompatibilität; jedes Vorhaben ist im **Änderungsmanagement** zu erfassen, zu testen und DSGVO-konform zu loggen.

---

### Weitere relevante IHK-Themen zum "Zweevögel-einem-Stein"-Prinzip

- **DSGVO & Audit**: Jedes Patch-Vorhaben muss im Log stehen (Art. 32 DSGVO: "Technische Maßnahmen"). Logging muss manipulationssicher sein → **WORM** (Write Once Read Many) Speicher oder TLS-verschlüsselte Endpunkte.
- **OSI Modell & Layer**: Patchmanagement berührt Layer 4 (Ports/Protokolle, z. B. WSUS 8530/80/443) und Layer 7 (Applikationsabhängige Dienste). Funktioniert das Update, wenn Firewall Layer 4 blockiert? → Antwort: Ports öffnen nur vorübergehend, Log beachten.
- **Protokolle & Ports**: Windows Update nutzt TCP 8530 (Server), TCP/UDP 80, 443 (Client). Linux: APT TCP 80/443 für Repos, Yum/DNF ähnliche Ports. Prüfungsfrage: "Welcher Port blockiert das Update, wenn er geschlossen ist?" → Antwort: TCP 8530 (Windows) bzw. Repo-Ports (Linux).
- **Hardening-Bezug**: Nach dem Patch immer erneut prüfen, ob nicht versehentlich unsichere Dienste (z. B. Telnet 23, SMBv1 445) reaktiviert wurden. Härtung ist iterativer Prozess, kein Einzelakt.

---

### Savage Check für dich

Wenn du in der Prüfung schreibst: *"Ich teste Updates immer nur an einer VM und gut."* → **Probe**: Note abziehen. Du musst nachweisen, dass du **Risikobewertung**, **Kritikalitätsstufung**, **Rollback-Plan** und **DSGVO-Logging** kennst. Ein simples "Funktioniert bei mir" reicht nicht.

Wenn du schreibst: *"Updates sind sofort zu installieren, sonst hacken uns die."* → **Probe**: Note abziehen. Du musst erklären, warum **Testing (TU/SU)** zwingend ist, trotz Sicherheitsdrang. "Sofort" ist in IHK-Kontext falsch.

**Fazit**: Dein Text ist solide, braucht nur die strukturierten IHK-Begriffe und die 3-Phasen-Phase TU/SU/PU sauber definiert. Das bringst du in die Prüfung am besten auf einem Blatt mit Überschriften: `1. Testumgebung (TU) 2. Staging (SU) 3. Produktion (PU) 4. Rollback & Doku 5. Kritikalität priorisieren.`
