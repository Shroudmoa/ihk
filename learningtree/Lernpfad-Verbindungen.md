# Lernpfad & Verbindungen

Ein schneller Überblick über die wichtigsten thematischen Verbindungen für die IHK-AP2.

## 1) Hardware & Bootprozess
```text
BIOS/UEFI → Bootloader (GRUB/Windows Boot Manager) → Kernel (Kernel Space/User Space) → Betriebssystem
          ↓
         TPM/Secure Boot → Bitlocker
```
- [BIOS-UEFI](IT/bios-uefi.md) | [BIOS/UEFI-Kernel](IT/bios-uefi-kernel.md) | [Bitlocker](IT/bitlocker.md)

## 2) CPU, Speicher & Performance
```text
CPU (Kerne/Threads/Cache) → Speicherhierarchie (Register → Cache → RAM → Storage)
                        ↓
                       Performance: IPC, Latenz vs. Bandbreite
```
- [CPU](IT/cpu.md) | [Speicherhirachie](IT/speicherhirachie.md)

## 3) Netzwerkgrundlagen
```text
OSI-Modell (7 Schichten)
    ├─ L1: Hub/Repeater
    ├─ L2: Switch/MAC/ARP/VLAN
    ├─ L3: Router/IPv4/IPv6/NAT
    ├─ L4: TCP/UDP/Ports
    └─ L7: HTTP/DNS/Proxy/Firewall (NGFW)
```
- [OSI](IT/osi.md) | [MAC](IT/mac.md) | [IPv4](IT/ipv4.md) | [IPv6](IT/ipv6.md) | [Ports](IT/ports.md) | [Firewall](IT/firewall.md) | [Proxy](IT/proxy.md) | [VLAN](IT/vlan.md)

## 4) Zugriff, Identität & Berechtigungen
```text
Authentifizierung (wer?) → Autorisierung (darf?) → Accountability (wer hat was getan?)
    ↓                        ↓                        ↓
   MFA/Passwörter         RBAC/Least Privilege      AAA/Logging/SIEM
    ↓                        ↓
   LDAP/AD/Kerberos       Gruppen/Rechte
```
- [Authentifizierung-Autoritisierung-MFA](IT/authentifizierung-autoritisierung-mfa.md) | [RBAC](IT/rbac.md) | [Least Privilege](IT/least-privlage.md) | [LDAP](IT/ldap.md) | [Active-Directory](IT/active-directory.md) | [Kerberos](IT/kerberos.md) | [AAA-SIEM](IT/aaa-siem.md)

## 5) Kryptografie
```text
Symmetrisch (schnell) + Asymmetrisch (Schlüsselverteilung)
                ↓
               TLS
                ↓
               PKI (CA, Zertifikate)
                ↓
               Hash (Integrität)
                ↓
               PGP (Ende-zu-Ende)
```
- [Symmetrische-und-Asymmetrische](IT/symmetrische-und-asymmetrische.md) | [PKI](IT/pki.md) | [Hash](IT/hash.md) | [PGP](IT/pgp.md)

## 6) Verfügbarkeit, Backup & Storage
```text
Speicher (DAS/NAS/SAN) + RAID → Verfügbarkeit
                            ↓
                          HA (Cluster/Failover/Loadbalancer)
                            ↓
                    Backup (3-2-1-1-0) ≠ Snapshots
                            ↓
                          DR (RTO/RPO)
```
**Wichtiger Grundsatz:** RAID = Ausfallsicherheit, Backup = Datensicherung!
- [DAS-NAS-SAN](IT/das-nas-san.md) | [RAID](IT/raid.md) | [Snapshots-Backups](IT/snapshots-backups.md) | [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md) | [SLA](IT/sla.md)

## 7) Virtualisierung, Cloud & Wirtschaftlichkeit
```text
Bare Metal → Hypervisor (Typ1/Typ2) → VM
                ↓                        ↓
              Container                Live-Migration
                ↓
               Cloud (IaaS/PaaS/SaaS)
                ↓
           Skalierbar/Elastisch
                ↓
           CAPEX/OPEX, TCO, Amortisation, SLA
```
- [VM-Container-Server](IT/vm-container-server.md) | [Hypervisor-Typen](IT/hypervisor-typen.md) | [Live-Migration-VMs](IT/live-migration-vms.md) | [Cloud_computing](IT/cloud-computing.md) | [Skalierbarkeit-Elastizität](IT/skalierbarkeit-elastizitaet.md) | [CAPEX-OPEX](IT/capex-opex.md) | [TCO](IT/tco.md) | [Amortisation](IT/amortisation.md)

## 8) Projektmanagement
```text
PSP → Netzplan (kritischer Pfad) → Gantt
 ↓           ↓
Aufwand    Ressourcenplan
 ↓
PERT/AOB/Stundensatz
 ↓
Controlling: Soll-Ist, Amortisation, BEP
```
- [Projektstruckturplan-PSP](IT/projektstruckturplan-psp.md) | [Netzplan](IT/netzplan.md) | [PERT](IT/pert.md) | [Gantt-Diagramm](IT/gantt-diagramm.md) | [Ressourcenplan](IT/ressourcenplan.md) | [Aufwandsschätzung](IT/aufwandsschaetzung.md)

## Schnelle Prüfungsfallen
- **RAID vs Backup:** Niemals gleichsetzen!
- **Hub vs Switch:** Hub=L1, Switch=L2
- **Router vs Switch:** Router=L3 (IP), Switch=L2 (MAC)
- **2^Hostbits-2:** Netz- & Broadcast nicht mitzählen
- **GHz vs Leistung:** IPC + Kerne + Cache sind entscheidend
- **NAT filtert nicht:** NAT übersetzt Adressen, Firewall filtert
- **TLS ist keine OSI-Schicht:** Liegt zwischen 4 und 7
- **Privates IPv4:** 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16 (172.32+ ist NICHT privat)
- **Authentifizierung vs Autorisierung:** Wer bin ich? vs. Was darf ich?
- **VM vs Container:** VM virtualisiert Hardware, Container virtualisiert OS
