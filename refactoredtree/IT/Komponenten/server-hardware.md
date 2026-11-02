# Server-Hardware und Serverauswahl

## In 30 Sekunden

- Ein Server unterscheidet sich vom PC durch **Redundanz** (zwei Netzteile, ECC-RAM, RAID, mehrere Netzwerkkarten), **Hot-Swap** (Tausch im laufenden Betrieb), **Fernwartung** (BMC: iDRAC, iLO, IPMI) und Dauerbetrieb.
- Bauformen: **Tower**, **Rack** (19 Zoll, Höhe in **HE/U**, 1 HE = 44,45 mm) und **Blade** (Einschübe in einem Chassis mit gemeinsamem Strom und Netz).
- Auswahl nach **Einsatzzweck**: Datenbank → hoher Takt, viel RAM, schnelles lokales Storage (NVMe). Virtualisierung → viele Kerne, sehr viel RAM, schnelles Netz. Fileserver/Mail → viele Laufwerksschächte, I/O, Netz.
- Geprüft: Serverauswahl 2024W (Mailserver) und 2025W (DB vs. Virtualisierung), Ausfallsicherheit ohne RAID 2021W/2022W, RAID-Controller-Merkmale 2022S, Netzteil 2026S.

## Kernwissen

### Ausfallsicherheit im Server

| Komponente | Maßnahme | schützt vor |
|---|---|---|
| Netzteil | **2 redundante Hot-Swap-Netzteile** an **zwei Stromkreisen/USVs** | Netzteildefekt, Ausfall einer Phase |
| Arbeitsspeicher | **ECC-RAM**, Memory Mirroring | Bitfehler, Datenkorruption |
| Datenträger | **RAID** + Hot Spare, Hot-Swap-Schächte | Plattenausfall |
| Netzwerk | **NIC-Teaming/Bonding** an zwei Switches | Ausfall von Karte, Kabel oder Switch |
| Kühlung | redundante Lüfter | Überhitzung |
| Storage-Anbindung | Multipath (zwei HBAs, zwei Fabrics) | Pfadausfall zum SAN |
| Gesamtsystem | Cluster, zweiter Server | Ausfall des ganzen Servers |

### Fernwartung (Out-of-Band)

Der **BMC** (Baseboard Management Controller, z. B. Dell iDRAC, HPE iLO, IPMI) hat einen eigenen Netzwerkport und läuft auch bei ausgeschaltetem Server. Er ermöglicht Einschalten, Konsole, Firmware-Updates und Hardware-Monitoring. Sicherheit: eigenes Management-VLAN, Standardpasswort ändern, Firmware aktuell halten.

### Serverauswahl nach Einsatzzweck

| Einsatz | wichtig | weniger wichtig |
|---|---|---|
| **Datenbankserver** | hoher Takt (Single-Thread), viel RAM (Cache), sehr schnelles Storage mit niedriger Latenz (NVMe, RAID 10) | GPU, sehr viele Kerne |
| **Virtualisierungshost** | viele Kerne, **sehr viel RAM**, schnelle Netzwerkanbindung (25/100 GbE) für Storage und Live-Migration, Anbindung an SAN | einzelner hoher Takt, viele lokale Platten |
| **Mail-/Fileserver** | viele Laufwerksschächte, I/O-Leistung, redundante 10-GbE-Anbindung | höchste CPU-Leistung |
| **Webserver** | horizontal skalierbar, mehrere kleine Server hinter einem Load Balancer | große Einzelmaschine |

**Vorgehen bei Tabellenvergleich:** 1. Anforderungen aus dem Szenario herausschreiben. 2. K.-o.-Kriterien prüfen (z. B. nur 2 × 1 GbE = keine Redundanz, zu wenig RAM). 3. Mit konkreten Werten aus der Tabelle begründen. 4. Reserve für Wachstum nennen.

### RAID-Controller: leistungsbestimmende Merkmale

| Merkmal | Beispielwert |
|---|---|
| unterstützte RAID-Level | 0, 1, 5, 6, 10, 50, 60 |
| Cache-Größe | 2 GB, 4 GB |
| Cache-Schutz | BBU (Batterie) oder Flash-Backup, ermöglicht Write-Back |
| Host-Schnittstelle | PCIe 4.0 x8 |
| Laufwerksschnittstelle | SAS 12 Gbit/s, SATA 6 Gbit/s, NVMe (Tri-Mode) |
| Anzahl Ports/Laufwerke | 8, 16 |

Siehe auch [Übung RAID-Controller](../analyse-der-exams/uebung-raid-controller.md).

### Bauformen

| Bauform | Vorteile | Nachteile |
|---|---|---|
| **Tower** | leise, günstig, kein Rack nötig | viel Platz, schlecht stapelbar |
| **Rack** (1–4 HE) | standardisiert (19"), hohe Dichte, gute Verkabelung | Rack und Klimatisierung nötig, lauter |
| **Blade** | sehr hohe Dichte, gemeinsames Netz und Strom, schneller Tausch | teures Chassis, Herstellerbindung, hohe Wärmedichte |

## Prüfungsfrage

**Frage:** Nennen Sie außer RAID drei technische Maßnahmen, die die Ausfallsicherheit eines Datenbankservers erhöhen, und erläutern Sie jeweils kurz.

**Musterantwort:** Erstens zwei redundante Hot-Swap-Netzteile, angeschlossen an zwei getrennte Stromkreise mit USV. Fällt ein Netzteil oder ein Stromkreis aus, versorgt das andere den Server weiter, und das defekte wird im laufenden Betrieb getauscht. Zweitens ECC-Arbeitsspeicher, der Einzelbitfehler erkennt und korrigiert und so Abstürze und verfälschte Daten verhindert. Drittens NIC-Teaming: Zwei Netzwerkkarten werden mit zwei verschiedenen Switches verbunden, sodass der Ausfall einer Karte, eines Kabels oder eines Switches die Erreichbarkeit nicht unterbricht. Darüber hinaus kann ein Failover-Cluster mit einem zweiten Server den Ausfall des gesamten Systems abfangen.

## Falle

- Zwei Netzteile an **dieselbe** Steckdosenleiste hängen. Dann bleibt der Stromkreis ein SPOF.
- Für Virtualisierung den Server mit dem höchsten Takt statt dem meisten RAM und den meisten Kernen wählen.
- Für einen Datenbankserver langsame große HDDs statt schneller NVMe-SSDs wählen.
- RAID als einzige Maßnahme nennen, wenn „außer RAID“ gefragt ist.
- BMC/iDRAC ins normale Benutzernetz hängen. Es ist ein beliebtes Angriffsziel.

## Merksatz

**„Ein Server hat alles doppelt, was kaputtgehen kann. Ausgewählt wird nach Einsatzzweck, nicht nach dem höchsten Wert in der Tabelle.“**

## Verwandt

- [CPU](cpu.md) – Kerne, Takt, Architektur
- [RAM](ram.md) – DDR, ECC, Datenrate
- [USB-SSD-HDD](usb-ssd-hdd.md) – Datenträger und Schnittstellen
- [RAID](../FiSi-Administrator/Server/raid.md)
- [HA-Hochverfügbarkeit](../FiSi-Administrator/Server/ha-hochverfuegbarkeit.md)
- [USV](../FiSi-Administrator/Server/usv.md)
- [Energieeffizienz & Stromkosten](../FiSi-Administrator/Server/energieeffizienz-stromkosten.md) – Netzteil dimensionieren
- [Monitoring](../FiSi-Administrator/Server/monitoring.md) – Bottleneck-Analyse
- [Lektion 03 Verfügbarkeit](../analyse-der-exams/03_Verfuegbarkeit_Ausfallsicherheit.md)
