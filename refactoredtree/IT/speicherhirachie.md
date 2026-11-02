# Speicherhirachie

# Speicherhierarchie – Schnell = teuer = klein

## Grundprinzip

Die Speicherhierarchie ordnet Speicher nach Zugriffsgeschwindigkeit, Kapazität und Kosten pro Bit.

**Je schneller der Speicher, desto kleiner und teurer pro Bit ist er normalerweise.** Langsamere Speicher bieten meist mehr Kapazität zu geringeren Kosten pro Bit.

Quelle: https://openstax.org/books/introduction-computer-science/pages/5-5-memory-hierarchy

## Speicherpyramide

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

## Ebenen im Vergleich

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

## Aufgaben der einzelnen Ebenen

1. **Register:** Enthalten Werte, mit denen die CPU unmittelbar rechnet.
2. **Cache:** Speichert häufig benötigte Daten und Befehle nahe an der CPU.
3. **RAM:** Hält Daten und Programme, die gerade verwendet werden.
4. **SSD/HDD:** Speichert Betriebssystem, Programme und Dateien dauerhaft.
5. **Archivspeicher:** Dient der langfristigen Aufbewahrung großer Datenmengen.

## Cache Hit und Cache Miss

- **Cache Hit:** Die gesuchten Daten befinden sich im Cache. Der Zugriff ist schnell.
- **Cache Miss:** Die Daten fehlen im Cache und müssen aus einer langsameren Speicherebene geladen werden.

Quelle: https://learn.arm.com/learning-paths/servers-and-cloud-computing/memory-subsystem/cache-hierarchy/

## SRAM vs. DRAM

| Eigenschaft | SRAM | DRAM |
|---|---|---|
| Verwendung | Typischerweise CPU-Cache | Typischerweise Arbeitsspeicher |
| Geschwindigkeit | Sehr schnell | Langsamer als SRAM |
| Kosten pro Bit | Hoch | Niedriger |
| Speicherdichte | Geringer | Höher |
| Refresh erforderlich | Nein | Ja |

## Flüchtig vs. nichtflüchtig

- **Flüchtig:** Register, Cache und RAM verlieren ihren Inhalt ohne Strom.
- **Nichtflüchtig:** SSD und HDD behalten gespeicherte Daten ohne Strom.

## Warum nutzt man nicht nur schnellen Speicher?

- Schneller Speicher ist teuer und deshalb nur begrenzt sinnvoll einsetzbar.
- Langsamer Speicher ist günstiger und kann große Datenmengen speichern.
- Die Kombination bietet ein gutes Verhältnis aus Leistung, Kapazität und Kosten.

## IHK-Merksatz

**Nach oben: schneller, kleiner, teurer pro Bit.**

**Nach unten: langsamer, größer, günstiger pro Bit.**

Ziel der Speicherhierarchie ist es, hohe Geschwindigkeit mit großer Speicherkapazität und vertretbaren Kosten zu kombinieren.
