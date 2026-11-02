# Bit-Byte

# Bit, Byte, GB, GiB und Datenraten – IHK AP2

## Bit und Byte

| Einheit | Bedeutung |
|---|---|
| bit (`b`) | Kleinste digitale Informationseinheit: `0` oder `1` |
| Byte (`B`) | 8 Bit |
| Kilobit (`kbit`) | 1.000 Bit |
| Kilobyte (`kB`) | 1.000 Byte |
| Megabyte (`MB`) | 1.000.000 Byte |
| Gigabyte (`GB`) | 1.000.000.000 Byte |
| Terabyte (`TB`) | 1.000.000.000.000 Byte |

**Groß- und Kleinschreibung ist wichtig:**
- `b` = Bit
- `B` = Byte
- `Mbit/s` = Megabit pro Sekunde
- `MB/s` = Megabyte pro Sekunde

## GB vs. GiB – dezimal vs. binär

Hersteller verwenden für Speicherkapazitäten üblicherweise das Dezimalsystem. Betriebssysteme zeigen Speichergrößen teilweise in binären Einheiten an.

| Dezimal | Wert | Binär | Wert |
|---|---:|---|---:|
| 1 kB | 1.000 Byte | 1 KiB | 1.024 Byte |
| 1 MB | 1.000.000 Byte | 1 MiB | 1.048.576 Byte |
| 1 GB | 1.000.000.000 Byte | 1 GiB | 1.073.741.824 Byte |
| 1 TB | 1.000.000.000.000 Byte | 1 TiB | 1.099.511.627.776 Byte |

**Merke:**
- `GB` = Gigabyte, dezimal: \(10^9\) Byte
- `GiB` = Gibibyte, binär: \(2^{30}\) Byte
- `1 GiB ≈ 1,074 GB`
- `1 GB ≈ 0,931 GiB`

### Beispiel: Warum zeigt eine 1-TB-Festplatte weniger an?

Herstellerangabe: `1 TB = 1.000.000.000.000 Byte`

Umrechnung in TiB:

\[
\frac{1.000.000.000.000}{2^{40}}
\approx 0{,}9095\ \text{TiB}
\]

Die Kapazität ist nicht verschwunden. Die Anzeige verwendet eine andere Einheit. Zusätzlich können Dateisysteme und Formatierung etwas Speicherplatz belegen.

## Datenrate umrechnen

Netzwerkgeschwindigkeiten werden häufig in Bit/s angegeben, Speicherdurchsatz dagegen in Byte/s.

**Grundregel: 8 Bit = 1 Byte**

| Datenrate | Theoretischer Durchsatz |
|---|---:|
| 8 Mbit/s | 1 MB/s |
| 100 Mbit/s | 12,5 MB/s |
| 1 Gbit/s | 125 MB/s |
| 2,5 Gbit/s | 312,5 MB/s |
| 10 Gbit/s | 1.250 MB/s = 1,25 GB/s |

Das sind theoretische Werte ohne Protokoll-Overhead. In der Praxis ist der Durchsatz meist niedriger.

### Beispiel: Downloadzeit

Datei: `2 GB`  
Verbindung: `100 Mbit/s`

1. `2 GB = 16.000 Mbit`
2. `Zeit = Datenmenge ÷ Datenrate`
3. `16.000 Mbit ÷ 100 Mbit/s = 160 Sekunden`

**Ergebnis:** Theoretisch 2 Minuten und 40 Sekunden. In der Praxis dauert es wegen Overhead und schwankender Verbindung länger.

## Wichtige Formeln

| Gesucht | Formel |
|---|---|
| Bit in Byte | Bit ÷ 8 |
| Byte in Bit | Byte × 8 |
| Übertragungszeit | Datenmenge in Bit ÷ Datenrate in Bit/s |
| Datenmenge | Datenrate × Zeit |
| Dezimal GB in GiB | GB ÷ 1,073741824 |
| Binär GiB in GB | GiB × 1,073741824 |

Bei Übertragungszeit-Aufgaben müssen Datenmenge und Datenrate in kompatiblen Einheiten stehen.

## IHK-Fallen

- `MB/s` ist achtmal so groß wie der Zahlenwert in `Mbit/s`, wenn dieselbe Datenrate gemeint ist.
- `1 GB` sind dezimal 1.000.000.000 Byte, `1 GiB` sind 1.073.741.824 Byte.
- `1 Gbit/s` bedeutet nicht `1 GB/s`.
- Eine Leitung mit 100 Mbit/s lädt theoretisch höchstens 12,5 MB pro Sekunde.
- Herstellerkapazität und Betriebssystemanzeige können sich durch unterschiedliche Einheiten unterscheiden.
