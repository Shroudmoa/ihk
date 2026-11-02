# IHK AP2 FiSi (AO 2020) – Lernunterlagen

> Ein Thema pro Datei, alle Blätter untereinander verlinkt. Zum Lesen im Terminal: [glow](https://github.com/charmbracelet/glow).

## Prüfungsaufbau

| Teil | Inhalt | Dauer | Form |
|---|---|---|---|
| **GA1** – Konzeption und Administration von IT-Systemen | Server, Storage, Backup, Virtualisierung, Cloud, Sicherheit, Datenschutz, Programmierung, Wirtschaftlichkeit | 90 Min | 4 Aufgaben, 100 Punkte |
| **GA2** – Analyse und Entwicklung von Netzwerken | IPv4/IPv6, Subnetting, Routing, VLAN, DHCP/DNS, Firewall, WLAN, Fehlersuche | 90 Min | 4 Aufgaben, 100 Punkte |
| **WiSo** – Wirtschafts- und Sozialkunde | Ausbildung, Arbeitsrecht, Sozialversicherung, Betrieb, Volkswirtschaft | 60 Min | Multiple Choice + Kurzantworten |

AP2 prüft **Anwendung**, nicht nur Wissen: Eine Ausgangssituation, daraus Teilaufgaben – begründen, rechnen, vergleichen.

## Einstieg

| Datei | Wofür |
|---|---|
| [IT.md](IT.md) | IT-Lerntree: Lerngeschichte „Ein Server für Rheinblick“ + Themen-Tabellen (GA1 & GA2) |
| [Wirtschaft.md](Wirtschaft.md) | WiSo-Lerntree nach Kategorien |
| [Prüfungsanalyse – Master-Index](IT/analyse-der-exams/README.md) | Welche Themen wie oft drankommen (🔴/🟡/🟢) + Lernplan |
| [Prüfungsanalyse – Lektionen](IT/analyse-der-exams/00_Index.md) | Lektionen 01–L29 aus den Prüfungen 2021–2026 |
| [GA1-/GA2-Fragen](IT/analyse-der-exams/fragen/) | Aufgaben der echten Termine Winter 2021 – Sommer 2026 |
| [WiSo-Fragen](Wirtschaft/fragen/) | WiSo-Aufgaben der echten Termine |
| [AP1-Grundlagen (LF1–LF6)](IT/nice-to-know/ap1/lf1.md) | Wiederholung Grundlagen aus Teil 1 |

Die Originalprüfungen mit Lösungen: <https://github.com/Shroudmoa/IHK-Abschlusspruefungen-AP2-FiSi-AO2020>

## Ordnerstruktur

```
refactoredtree/
├── IT.md · Wirtschaft.md · readme.md
├── IT/
│   ├── FiSi-Administrator/
│   │   ├── OSI-IP-mac-bit-byte/      Netzwerk-Grundlagen: OSI, IPv4/IPv6, Subnetting, DHCP, DNS, Ports
│   │   ├── Netzwerk-Administration/  Routing, VLAN, WLAN, Proxy, AD-Dienste (GPO, LDAP, Kerberos), Troubleshooting
│   │   ├── Sicherheit/               Schutzziele, Firewall, Verschlüsselung, MFA, hacking/
│   │   └── Server/                   RAID, HA, Cluster, USV, Backup, virtualisierung/, Cloud/
│   ├── Komponenten/                  CPU, Datenträger, Storage (DAS/NAS/SAN)
│   ├── Management/                   Projektmanagement, Wirtschaftlichkeit, DSGVO/
│   ├── UML-Code-SQL/                 Pseudocode/, UML/, sql/
│   ├── nice-to-know/                 VPN/, ap1/, Formate, Begriffe
│   └── analyse-der-exams/            Prüfungsanalyse + fragen/ga1, fragen/ga2
└── Wirtschaft/                       WiSo-Blätter + fragen/
```

## Aufbau jedes Lernblatts

`In 30 Sekunden` → `Kernwissen` → `Prüfungsfrage` (mit Musterantwort) → `Falle` → `Merksatz` → `Verwandt`

## Lernregeln

- Antworten technisch **begründen** – meist reichen 2–3 Sätze. *Nennen* < *Beschreiben* < *Erläutern* (Erläutern = mit Grund oder Beispiel).
- Geforderte Anzahl einhalten: Bei „Nennen Sie drei …“ werden nur die ersten drei gewertet.
- Vor- und Nachteile, Alternativen, Protokolle, Ports und Geräte **vergleichen**.
- Frage unklar? Annahme hinschreiben und fachlich begründet antworten.
- Verbindungen sehen: OSI ↔ Geräte, RAID ↔ Backup (RAID ≠ Backup!), RBAC ↔ LDAP, Schutzziele ↔ jede Maßnahme.

## Lernreihenfolge (Vorschlag)

1. **GA2 Netzwerk:** OSI → IPv4/Subnetting → Routing → VLAN → DHCP/DNS → IPv6 → Firewall/NAT → WLAN → Fehlersuche
2. **GA1 Systeme:** Hardware → RAID → Backup → Verfügbarkeit → Virtualisierung → Cloud → Sicherheit → DSGVO → Pseudocode/SQL → Wirtschaftlichkeit
3. **WiSo:** BBiG → Arbeitsrecht → Sozialversicherung → Betriebsrat → Unternehmen → Volkswirtschaft
4. **Simulation:** Ein kompletter Prüfungstermin pro Tag aus `fragen/`, Lücken über die Lernblätter schließen.
