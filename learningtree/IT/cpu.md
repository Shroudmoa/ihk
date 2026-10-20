# CPU

# CPU, GPU, Architektur und Speicherhierarchie

## CPU – Central Processing Unit

Die CPU ist der Hauptprozessor eines Computers. Sie führt Maschinenbefehle aus, berechnet Daten und steuert Abläufe im System.

Eine CPU wird nicht nur anhand ihrer GHz bewertet. Entscheidend sind Architektur, Kernanzahl, Cache, Speicheranbindung, Energieverbrauch und der jeweilige Workload.

## CPU-Aufbau

- **Core (Kern):** Führt Befehle unabhängig von anderen Kernen aus.
- **Thread:** Ausführungsstrang eines Programms. Ein Kern kann je nach Architektur mehrere Hardware-Threads unterstützen.
- **ALU:** Führt ganzzahlige arithmetische und logische Operationen aus.
- **FPU:** Führt Gleitkommaberechnungen aus.
- **Register:** Extrem schneller Speicher innerhalb des Prozessorkerns.
- **Control Unit:** Steuert die Verarbeitung und Ausführung von Befehlen.
- **Cache:** Speichert häufig benötigte Daten nahe am Kern.
- **MMU:** Übersetzt virtuelle in physische Speicheradressen und unterstützt den Speicherschutz.

### Wie verarbeitet eine CPU Befehle?

Der typische Ablauf ist:

1. **Fetch:** Befehl aus dem Speicher laden.
2. **Decode:** Befehl interpretieren.
3. **Execute:** Befehl ausführen.
4. **Write-back:** Ergebnis zurückschreiben.

Moderne CPUs führen diese Schritte überlappend und teilweise außerhalb der ursprünglichen Programmreihenfolge aus (Pipelining, Superscalar Execution, Out-of-Order Execution).

## CPU-Taktfrequenz

Die Taktfrequenz wird in Hertz angegeben. 1 GHz entspricht einer Milliarde Taktzyklen pro Sekunde.

Beispiel:
- CPU A: 3,0 GHz
- CPU B: 4,0 GHz

CPU B hat die höhere Taktfrequenz, ist aber nicht automatisch schneller. CPU A kann pro Takt mehr Arbeit erledigen oder eine effizientere Architektur besitzen.

**Wichtige Begriffe:**
- **Base Clock:** Referenztakt unter definierten Betriebsbedingungen.
- **Boost Clock:** Kurzzeitig oder abhängig von Temperatur, Leistungsbudget und Last erreichbare höhere Frequenz.
- **IPC (Instructions Per Cycle):** Anzahl der pro Takt ausgeführten Befehle, abhängig von Programm und Architektur.
- **TDP:** Thermische Auslegungsgröße des Herstellers; nicht gleichbedeutend mit dem tatsächlichen Maximalverbrauch.
- **Latenz:** Zeit bis zum Abschluss einer Operation.
- **Durchsatz:** Menge der Arbeit pro Zeiteinheit.

**IHK-Merksatz:** Mehr GHz bedeuten nicht automatisch mehr Leistung. Taktfrequenz, IPC, Parallelität und Workload bestimmen gemeinsam die Performance.

## CPU-Kerne und Threads

| Konzept | Bedeutung |
|---|---|
| Single-Core | Ein CPU-Kern |
| Multi-Core | Mehrere CPU-Kerne auf einem Prozessor |
| Multithreading | Mehrere Ausführungsstränge innerhalb eines Programms |
| SMT / Hyper-Threading | Ein physischer Kern verwaltet mehrere Hardware-Threads |
| Multitasking | Betriebssystem führt mehrere Prozesse scheinbar oder tatsächlich parallel aus |

Beispiel: Eine CPU mit 8 Kernen und SMT kann 16 logische Prozessoren bereitstellen. Das bedeutet nicht, dass sie die doppelte Leistung einer CPU mit 8 Threads erreicht.

- **Single-Thread-Last:** Profitiert häufig von hoher IPC und hoher Boost-Frequenz.
- **Multi-Thread-Last:** Kann von mehr Kernen und effizienter Parallelisierung profitieren.
- **IHK-Bezug:** CPU-Auslastung, Prozessverwaltung und CPU Contention bei virtuellen Maschinen.

## CPU-Architekturen und Befehlssätze

Architektur und Befehlssatz sind nicht dasselbe.

- **Befehlssatz (ISA):** Definiert, welche Maschinenbefehle ein Prozessor versteht.
- **Mikroarchitektur:** Beschreibt, wie der Prozessor diese Befehle intern ausführt.

| Architektur / ISA | Typische Verwendung | Beispiele |
|---|---|---|
| x86-64 | PCs, Workstations, Server | Intel Core/Xeon, AMD Ryzen/EPYC |
| ARM64 / AArch64 | Smartphones, Notebooks, Server | Apple Silicon, Snapdragon, Ampere |
| RISC-V | Embedded, Forschung, zunehmend weitere Systeme | RISC-V-basierte Prozessoren |
| Power ISA | Spezialisierte Systeme und Server | IBM Power |

### x86-64: Intel und AMD

x86-64 ist eine 64-Bit-Erweiterung der x86-Architektur. Intel und AMD stellen kompatible Prozessoren her, unterscheiden sich aber bei Mikroarchitektur, Cache, Kernaufbau und Implementierung.

- **Intel Core / Core Ultra:** PCs und Notebooks.
- **Intel Xeon:** Server und Workstations.
- **AMD Ryzen:** PCs und Workstations.
- **AMD EPYC:** Server und Rechenzentren.

### ARM: Apple und andere Hersteller

ARM-basierte Prozessoren verwenden eine andere ISA als x86-64. ARM ist besonders in mobilen Geräten verbreitet, wird aber auch in Notebooks und Servern eingesetzt.

- **Apple M-Serie:** Macs; CPU, GPU und weitere Beschleuniger auf einem SoC.
- **Qualcomm Snapdragon:** Smartphones und ARM-basierte PCs.
- **Ampere:** ARM-basierte Serverprozessoren.

ARM bedeutet nicht automatisch schneller oder sparsamer. Das Ergebnis hängt von Mikroarchitektur, Fertigung, Leistungsbudget und Workload ab.

### RISC und CISC

- **RISC:** Traditionell einfachere, regelmäßig strukturierte Befehle; beispielsweise ARM und RISC-V.
- **CISC:** Umfangreicher Befehlssatz; beispielsweise x86.

Moderne Prozessoren sind intern komplexer als diese vereinfachte Einteilung vermuten lässt. Moderne x86-CPUs zerlegen viele Befehle in interne Mikrooperationen.

## Hybrid-CPUs

Einige Prozessoren kombinieren unterschiedliche Kerntypen.

- **Performance-Kerne (P-Cores):** Für hohe Leistung pro Thread.
- **Efficiency-Kerne (E-Cores):** Für effiziente Verarbeitung geeigneter paralleler oder weniger zeitkritischer Aufgaben.

Beispiel: Einige Intel-Core-Ultra- und Apple-Silicon-Prozessoren nutzen unterschiedliche Kerntypen.

Das Betriebssystem verteilt Threads anhand von Prioritäten, Auslastung und verfügbaren Informationen auf die Kerne.

## CPU-Auswahl nach Einsatz

| Einsatz | Wichtige Kriterien |
|---|---|
| Office und Ausbildung | Ausreichende Single-Core-Leistung, RAM, SSD |
| Gaming | CPU-Leistung, GPU, Cache, Latenz |
| Virtualisierung | Kerne, RAM, Speicherbandbreite, I/O |
| Datenbankserver | Latenz, RAM, I/O, Parallelität |
| Rendering | Multi-Core-Leistung oder GPU-Beschleunigung |
| Notebook | Leistung pro Watt, Kühlung, Akkulaufzeit |
| Rechenzentrum | Durchsatz, Energiebedarf, Skalierbarkeit, Verfügbarkeit |



## CPU-Speicherhierarchie

Die CPU ist deutlich schneller als der Arbeitsspeicher (RAM). Caches reduzieren deshalb die Wartezeit beim Zugriff auf häufig benötigte Daten.

CPU-Register → L1-Cache → L2-Cache → L3-Cache → RAM → SSD/HDD

Je weiter eine Ebene vom Rechenkern entfernt ist, desto größer und normalerweise langsamer ist sie. Die genaue Hierarchie hängt von der CPU ab. [Intel Cache-Dokumentation](https://www.intel.de/content/www/de/de/support/articles/000057727/processors.html) · [Arm Speicherhierarchie](https://learn.arm.com/learning-paths/servers-and-cloud-computing/performix-memory-access/how-to-0/)

### Register

Kleinster und schnellster Speicher direkt im CPU-Kern.

- Speichert Operanden, Adressen und Zwischenergebnisse.
- Wird direkt von Maschinenbefehlen verwendet.

### L1-Cache

- Sehr klein und extrem schnell.
- Normalerweise direkt einem CPU-Kern zugeordnet.
- Häufig getrennt in **L1 Data Cache** und **L1 Instruction Cache**.

### L2-Cache

- Größer als L1, aber langsamer.
- Häufig pro Kern oder pro Kerngruppe organisiert.
- Dient als weitere Cache-Ebene vor dem L3-Cache oder RAM.

### L3-Cache

- Häufig größer und von mehreren CPU-Kernen gemeinsam genutzt.
- Reduziert Zugriffe auf den langsameren RAM.
- Größe und Aufbau unterscheiden sich je nach Mikroarchitektur.

### Cache Hit und Cache Miss

- **Cache Hit:** Die benötigten Daten liegen bereits im Cache.
- **Cache Miss:** Die Daten müssen aus einer weiteren Speicherhierarchie geladen werden.

**Beispiel:** Eine Schleife verarbeitet dieselben Daten mehrfach. Wenn diese im Cache bleiben, muss die CPU seltener auf den RAM zugreifen.

### RAM – Arbeitsspeicher

RAM speichert aktuell benötigte Programme und Daten.

- Flüchtiger Speicher: Inhalt geht ohne Stromversorgung verloren.
- Deutlich größer als CPU-Caches.
- DDR4 und DDR5 sind verbreitete RAM-Technologien für PCs und Server.

**Wichtig:** Mehr RAM erhöht nicht automatisch die CPU-Leistung. Er verhindert aber, dass Speichermangel zu starkem Swapping führt.

### Virtueller Speicher

Betriebssysteme verwenden virtuelle Adressen, um Prozesse voneinander zu isolieren und Speicher flexibel zu verwalten.

- **MMU:** Unterstützt die Übersetzung virtueller in physische Adressen.
- **Page Table:** Enthält die Zuordnung von virtuellen zu physischen Speicherseiten.
- **TLB:** Cache für häufig benötigte Adressübersetzungen.
- **Swap/Pagefile:** Kann Speicherseiten auf Storage auslagern.

**IHK-Verknüpfung:** Virtueller Speicher, Prozessisolation, Kernel Space und User Space.

## GPU – Graphics Processing Unit

Eine GPU ist auf stark parallele Berechnungen ausgelegt. Während CPUs wenige leistungsfähige Kerne für vielseitige Aufgaben verwenden, besitzt eine GPU viele Recheneinheiten für zahlreiche parallele Operationen.

### GPU-Typen

| Typ | Eigenschaften | Einsatz |
|---|---|---|
| iGPU | In CPU/SoC integriert, häufig gemeinsamer RAM | Office, Video, einfache Grafik |
| dGPU | Dedizierter Grafikprozessor, meist eigener VRAM | Gaming, Rendering, GPU-Berechnungen |
| Compute-GPU | Auf parallele Berechnungen optimiert | KI, Simulationen, wissenschaftliche Berechnungen |

### CPU vs. GPU

| Kriterium | CPU | GPU |
|---|---|---|
| Optimierung | Vielseitige Aufgaben, niedrige Latenz | Hoher paralleler Durchsatz |
| Recheneinheiten | Weniger, komplexe Kerne | Viele parallele Recheneinheiten |
| Typische Aufgaben | Betriebssystem, Anwendungen, Datenbanklogik | Grafik, Matrixberechnungen, KI |
| Speicher | RAM über Speichercontroller | VRAM oder gemeinsam genutzter Speicher |

### GPU-Speicher

- **VRAM:** Dedizierter Grafikspeicher, beispielsweise GDDR6 oder GDDR7.
- **HBM:** Speicher mit hoher Bandbreite, häufig bei Beschleunigern im Rechenzentrum.
- **Unified Memory:** CPU und GPU greifen auf einen gemeinsamen oder einheitlich verwalteten Speicherbereich zu; die genaue Implementierung ist plattformabhängig.

**Beispiel:** Eine GPU verarbeitet viele Matrixoperationen parallel. Eine CPU übernimmt dagegen häufig Programmsteuerung, Betriebssystemaufgaben und die Koordination der Berechnung.

## Weitere wichtige Begriffe

### Speicherbandbreite

Datenmenge, die pro Sekunde übertragen werden kann. Angegeben beispielsweise in GB/s.

### Latenz

Zeit zwischen einer Anfrage und dem Beginn beziehungsweise Abschluss der Datenbereitstellung.

### NUMA

Bei Mehrsockelservern hängt die Speicherzugriffszeit davon ab, an welchem Prozessor der RAM angeschlossen ist. Lokaler Speicherzugriff ist normalerweise günstiger als entfernter Zugriff.

### PCIe

Schnittstelle zur Verbindung von Komponenten wie GPUs, NVMe-SSDs und Netzwerkkarten mit dem System.

### SoC – System on a Chip

Integriert mehrere Komponenten auf einem Chip beziehungsweise in einem Chip-Paket, beispielsweise CPU, GPU, Speichercontroller und Medienbeschleuniger.

Beispiel: Apple Silicon kombiniert CPU- und GPU-Recheneinheiten mit weiteren Funktionen und einer gemeinsamen Speicherarchitektur.

## IHK-Prüfungswissen

- **Mehr GHz ≠ automatisch mehr Leistung:** IPC und Mikroarchitektur beeinflussen die tatsächliche Rechenleistung.
- **Mehr Kerne ≠ automatisch schneller:** Die Anwendung muss parallelisierbar sein.
- **Cache ≠ RAM:** Cache ist kleiner und näher an der CPU; RAM bietet wesentlich mehr Kapazität.
- **RAM ≠ Storage:** RAM ist flüchtiger Arbeitsspeicher, SSD/HDD sind dauerhafte Datenspeicher.
- **CPU ≠ GPU:** CPU für vielseitige Verarbeitung, GPU für stark parallele Workloads.
- **Bandbreite ≠ Latenz:** Bandbreite beschreibt Datenmenge pro Zeit, Latenz die Verzögerung eines Zugriffs.

## 3-Zeilen-IHK-Antwort

Die CPU führt Maschinenbefehle aus; ihre Leistung hängt unter anderem von Taktfrequenz, IPC, Kernanzahl und Cache ab.  
Die Speicherhierarchie aus Registern, Cache und RAM reduziert Zugriffszeiten und versorgt die CPU mit Daten.  
GPUs optimieren stark parallele Berechnungen, während CPUs vielseitige Aufgaben und die Programmsteuerung übernehmen.

## Siehe auch

- [BIOS/UEFI-Kernel](bios-uefi-kernel.md) – Kernel tasks und System Calls
- [VM-Container-Server](vm-container-server.md) – Kerne und Ressourcen für VMs

