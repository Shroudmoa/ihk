# IT

> Themen vernetzt für die IHK-AP2. Fokus: Verstehen statt Auswendiglernen.

## Lernpfad für Einsteiger
1. **Grundlagen:** [CPU](IT/cpu.md) → [Speicherhirachie](IT/speicherhirachie.md) → [BIOS/UEFI-Kernel](IT/bios-uefi-kernel.md)
2. **Netzwerkverständnis:** [OSI](IT/osi.md) → [MAC](IT/mac.md) → [IPv4](IT/ipv4.md) → [IPv6](IT/ipv6.md)
3. **Sicherheit & Zugriff:** [Authentifizierung-Autorisierung-MFA](IT/authentifizierung-autoritisierung-mfa.md) → [RBAC](IT/rbac.md) → [LDAP](IT/ldap.md) → [Active-Directory](IT/active-directory.md)
4. **Verfügbarkeit & Backup:** [RAID](IT/raid.md) → [Snapshots-Backups](IT/snapshots-backups.md) → [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md)
5. **Virtualisierung & Cloud:** [VM-Container-Server](IT/vm-container-server.md) → [Hypervisor-Typen](IT/hypervisor-typen.md) → [Cloud_computing](IT/cloud-computing.md)

## Grundlagen & Hardware (8)

| Thema | In einem Satz | Verknüpfung |
|---|---|---|
| [CPU](IT/cpu.md) | Kerne, Threads, Takt, Cache – warum GHz allein nichts aussagt. | [Speicherhirachie](IT/speicherhirachie.md), [BIOS/UEFI-Kernel](IT/bios-uefi-kernel.md) |
| [USB-SSD-HDD](IT/usb-ssd-hdd.md) | Schnittstellen, Geschwindigkeit und Einsatz von HDD, SSD und NVMe. | [Speicherhirachie](IT/speicherhirachie.md), [RAID](IT/raid.md), [Snapshots-Backups](IT/snapshots-backups.md) |
| [Bit-Byte](IT/bit-byte.md) | Bit, Byte, KB vs. KiB – die Umrechnungsfalle in Aufgaben. | [IPv4](IT/ipv4.md) (Subnetting), [IPv6](IT/ipv6.md) |
| [Speicherhirachie](IT/speicherhirachie.md) | Register bis Band: Geschwindigkeit, Kosten, Flüchtigkeit. | [CPU](IT/cpu.md), [RAID](IT/raid.md), [DAS-NAS-SAN](IT/das-nas-san.md) |
| [chmod](IT/chmod.md) | rwx, Oktalzahlen und die 755/644-Frage. | [Least Privilege](IT/least-privlage.md), [RBAC](IT/rbac.md) |
| [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md) | Verfügbarkeit durch Redundanz – Failover, Cluster, RTO/RPO. | [RAID](IT/raid.md), [Snapshots-Backups](IT/snapshots-backups.md), [SLA](IT/sla.md) |
| [LDAP](IT/ldap.md) | Lightweight Directory Access Protocol für zentrale Verzeichnisdienste. | [Active-Directory](IT/active-directory.md), [Authentifizierung-Autoritisierung-MFA](IT/authentifizierung-autoritisierung-mfa.md), [RBAC](IT/rbac.md) |
| [BIOS-UEFI](IT/bios-uefi.md) | Firmware, Secure Boot, TPM – Boot-Root-of-Trust. | [BIOS/UEFI-Kernel](IT/bios-uefi-kernel.md), [Bitlocker](IT/bitlocker.md) |

## Netzwerk (10)

| Thema | In einem Satz | Verknüpfung |
|---|---|---|
| [OSI](IT/osi.md) | Die 7 Schichten und wo Switch, Router und Proxy hingehören. | [MAC](IT/mac.md), [IPv4](IT/ipv4.md), [Firewall](IT/firewall.md), [Proxy](IT/proxy.md) |
| [MAC](IT/mac.md) | 48 Bit, OUI, ARP – und warum MAC-Filter keine Sicherheit sind. | [OSI](IT/osi.md), [Netzwerktypen](IT/netzwerktypen.md), [VLAN](IT/vlan.md) |
| [Netzwerktypen](IT/netzwerktypen.md) | PAN bis GAN und welche Risiken pro Größe dazukommen. | [OSI](IT/osi.md), [Firewall](IT/firewall.md), [DAS-NAS-SAN](IT/das-nas-san.md) |
| [Firewall](IT/firewall.md) | Regelwerke lesen und schreiben, Stateful vs. NGFW, DMZ. | [OSI](IT/osi.md), [Proxy](IT/proxy.md), [Netzwerktypen](IT/netzwerktypen.md) |
| [IPv6](IT/ipv6.md) | 128 Bit, SLAAC, NDP – und was du von IPv4 nicht übernehmen darfst. | [IPv4](IT/ipv4.md), [OSI](IT/osi.md), [NAT/PAT](IT/nat-pat.md) |
| [Proxy](IT/proxy.md) | Forward, Reverse und Port Forwarding in einer Tabelle. | [Firewall](IT/firewall.md), [OSI](IT/osi.md), [DMZ](IT/firewall.md) |
| [IPv4](IT/ipv4.md) | Subnetting, CIDR und die 2-Host-Regel – kommt in jeder GA2. | [IPv6](IT/ipv6.md), [OSI](IT/osi.md), [VLAN](IT/vlan.md), [NAT/PAT](IT/nat-pat.md) |
| [Active-Directory](IT/active-directory.md) | Microsoft-Verzeichnisdienst, Kerberos, Gruppenrichtlinien. | [LDAP](IT/ldap.md), [RBAC](IT/rbac.md), [Kerberos](IT/kerberos.md), [GPO](IT/gpo.md) |
| [RBAC](IT/rbac.md) | Role-Based Access Control – Rollen statt direkter Berechtigungen. | [Least Privilege](IT/least-privlage.md), [Active-Directory](IT/active-directory.md), [LDAP](IT/ldap.md) |
| [Ports](IT/ports.md) | Wichtige Ports, Protokolle und Dienste. | [Firewall](IT/firewall.md), [OSI](IT/osi.md), [IPv4](IT/ipv4.md) |

## Sicherheit & Datenschutz (13)

| Thema | In einem Satz | Verknüpfung |
|---|---|---|
| [DSGVO & TOM](IT/dsgvo-tom.md) | Rechtsgrundlagen, Betroffenenrechte und die TOM, die die Prüfung sehen will. | [Schutzziele](IT/schutzziele.md), [Bitlocker](IT/bitlocker.md) |
| [Schutzziele](IT/schutzziele.md) | Vertraulichkeit, Integrität, Verfügbarkeit – und die zwei, die alle vergessen. | [DSGVO & TOM](IT/dsgvo-tom.md), [RAID](IT/raid.md), [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md) |
| [hackers-can](IT/hackers-can.md) | Die Angriffe aus den Prüfungsszenarien, jeweils mit Gegenmaßnahme. | [Firewall](IT/firewall.md), [OS-Härtung](IT/os-haertung.md), [Schutzziele](IT/schutzziele.md) |
| [Symmetrische-und-Asymmetrische](IT/symmetrische-und-asymmetrische.md) | Wann symmetrisch, wann asymmetrisch – und warum TLS beides nutzt. | [PKI](IT/pki.md), [Hash](IT/hash.md), [PGP](IT/pgp.md) |
| [Least Privilege](IT/least-privlage.md) | Minimalrechtprinzip – nur benötigte Rechte. | [RBAC](IT/rbac.md), [chmod](IT/chmod.md), [OS-Härtung](IT/os-haertung.md) |
| [Authentifizierung-Autoritisierung-MFA](IT/authentifizierung-autoritisierung-mfa.md) | Authentifizierung vs. Autorisierung, MFA-Varianten. | [LDAP](IT/ldap.md), [Active-Directory](IT/active-directory.md), [Kerberos](IT/kerberos.md), [RBAC](IT/rbac.md) |
| [OS-Härtung](IT/os-haertung.md) | Betriebssystem absichern – Firewalls, Updates, Rechte. | [Least Privilege](IT/least-privlage.md), [Patchmanagment-TU-SU-PU](IT/patchmanagment-tu-su-pu.md), [hackers-can](IT/hackers-can.md) |
| [Patchmanagment-TU-SU-PU](IT/patchmanagment-tu-su-pu.md) | Test-, Sicherheits-, Produktionsumfeld und Patch-Prozesse. | [OS-Härtung](IT/os-haertung.md), [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md) |
| [AAA-SIEM](IT/aaa-siem.md) | Authentication, Authorization, Accounting und Sicherheitsüberwachung. | [Authentifizierung-Autoritisierung-MFA](IT/authentifizierung-autoritisierung-mfa.md), [Firewall](IT/firewall.md), [RADIUS/TACACS](IT/aaa-siem.md) |
| [Bitlocker](IT/bitlocker.md) | TPM-basierte Festplattenverschlüsselung. | [BIOS-UEFI](IT/bios-uefi.md), [Schutzziele](IT/schutzziele.md) |
| [Hash](IT/hash.md) | Einwegfunktionen, Hashwerte, Integritätsprüfung. | [Symmetrische-und-Asymmetrische](IT/symmetrische-und-asymmetrische.md), [PKI](IT/pki.md), [PGP](IT/pgp.md) |
| [PKI](IT/pki.md) | Public Key Infrastructure, Zertifikate, CAs, TLS. | [Symmetrische-und-Asymmetrische](IT/symmetrische-und-asymmetrische.md), [Hash](IT/hash.md), [Kerberos](IT/kerberos.md) |
| [PGP](IT/pgp.md) | Pretty Good Privacy – E-Mail-Verschlüsselung und Signaturen. | [Symmetrische-und-Asymmetrische](IT/symmetrische-und-asymmetrische.md), [PKI](IT/pki.md), [Hash](IT/hash.md) |

## Virtualisierung & Server (4)

| Thema | In einem Satz | Verknüpfung |
|---|---|---|
| [VM-Container-Server](IT/vm-container-server.md) | Bare Metal, VM, Container: Isolation, Dichte, Startzeit. | [Hypervisor-Typen](IT/hypervisor-typen.md), [BIOS/UEFI-Kernel](IT/bios-uefi-kernel.md), [Live-Migration-VMs](IT/live-migration-vms.md) |
| [Hypervisor-Typen](IT/hypervisor-typen.md) | Typ 1 vs. Typ 2 und wann welcher in der Aufgabe passt. | [VM-Container-Server](IT/vm-container-server.md), [BIOS/UEFI-Kernel](IT/bios-uefi-kernel.md) |
| [BIOS/UEFI-Kernel](IT/bios-uefi-kernel.md) | Bootkette, Kernel Space vs. User Space, System Calls. | [BIOS-UEFI](IT/bios-uefi.md), [CPU](IT/cpu.md), [VM-Container-Server](IT/vm-container-server.md) |
| [Live-Migration-VMs](IT/live-migration-vms.md) | VMs ohne Ausfall verschieben: Ablauf, Voraussetzungen, Grenzen. | [VM-Container-Server](IT/vm-container-server.md), [Hypervisor-Typen](IT/hypervisor-typen.md), [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md) |

## Storage & Backup (3)

| Thema | In einem Satz | Verknüpfung |
|---|---|---|
| [Snapshots-Backups](IT/snapshots-backups.md) | 3-2-1-1-0, Backup-Arten und warum Snapshots kein Backup sind. | [RAID](IT/raid.md), [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md), [DAS-NAS-SAN](IT/das-nas-san.md) |
| [RAID](IT/raid.md) | Level, nutzbare Kapazität und Ausfalltoleranz – inklusive Rechenweg. | [Speicherhirachie](IT/speicherhirachie.md), [Snapshots-Backups](IT/snapshots-backups.md), [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md) |
| [DAS-NAS-SAN](IT/das-nas-san.md) | Block, Datei, Objekt – wer was nutzt und wo der Unterschied Punkte bringt. | [Speicherhirachie](IT/speicherhirachie.md), [RAID](IT/raid.md), [Netzwerktypen](IT/netzwerktypen.md) |

## Cloud & Wirtschaftlichkeit (4)

| Thema | In einem Satz | Verknüpfung |
|---|---|---|
| [Cloud_computing](IT/cloud-computing.md) | IaaS bis SaaS, Skalierung und die Kostenfalle Lock-in. | [VM-Container-Server](IT/vm-container-server.md), [CAPEX-OPEX](IT/capex-opex.md), [SLA](IT/sla.md) |
| [CAPEX-OPEX](IT/capex-opex.md) | Investition vs. Betriebskosten – die Rechnung hinter Cloud-Entscheidungen. | [Cloud_computing](IT/cloud-computing.md), [TCO](IT/tco.md), [Amortisation](IT/amortisation.md) |
| [SLA](IT/sla.md) | Verfügbarkeit in Prozent, Reaktionszeit und was ein SLA nicht verspricht. | [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md), [Cloud_computing](IT/cloud-computing.md) |
| [Skalierbarkeit-Elastizität](IT/skalierbarkeit-elastizitaet.md) | Vertikal vs. horizontal – und was elastisch wirklich heißt. | [Cloud_computing](IT/cloud-computing.md), [VM-Container-Server](IT/vm-container-server.md) |

## Referenz - Projektmanagement, Programmierung & Wirtschaft (15)

| Thema | In einem Satz | Verknüpfung |
|---|---|---|
| [Abkürzungen](IT/abkuerzungen.md) | Nachschlagen, nicht lernen: die Abkürzungen der Prüfung. | Alle Themen |
| [Fachbegriffe](IT/fachbegriffe.md) | Slang und Architektur-Begriffe, die in Aufgaben vorkommen. | Alle Themen |
| [Pseudocode](IT/Pseudocode/readme.md) | **NEU:** Prüfungsvorbereitung für Pseudocode/C#-Aufgaben (Arrays, Schleifen, Fehlersuche). | [Analyse Prüfungen - Programmierung](IT/analyse-der-exams/04_Programmierung_Grundlagen.md) |
| [AOB](IT/aob.md) | Aufwandsorientierte Budgetierung. | [Aufwandsschätzung](IT/aufwandsschaetzung.md), [Ressourcenplan](IT/ressourcenplan.md) |
| [Netzplan](IT/netzplan.md) | Vorgangspfeil-Netzplan, kritischer Pfad. | [PERT](IT/pert.md), [Gantt-Diagramm](IT/gantt-diagramm.md) |
| [Gantt-Diagramm](IT/gantt-diagramm.md) | Balkendiagramm zur Terminplanung. | [Netzplan](IT/netzplan.md), [Ressourcenplan](IT/ressourcenplan.md) |
| [Projektstruckturplan-PSP](IT/projektstruckturplan-psp.md) | Arbeitspakete, Gliederungstiefe. | [Aufwandsschätzung](IT/aufwandsschaetzung.md), [Netzplan](IT/netzplan.md) |
| [Aufwandsschätzung](IT/aufwandsschaetzung.md) | Schätzverfahren für Projekte. | [PERT](IT/pert.md), [Projektstruckturplan-PSP](IT/projektstruckturplan-psp.md), [AOB](IT/aob.md) |
| [PERT](IT/pert.md) | Program Evaluation Review Technique – Drei-Zeiten-Methode. | [Netzplan](IT/netzplan.md), [Aufwandsschätzung](IT/aufwandsschaetzung.md) |
| [Ressourcenplan](IT/ressourcenplan.md) | Personelle und materielle Ressourcen. | [Gantt-Diagramm](IT/gantt-diagramm.md), [AOB](IT/aob.md) |
| [Soll-Ist-Vergleich](IT/soll-ist-vergleich.md) | Abweichungsanalyse im Projektcontrolling. | [Ressourcenplan](IT/ressourcenplan.md), [Amortisation](IT/amortisation.md) |
| [Amortisation](IT/amortisation.md) | Amortisationszeit einer Investition. | [CAPEX-OPEX](IT/capex-opex.md), [TCO](IT/tco.md), [BEP](IT/bep.md) |
| [TCO](IT/tco.md) | Total Cost of Ownership – Gesamtkosten über Lebenszyklus. | [CAPEX-OPEX](IT/capex-opex.md), [Cloud_computing](IT/cloud-computing.md), [Amortisation](IT/amortisation.md) |
| [BEP](IT/bep.md) | Break-Even-Point – Gewinnschwelle. | [Amortisation](IT/amortisation.md), [TCO](IT/tco.md) |
| [Stundensatz-(intern)](IT/stundensatz-intern.md) | Kalkulation interner Stundensätze. | [CAPEX-OPEX](IT/capex-opex.md), [Aufwandsschätzung](IT/aufwandsschaetzung.md) |
