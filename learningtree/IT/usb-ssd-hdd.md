# USB-SSD-HDD

# Speicher, USB, SSD, HDD und Kabel – IHK AP2

**Wichtig:** Gbit/s (Gigabit pro Sekunde) und GB/s (Gigabyte pro Sekunde) sind nicht dasselbe. 8 Gbit/s entsprechen theoretisch 1 GB/s, bevor Protokoll-Overhead berücksichtigt wird.

## HDD – Hard Disk Drive

Speichert Daten magnetisch auf rotierenden Scheiben. Günstig bei großen Speichermengen, aber langsamer und empfindlicher gegenüber Erschütterungen als SSDs.

| Typ | Typische Geschwindigkeit | Eigenschaften / Einsatz |
|---|---:|---|
| HDD, 5.400 RPM | ca. 80–150 MB/s | Leiser, oft weniger Stromverbrauch |
| HDD, 7.200 RPM | ca. 120–250 MB/s | Desktop-PCs, NAS, große Datenmengen |
| HDD, 10.000/15.000 RPM | stark modellabhängig | Ältere Enterprise-Systeme; heute oft durch SSDs ersetzt |

- **RPM** = Umdrehungen pro Minute.
- Zugriffszeiten liegen typischerweise im Millisekundenbereich.
- HDDs eignen sich für Archive, Backups und große Datenbestände.
- Nachteile: mechanischer Verschleiß, Geräusche, höhere Latenz.

## SSD – Solid State Drive

Speichert Daten in Flash-Speicher ohne bewegliche Teile. Deutlich kürzere Zugriffszeiten als HDDs.

| Schnittstelle / Typ | Theoretische Schnittstellenrate | Eigenschaften / Einsatz |
|---|---:|---|
| SATA SSD | bis 6 Gbit/s, ca. 600 MB/s vor Overhead | Ältere PCs, günstiges Upgrade |
| NVMe über PCIe 3.0 x4 | ca. 3,9 GB/s | Schnelle System- und Datenträger |
| NVMe über PCIe 4.0 x4 | ca. 7,9 GB/s | Gaming, Workstations, schnelle Dateiübertragung |
| NVMe über PCIe 5.0 x4 | ca. 15,8 GB/s | Sehr schnelle Workstations und professionelle Systeme |
| NVMe über PCIe 6.0 x4 | theoretisch ca. 31,5 GB/s | Neuere Plattformen; tatsächliche Produkte variieren |

Die Werte sind theoretische Richtwerte für die Schnittstelle, keine garantierten SSD-Transferraten. Die reale Geschwindigkeit hängt von SSD, Controller, Temperatur, Datenart und System ab.

### SATA vs. NVMe

- **SATA** ist eine Schnittstelle, die bei HDDs und SSDs eingesetzt wird.
- **NVMe** ist ein für Flash-Speicher optimiertes Protokoll, das meist über PCIe angebunden ist.
- Eine **M.2-SSD** beschreibt zunächst den Formfaktor. M.2 kann SATA oder NVMe/PCIe nutzen – nicht jede M.2-SSD ist NVMe.

## HDD vs. SSD

| Eigenschaft | HDD | SATA SSD | NVMe SSD |
|---|---|---|---|
| Technik | Magnetische Scheiben | Flash-Speicher | Flash-Speicher |
| Mechanische Teile | Ja | Nein | Nein |
| Zugriffszeit | Millisekunden | Sehr niedrig | Sehr niedrig |
| Geschwindigkeit | Niedrig | Mittel | Hoch bis sehr hoch |
| Preis pro GB | Häufig günstig | Mittel | Häufig höher |
| Typischer Einsatz | Archive, NAS, Backups | Office-PCs, Upgrades | Betriebssystem, Spiele, Workstations |

**Merksatz:** HDD = viel Speicher günstig. SATA-SSD = gutes Upgrade. NVMe-SSD = hohe Geschwindigkeit über PCIe.

---

## USB – Generationen und Geschwindigkeit

USB (Universal Serial Bus) überträgt Daten und kann Geräte mit Strom versorgen. Die tatsächliche Geschwindigkeit hängt von Host, Gerät und Kabel ab.

| Standard / Bezeichnung | Maximale Datenrate | Hinweise |
|---|---:|---|
| USB 1.1 Low Speed | 1,5 Mbit/s | Langsame Eingabegeräte |
| USB 1.1 Full Speed | 12 Mbit/s | Ältere Peripherie |
| USB 2.0 | 480 Mbit/s | Tastaturen, Mäuse, viele einfache Geräte |
| USB 3.2 Gen 1 | 5 Gbit/s | Früher USB 3.0 / USB 3.1 Gen 1 |
| USB 3.2 Gen 2 | 10 Gbit/s | Früher USB 3.1 Gen 2 |
| USB 3.2 Gen 2x2 | 20 Gbit/s | Nutzt zwei Lanes; häufig USB-C |
| USB4 20 Gbit/s | 20 Gbit/s | USB-C, Daten und optional DisplayPort-Tunneling |
| USB4 40 Gbit/s | 40 Gbit/s | USB-C, hohe Datenrate |
| USB4 80 Gbit/s | 80 Gbit/s | USB4 Version 2.0, geeignetes Gerät und Kabel erforderlich |

Die Namen älterer USB-3.x-Versionen wurden mehrfach geändert. Für den Kauf ist die angegebene **Datenrate** oft eindeutiger als ein allgemeiner Begriff wie „USB 3“.

Quelle: [USB-IF – USB 3.2](https://www.usb.org/usb-32-0), [USB-IF – USB4](https://www.usb.org/usb4)

```markdown
## USB-Stecker und Kabeltypen

**Steckerform ≠ Übertragungsgeschwindigkeit.** USB-C beschreibt die Bauform des Steckers, nicht automatisch USB4 oder eine bestimmte Geschwindigkeit.

| Anschluss | Eigenschaften | Typischer Einsatz |
|---|---|---|
| USB-A | Rechteckiger Stecker, nicht umkehrbar | PCs, USB-Sticks, Tastaturen |
| USB-B | Meist quadratisch | Drucker, ältere Geräte |
| Mini-USB | Kleiner älterer Anschluss | Ältere Kameras |
| Micro-USB | Sehr kleiner Anschluss | Ältere Smartphones, Geräte |
| USB-C | Klein, beidseitig einsteckbar | Moderne Smartphones, Laptops, Docks, SSDs |

### USB-C-Kabel: Worauf achten?

- **Datenrate:** z. B. 5, 10, 20, 40 oder 80 Gbit/s – abhängig von Kabel und Geräten.
- **Ladeleistung:** Nicht jedes Kabel unterstützt dieselbe Stromstärke oder Leistung.
- **DisplayPort-Unterstützung:** Nicht jeder USB-C-Port unterstützt Bildausgabe.
- **USB Power Delivery (USB PD):** Standard zur Aushandlung der Stromversorgung; geeignete Kabel und Geräte vorausgesetzt.
- **Aktive Kabel:** Enthalten Elektronik zur Signalaufbereitung; bei hohen Datenraten und größeren Längen relevant.
- **Passive Kabel:** Übertragen Signale ohne aktive Signalaufbereitung.

Ein Kabel kann äußerlich wie ein anderes aussehen und trotzdem eine deutlich geringere Datenrate unterstützen.

## Thunderbolt

Thunderbolt nutzt ebenfalls den USB-C-Stecker, bietet aber definierte Funktionen und Anforderungen je nach Generation. Es kann Daten, DisplayPort-Signale und PCIe über eine Verbindung transportieren.

| Generation | Maximale Bandbreite | Typische Verwendung |
|---|---:|---|
| Thunderbolt 1 | 10 Gbit/s | Ältere schnelle Peripherie |
| Thunderbolt 2 | 20 Gbit/s | Ältere professionelle Systeme |
| Thunderbolt 3 | 40 Gbit/s | Docks, schnelle externe SSDs, Displays |
| Thunderbolt 4 | 40 Gbit/s | Docks, mehrere Displays, externe Speicher |
| Thunderbolt 5 | 80 Gbit/s bidirektional; bis zu 120 Gbit/s in einem speziellen Bandwidth-Boost-Modus | Hochleistungs-Docks, schnelle Speicher, professionelle Displays |

Die Geschwindigkeit allein unterscheidet Thunderbolt 3 und 4 nicht. Thunderbolt 4 stellt strengere Mindestanforderungen an bestimmte Funktionen und Kompatibilität.

Quelle: [Intel – Thunderbolt-Technik](https://www.intel.de/content/www/de/de/architecture-and-technology/thunderbolt/overview.html)

### Daisy Chaining

**Daisy Chaining** bedeutet, mehrere Geräte in Reihe zu verbinden.

Beispiel:

`Laptop → Thunderbolt-Monitor → Thunderbolt-SSD → weiteres Thunderbolt-Gerät`

- Unterstützt von Thunderbolt-Geräten, wenn die Geräte entsprechende Downstream-Ports besitzen.
- Reduziert den Kabelbedarf am Laptop.
- Die gemeinsame Verbindung hat eine begrenzte Bandbreite.
- Nicht jeder Monitor oder jedes USB-C-Gerät kann das Signal weiterleiten.
- USB-C allein garantiert kein Daisy Chaining.

**Hub vs. Daisy Chain:**
- **Hub/Dock:** Mehrere Geräte werden an einen zentralen Verteiler angeschlossen.
- **Daisy Chain:** Geräte werden hintereinander verbunden.

Quelle: [Intel – Thunderbolt 4 und USB-C](https://www.intel.com/content/www/us/en/architecture-and-technology/thunderbolt/thunderbolt-4-vs-usb-c.html)

## Weitere wichtige Kabel und Schnittstellen

| Typ | Geschwindigkeit / Eigenschaft | Einsatz |
|---|---|---|
| SATA-Datenkabel | SATA III: 6 Gbit/s brutto | Interne SATA-HDDs und SATA-SSDs |
| SATA-Stromkabel | Stromversorgung, keine Datenübertragung | Interne SATA-Laufwerke |
| PCIe | Je nach Generation und Lane-Anzahl sehr hohe Bandbreite | NVMe-SSDs, Grafikkarten, Erweiterungskarten |
| Ethernet Cat 5e | Bis 1 Gbit/s bei bis zu 100 m | Heim- und Firmennetzwerke |
| Ethernet Cat 6 | Bis 10 Gbit/s auf kürzeren Strecken; 1 Gbit/s bis 100 m | LAN-Verkabelung |
| Ethernet Cat 6A | 10 Gbit/s bis 100 m | Professionelle LANs |
| Ethernet Cat 8 | Bis 25/40 Gbit/s auf kurzen Strecken, typischerweise bis 30 m | Rechenzentrum |
| HDMI | Version und Kabelklasse bestimmen die unterstützten Datenraten und Auflösungen | Fernseher, Monitore, Beamer |
| DisplayPort | Version und Kabel bestimmen die mögliche Bandbreite | PC-Monitore, hohe Auflösungen und Bildwiederholraten |
| Glasfaser | Hohe Bandbreite, große Reichweite, unempfindlich gegen elektromagnetische Störungen | Backbone, Rechenzentrum, Provider |

**Achtung:** Bei Ethernet hängen die tatsächlich unterstützten Geschwindigkeiten von Kategorie, Länge, Installation und den Netzwerkports ab. Bei HDMI und DisplayPort sollte man die unterstützte Version bzw. zertifizierte Kabelklasse prüfen, nicht nur die Steckerform.

## IHK-Merksätze

- **HDD vs. SSD:** HDD ist mechanisch; SSD nutzt Flash-Speicher.
- **SATA vs. NVMe:** SATA ist eine Schnittstelle; NVMe ist ein Speicherprotokoll, das meist PCIe nutzt.
- **M.2 vs. NVMe:** M.2 ist ein Formfaktor, NVMe ein Protokoll.
- **USB-C vs. USB4:** USB-C ist ein Steckertyp, USB4 ein Übertragungsstandard.
- **USB-C vs. Thunderbolt:** Gleiche Steckerform möglich, aber nicht dieselben garantierten Funktionen.
- **Gbit/s vs. GB/s:** 8 Gbit/s entsprechen rechnerisch 1 GB/s vor Overhead.
- **Daisy Chain vs. Hub:** Reihenschaltung gegenüber zentralem Verteiler.
- **SATA-Datenkabel vs. SATA-Stromkabel:** Datenübertragung gegenüber Stromversorgung.
- **Ethernet vs. Glasfaser:** Ethernet bezeichnet eine Netzwerktechnik; Glasfaser ist ein Übertragungsmedium, über das Ethernet laufen kann.
