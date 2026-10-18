# Lektion 13 – Virtualisierung (Hypervisor & Container)

**Bereich:** GA1 (Konzeption und Administration von IT-Systemen)
**Relevanz:** hoch – Virtualisierung ist Standard in Rechenzentren und regelmäßig Prüfungsthema (Vergleich, Vor-/Nachteile)
**Lernzeit:** ca. 2h

---

## 1. Warum das wichtig ist

Virtualisierung spart Hardware, erleichtert Migration und Disaster Recovery und ist die Basis für Cloud. Die AP2 prüft typisch: Unterschied Hypervisor Typ 1/2, Vor-/Nachteile und das Abgrenzen von Containern zu VMs.

> ✅ **Verifiziert am echten Prüfungskorpus:** Virtualisierung kam in
> **3 von 10 Jahrgängen** vor: **2022S, 2023W, 2025S**. **2022S und 2025S**
> nutzten dabei eine **identische Typ-1/Typ-2-Tabellenaufgabe** (Vergleich
> Bare-Metal vs. gehostet). **2023W** bezeichnete den Hypervisor explizit als
> **„Virtual-Machine-Monitor"** und stellte **Containerisierung als
> Alternative** zur klassischen Virtualisierung gegenüber.

## 2. Grundlagen – Hypervisor

Ein **Hypervisor** (auch **Virtual-Machine-Monitor**, VMM, so explizit
benannt in **2023W**) erstellt und verwaltet virtuelle Maschinen (VMs) und
teilt die physischen Ressourcen (CPU, RAM, Netz) auf.

### Typ 1 – Bare-Metal (läuft direkt auf der Hardware)
- Kein Host-Betriebssystem nötig, sehr effizient und performant
- Beispiele: **VMware ESXi, Microsoft Hyper-V Server, KVM (Linux), Proxmox VE**

### Typ 2 – Gehostet (läuft auf einem Betriebssystem)
- Hypervisor ist eine normale Anwendung auf einem OS (Windows/macOS/Linux)
- Beispiele: **Oracle VirtualBox, VMware Workstation, VMware Player**

| Kriterium | Typ 1 (Bare-Metal) | Typ 2 (gehostet) |
|---|---|---|
| Layers | Hardware → Hypervisor → VM | Hardware → OS → Hypervisor → VM |
| Performance | hoch (geringer Overhead) | geringer (durch Host-OS) |
| Einsatz | Server / Rechenzentrum | Desktop / Test / Schulung |
| Isolation | stark | schwächer (abhängig vom Host-OS) |
| Beispiele | ESXi, Hyper-V Server, KVM, Proxmox | VirtualBox, VMware Workstation |

## 3. Vor- und Nachteile der Virtualisierung

**Vorteile:**
- **Konsolidierung:** mehrere Server auf einer physischen Maschine → weniger Hardware, Strom, Platz
- **Snapshots:** Zustandsabbild zum Wiederherstellen/Testen
- **Migration / HA / Live-Migration:** VM kann auf anderen Host verschoben werden (kein Stillstand)
- **Isolation:** Ausfall/Fehler einer VM beeinflusst andere kaum

**Nachteile:**
- **Ressourcen-Overhead:** durch Abstraktionsschicht etwas weniger Performance als Bare-Metal
- **Single Point of Failure:** fällt der Host aus, sind **alle** darauf laufenden VMs betroffen → Hochverfügbarkeit (Cluster) nötig

## 4. Container (Docker) vs. VM

**2023W** stellte die **Containerisierung explizit als Alternative** zur
klassischen (Hypervisor-basierten) Virtualisierung gegenüber.

| Merkmal | Virtuelle Maschine (VM) | Container (Docker) |
|---|---|---|
| Enthält | eigenes **Gast-Betriebssystem** | nutzt den **Host-Kernel** (geteilt) |
| Overhead | hoch (komplettes OS) | gering (nur App + Libs) |
| Startzeit | Sekunden–Minuten | Millisekunden–Sekunden |
| Isolation | stark (vollständig getrennt) | schwächer (Kernel geteilt) |
| Image-Größe | GB | MB |

Kurz: Container sind leichtgewichtig und schnell, aber weniger isoliert; VMs sind schwerer, dafür stärker getrennt und flexibler bei unterschiedlichen OS.

## 5. Übungsaufgabe (im Prüfungsstil, selbst erstellt)

> Ein Unternehmen betreibt 20 virtuelle Server auf einem einzigen ESXi-Host.
>
> a) Um welchen Hypervisor-Typ handelt es sich und warum?
> b) Nennen Sie zwei Vorteile dieser Konsolidierung und einen entscheidenden Nachteil.
> c) Das Unternehmen möchte eine Anwendung isoliert, aber mit minimalem Ressourcenbedarf ausrollen. Würden Sie eher eine VM oder einen Container empfehlen? Begründen Sie kurz.

<details>
<summary>Lösung (aufklappen)</summary>

a) **Typ 1 (Bare-Metal)**, da ESXi direkt auf der Hardware läuft und kein eigenes Host-Betriebssystem benötigt.

b) Vorteile: **Konsolidierung** (weniger Hardware/Strom/Platz) und einfache **Migration/Snapshots/HA**. Nachteil: **Single Point of Failure** – Host-Ausfall trifft alle 20 VMs gleichzeitig.

c) Eher einen **Container**, da er den Host-Kernel teilt, sehr wenig Overhead hat und schnell startet; die geringere Isolation ist bei einer einzelnen Anwendung meist vertretbar.

</details>

## 6. Selbstcheck – kannst du das?

- [ ] Ich kann Hypervisor Typ 1 und Typ 2 benennen und je 2 Beispiele nennen
- [ ] Ich kann die Typ-1/Typ-2-Tabelle (Performance, Einsatz, Isolation) erklären
- [ ] Ich kenne mindestens 3 Vorteile und 2 Nachteile der Virtualisierung
- [ ] Ich kann VM und Container anhand von Kernel/Overhead/Isolation unterscheiden
- [ ] Ich weiß, warum ein Host-Ausfall ein Single Point of Failure ist

## 7. Weiterführende Ressourcen

- **Docker Docs** (docs.docker.com) – offizielle Einführung in Container
- **Proxmox VE / KVM Dokumentation** – Typ-1-Hypervisor in der Praxis
- **VMware ESXi** – Datenblatt und Vergleiche zu Typ 1
- Altklausuren in deinem Repo (GA1-Aufgaben zu "Server-Virtualisierung", "Konsolidierung")

---
*Nächste Lektion: Datenmodellierung, SQL & UML*
