```markdown
## 1. Aufgabe (SaaS & Private Cloud)

### a) SaaS – 3 Aspekte
- **Kein Betriebsaufwand:** Provider übernimmt Wartung, Patches, Updates und Infrastruktur; Kunde nutzt nur die Anwendung.
- **Kostentransparenz:** Pay-per-Use oder Abo-Modell; keine Investitionskosten (CAPEX → OPEX), keine Hardware.
- **Hohe Verfügbarkeit/Skalierbarkeit:** Provider garantiert SLA (z.B. 99,9%); elastische Anpassung an Lastspitzen.
- **Ortsunabhängigkeit:** Zugriff über Browser/VPN; keine lokale Installation.

### ba) Server-Cluster – 2 Sachverhalte
- **Redundanz/HA:** Cluster muss Ausfälle einzelner Knoten kompensieren (Quorum, Failover, Heartbeat). Shared Storage (SAN) und redundante Netzwerkanbindung (zwei Switches) notwendig.
- **Ressourcenplanung/Cluster-FS:** CPU/RAM/Storage müssen für Live-Migration (z.B. vMotion) ausreichen; N+1-Redundanz einplanen. Cluster-Dateisystem (VMFS, CSV) und einheitliche Konfiguration.

### bb) Zweck Hypervisor
- **Abstraktion:** Trennt virtuelle Maschinen (Guest-OS) von der physischen Hardware. Ressourcen (CPU, RAM, Storage, Netzwerk) werden virtualisiert und auf VMs verteilt.
- **Isolation:** VMs laufen gekapselt; Fehler/Kompromittierung einer VM bleiben isoliert.
- **Management:** Erlaubt Snapshots, Live-Migration, Ressourcen-Limits (QoS) und zentrale Verwaltung.

---

## 2. Aufgabe (Kryptographie & Linux-Rechte)

### a) Ziele der Kryptographie
- **Vertraulichkeit:** Nur autorisierte Parteien können Daten lesen. Verschlüsselung mit symmetrischen (AES) oder asymmetrischen (RSA) Verfahren.
- **Integrität:** Daten sind unverändert und vollständig. Hashfunktionen (SHA-256) und digitale Signaturen erkennen Manipulation.
- **Authentizität:** Absender ist eindeutig identifizierbar; Nachricht stammt vom angegebenen Sender. Digitale Zertifikate (X.509) und Signaturen.

### b) Algorithmen – Eignung
- **DES:** Veraltet, 56-Bit-Schlüssel; per Brute-Force gebrochen. **Nicht geeignet** für vertrauliche Daten.
- **AES:** Symmetrisch, 128/192/256 Bit; schnell, sicher, hardwarebeschleunigt (AES-NI). **Sehr gut geeignet** für Massendaten auf externen Datenträgern (z.B. BitLocker, LUKS).
- **RSA:** Asymmetrisch, langsamer; für Schlüsselaustausch und Signaturen geeignet, nicht für Massenverschlüsselung. **Bedingt geeignet** (Hybrid: RSA für Key Exchange, AES für Daten).
- **Empfehlung:** **AES-256** für die Datenverschlüsselung (symmetrisch, performant). RSA nur für Schlüsselaustausch/Signaturen im Hybrid-Verfahren. DES verwerfen.

### ca) Ursache „Keine Berechtigung“
- **Ursache:** Datei `2023-01-10_Aenderungen.txt` hat Rechte `-rw-r--r--` (644): Eigentümer `hmeyer` hat `rw-` (lesen/schreiben), Gruppe `alster` hat `r--` (nur lesen), Andere `r--`. Herbert Meyer ist Eigentümer, sollte also schreiben dürfen. Fehler liegt darin, dass er als anderer Benutzer (nicht `hmeyer`) agiert oder die Datei auf einem **read-only** gemounteten verschlüsselten Datenträger liegt.
- **Lösungsvorschlag:** Als `hmeyer` anmelden oder `chmod u+w` (Eigentümer-Schreibrecht setzen) bzw. `chmod 664` ausführen. Bei Mount: Read-Write-Mount prüfen (`mount -o remount,rw`).

### cb) chmod 664 – Auswirkungen
- **Eigentümer (6):** `rw-` → Lesen (4) + Schreiben (2) = 6.
- **Gruppe (6):** `rw-` → Lesen (4) + Schreiben (2) = 6.
- **Andere (4):** `r--` → Nur Lesen (4).

---

## 3. Aufgabe (Programmierung & Exceptions)

### a) MemMon – Erweiterung
```csharp
int sumLast = 0;
int mwTag = 0;
int mwVormittag = 0;
int mwNachmittag = 0;
int[] maxRAM20231119 = new int[24] {33,44,40,52,60,56,33,44,40,52,60,56,33,44,40,52,60,56,33,44,40,52,60,56};

for (int i = 0; i < 24; i++)
{
    sumLast = sumLast + maxRAM20231119[i];
}
mwTag = sumLast / 24;
Console.WriteLine("Mittelwert Tag: " + mwTag);

// Vormittag (Index 0–11)
int sumVormittag = 0;
for (int i = 0; i < 12; i++)
{
    sumVormittag = sumVormittag + maxRAM20231119[i];
}
mwVormittag = sumVormittag / 12;

// Nachmittag (Index 12–23)
int sumNachmittag = 0;
for (int i = 12; i < 24; i++)
{
    sumNachmittag = sumNachmittag + maxRAM20231119[i];
}
mwNachmittag = sumNachmittag / 12;

if (mwVormittag <= mwNachmittag)
{
    Console.WriteLine("Mittelwert Vormittag: " + mwVormittag);
    Console.WriteLine("Mittelwert Nachmittag: " + mwNachmittag);
}
else
{
    Console.WriteLine("Mittelwert Nachmittag: " + mwNachmittag);
    Console.WriteLine("Mittelwert Vormittag: " + mwVormittag);
}
```

### b) Pseudocode – 2 Vorteile
- **Sprachunabhängigkeit:** Algorithmus ist unabhängig von Syntax einer Programmiersprache; leicht verständlich für Nicht-Programmierer und in der Dokumentation.
- **Fokus auf Logik:** Keine Ablenkung durch Syntax-Details (Semikolons, Typen); ermöglicht schnelle Validierung des Algorithmus vor der Implementierung.

### c) Exceptions – 2 Umstände
- **Division durch Null:** `int x = 10 / 0;` → `DivideByZeroException`. Tritt auf, wenn Nenner 0 ist (z.B. bei leerem Array-Length).
- **Array-Index außerhalb des Bereichs:** `arr[10]` bei Array-Länge 5 → `IndexOutOfRangeException`. Tritt bei fehlerhaften Schleifengrenzen auf.
- **Weitere:** Null-Referenz (`NullReferenceException`), Datei nicht gefunden (`FileNotFoundException`), Konvertierungsfehler (`FormatException`).

---

## 4. Aufgabe (Backup & Patch-Management)

### a) Restore-Bänder (Ausfall 18.10.2023, 18:12 Uhr)
- **Letztes Vollbackup:** V4 am So 08.10.2023.
- **Differenzielle Backups seit V4:** D6 (Mo 09.10.), 4D1 (Di 10.10.), 4D2 (Mi 11.10.), 4D3 (Do 12.10.), 4D4 (Fr 13.10.), 4D5 (Sa 14.10.).
- **V5 (So 15.10.)** ist das nächste Vollbackup. Da der Ausfall am **Mi 18.10. um 18:12** ist, liegt das letzte Backup **vor** dem Ausfall am **Di 17.10.** (5D2) oder **Mi 18.10.** (5D3) – je nachdem, ob das Backup um 23 Uhr noch vor dem Ausfall liegt. Ausfall 18:12 < 23:00 → Das Backup am **Mi 18.10.** (5D3) findet **nach** dem Ausfall statt und ist unbrauchbar.
- **Letztes gültiges Backup:** **Di 17.10. (5D2)**.
- **Bänder:** **V5** (Voll So 15.10.) + **5D1** (Mo 16.10., diff.) + **5D2** (Di 17.10., diff.).
- **Reihenfolge:** **V5, 5D1, 5D2**.

### b) Backup → Archivierung – 2 Maßnahmen
- **Unveränderbarkeit (WORM):** Archivierte Daten müssen unveränderbar sein (Write Once Read Many). Maßnahme: Band mit WORM-Technologie oder Object Lock (S3) verwenden; Hash-/Signaturprüfung.
- **Aufbewahrungsfristen/Index:** Definierte Aufbewahrungsdauer (z.B. 10 Jahre, § 257 HGB) und Volltextindex/Metadaten für Wiederauffindbarkeit. Maßnahme: Katalogisierung und regelmäßige Integritätsprüfung (Bit Rot).

### c) Cluster-Size – Anwendungsfälle
- **Kleine Cluster (z.B. 512 B/4 KiB):** Viele kleine Dateien (Logs, Dokumente). Reduziert Slack Space (interner Fragmentierungsverlust).
- **Große Cluster (z.B. 64 KiB):** Große Dateien (Videos, Images, DB-Dumps). Weniger Verwaltungsaufwand, höhere sequenzielle Performance.

### e) Patch vs. Upgrade
- **Patch:** Fehlerbehebung innerhalb der gleichen Major-Version (z.B. 1.0 → 1.1). **Kostenlos** für Lizenzinhaber. Einsatz: Sicherheitslücken, Bugfixes, kleine Verbesserungen. Keine neuen Hauptfunktionen.
- **Upgrade:** Wechsel auf eine höhere Major-Version (z.B. 1.x → 2.x). **Kostenpflichtig** (Rabatt für Bestandskunden). Einsatz: Neue Funktionen, Architekturänderungen, Migrationsaufwand. Neue Lizenz erforderlich.

---

**Merksatz:** IHK-Aufgaben sind **Kausalitätsketten** – wer Linux-Rechte (rwx = 421), Backup-Ketten (Voll→Diff) und Krypto-Eignung (AES vs. DES vs. RSA) nicht als Hard-Facts sitzen hat, verliert Punkte an der Oberfläche. Lerne Standards als Vokabeln, nicht als Prosa.
```
