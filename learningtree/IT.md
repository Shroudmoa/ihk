# IT

> Lernblätter für IHK AP2 FiSi AO2020

**Hinweis:** Die Prüfungen selber + Antworten sind unter [https://github.com/Shroudmoa/IHK-Abschlusspruefungen-AP2-FiSi-AO2020](https://github.com/Shroudmoa/IHK-Abschlusspruefungen-AP2-FiSi-AO2020) erhältlich.

## Grundlagen & Hardware (9)

| Thema | In einem Satz | Verknüpfung |
|---|---|---|
| [CPU](IT/cpu.md) | Kerne, Threads, Takt, Cache – warum GHz allein nichts aussagt. | [Speicherhirachie](IT/speicherhirachie.md), [Virtualisierung](IT/virtualisierung.md) |
| [USB-SSD-HDD](IT/usb-ssd-hdd.md) | Schnittstellen, Geschwindigkeit und Einsatz von HDD, SSD und NVMe. | [Speicherhirachie](IT/speicherhirachie.md), [DAS-NAS-SAN](IT/das-nas-san.md) |
| [Bit-Byte](IT/bit-byte.md) | Bit, Byte, KB vs. KiB – die Umrechnungsfalle in Aufgaben. | [Abkürzungen](IT/abkuerzungen.md) |
| [Speicherhirachie](IT/speicherhirachie.md) | Register bis Band: Geschwindigkeit, Kosten, Flüchtigkeit. | [CPU](IT/cpu.md), [USB-SSD-HDD](IT/usb-ssd-hdd.md) |
| [chmod](IT/chmod.md) | rwx, Oktalzahlen und die 755/644-Frage. | [Least Privilege](IT/least-privlage.md), [OS-Härtung](IT/os-haertung.md) |
| [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md) | RTO/RPO, USV-Typen, HA vs DR, SPOF vermeiden. | [RAID](IT/raid.md), [Snapshots-Backups](IT/snapshots-backups.md), [Load Balancer](IT/load-balancer.md), [Cluster](IT/cluster.md) |
| [LDAP](IT/ldap.md) | Lightweight Directory Access Protocol für zentrale Verzeichnisdienste. | [Active-Directory](IT/active-directory.md), [Authentifizierung-Autoritisierung-MFA](IT/authentifizierung-autoritisierung-mfa.md), [Kerberos](IT/kerberos.md) |
| [BIOS-UEFI](IT/bios-uefi.md) | Bootvorgang, Secure Boot, Firmware vs UEFI. | [BIOS/UEFI-Kernel](IT/bios-uefi-kernel.md), [Bitlocker](IT/bitlocker.md) |
| [Virtualisierung](IT/virtualisierung.md) | Grundlagen der Virtualisierung, Vorteile/Nachteile. | [Hypervisor-Typen](IT/hypervisor-typen.md), [VM-Container-Server](IT/vm-container-server.md) |

## Netzwerk (23)

| Thema | In einem Satz | Verknüpfung |
|---|---|---|
| [OSI](IT/osi.md) | Die 7 Schichten und wo Switch, Router und Proxy hingehören. | [MAC](IT/mac.md), [IPv4](IT/ipv4.md), [Ports](IT/ports.md), [Firewall](IT/firewall.md), [Proxy](IT/proxy.md) |
| [MAC](IT/mac.md) | 48 Bit, OUI, ARP – und warum MAC-Filter keine Sicherheit sind. | [OSI](IT/osi.md), [Netzwerktypen](IT/netzwerktypen.md), [Switches und VLANs](IT/switches-und-vlans.md) |
| [Netzwerktypen](IT/netzwerktypen.md) | PAN bis GAN und welche Risiken pro Größe dazukommen. | [OSI](IT/osi.md), [Firewall](IT/firewall.md), [DAS-NAS-SAN](IT/das-nas-san.md), [WLAN](IT/wlan.md) |
| [Firewall](IT/firewall.md) | Regelwerke lesen und schreiben, Stateful vs. NGFW, DMZ. | [OSI](IT/osi.md), [Proxy](IT/proxy.md), [Netzwerktypen](IT/netzwerktypen.md), [DDoS](IT/ddos.md) |
| [IPv6](IT/ipv6.md) | 128 Bit, SLAAC, NDP – und was du von IPv4 nicht übernehmen darfst. | [IPv4](IT/ipv4.md), [OSI](IT/osi.md), [NAT/PAT](IT/nat-pat.md), [Subnetting IPv4 & IPv6](IT/subnetting-ipv4-ipv6.md) |
| [Proxy](IT/proxy.md) | Forward, Reverse und Port Forwarding in einer Tabelle. | [Firewall](IT/firewall.md), [OSI](IT/osi.md), [DMZ](IT/firewall.md) |
| [IPv4](IT/ipv4.md) | Subnetting, CIDR und die 2-Host-Regel – kommt in jeder GA2. | [IPv6](IT/ipv6.md), [OSI](IT/osi.md), [Switches und VLANs](IT/switches-und-vlans.md), [NAT/PAT](IT/nat-pat.md), [Subnetting IPv4 & IPv6](IT/subnetting-ipv4-ipv6.md) |
| [Active-Directory](IT/active-directory.md) | Microsoft-Verzeichnisdienst, Kerberos, Gruppenrichtlinien. | [LDAP](IT/ldap.md), [RBAC](IT/rbac.md), [Kerberos](IT/kerberos.md), [GPO](IT/gpo.md) |
| [RBAC](IT/rbac.md) | Role-Based Access Control – Rollen statt direkter Berechtigungen. | [Least Privilege](IT/least-privlage.md), [Active-Directory](IT/active-directory.md), [LDAP](IT/ldap.md) |
| [Ports](IT/ports.md) | Wichtige Ports, Protokolle und Dienste. | [Firewall](IT/firewall.md), [OSI](IT/osi.md), [IPv4](IT/ipv4.md), [DNS](IT/dns.md) |
| [Subnetting IPv4 & IPv6](IT/subnetting-ipv4-ipv6.md) | CIDR, Blockgrößen, SLAAC und IPv6-Adressierung. | [IPv4](IT/ipv4.md), [IPv6](IT/ipv6.md), [Routing](IT/routing.md) |
| [DHCP](IT/dhcp.md) | DORA-Vorgang, Lease, DHCP-Relay, DHCP-Snooping. | [DNS](IT/dns.md), [Subnetting IPv4 & IPv6](IT/subnetting-ipv4-ipv6.md), [Switches und VLANs](IT/switches-und-vlans.md) |
| [DNS](IT/dns.md) | Namensauflösung, A/AAAA/MX/CNAME, DNSSEC, DoH/DoT. | [DHCP](IT/dhcp.md), [Ports](IT/ports.md), [Firewall](IT/firewall.md) |
| [Routing](IT/routing.md) | Statisch/Dynamisch, OSPF/RIP/BGP, Routingtabelle, Longest Prefix Match. | [Subnetting IPv4 & IPv6](IT/subnetting-ipv4-ipv6.md), [OSI](IT/osi.md), [Firewall](IT/firewall.md), [Load Balancer](IT/load-balancer.md) |
| [Switches und VLANs](IT/switches-und-vlans.md) | Access/Trunk, 802.1Q, Inter-VLAN-Routing, STP/RSTP. | [Routing](IT/routing.md), [MAC](IT/mac.md), [Netzwerktypen](IT/netzwerktypen.md) |
| [Firewalls (alle Generationen) & Sicherheitskonzepte (Antivirus)](IT/firewalls-alle-generationen-sicherheitskonzepte-antivirus.md) | Paketfilter bis NGFW, Zonen, DMZ, IDS/IPS, EDR. | [Firewall](IT/firewall.md), [VPN](IT/vpn.md), [OS-Härtung](IT/os-haertung.md), [DDoS](IT/ddos.md) |
| [VPN](IT/vpn.md) | Remote-Access/Site-to-Site, IPSec, SSL/TLS-VPN, WireGuard. | [Firewall](IT/firewall.md), [Routing](IT/routing.md), [PKI](IT/pki.md), [WLAN](IT/wlan.md) |
| [WLAN](IT/wlan.md) | 802.11-Standards, 2,4/5/6GHz, WPA2/WPA3, 802.1X. | [Netzwerktypen](IT/netzwerktypen.md), [Authentifizierung-Autoritisierung-MFA](IT/authentifizierung-autoritisierung-mfa.md), [VPN](IT/vpn.md) |
| [Netzwerkanalyse und Troubleshooting](IT/netzwerkanalyse-und-troubleshooting.md) | OSI-orientiert, Ping/Traceroute/NSLookup/Wireshark, systematisch. | [OSI](IT/osi.md), [Routing](IT/routing.md), [DNS](IT/dns.md), [DHCP](IT/dhcp.md), [Ports](IT/ports.md) |
| [Load Balancer](IT/load-balancer.md) | Lastverteilung, Algorithmen, Layer 4/7, Health Checks, VIP. | [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md), [Routing](IT/routing.md), [Firewall](IT/firewall.md), [DDoS](IT/ddos.md), [Skalierbarkeit-Elastizität](IT/skalierbarkeit-elastizitaet.md) |
| [DDoS](IT/ddos.md) | DoS/DDoS, Angriffsarten, Botnet, Gegenmaßnahmen – Verfügbarkeitsangriff. | [Load Balancer](IT/load-balancer.md), [Firewall](IT/firewall.md), [Schutzziele](IT/schutzziele.md), [Netzwerkanalyse und Troubleshooting](IT/netzwerkanalyse-und-troubleshooting.md) |
| [Blue-Green Deployment](IT/blue-green-deployment.md) | Zero-Downtime-Deployment, Umschaltung, schneller Rollback. | [Load Balancer](IT/load-balancer.md), [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md), [Skalierbarkeit-Elastizität](IT/skalierbarkeit-elastizitaet.md) |

## Sicherheit & Datenschutz (22)

| Thema | In einem Satz | Verknüpfung |
|---|---|---|
| [DSGVO & TOM](IT/dsgvo-tom.md) | Rechtsgrundlagen, Betroffenenrechte und die TOM, die die Prüfung sehen will. | [TOMs](IT/toms.md), [DBS-Datenschutzbeauftragten](IT/dbs-datenschutzbeauftragten.md), [SCC-Datenpanne](IT/scc-datenpanne.md) |
| [Schutzziele](IT/schutzziele.md) | Vertraulichkeit, Integrität, Verfügbarkeit – CIA. | [RAID](IT/raid.md), [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md), [DDoS](IT/ddos.md) |
| [hackers-can](IT/hackers-can.md) | Angriffe aus Prüfungsszenarien mit passenden Gegenmaßnahmen. | [Firewalls (alle Generationen) & Sicherheitskonzepte (Antivirus)](IT/firewalls-alle-generationen-sicherheitskonzepte-antivirus.md), [OS-Härtung](IT/os-haertung.md), [DDoS](IT/ddos.md) |
| [Symmetrische-und-Asymmetrische](IT/symmetrische-und-asymmetrische.md) | Wann symmetrisch, wann asymmetrisch – warum TLS beides nutzt. | [PKI](IT/pki.md), [Hash](IT/hash.md), [PGP](IT/pgp.md) |
| [Least Privilege](IT/least-privlage.md) | Minimalrechtprinzip – nur benötigte Rechte. | [RBAC](IT/rbac.md), [chmod](IT/chmod.md), [OS-Härtung](IT/os-haertung.md) |
| [Authentifizierung-Autoritisierung-MFA](IT/authentifizierung-autoritisierung-mfa.md) | Authentifizierung vs. Autorisierung, MFA-Varianten. | [LDAP](IT/ldap.md), [Active-Directory](IT/active-directory.md), [Kerberos](IT/kerberos.md), [RBAC](IT/rbac.md) |
| [OS-Härtung](IT/os-haertung.md) | Betriebssystem absichern – Updates, Rechte, Firewalls. | [Least Privilege](IT/least-privlage.md), [Patchmanagment-TU-SU-PU](IT/patchmanagment-tu-su-pu.md), [hackers-can](IT/hackers-can.md) |
| [Patchmanagment-TU-SU-PU](IT/patchmanagment-tu-su-pu.md) | Test-/Sicherheits-/Produktionsumfeld, Patch-Prozesse. | [OS-Härtung](IT/os-haertung.md), [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md) |
| [AAA-SIEM](IT/aaa-siem.md) | Authentication, Authorization, Accounting & Sicherheitsüberwachung. | [Authentifizierung-Autoritisierung-MFA](IT/authentifizierung-autoritisierung-mfa.md), [Firewalls (alle Generationen) & Sicherheitskonzepte (Antivirus)](IT/firewalls-alle-generationen-sicherheitskonzepte-antivirus.md) |
| [Bitlocker](IT/bitlocker.md) | TPM-basierte Festplattenverschlüsselung. | [BIOS-UEFI](IT/bios-uefi.md), [Schutzziele](IT/schutzziele.md) |
| [Hash](IT/hash.md) | Einwegfunktionen, Hashwerte, Integritätsprüfung. | [Symmetrische-und-Asymmetrische](IT/symmetrische-und-asymmetrische.md), [PKI](IT/pki.md), [PGP](IT/pgp.md) |
| [PKI](IT/pki.md) | Public Key Infrastructure, Zertifikate, CAs, TLS. | [Symmetrische-und-Asymmetrische](IT/symmetrische-und-asymmetrische.md), [Hash](IT/hash.md), [VPN](IT/vpn.md) |
| [PGP](IT/pgp.md) | Pretty Good Privacy – E-Mail-Verschlüsselung & Signaturen. | [Symmetrische-und-Asymmetrische](IT/symmetrische-und-asymmetrische.md), [PKI](IT/pki.md), [Hash](IT/hash.md) |
| [TOMs](IT/toms.md) | Technische und organisatorische Maßnahmen nach Art. 32 DSGVO. | [DSGVO & TOM](IT/dsgvo-tom.md), [Schutzziele](IT/schutzziele.md) |
| [DBS-Datenschutzbeauftragten](IT/dbs-datenschutzbeauftragten.md) | Aufgaben, Benennungspflicht, Stellung des DSB. | [DSGVO & TOM](IT/dsgvo-tom.md), [TOMs](IT/toms.md) |
| [SCC-Datenpanne](IT/scc-datenpanne.md) | Datenpanne, Meldepflicht 72h, Standard Contractual Clauses. | [DSGVO & TOM](IT/dsgvo-tom.md), [DBS-Datenschutzbeauftragten](IT/dbs-datenschutzbeauftragten.md) |
| [KI in IT - Vergleich mit klassisch](IT/ki-in-it-vergleich-mit-klassisch.md) | Einsatz, Chancen, Risiken, verantwortungsvoller Einsatz. | [Netzwerkanalyse und Troubleshooting](IT/netzwerkanalyse-und-troubleshooting.md), [OS-Härtung](IT/os-haertung.md) |
| [MDM, BYOD & Mobile Device Management](IT/mdm-byod.md) | BYOD/CYOD/COPE, Containerisierung, Full/Selective Wipe. | [Authentifizierung-Autoritisierung-MFA](IT/authentifizierung-autoritisierung-mfa.md), [Least Privilege](IT/least-privlage.md), [VPN](IT/vpn.md), [DSGVO & TOM](IT/dsgvo-tom.md) |

## Virtualisierung & Server (5)

| Thema | In einem Satz | Verknüpfung |
|---|---|---|
| [Virtualisierung](IT/virtualisierung.md) | Grundlagen, Ziele, Vorteile/Nachteile der Virtualisierung. | [Hypervisor-Typen](IT/hypervisor-typen.md), [VM-Container-Server](IT/vm-container-server.md) |
| [VM-Container-Server](IT/vm-container-server.md) | Bare Metal, VM, Container: Isolation, Dichte, Startzeit, Kernel-Unterschied. | [Hypervisor-Typen](IT/hypervisor-typen.md), [Virtualisierung](IT/virtualisierung.md), [Skalierbarkeit-Elastizität](IT/skalierbarkeit-elastizitaet.md) |
| [Hypervisor-Typen](IT/hypervisor-typen.md) | Typ 1 (Bare-Metal) vs. Typ 2 (Hosted) – Einsatz & Overhead. | [Virtualisierung](IT/virtualisierung.md), [VM-Container-Server](IT/vm-container-server.md), [Live-Migration-VMs](IT/live-migration-vms.md) |
| [BIOS/UEFI-Kernel](IT/bios-uefi-kernel.md) | Bootkette, Kernel Space vs. User Space, System Calls. | [BIOS-UEFI](IT/bios-uefi.md), [OS-Härtung](IT/os-haertung.md) |
| [Live-Migration-VMs](IT/live-migration-vms.md) | VMs ohne Ausfall verschieben: Ablauf, Voraussetzungen, Grenzen. | [Hypervisor-Typen](IT/hypervisor-typen.md), [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md), [Cluster](IT/cluster.md) |

## Storage & Backup (3)

| Thema | In einem Satz | Verknüpfung |
|---|---|---|
| [Snapshots-Backups](IT/snapshots-backups.md) | 3-2-1-1-0, Voll/Differenziell/Inkrementell, warum Snapshots kein Backup sind. | [RAID](IT/raid.md), [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md), [DAS-NAS-SAN](IT/das-nas-san.md) |
| [RAID](IT/raid.md) | Level 0/1/5/6/10, nutzbare Kapazität, Ausfalltoleranz, Rechenweg. | [Snapshots-Backups](IT/snapshots-backups.md), [Schutzziele](IT/schutzziele.md), [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md), [DAS-NAS-SAN](IT/das-nas-san.md) |
| [DAS-NAS-SAN](IT/das-nas-san.md) | Block/Datei/Objekt, Direkt-/Netzwerkspeicher, Einsatzgebiete. | [RAID](IT/raid.md), [Snapshots-Backups](IT/snapshots-backups.md), [Netzwerktypen](IT/netzwerktypen.md) |

## Cloud & Wirtschaftlichkeit (4)

| Thema | In einem Satz | Verknüpfung |
|---|---|---|
| [Cloud_computing](IT/cloud-computing.md) | IaaS/PaaS/SaaS + FaaS/CaaS/DBaaS/STaaS/NaaS, Public/Private/Hybrid, CAPEX→OPEX. | [Virtualisierung](IT/virtualisierung.md), [Skalierbarkeit-Elastizität](IT/skalierbarkeit-elastizitaet.md), [CAPEX-OPEX](IT/capex-opex.md), [SLA](IT/sla.md) |
| [CAPEX-OPEX](IT/capex-opex.md) | Investition vs. Betriebskosten – die Rechnung hinter Cloud-Entscheidungen. | [Cloud_computing](IT/cloud-computing.md), [TCO](IT/tco.md), [Amortisation](IT/amortisation.md) |
| [SLA](IT/sla.md) | Verfügbarkeit in Prozent, Reaktionszeit und was ein SLA nicht verspricht. | [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md), [Cloud_computing](IT/cloud-computing.md), [TCO](IT/tco.md) |
| [Skalierbarkeit-Elastizität](IT/skalierbarkeit-elastizitaet.md) | Vertikal vs. horizontal, Scale-Up/Out/In/Down – und was elastisch wirklich heißt. | [Cloud_computing](IT/cloud-computing.md), [Load Balancer](IT/load-balancer.md), [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md), [VM-Container-Server](IT/vm-container-server.md) |

## Referenz - Projektmanagement, Programmierung, UML & Wirtschaft (16)

| Thema | In einem Satz | Verknüpfung |
|---|---|---|
| [Abkürzungen](IT/abkuerzungen.md) | Nachschlagen, nicht lernen: die Abkürzungen der Prüfung. | Alle Themen |
| [Fachbegriffe](IT/fachbegriffe.md) | Slang und Architektur-Begriffe, die in Aufgaben vorkommen. | Alle Themen |
| [Pseudocode](IT/Pseudocode/readme.md) | Prüfungsvorbereitung für Pseudocode/C#-Aufgaben (Arrays, Schleifen, Fehlersuche). | [Analyse Prüfungen - Programmierung](IT/analyse-der-exams/04_Programmierung_Grundlagen.md) |
| [UML](IT/UML/readme.md) | UML-Diagramme für AP2 - Klassendiagramm, Use-Case, Aktivitätsdiagramm, Sequenzdiagramm. | [Analyse Prüfungen - Datenmodellierung/SQL/UML](IT/analyse-der-exams/L14_Datenmodellierung_SQL_UML.md) |
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
