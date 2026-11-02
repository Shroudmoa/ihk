# VM, Container, Server – Vergleich

## In 30 Sekunden

- **Bare Metal** = OS direkt auf Hardware → maximale Leistung, schlechte Auslastung
- **VM** = Hypervisor virtualisiert Hardware, jedes VM hat **eigenes Gast-OS** → starke Isolation
- **Container** = Container-Engine virtualisiert OS, teilen sich **Host-OS-Kernel** → leicht & schnell
- 20 VMs = 20 Gastbetriebssysteme + 20 Kernel | 20 Container = **1 Kernel + 20 isolierte Prozesse**

## Kernvergleich

| Kriterium | **Bare Metal** | **VM (Virtual Machine)** | **Container** |
|---|---|---|---|
| **Virtualisierung** | Keine | Hardware-Virtualisierung (Hypervisor) | OS-Virtualisierung (Container-Runtime) |
| **Betriebssystem** | Direkt installiert | Eigenes Gast-OS pro VM | Teilt sich Host-OS-Kernel |
| **Isolation** | Physisch/keine Virtualisierung | **Sehr stark** (eigener Kernel) | Gut (Namespaces/cgroups), schwächer als VM |
| **Startzeit** | Minuten | Sekunden – Minuten | **Millisekunden – Sekunden** |
| **Größe** | Vollständiges System | GB-Bereich (inkl. Gast-OS) | **MB-Bereich** (App + Abhängigkeiten) |
| **Ressourcenverbrauch** | Sehr hoch pro Dienst | Hoch (pro Gast-OS) | **Sehr gering** |
| **Dichte** | Gering (meist 1 Hauptdienst) | Mittel-Hoch | **Sehr hoch** |
| **Portabilität** | Gering | Gut | **Sehr gut** (Image-basiert) |
| **Mischbetrieb verschiedener OS** | Nein | **Ja** (Linux/Windows parallel) | **Nein** (gleicher Kernel wie Host) |
| **Typischer Einsatz** | Datenbanken, Storage, latenzkritisch | AD, Legacy-Apps, gemischte Umgebungen | Microservices, CI/CD, Cloud-native Apps |

## Entscheidungslogik für die Prüfung

1. **Anderes Betriebssystem oder maximale Isolation nötig?** → **VM**
2. **Viele identische, kurzlebige Dienste, schnell & ressourcenschonend?** → **Container**
3. **Maximale I/O-Leistung, Direktzugriff auf Hardware?** → **Bare Metal**
4. **Verschiedene Kernel notwendig?** → **Nur VM** (Container unmöglich)

## Wichtige Punkte

- **Container sind keine kleinen VMs.** Der größte Unterschied: gemeinsamer Kernel vs. eigener Kernel
- **Kernel-Bug:** Trifft bei Containern potentiell **alle** Container auf diesem Host
- **Mischbetrieb ist üblich:** **Container in VMs** kombinieren hohe Dichte mit starker Isolation
- **Hypervisor vs. Container-Engine:** Hypervisor virtualisiert Hardware, Container-Engine virtualisiert das Betriebssystem

## Prüfungsfrage

**Frage:** Ein Unternehmen will 20 Instanzen einer Webanwendung möglichst platzsparend und schnell skalierbar betreiben. Welches Verfahren ist am geeignetsten und warum?

**Musterantwort:** **Container**, da alle Instanzen den gleichen Host-OS-Kernel nutzen und nur Anwendung + Bibliotheken im Image enthalten. Dadurch verbrauchen sie deutlich weniger RAM/Storage als VMs, lassen sich in Sekunden starten und sehr dicht betreiben. Für erhöhte Isolation kann man sie zusätzlich in virtuellen Maschinen laufen lassen.

## Verwandt

- [Hypervisor-Typen](hypervisor-typen.md)
- [Virtualisierung](virtualisierung.md)
- [Live-Migration-VMs](live-migration-vms.md)
- [Skalierbarkeit-Elastizität](skalierbarkeit-elastizitaet.md)
- [Cloud Computing](../Cloud/cloud-computing.md)


## Falle

- „Container = VM ohne Hypervisor“ → falsch, es fehlt das eigene Gastbetriebssystem.
- Container pauschal als sicher bezeichnen → die Isolation ist schwächer als bei VMs.
- Live-Migration mit Containern gleichsetzen → bei VMs wird der Speicherzustand verschoben, Container werden meist neu gestartet oder auf einem anderen Host gestartet.
- Bare Metal als „veraltet“ abtun → bei Datenbanken und Spezialhardware oft die richtige Antwort.

## Merksatz

**„VM = eigenes Haus, Container = eigene Wohnung im selben Haus.“** Der Kernel ist das Fundament.

## Verwandt

- [Hypervisor-Typen](hypervisor-typen.md) – Typ 1 vs. Typ 2
- [Live-Migration-VMs](live-migration-vms.md) – laufende VMs verschieben
- [BIOS/UEFI-Kernel](../bios-uefi-kernel.md) – Kernel und User Space
- [Snapshots-Backups](snapshots-backups.md) – Sicherung von VMs
