# Hypervisoren

Ein Hypervisor verwaltet VMs und verteilt Hardware-Ressourcen wie CPU, RAM, Storage und Netzwerk.

## Typen

**Typ 1 – Bare Metal**
- Läuft direkt auf der Hardware.
- Einsatz: Rechenzentren und produktive Server.
- Beispiele: VMware ESXi, Microsoft Hyper-V, KVM.

**Typ 2 – Hosted**
- Läuft als Anwendung auf einem Host-Betriebssystem.
- Einsatz: Entwicklung, Tests und Schulungen.
- Beispiele: VirtualBox, VMware Workstation.

| Kriterium | Typ 1 | Typ 2 |
|---|---|---|
| Overhead | Typischerweise geringer | Typischerweise höher |
| Host-OS | Kein allgemeines Host-OS erforderlich | Erforderlich |
| Einsatz | Produktivbetrieb | Lokale Tests |

**Wichtig:** KVM nutzt den Linux-Kernel als Virtualisierungsschicht und wird üblicherweise als Typ 1 eingeordnet, obwohl es auf Linux basiert.

## Ressourcenverwaltung

### CPU Contention

Mehrere VMs benötigen gleichzeitig mehr CPU-Leistung, als physisch verfügbar ist. Der Hypervisor verteilt die CPU-Zeit; dadurch entstehen Wartezeiten und höhere Latenzen.

**Lösung:** CPU-Auslastung analysieren, Überbelegung reduzieren und kritische Workloads priorisieren.

### Memory Ballooning

Ein Balloon-Treiber innerhalb einer VM gibt ungenutzten RAM an den Hypervisor zurück, damit dieser ihn anderen VMs zuweisen kann.

**Nachteil:** Gibt die VM zu viel Speicher frei, kann ihre Performance sinken.

### Memory Sharing

Identische Speicherseiten mehrerer VMs werden gemeinsam genutzt, sofern die Virtualisierungsplattform dies unterstützt.

**Vorteil:** Geringerer physischer RAM-Verbrauch.

**Risiko:** Zusätzlicher Verwaltungsaufwand; sicherheitsrelevante Auswirkungen hängen von der Implementierung ab.

### Swapping

Reicht der physische RAM nicht aus, werden Speicherseiten auf ein Laufwerk ausgelagert.

**Problem:** Storage ist deutlich langsamer als RAM. Starkes Swapping verursacht hohe Latenzen und Performance-Einbrüche.

**Wichtig:** Unterscheide zwischen Swapping im Gastbetriebssystem und Swapping durch den Hypervisor.

## Weitere relevante Konzepte

### Overcommitment

Virtuellen Maschinen werden insgesamt mehr Ressourcen zugesichert, als physisch vorhanden sind.

- **CPU:** Kann funktionieren, da VMs selten dauerhaft volle CPU-Leistung benötigen.
- **RAM:** Riskanter, weil gleichzeitiger Speicherbedarf zu Ballooning, Swapping oder Speichermangel führen kann.

### CPU Ready Time

Zeit, in der eine VM CPU-Zeit benötigt, aber auf die Zuteilung durch den Hypervisor warten muss.

**Prüfungsbezug:** Hohe CPU-Auslastung und CPU Contention können die Ready Time erhöhen.

### Thin vs. Thick Provisioning

- **Thin:** Virtueller Datenträger belegt physischen Speicher erst bei tatsächlicher Nutzung. Spart zunächst Storage, birgt aber das Risiko eines vollen Datastores.
- **Thick:** Speicher wird vorab vollständig oder weitgehend reserviert. Planbarer, aber höherer initialer Speicherbedarf.

### Hochverfügbarkeit (HA)

Bei einem Host-Ausfall werden betroffene VMs auf anderen Hosts neu gestartet, sofern die Infrastruktur dies unterstützt.

**Voraussetzungen:** Ausreichende Ersatzkapazität, geeignete Speicher- und Netzwerkinfrastruktur.

**Abgrenzung:** HA bedeutet nicht automatisch unterbrechungsfreien Betrieb.

### Live-Migration

Eine laufende VM wird auf einen anderen Host verschoben, normalerweise mit nur kurzer Unterbrechung.

**Voraussetzungen:** Kompatible Hosts, Netzwerkverbindung und geeignete Speicherarchitektur.

### Snapshots

Speichern den Zustand einer VM zu einem bestimmten Zeitpunkt.

**Wichtig:** Snapshots sind kein Ersatz für Backups. Lange bestehende Snapshots können zusätzlichen Storage und I/O verursachen.

## Fehlerbilder zuordnen

| Symptom | Mögliche Ursache |
|---|---|
| VM reagiert langsam trotz geringer eigener CPU-Auslastung | CPU Contention / hohe CPU Ready Time |
| RAM knapp, andere VMs benötigen Speicher | Ballooning / Overcommitment |
| Hohe Storage-Latenz bei Speichermangel | Swapping |
| Datastore läuft voll | Thin Provisioning / große Snapshots |
| VM fällt bei Host-Ausfall aus | Fehlende oder fehlerhafte HA-Konfiguration |

## Verknüpfungen

- [Cloud Computing](cloud-computing.md) – Ressourcenpooling und Skalierung
- [VMs, Container und Bare Metal](vm-container-server.md) – Architekturvergleich
- [Schutzziele](schutzziele.md) – Verfügbarkeit und Integrität

## IHK-Merksatz

Virtualisierung verbessert die Ressourcenauslastung, garantiert aber keine höhere Performance. Overcommitment, CPU Contention und Speichermangel können zu erheblichen Leistungseinbußen führen. Monitoring und Kapazitätsplanung verhindern Engpässe.
