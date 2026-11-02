# AP2 FiSi – Lernpfad & Struktur (Master-Index)

Zentrale Übersicht über alle Lernunterlagen im Ordner `Tuts/`.
Die Struktur folgt deiner `V1/List.md` (Themenliste aus den Altklausuren
2021–2026) und ordnet **jedem 🔴/🟡-Thema eine Lektion** zu.

**Legende:** 🔴 = fast jeder Termin · 🟡 = regelmäßig · 🟢 = vereinzelt
**Status:** ✅ vorhanden · 🆕 neu ergänzt · ➡️ weiterführend

---

## So nutzt du den Lernpfad

1. Lies zuerst `V1/00-ressourcen-und-guideline.md` (Leitfaden & Ressourcen).
2. Arbeite die Lektionen **nach Priorität** ab – zuerst alle 🔴-Themen.
3. Jede Lektion hat einen **Selbstcheck** und eine **Übungsaufgabe** (Lösung
   per `<details>` aufklappbar). Altklausuren aus dem Repo (`*/GA*_SI_*.pdf`)
   zum Üben nutzen.
4. Lernfortschritt in den Checklisten abhaken und committen.

---

## GA1 – Konzeption und Administration von IT-Systemen

| Thema | Prio | Lektion |
|-------|------|---------|
| Cloud-Computing (SaaS/PaaS/IaaS, Public/Private) | 🔴 | [01_Cloud_Computing.md](01_Cloud_Computing.md) |
| Speicher & Storage: RAID 0/1/5/6/10, Kapazität berechnen | 🔴 | [02_Speicher_Backup_RAID.md](02_Speicher_Backup_RAID.md) |
| Backup-Strategien (Voll/Diff/Inkr, GVS, 3-2-1, D2D2T) + MTBF/MTTF | 🔴 | [L12_Backup_Strategien.md](L12_Backup_Strategien.md) 🆕 |
| Virtualisierung (Hypervisor Typ 1/2, Container) | 🔴 | [L13_Virtualisierung.md](L13_Virtualisierung.md) 🆕 |
| Verschlüsselung & Hashing (AES/RSA/MD5/SHA) | 🔴 | [V1/02-verschluesselung-hashing.md](../V1/02-verschluesselung-hashing.md) |
| Datenschutz & Informationssicherheit (DSGVO, TOM, Malware, Phishing) | 🔴 | [05_Datenschutz_Informationssicherheit.md](05_Datenschutz_Informationssicherheit.md) |
| Verfügbarkeit & Ausfallsicherheit (CIA, USV, Cluster) | 🟡 | [03_Verfuegbarkeit_Ausfallsicherheit.md](03_Verfuegbarkeit_Ausfallsicherheit.md) |
| Programmierung: Datentypen, Arrays, Struct, Bit-Bedarf | 🔴 | [04_Programmierung_Grundlagen.md](04_Programmierung_Grundlagen.md) |
| SQL & UML & ER-Modell | 🔴 | [L14_Datenmodellierung_SQL_UML.md](L14_Datenmodellierung_SQL_UML.md) 🆕 |
| Lizenzierung (User/Device CAL) | 🟡 | [10_Lizenzierung.md](10_Lizenzierung.md) |

## GA2 – Analyse und Entwicklung von Netzwerken

| Thema | Prio | Lektion |
|-------|------|---------|
| IPv4-Adressierung & Subnetting (inkl. VLSM) | 🔴 | [V1/01-subnetting-ipv4.md](../V1/01-subnetting-ipv4.md) |
| IPv4 Routing, Broadcast, Unicast/Multicast, ARP, NAT | 🔴 | [06_IPv4_Adressierung_Routing.md](06_IPv4_Adressierung_Routing.md) |
| IPv6 (Subnetting, SLAAC, DHCPv6) | 🟡 | [08_IPv6.md](08_IPv6.md) |
| VLANs, Trunk/Access, Router-on-a-Stick, STP, LinkAgg, FHRP | 🔴 | [L15_VLAN_Routing.md](L15_VLAN_Routing.md) 🆕 |
| DHCP (DORA) & DNS (Auflösung, Split-Horizon) | 🔴 | [L17_DHCP_DNS.md](L17_DHCP_DNS.md) 🆕 |
| Firewall (Paketfilter, SPI/NGFW, Regeln, Port-Security) | 🔴 | [L16_Firewall.md](L16_Firewall.md) 🆕 |
| Fehlersuche (ipconfig, APIPA, DNS/DHCP-Fehler) | 🔴 | [07_Netzwerk_Fehlersuche.md](07_Netzwerk_Fehlersuche.md) |
| WLAN (WPA2/3, Gast-VLAN, Scan, Voucher) | 🟡 | [09_WLAN.md](09_WLAN.md) |
| Verfügbarkeits-/Redundanzberechnung (99,x %, Parallel) | 🟡 | [L18_Netzwerk_Berechnungen.md](L18_Netzwerk_Berechnungen.md) 🆕 |

## WISO – Wirtschafts- und Sozialkunde

| Thema | Prio | Lektion |
|-------|------|---------|
| BBiG: Ausbildungsende, Weiterbeschäftigung (§21/§24) | 🔴 | [V1/03-wiso-arbeitsrecht.md](../V1/03-wiso-arbeitsrecht.md) |
| Kündigungsfristen (§622 BGB, Probezeit) | 🔴 | [V1/03-wiso-arbeitsrecht.md](../V1/03-wiso-arbeitsrecht.md) |
| Sozialversicherung (5 Zweige, Beitragssatz) | 🔴 | [L19_Sozialversicherung.md](L19_Sozialversicherung.md) 🆕 |
| Betriebsrat & Mitbestimmung (JAV, Betriebsvereinbarung) | 🔴 | [L20_Betriebsrat_Mitbestimmung.md](L20_Betriebsrat_Mitbestimmung.md) 🆕 |
| Unternehmensorganisation (Leitungssysteme, Sektoren) | 🔴 | [L21_Unternehmensorganisation.md](L21_Unternehmensorganisation.md) 🆕 |
| Kosten- & Wirtschaftlichkeitsrechnung (DB, Amortisation) | 🔴 | [L22_Kostenrechnung.md](L22_Kostenrechnung.md) 🆕 |
| Recht, Verträge & Steuern (Kaufrecht, Gesellschaftsformen) | 🟡 | [L23_WISO_Recht_Steuern.md](L23_WISO_Recht_Steuern.md) 🆕 |
| WiSo-Überblick & weitere Randthemen | 🟢 | [11_WiSo_Grundlagen.md](11_WiSo_Grundlagen.md) |

---

## Vorgeschlagener Lernplan (4 Wochen à ~2h/Tag)

- **Woche 1 – GA2 Netzwerk:** Subnetting → IPv4/Routing → IPv6 → VLAN → DHCP/DNS → Firewall → Fehlersuche → WLAN → Berechnungen
- **Woche 2 – GA1 Systeme:** Cloud → Speicher/RAID → Backup/MTBF → Virtualisierung → Verfügbarkeit → Datenschutz → Verschlüsselung → Programmierung → SQL/UML → Lizenzierung
- **Woche 3 – WiSo:** Arbeitsrecht → Sozialversicherung → Betriebsrat → Unternehmensorganisation → Kostenrechnung → Recht/Steuern
- **Woche 4 – Simulation:** Pro Tag eine komplette Altklausur (GA1 + GA2 + WiSo) aus dem Repo, dann Lücken mit den Lektionen schließen.

---

## Vollständigkeit

Alle 🔴- und 🟡-Themen aus deiner `V1/List.md` sind jetzt durch eine Lektion
abgedeckt. Die 🟢-Randthemen finden sich komprimiert in
`11_WiSo_Grundlagen.md` bzw. `00_Index.md`.

> **Hinweis:** Die Original-PDFs im Repo sind urheberrechtlich geschützte
> IHK-Prüfungsunterlagen. Die Lektionen hier enthalten deshalb selbst
> erstellte Beispielaufgaben im Prüfungsstil – bedenkenlos im eigenen Repo
> nutzbar. Zur Vertiefung die echten Altklausuren nur zum eigenen Lernen nutzen.
