# RAM (Arbeitsspeicher, DDR, ECC)

## In 30 Sekunden

- RAM ist **flüchtiger** Arbeitsspeicher (DRAM) für laufende Programme und Daten. Zu wenig RAM führt zum **Auslagern** auf den Datenträger, und das System wird langsam.
- Bezeichnung **DDR4-3200** = 3.200 Millionen Transfers pro Sekunde (MT/s, „effektiver Takt“). **PC4-25600** = 25.600 MB/s Spitzen-Datenrate.
- **Datenrate = Transferrate · 8 Byte** (64-Bit-Bus): 3.200 MT/s · 8 B = **25,6 GB/s** pro Kanal.
- **CL16** (CAS Latency) = 16 Taktzyklen Wartezeit zwischen Spaltenadresse und Daten.
- Server nutzen **ECC** (Fehlerkorrektur) und meist **Registered DIMMs (RDIMM)**.
- Geprüft: DDR4-3200 PC4-25600 CL16 ECC 2022W, RAM als Bottleneck 2025W.

## Kernwissen

### Bezeichnung entschlüsseln: DDR4-3200 (PC4-25600) CL16 ECC

| Teil | Bedeutung | Wert |
|---|---|---|
| DDR4 | Generation (Double Data Rate: 2 Transfers je Takt) | 4. Generation |
| 3200 | effektive Transferrate („effektiver Takt“) | 3.200 MT/s |
| I/O-Bustakt | Transferrate / 2 | 1.600 MHz |
| PC4-25600 | Spitzen-Datenrate eines Moduls | 25.600 MB/s = 25,6 GB/s |
| CL16 | CAS Latency, Verzögerung bei der Adressumschaltung | 16 Taktzyklen |
| ECC | Error Correcting Code | korrigiert 1-Bit-Fehler, erkennt 2-Bit-Fehler |

**Rechenwege:**
- Datenrate: 3.200 · 10⁶ Transfers/s · 64 Bit / 8 = 25.600 MB/s = **25,6 GB/s**
- Dual Channel (2 Kanäle): 2 · 25,6 = **51,2 GB/s**
- Latenz in ns: CL · 2.000 / Transferrate = 16 · 2.000 / 3.200 = **10 ns**
- Umgekehrt: PC4-21300 / 8 ≈ 2.666 → DDR4-2666

### Generationen

| Generation | typische Transferrate | Modulbezeichnung (Beispiel) | Spannung |
|---|---|---|---|
| DDR3 | 1.066–2.133 MT/s | PC3-12800 (DDR3-1600) | 1,5 V |
| DDR4 | 2.133–3.200 MT/s | PC4-25600 (DDR4-3200) | 1,2 V |
| DDR5 | 4.800–8.000+ MT/s | PC5-38400 (DDR5-4800) | 1,1 V, On-Die-ECC, 2 Subkanäle je Modul |

Generationen sind **nicht kompatibel** (andere Kerbe, anderer Sockel).

### Servertypische Merkmale

| Merkmal | Bedeutung |
|---|---|
| **ECC** | Prüfbits erkennen und korrigieren Bitfehler (z. B. durch kosmische Strahlung). Wichtig für Integrität bei Datenbanken und Virtualisierung |
| **RDIMM** (Registered) | Register puffert Steuersignale → mehr Module je Kanal, größere Kapazitäten, minimal mehr Latenz |
| **LRDIMM** | Load Reduced, puffert auch Daten → sehr große Kapazitäten |
| **UDIMM** | Unbuffered, typisch für Desktop-PCs |
| **Speicherkanäle** | Server-CPUs haben 8–12 Kanäle. Gleichmäßig bestücken, sonst geht Bandbreite verloren |
| **NUMA** | jeder CPU-Sockel hat „eigenen“ RAM, Zugriff auf den RAM des anderen Sockels ist langsamer |

### RAM-Engpass erkennen

- Belegung dauerhaft > 90 %, hohe Auslagerungsrate (Pagefile/Swap), Datenträgeraktivität ohne erkennbaren Grund.
- Maßnahmen: RAM erweitern, speicherhungrige Prozesse optimieren oder beenden, Speicherleck beheben, Dienste auf andere Server verteilen → [Monitoring](../FiSi-Administrator/Server/monitoring.md).

## Prüfungsfrage

**Frage:** Für den Server ist das Modul „DDR4-3200 (PC4-25600) CL16 ECC“ vorgesehen. Geben Sie den effektiven Takt, die Datenrate am 64-Bit-Bus und die Bedeutung von CL16 an. Wozu dient ECC?

**Musterantwort:** Der effektive Takt beträgt 3.200 MHz bzw. genauer 3.200 MT/s. Die Datenrate am 64-Bit-Bus ergibt sich aus 3.200 · 10⁶ · 8 Byte = 25.600 MB/s, also **25,6 GB/s**, was der Angabe PC4-25600 entspricht. CL16 bedeutet, dass zwischen dem Anlegen der Spaltenadresse und der Datenausgabe 16 Taktzyklen vergehen. ECC erkennt und korrigiert Einzelbitfehler im Speicher und verhindert damit unbemerkte Datenverfälschung und Abstürze. Für Server ist das Pflicht.

## Falle

- Datenrate in Bit statt Byte angeben, oder den Faktor 8 (64 Bit = 8 Byte) vergessen.
- DDR4-3200 als „3,2 GHz Taktfrequenz“ der Speicherzellen deuten. Der I/O-Takt liegt bei 1.600 MHz, durch DDR werden es 3.200 MT/s.
- Eine niedrigere CL pauschal für „schneller“ halten. Entscheidend ist CL **im Verhältnis** zur Transferrate (Latenz in ns).
- ECC-Module in einem Board ohne ECC-Unterstützung erwarten, dass es korrigiert. CPU und Board müssen ECC beherrschen.
- RDIMM und UDIMM mischen. Das funktioniert nicht.

## Merksatz

**„MT/s mal 8 = MB/s. CL zählt Takte, ECC rettet Bits.“**

## Verwandt

- [CPU](cpu.md) – Cache, Speicherhierarchie, NUMA
- [Speicherhierarchie](Storage/speicherhierarchie.md)
- [Server-Hardware](server-hardware.md) – Serverauswahl, Redundanz
- [Monitoring](../FiSi-Administrator/Server/monitoring.md) – Bottleneck-Analyse
- [Bit-Byte](../FiSi-Administrator/OSI-IP-mac-bit-byte/bit-byte.md)
- [Lektion 03 Verfügbarkeit](../analyse-der-exams/03_Verfuegbarkeit_Ausfallsicherheit.md) – RAM-Kennwerte 2022W
