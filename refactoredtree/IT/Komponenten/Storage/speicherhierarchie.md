# Speicherhierarchie – schnell = teuer = klein

## In 30 Sekunden

- Register → L1 → L2 → L3 → RAM → SSD/NVMe → HDD → Band/Archiv.
- **Nach oben:** schneller, kleiner, teurer pro Bit. **Nach unten:** langsamer, größer, günstiger pro Bit.
- Register, Cache und RAM sind **flüchtig**, SSD, HDD und Band **nichtflüchtig**.
- Ziel: hohe Geschwindigkeit bei großer Kapazität und vertretbaren Kosten. Das gleiche Prinzip nutzt **Storage-Tiering** (SSD-Tier, HDD-Tier, Band-Archiv).

## Kernwissen

### Grundprinzip

Die Speicherhierarchie ordnet Speicher nach Zugriffsgeschwindigkeit, Kapazität und Kosten pro Bit.

**Je schneller der Speicher, desto kleiner und teurer pro Bit ist er normalerweise.** Langsamere Speicher bieten meist mehr Kapazität zu geringeren Kosten pro Bit.

Quelle: https://openstax.org/books/introduction-computer-science/pages/5-5-memory-hierarchy

### Speicherpyramide

CPU-Register  
↓  
L1-Cache  
↓  
L2-Cache  
↓  
L3-Cache  
↓  
RAM (DRAM)  
↓  
SSD / NVMe  
↓  
HDD  
↓  
Archivspeicher

- Oben: schnell, klein, teuer pro Bit
- Unten: langsamer, größer, günstiger pro Bit

### Ebenen im Vergleich

| Speicher | Geschwindigkeit | Typische Kapazität | Kosten pro Bit |
|---|---|---|---|
| CPU-Register | Extrem schnell | Sehr klein | Sehr hoch |
| L1-Cache | Extrem schnell | KB bis hunderte KB | Sehr hoch |
| L2-Cache | Sehr schnell | Hunderte KB bis wenige MB | Hoch |
| L3-Cache | Schnell | Einige bis hunderte MB | Hoch |
| RAM (DRAM) | Schnell | GB bis TB bei großen Systemen | Mittel |
| SSD / NVMe | Langsamer als RAM | Hunderte GB bis mehrere TB | Niedrig |
| HDD | Langsamer als SSD | Mehrere TB | Sehr niedrig |
| Archivspeicher | Je nach Medium langsam | Groß bis sehr groß | Häufig sehr niedrig |

*Die Werte sind typische Größenordnungen und hängen von der Hardware ab.*

### Aufgaben der einzelnen Ebenen

1. **Register:** Enthalten Werte, mit denen die CPU unmittelbar rechnet.
2. **Cache:** Speichert häufig benötigte Daten und Befehle nahe an der CPU.
3. **RAM:** Hält Daten und Programme, die gerade verwendet werden.
4. **SSD/HDD:** Speichert Betriebssystem, Programme und Dateien dauerhaft.
5. **Archivspeicher:** Dient der langfristigen Aufbewahrung großer Datenmengen.

### Cache Hit und Cache Miss

- **Cache Hit:** Die gesuchten Daten befinden sich im Cache. Der Zugriff ist schnell.
- **Cache Miss:** Die Daten fehlen im Cache und müssen aus einer langsameren Speicherebene geladen werden.

Quelle: https://learn.arm.com/learning-paths/servers-and-cloud-computing/memory-subsystem/cache-hierarchy/

### SRAM vs. DRAM

| Eigenschaft | SRAM | DRAM |
|---|---|---|
| Verwendung | Typischerweise CPU-Cache | Typischerweise Arbeitsspeicher |
| Geschwindigkeit | Sehr schnell | Langsamer als SRAM |
| Kosten pro Bit | Hoch | Niedriger |
| Speicherdichte | Geringer | Höher |
| Refresh erforderlich | Nein | Ja |

### Flüchtig vs. nichtflüchtig

- **Flüchtig:** Register, Cache und RAM verlieren ihren Inhalt ohne Strom.
- **Nichtflüchtig:** SSD und HDD behalten gespeicherte Daten ohne Strom.

### Warum nutzt man nicht nur schnellen Speicher?

- Schneller Speicher ist teuer und deshalb nur begrenzt sinnvoll einsetzbar.
- Langsamer Speicher ist günstiger und kann große Datenmengen speichern.
- Die Kombination bietet ein gutes Verhältnis aus Leistung, Kapazität und Kosten.

### Größenordnungen der Zugriffszeit

| Ebene | typische Zugriffszeit |
|---|---|
| Register | < 1 ns |
| L1-Cache | ca. 1 ns |
| L3-Cache | ca. 10–20 ns |
| RAM | ca. 60–100 ns |
| NVMe-SSD | ca. 20–100 µs |
| HDD | ca. 5–10 ms (Suchzeit + Rotation) |
| Band | Sekunden bis Minuten (Spulen) |

## Prüfungsfrage

**Frage:** Erklären Sie, warum ein Computer nicht ausschließlich mit dem schnellsten Speicher aufgebaut wird.

**Musterantwort:** Schneller Speicher wie Cache (SRAM) ist pro Bit sehr teuer, braucht viel Chipfläche und ist flüchtig. Große Datenmengen damit zu speichern, wäre unbezahlbar, und nach dem Ausschalten wären sie verloren. Deshalb kombiniert man mehrere Ebenen: Kleine, schnelle Speicher nahe der CPU halten die gerade benötigten Daten, große, günstige und nichtflüchtige Speicher wie SSD und HDD halten den Gesamtbestand. Weil Programme meist wiederholt auf dieselben Daten zugreifen (Lokalität), liegen diese mit hoher Trefferquote im schnellen Cache.

## Falle

- RAM als dauerhaften Speicher bezeichnen. Er ist flüchtig.
- Cache und RAM gleichsetzen. Der Cache ist ein kleiner, schneller Puffer vor dem RAM.
- „SSD ist so schnell wie RAM“. Es liegen rund drei Größenordnungen dazwischen.

## Merksatz

**Nach oben: schneller, kleiner, teurer pro Bit.**

**Nach unten: langsamer, größer, günstiger pro Bit.**

Ziel der Speicherhierarchie ist es, hohe Geschwindigkeit mit großer Speicherkapazität und vertretbaren Kosten zu kombinieren.

## Verwandt

- [CPU](../cpu.md) – Cache-Ebenen
- [RAM](../ram.md) – DDR, ECC
- [USB-SSD-HDD](../usb-ssd-hdd.md) – Datenträger
- [DAS-NAS-SAN](das-nas-san.md)
- [Backup-Strategien](../../FiSi-Administrator/Server/backup/backup-strategien.md) – Band als unterste Ebene
