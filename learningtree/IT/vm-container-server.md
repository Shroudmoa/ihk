# VM-Container-Server

## In 30 Sekunden

- **Bare Metal** = OS direkt auf der Hardware: maximale Leistung, schlechte Auslastung.
- **VM** = eigener Kernel pro Gast, Hypervisor dazwischen: starke Isolation, mehr Overhead.
- **Container** = teilen den Host-Kernel: leicht, schnell, aber schwächere Isolation.
- 20 VMs = 20 Gastbetriebssysteme und 20 Kernel. 20 Container = 1 Kernel und 20 Prozesse.
- Die Prüfungsfrage ist fast immer: **„Was passt für Szenario X und warum?“**

## Kernwissen

| Kriterium | Bare Metal | VM | Container |
|---|---|---|---|
| Isolation | physisch getrennt | stark, eigener Kernel | prozessbasiert |
| Startzeit | Minuten | Sekunden bis Minuten | Millisekunden bis Sekunden |
| Overhead | keiner | Hypervisor + Gast-OS | minimal |
| Dichte | 1 Dienst pro Host | wenige bis viele | sehr viele |
| Betriebssystem | ein OS | beliebig gemischt | Host-Kernel nötig |
| Typischer Einsatz | DB, Storage, Latenz | AD, Legacy, Mischbetrieb | Microservices, CI/CD |

**Entscheidungslogik für die Prüfung**
1. Anderer Kernel oder volle Isolation nötig? → **VM**.
2. Viele gleiche, kurzlebige Dienste und schnelles Deployment? → **Container**.
3. Maximale I/O-Leistung oder Spezialhardware? → **Bare Metal**.

**Wichtig:** Container sind **keine** kleinen VMs. Ohne zusätzliche Maßnahmen wie Namespaces, cgroups, seccomp und Rootless ist die Isolation schwächer. Ein Kernel-Bug betrifft alle Container gleichzeitig.

**Mischbetrieb:** Container in VMs ist üblich – du bekommst die Dichte der Container und die Isolation der VM.

## Prüfungsfrage

**Frage:** Ein Kunde will 20 Instanzen einer Alpine-basierten Webanwendung möglichst platzsparend und schnell skalierbar betreiben. Empfiehl eine Technik und begründe.

**Musterantwort:** Container, weil alle Instanzen denselben Kernel nutzen und nur die Anwendung samt Bibliotheken mitbringen. 20 VMs würden 20 Gastbetriebssysteme und 20 Kernel benötigen, also deutlich mehr RAM, Storage und Startzeit. Für stärkere Isolation kann man die Container in VMs betreiben.

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
- [BIOS/UEFI-Kernel](bios-uefi-kernel.md) – Kernel und User Space
- [Snapshots-Backups](snapshots-backups.md) – Sicherung von VMs
