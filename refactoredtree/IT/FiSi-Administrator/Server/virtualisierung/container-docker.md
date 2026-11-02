# Container und Docker

## In 30 Sekunden

- Ein **Container** ist ein isolierter Prozess mit eigener Dateisystemsicht, eigenen Bibliotheken und eigenem Netz, aber **ohne eigenen Kernel**. Er teilt sich den Kernel des Hosts.
- **Image** = unveränderliche Vorlage aus Schichten (Layers). **Container** = laufende Instanz eines Images.
- **Dockerfile** beschreibt den Bau eines Images, die **Registry** (z. B. Docker Hub, Harbor) speichert Images.
- **Orchestrierung** (Kubernetes) verteilt viele Container auf mehrere Hosts, startet ausgefallene neu und skaliert.
- Technik unter der Haube (Linux): **Namespaces** (Isolation), **cgroups** (Ressourcenlimits).
- Geprüft: Containerisierung als Alternative zur Virtualisierung 2023W, Vergleich VM/Container in L13.

## Kernwissen

### Begriffe

| Begriff | Bedeutung |
|---|---|
| **Image** | schreibgeschützte Vorlage: Basis-OS-Dateien, Laufzeit, Anwendung, Konfiguration. Aus Schichten aufgebaut, gleiche Schichten werden geteilt |
| **Container** | laufende Instanz eines Images mit dünner beschreibbarer Schicht. Wird gelöscht, sind deren Änderungen weg |
| **Dockerfile** | Bauanleitung (`FROM`, `COPY`, `RUN`, `EXPOSE`, `CMD`) |
| **Registry** | Ablage für Images, öffentlich oder privat |
| **Volume** | dauerhafter Speicher außerhalb des Containers (Datenbanken, Uploads) |
| **Port-Mapping** | `-p 8080:80` → Host-Port 8080 wird auf Container-Port 80 weitergeleitet |
| **Docker Compose** | mehrere Container gemeinsam per YAML-Datei starten (z. B. Web + DB) |
| **Kubernetes (K8s)** | Orchestrierung: Pods, Deployments, Self-Healing, Auto-Scaling, Rolling Updates |

### Beispiel

```dockerfile
FROM nginx:1.27-alpine
COPY ./webseite/ /usr/share/nginx/html/
EXPOSE 80
```

```bash
docker build -t firmen-web:1.0 .
docker run -d --name web -p 8080:80 firmen-web:1.0
docker ps                # laufende Container
docker logs web          # Ausgabe ansehen
docker stop web && docker rm web
```

### Container vs. VM (Kurzfassung)

| Kriterium | VM | Container |
|---|---|---|
| eigener Kernel | ja | nein (Host-Kernel) |
| Größe | GB | MB |
| Start | Sekunden bis Minuten | Millisekunden bis Sekunden |
| Isolation | stark | schwächer (gemeinsamer Kernel) |
| anderes Betriebssystem | möglich (Windows-VM auf Linux-Host) | nicht möglich (Linux-Container braucht Linux-Kernel) |

Ausführlicher Vergleich mit Bare Metal: [VM-Container-Server](vm-container-server.md).

### Vor- und Nachteile

| Vorteile | Nachteile |
|---|---|
| geringer Ressourcenbedarf, hohe Dichte | schwächere Isolation, ein Kernel-Fehler trifft alle Container |
| „läuft überall gleich“ (Entwicklung = Produktion) | Daten sind ohne Volume flüchtig |
| schnelles Skalieren und Ausrollen (CI/CD, Microservices) | Orchestrierung ist komplex (Kubernetes) |
| unveränderliche Images, einfache Rollbacks | Images aus öffentlichen Registries können Schwachstellen oder Malware enthalten |

### Sicherheit

- Nur vertrauenswürdige, **signierte** Basis-Images verwenden und regelmäßig neu bauen (Patches).
- Images auf Schwachstellen scannen (z. B. Trivy).
- Container **nicht als root** laufen lassen, Ressourcenlimits setzen, nur nötige Ports öffnen.
- Keine Passwörter ins Image schreiben → Secrets-Verwaltung.

## Prüfungsfrage

**Frage:** Statt weiterer VMs schlägt ein Kollege Containerisierung für die Webanwendung vor. Erläutern Sie zwei Vorteile und einen Nachteil gegenüber virtuellen Maschinen.

**Musterantwort:** Container teilen sich den Kernel des Hosts und enthalten nur Anwendung und Bibliotheken. Sie brauchen deshalb deutlich weniger RAM und Speicher als VMs, von denen jede ein komplettes Gastbetriebssystem mitbringt. Auf derselben Hardware laufen so mehr Instanzen. Außerdem starten Container in Sekunden und lassen sich aus demselben Image identisch und schnell ausrollen und skalieren. Nachteil: Die Isolation ist schwächer, weil alle Container denselben Kernel nutzen. Eine Kernel-Schwachstelle kann alle Container auf dem Host gefährden, und andere Betriebssysteme (z. B. Windows-Anwendungen auf einem Linux-Host) lassen sich nicht betreiben.

## Falle

- „Container = kleine VM“. Es gibt **keinen Gast-Kernel** und keinen Hypervisor.
- Docker selbst als Hypervisor bezeichnen. Es ist eine Container-Engine bzw. Container-Runtime.
- Datenbankdaten im Container ohne Volume speichern. Beim Neuerstellen ist alles weg.
- Live-Migration erwarten. Container werden in der Regel auf einem anderen Host **neu gestartet**.

## Merksatz

**„Image ist das Rezept, Container ist der gebackene Kuchen. Alle Kuchen stehen auf demselben Kernel-Ofen.“**

## Verwandt

- [VM-Container-Server](vm-container-server.md) – ausführlicher Vergleich
- [Virtualisierung](virtualisierung.md)
- [Hypervisor-Typen](hypervisor-typen.md)
- [Skalierbarkeit-Elastizität](skalierbarkeit-elastizitaet.md)
- [Cloud Computing](../Cloud/cloud-computing.md) – CaaS, Kubernetes-Dienste
- [Blue-Green-Deployment](../blue-green-deployment.md)
- [BIOS/UEFI-Kernel](../bios-uefi-kernel.md) – Kernel und User Space
- [Lektion L13 Virtualisierung](../../../analyse-der-exams/L13_Virtualisierung.md)
