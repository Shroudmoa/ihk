# Aufgabe: RAID-Controller

## Aufgabenstellung
Ergänzen Sie die Tabelle um zwei weitere leistungsbestimmende technische Merkmale eines RAID-Controllers und geben Sie jeweils einen möglichen Wert an.


| Technisches Merkmal | Möglicher Wert |
|---|---|
| Unterstützte RAID-Level | RAID 0, 1, 5, 6, 10, 50, 60 |

## Lösung

| Technisches Merkmal | Möglicher Wert |
|---|---|
| Unterstützte RAID-Level | RAID 0, 1, 5, 6, 10 |
| Cache-Speichergröße | 512 MB, 1 GB, 2 GB |
| Datentransferrate | 6 Gbit/s, 12 Gbit/s |

## Erklärung

- **Cache-Speichergröße:** Der Cache ist ein schneller Zwischenspeicher. Er kann Lese- und Schreibzugriffe beschleunigen. Bei Schreibzugriffen schützt eine geeignete Cache-Sicherung, z. B. Flash-Backup, vor Datenverlust bei einem Stromausfall.
- **Datentransferrate:** Gibt an, wie schnell Daten übertragen werden können. Sie hängt unter anderem von der Schnittstelle und dem verwendeten Übertragungsstandard ab.

## Weitere mögliche Merkmale

- **Anzahl der unterstützten Festplatten:** z. B. 8 oder 16 Laufwerke.
- **Schnittstelle des Controllers:** z. B. PCIe 3.0 oder PCIe 4.0.
- **Cache-Sicherung:** z. B. Batterie oder Flash-Speicher.

**Wichtig:** Die Werte sind Beispiele. Die Datentransferrate muss zur jeweiligen Schnittstelle passen. Bei SATA und SAS werden häufig 6 Gbit/s bzw. 12 Gbit/s als Schnittstellengeschwindigkeiten angegeben.
