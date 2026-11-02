```markdown
## 1. Aufgabe (Datenbank-Infrastruktur)

### a) On-Premises vs. Public Cloud – Vorteile
- **On-Premises:**
  - Volle Datenhoheit und Kontrolle; keine Abhängigkeit vom Provider (kein Vendor Lock-in).
  - DSGVO-konform ohne Auftragsverarbeitungsvertrag (AVV) und Drittlandtransfer.
  - Latenzarm: Direkter LAN-Zugriff, kein Internet-Overhead; wichtig für latenzkritische DB-Queries.
  - Individuelle Hardware-Konfiguration (z.B. dedizierte NVMe, RAID).
- **Public Cloud:**
  - Elastische Skalierbarkeit (Scale out/up) bei Lastspitzen; Pay-as-you-go.
  - Keine Investitionskosten (CAPEX → OPEX); keine Hardware-Wartung.
  - Hohe Verfügbarkeit durch georedundante Rechenzentren und Managed Services.
  - Schnelles Provisioning in Minuten.

### ba) Latenz – Definition und Ursachen
- **Definition:** Zeitverzögerung zwischen Senden einer Anfrage und Empfangen der Antwort (Round-Trip-Time, RTT). Gemessen in ms; OSI Layer 1–4 relevant.
- **Ursache 1 – Physikalische Distanz:** Lichtgeschwindigkeit in Glasfaser; Cloud-RZ in USA → höhere RTT als lokales LAN.
- **Ursache 2 – Netzwerk-Overhead:** Router-Hops, Congestion, QoS-Priorisierung, Paketverlust mit Retransmission (TCP).
- **Weitere:** Langsame Storage-I/O, CPU-Engpässe, ineffiziente Queries.

### bb) Hohe Latenz – negativer Performance-Fall
- **Fall:** Viele kleine, synchrone Transaktionen (z.B. OLTP mit SELECT/UPDATE in Schleife). Jede Query wartet auf RTT. Bei 50 ms Latenz und 1000 Queries → 50 s reine Wartezeit. Datenbank-Connection-Pooling wird ausgelastet; Throughput bricht ein.

### ca) Hochverfügbarkeit – 3 Maßnahmen (ohne RAID)
- **Redundante Netzteile:** Hot-Swap-Netzteile; Ausfall eines Netzteils ohne Downtime.
- **USV (Online/VFI):** Überbrückt Stromausfälle; sauberes Herunterfahren.
- **Cluster (Active/Passive oder Active/Active):** Failover bei Serverausfall; Load Balancer verteilt Last.
- **ECC-RAM:** Fehlerkorrektur; verhindert Datenkorruption.
- **Redundante Netzwerkanbindung:** NIC-Teaming/Bonding an zwei Switches.

### cb) RAID 6 + Hot-Spare – Festplattenberechnung
- **Datenbedarf:** 5 TiB (DB) + 3 TiB (Nutzdaten) + 5 TiB (Puffer) = **13 TiB netto**.
- **RAID 6:** Netto = (N − 2) × 2 TiB. Hot-Spare = 1 zusätzliche Platte.
- **Rechnung:** (N − 2) × 2 ≥ 13 → N − 2 ≥ 6,5 → N ≥ 8,5 → N = 9 Daten-/Parity-Platten.
- **Zuzüglich Hot-Spare:** 9 + 1 = **10 Festplatten**.
- **Architektur:** Zwei Arrays à 8 Platten; 10 Platten passen nicht in ein Array → Aufteilung: 5+5 oder 6+4. Netto: (5−2)×2 + (5−2)×2 = 12 TiB (zu wenig). Besser: 6+4 → (6−2)×2 + (4−2)×2 = 8 + 4 = 12 TiB (zu wenig). 7+3 → (7−2)×2 + (3−2)×2 = 10 + 2 = 12 TiB. 8+2 → (8−2)×2 + (2−2)×2 = 12 + 0 = 12 TiB.
- **Korrektur:** 13 TiB netto benötigt → bei zwei Arrays à 8 Platten max. Netto = 2 × (8−2) × 2 = 24 TiB möglich. Für 13 TiB: Array 1 mit 8 Platten (Netto 12 TiB) + Array 2 mit 5 Platten (Netto 6 TiB) = 18 TiB netto. Hot-Spare: 1 Platte pro Array → 8 + 5 + 1 + 1 = **15 Platten**. Oder ein Array mit 14 Platten: (14−2)×2 = 24 TiB, +1 Hot-Spare = **15 Platten**. Maximal 8 pro Array → 2 Arrays: 8 + 7 = 15 Platten (Netto: 12 + 10 = 22 TiB), +1 Hot-Spare = **16 Platten**. Da max. 8 pro Array: Array 1 = 8 Platten (Netto 12 TiB), Array 2 = 5 Platten (Netto 6 TiB) → 13 TiB gedeckt. Hot-Spare: 1 pro Array → 8+1 + 5+1 = **15 Platten**.

### da) Archivierung – Speicherbedarf 10 Jahre
- **Pro Monat:** 30 GiB; Kompression 60% → 30 × 0,60 = **18 GiB/Monat** effektiv (60% Einsparung) oder 30 × 0,40 = 12 GiB (falls 60% verbleiben). Aufgabe: "mit einer Komprimierungsrate von 60% gepackt" → **60% Einsparung** → 30 × 0,40 = 12 GiB/Monat.
- **10 Jahre:** 12 GiB × 12 × 10 = **1.440 GiB** ≈ 1,41 TiB.

### db) Reservierter Speicher (1 TiB) – Reichweite
- **Verbrauch:** 12 GiB/Monat. 1 TiB = 1024 GiB.
- **Monate:** 1024 / 12 ≈ 85,33 Monate.
- **Jahre:** 85,33 / 12 ≈ 7,11 → **7 ganze Jahre**.

---

## 2. Aufgabe (SQL & Datenbank)

### aa) SQL-Datentypen
| Musterwerte | SQL-Datentyp |
|---|---|
| WerbungsID 1 | `INT` |
| Werbepartner Jubiläum | `VARCHAR(50)` |
| Werbung_verschickt 01.01.2022 | `DATE` |
| KundenNr 12345 | `INT` |
| Kunde_Aktiv true | `BOOLEAN` |
| PLZ 01234 | `CHAR(5)` (führende Null!) |
| Telefonnummer 0123/112233 | `VARCHAR(20)` |
| Erfasst am 11.05.2022 | `DATE` |
| IBAN DE12... | `CHAR(22)` (fixe Länge) |

### ab) SQL: Aktive Kunden in Augsburg
```sql
SELECT COUNT(*) FROM Kunde WHERE Ort = 'Augsburg' AND Kunde_Aktiv = true;
```

### ac) SQL: CREATE TABLE Werbeaktion
```sql
CREATE TABLE Werbeaktion (
    WerbungsID INT PRIMARY KEY,
    Werbepartner VARCHAR(50),
    Werbung_verschickt DATE,
    KundenNr INT,
    FOREIGN KEY (KundenNr) REFERENCES Kunde(KundenNr)
);
```

### ad) SQL: Werbungen vor 31.12.2021 an PLZ 8*
```sql
SELECT COUNT(*) 
FROM Werbeaktion 
JOIN Kunde ON Werbeaktion.KundenNr = Kunde.KundenNr 
WHERE Werbung_verschickt < '2021-12-31' 
AND Kunde.PLZ LIKE '8%';
```

### b) Nicht-relationale Datenbanktypen
- **Dokumentenorientiert:** MongoDB, CouchDB (JSON/BSON).
- **Key-Value:** Redis, Riak.
- **Spaltenorientiert:** Cassandra, HBase.
- **Graphdatenbank:** Neo4j.
- **Zeitreihen-DB:** InfluxDB.

---

## 3. Aufgabe (Backup & TOM)

### a) LTO & LTFS – Vorteile
- **LTO (Kosten):** TCO 86% niedriger als All-Disk über 10 Jahre; Band ist pro TiB deutlich günstiger als HDD/SSD. Ideal für Langzeitarchivierung.
- **LTFS (Handhabung):** Tape verhält sich wie ein Dateisystem (Drag & Drop); kein spezielles Backup-Tool nötig. Hohe Kompatibilität, einfacher Datenaustausch. Ideal für große Datenmengen und Archivierung.

### b) Backupverfahren
| Verfahren | Erläuterung |
|---|---|
| **Vollbackup** | Sichert alle Daten; setzt Archivbit aller Dateien zurück. Höchster Speicherbedarf, längste Dauer, einfachster Restore (1 Band). |
| **Differenzielles Backup** | Sichert alle Änderungen seit dem letzten Vollbackup; setzt Archivbit zurück. Speicherbedarf wächst täglich; Restore = Voll + letztes Diff. |
| **Inkrementelles Backup** | Sichert nur Änderungen seit dem letzten Backup (Voll oder Inkrement); setzt Archivbit zurück. Geringster Speicherbedarf; Restore = Voll + alle Inkremente (lange Kette). |
| **Klonen** | 1:1-Kopie eines Datenträgers/Volumes; kein Archivbit-Konzept. Hoher Speicherbedarf; schneller Restore (Image). |

### c) Restore-Bänder (09.07.2022, 10 Uhr)
- **Gegeben:** V2 (So 03.07. Voll), D1–D6 (Mo–Sa), V3 (So 10.07.).
- **Fehler:** Mittwoch (D3) wurde inkrementell statt differenziell gesichert.
- **Benötigt:** V2 (Voll) + D1 (Diff seit V2) + D2 (Diff seit V2) + D3 (Inkr. seit D2!) + D4 (Diff seit V2) + D5 (Diff seit V2) + D6 (Diff seit V2).
- **Reihenfolge:** **V2, D1, D2, D3, D4, D5, D6**.
- **Wichtig:** Da D3 inkrementell war, baut D4 (differenziell seit V2) nicht auf D3 auf, sondern auf V2. D3 muss trotzdem eingespielt werden, um die Änderungen von Mi zu erhalten. D4 überschreibt dann teilweise. Korrekte Reihenfolge: V2 → D1 → D2 → D3 → D4 → D5 → D6.

### d) TOM-Zuordnung
| TOM | Zugangskontrolle | Zugriffskontrolle |
|---|---|---|
| Alarmanlagen | X | |
| Passwortregeln | | X |
| Chipkarten für verschlossene Bereiche | X | |
| Besucheranmeldung am Empfang | X | |
| Verschlüsselung von Datenträgern | | X |
| Datenschutzkonforme Vernichtung von Datenträgern | | X |
| Generierung von Benutzerprofilen | | X |

---

## 4. Aufgabe (Programmierung & Hardware)

### a) Programmverweiterung – Mittelwert RAM
```csharp
string message = "Wenig freier Speicher";
string title = "Warnung";
int avgUsedRam = 0;
int sumTemp = 0;

int[] usedRAM = new int[24] {17,17,16,18,20,25,33,44,40,52,60,56,33,44,40,52,60,56,33,44,34,28,23,16};

for (int i = 0; i < usedRAM.Length; i++)
{
    sumTemp = sumTemp + usedRAM[i];
}

avgUsedRam = sumTemp / usedRAM.Length;

if (avgUsedRam > 75)
{
    MessageBox.Show(message, title);
}
```

### b) Datenkapselung vs. Vererbung
- **Datenkapselung:** Attribute (Felder) sind `private`; Zugriff nur über `public` Methoden/Properties (Getter/Setter). Schützt vor unkontrollierten Änderungen; ermöglicht Validierung. Beispiel: `private int alter; public int Alter { get; set; }`.
- **Vererbung:** Klasse erbt Attribute/Methoden von einer Basisklasse (`class Hund : Tier`). Wiederverwendung von Code; Polymorphie durch Überschreiben (`override`). Beispiel: `Tier` → `Hund`, `Katze`.

### c) RAM-Modul DDR4-3200 (PC4-25600) CL16 ECC
| Wert | Einheit | Effektiver Takt | Datenrate am 64-Bit-Bus | Verzögerung Adressumschaltung |
|---|---|---|---|---|
| 3200 | MHz | 3200 | 25,6 GB/s | 16 Taktzyklen |
- **Erklärung:** DDR4-3200 → effektiver Takt 3200 MT/s; PC4-25600 → 25.600 MB/s = 25,6 GB/s (64-Bit-Bus: 3200 × 8 Byte = 25.600 MB/s). CL16 = 16 Taktzyklen Latenz bei Adressumschaltung.

---

**Merksatz:** IHK-Aufgaben sind **Kausalitätsketten** – wer RAID-6-Formel `(N-2)*K`, Backup-Ketten (Voll→Diff→Inkr) und SQL-JOINs nicht als Hard-Facts sitzen hat, verliert Punkte an der Oberfläche. Lerne Standards als Vokabeln, nicht als Prosa.
```
