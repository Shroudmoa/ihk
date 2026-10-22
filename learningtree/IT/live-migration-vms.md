# Live-Migration-VMs

# Live Migration – Laufende VMs ohne spürbare Unterbrechung verschieben

## Was ist Live Migration?

Live Migration ist das Verschieben einer **laufenden virtuellen Maschine (VM)** von einem physischen Host auf einen anderen, ohne das Gastbetriebssystem herunterzufahren. Der Arbeitsspeicher und der aktuelle Laufzeitzustand werden zum Zielhost übertragen. Dadurch bleibt ein Dienst normalerweise erreichbar; eine kurze Unterbrechung beim Umschalten kann trotzdem auftreten.

**Beispiele für Technologien:**
- **VMware vSphere vMotion** – Migration zwischen ESXi-Hosts.
- **Microsoft Hyper-V Live Migration** – Migration zwischen Hyper-V-Hosts.
- **KVM/QEMU Live Migration** – Migration zwischen kompatiblen KVM-Hosts, häufig mit libvirt verwaltet.

Quelle: [Microsoft Learn – Live Migration Overview](https://learn.microsoft.com/en-us/windows-server/virtualization/hyper-v/manage/live-migration-overview)

## Real-Life-Anwendungen

### Wartung eines Servers ohne geplanten Dienstausfall

Ein Unternehmen betreibt auf Host A mehrere VMs:
- eine VM für Active Directory und DNS
- einen Webserver
- einen Datenbankserver
- einen Fileserver

Host A benötigt ein Firmware-Update oder neue RAM-Module. Statt alle VMs herunterzufahren, werden sie auf Host B migriert. Danach kann Host A gewartet werden. Anschließend können die VMs zurückverschoben werden.

**Nutzen:** weniger geplante Ausfallzeit und flexiblere Wartungsfenster.

### Lastverteilung im Rechenzentrum

Host A ist stark ausgelastet, während Host B noch freie CPU- und RAM-Kapazität hat. Das Virtualisierungsmanagement verschiebt eine oder mehrere VMs auf Host B.

**Nutzen:** bessere Ressourcenverteilung und weniger Engpässe. Eine Migration selbst benötigt allerdings CPU-, RAM- und Netzwerkressourcen.

### Rechenzentrum erweitern oder umbauen

Ein Unternehmen ersetzt ältere Hosts durch neue Server. VMs können – sofern die Kompatibilität und die Infrastruktur stimmen – auf die neuen Hosts verschoben werden, ohne jede Anwendung einzeln neu installieren zu müssen.

### Cloud- und Hosting-Betrieb

Cloud- und Hosting-Anbieter verwalten viele virtuelle Server auf einer gemeinsamen Infrastruktur. Migrationen helfen bei Wartung, Kapazitätsplanung und Lastverteilung. Ob eine konkrete Cloud-Plattform Live Migration anbietet, hängt vom jeweiligen Dienst ab.

---

## Wie funktioniert Live Migration technisch?

Ein Beispiel: Eine VM mit 16 GB RAM läuft auf Host A und soll auf Host B wechseln.

<box gap={2}>
  <box border radius="lg" padding={3} gap={1}>
    **1. Zielhost prüfen**

    Host B muss kompatibel sein und genug CPU-, RAM- und gegebenenfalls Speicherressourcen besitzen. Netzwerk, Berechtigungen und Migrationsverbindung werden geprüft.
  </box>
  <box align="center">
    <icon name="arrow-down" size="lg" />
  </box>
  <box border radius="lg" padding={3} gap={1}>
    **2. VM-Zustand übertragen**

    Der Quellhost überträgt den Speicherinhalt der VM über das Netzwerk. Bei Pre-Copy-Verfahren läuft die VM währenddessen weiter. Speicherseiten, die erneut geändert werden, können nochmals übertragen werden.
  </box>
  <box align="center">
    <icon name="arrow-down" size="lg" />
  </box>
  <box border radius="lg" padding={3} gap={1}>
    **3. Kurzes Umschalten**

    Wenn nur noch wenige Änderungen offen sind, wird die VM kurz angehalten. Der letzte CPU- und Gerätezustand sowie verbleibende Speicherseiten werden übertragen.
  </box>
  <box align="center">
    <icon name="arrow-down" size="lg" />
  </box>
  <box border radius="lg" padding={3} gap={1}>
    **4. VM auf Host B fortsetzen**

    Host B setzt die VM fort. Das virtuelle Netzwerk muss die VM weiterhin korrekt erreichbar machen. Der alte Host gibt die VM-Ressourcen frei.
  </box>
</box>

### Was wird übertragen?

- **RAM:** laufender Speicherinhalt und Änderungen während der Übertragung.
- **CPU-Zustand:** Register und Ausführungszustand der VM beim Umschalten.
- **Virtuelle Geräte:** Zustand virtueller Netzwerk- und anderer Geräte.
- **Speicher:** abhängig von der Migrationsart entweder bereits gemeinsam verfügbar oder zusätzlich zu übertragen.

Wichtig: Bei einer reinen **Compute-/Live Migration** bleiben die virtuellen Festplatten häufig auf gemeinsamem Storage. Bei einer **Storage Migration** werden die virtuellen Festplatten zusätzlich an einen anderen Speicherort verschoben. Manche Plattformen können beides kombinieren.

## Voraussetzungen

| Voraussetzung | Warum? |
|---|---|
| Kompatible Virtualisierung | Quelle und Ziel müssen die VM-Konfiguration und deren virtuelle Hardware unterstützen. |
| Ausreichend RAM | Der Zielhost benötigt genug freien Arbeitsspeicher für die VM. |
| Ausreichende CPU-Kapazität | Der Zielhost muss die zusätzliche Rechenlast übernehmen können. |
| CPU-Kompatibilität | Die CPU-Funktionen müssen kompatibel sein; unterschiedliche Generationen können Einschränkungen verursachen. |
| Netzwerkverbindung | RAM und Laufzeitzustand müssen zuverlässig übertragen werden. |
| Storage-Zugriff | Die VM-Festplatten müssen am Ziel verfügbar sein oder mitmigriert werden. |
| Netzwerk-Konfiguration | VLANs, virtuelle Switches, IP-Erreichbarkeit und Sicherheitsregeln müssen passen. |
| Authentifizierung und Berechtigungen | Die Hosts müssen die Migration sicher autorisieren können. |
| Passende Software und Lizenzen | Hypervisor-Version, Konfiguration und gegebenenfalls Lizenz müssen die Funktion unterstützen. |

Nicht jede Umgebung braucht einen Cluster oder Shared Storage zwingend. Die genauen Anforderungen hängen von Hypervisor, Version und gewählter Migrationsart ab.

Quelle: [Microsoft Learn – Voraussetzungen und Konfiguration](https://learn.microsoft.com/en-us/windows-server/virtualization/hyper-v/manage/use-live-migration-without-failover-clustering-to-move-a-virtual-machine)

## Shared Storage vs. lokaler Storage

### Shared Storage

Beide Hosts greifen auf denselben zentralen Speicher zu, zum Beispiel über SAN, NAS oder einen passenden Cluster-Speicher.

- Die VM-Dateien müssen beim Wechsel nicht vollständig kopiert werden.
- Die Migration betrifft vor allem RAM und Laufzeitzustand.
- Der gemeinsame Speicher muss erreichbar und korrekt konfiguriert sein.

### Lokaler Storage

Die virtuellen Festplatten liegen direkt auf dem Quellhost. Wenn der Zielhost keinen Zugriff darauf hat, müssen die VM-Dateien ebenfalls übertragen werden.

- Mehr Datenverkehr und längere Migrationsdauer möglich.
- Ausreichende Bandbreite und freier Speicherplatz erforderlich.
- Je nach Plattform wird dies als Storage Migration oder kombinierte Migration durchgeführt.

## Wie lange dauert eine Live Migration?

Die Dauer hängt unter anderem ab von:
- RAM-Größe der VM
- Netzwerkbandbreite und Latenz
- CPU- und Netzwerkauslastung
- Häufigkeit, mit der die VM ihren RAM verändert
- Größe der zu übertragenden virtuellen Festplatten

Beispiel: Eine VM mit 16 GiB RAM benötigt bei idealen 10 Gbit/s für die reine Übertragung ungefähr 14 Sekunden. Protokoll-Overhead, erneute Übertragungen, Storage-Daten und Umschaltvorgang können die tatsächliche Zeit deutlich erhöhen.

Eine hohe Änderungsrate im RAM kann die Migration verlangsamen, weil Speicherseiten wiederholt übertragen werden müssen.

## Vorteile und Nachteile

| Vorteile | Nachteile |
|---|---|
| Weniger geplante Ausfallzeit | Benötigt Netzwerkbandbreite |
| Wartung ohne reguläres Herunterfahren der VM | Zusätzliche CPU- und RAM-Last |
| Lastverteilung zwischen Hosts | Kompatibilität und Konfiguration müssen stimmen |
| Flexiblere Hardwarewechsel | Gemeinsamer Storage kann ein zentraler Engpass sein |
| Kann mit Clustering und Hochverfügbarkeit kombiniert werden | Keine Garantie gegen jeden Ausfall |

## Live Migration ist nicht dasselbe wie Failover

**Live Migration:** Ein Administrator oder das Management verschiebt eine noch laufende VM kontrolliert auf einen anderen Host.

**Failover:** Ein Host fällt aus und eine Hochverfügbarkeitslösung startet oder übernimmt die VM auf einem anderen Host. Dabei kann es zu einer Unterbrechung kommen, weil der ursprüngliche Host nicht mehr normal mitarbeitet.

**Cluster:** Eine Gruppe von Hosts, die Ressourcen und gegebenenfalls Ausfallsicherheit gemeinsam verwaltet. Ein Cluster kann Live Migration und automatisches Failover unterstützen, aber beide Funktionen sind unterschiedlich.

**Snapshot:** Ein gespeicherter VM-Zustand beziehungsweise eine Änderungsbasis. Ein Snapshot ist weder eine Live Migration noch automatisch ein Backup.

## Sicherheit und Betrieb

- Migrationsverkehr sollte über ein dafür vorgesehenes, geschütztes Netzwerk geführt werden.
- Authentifizierung und Berechtigungen müssen restriktiv konfiguriert sein.
- Bei sensiblen Daten sind Vertraulichkeit und angemessener Schutz des Übertragungswegs wichtig.
- Monitoring sollte CPU, RAM, Netzwerkauslastung und Migrationsfehler erfassen.
- Ein Wartungsplan braucht weiterhin eine Rückfallstrategie; Live Migration ersetzt kein Backup.

**DSGVO-Bezug:** Werden personenbezogene Daten übertragen, müssen Vertraulichkeit, Integrität und Verfügbarkeit angemessen geschützt werden. Dazu gehören passende Zugriffskontrollen und Sicherheitsmaßnahmen nach Art. 32 DSGVO. Live Migration allein erfüllt diese Anforderungen nicht automatisch.

## IHK-Merksatz

Live Migration verschiebt eine laufende VM zwischen kompatiblen Hosts, indem RAM und Laufzeitzustand übertragen und die VM anschließend am Ziel fortgesetzt werden. Voraussetzungen sind unter anderem kompatible Hosts, ausreichende Ressourcen, eine sichere Netzwerkverbindung sowie passender Zugriff auf den virtuellen Speicher. Typische Einsatzfälle sind Serverwartung und Lastverteilung; im Gegensatz zum Failover erfolgt die Migration normalerweise geplant und kontrolliert.

## Quellen

- [Microsoft Learn – Live Migration Overview](https://learn.microsoft.com/en-us/windows-server/virtualization/hyper-v/manage/live-migration-overview)
- [Microsoft Learn – Live Migration ohne Failover Clustering](https://learn.microsoft.com/en-us/windows-server/virtualization/hyper-v/manage/use-live-migration-without-failover-clustering-to-move-a-virtual-machine)
- [Broadcom – CPU-Kompatibilität bei VMware vMotion](https://knowledge.broadcom.com/external/article/316536/migration-to-a-different-processor.html)

## Verwandt

- [Hypervisor-Typen](hypervisor-typen.md) – Typ-1-Hypervisoren im Rechenzentrum
- [VM-Container-Server](vm-container-server.md) – VM vs. Container
- [Snapshots-Backups](snapshots-backups.md) – Rollback und Sicherung

