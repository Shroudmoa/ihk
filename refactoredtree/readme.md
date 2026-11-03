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
| [Wirtschaft.md](Wirtschaft.md) | WiSo-Lerntree: Lerngeschichte „Drei Jahre bei Nordlicht“ + Themen-Tabellen |
| [Prüfungsanalyse – Master-Index](IT/analyse-der-exams/README.md) | Welche Themen wie oft drankommen (🔴/🟡/🟢) + Lernplan |
| [Prüfungsanalyse – Lektionen](IT/analyse-der-exams/00_Index.md) | Lektionen 01–L29 aus den Prüfungen 2021–2026 |
| [GA1-/GA2-Fragen](IT/analyse-der-exams/fragen/) | Aufgaben der echten Termine Winter 2021 – Sommer 2026 |
| [WiSo-Prüfungsanalyse](Wirtschaft/analyse-der-exams/README.md) | WiSo-Themen nach Häufigkeit + Lektionen + Lernplan |
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
│   └── analyse-der-exams/            GA1/GA2-Prüfungsanalyse + fragen/ga1, fragen/ga2
└── Wirtschaft/
    ├── Ausbildung/                   BBiG, Ausbildungsordnung, IHK, JArbSchG, Fortbildung
    ├── Arbeitsrecht/                 Arbeitsvertrag, Arbeitszeit, Mutterschutz, Kündigung, AGG, Zeugnis
    ├── Mitbestimmung/                Betriebsrat & JAV, Tarifrecht, Gewerkschaften & Streik
    ├── Sozialversicherung/           Die fünf Zweige, Lohnabrechnung, Altersvorsorge
    ├── Unternehmen/                  Rechtsformen, Organisation, Ziele, Kennzahlen, Break-Even
    ├── Recht/                        Rechtsquellen, Geschäftsfähigkeit, Gerichte, Urheberrecht
    ├── Volkswirtschaft/              Grundbegriffe, Markt & Preis, Konjunktur, Globalisierung
    ├── Arbeitsschutz-Umwelt/         Arbeits- & Brandschutz, Umwelt, IT-Sicherheit im Alltag
    ├── nice-to-know/                 Kaufvertrag, Vollmachten, Lerneinheit LE
    ├── analyse-der-exams/            WiSo-Prüfungsanalyse (Lektionen 11, L19–L26)
    └── fragen/                       echte WiSo-Termine 2021–2026
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

## Version 1.0 (November 2026)

- **Struktur:** Ordner- und Dateinamen sind Windows-sicher (keine `?`, `:` oder Umlaute), Duplikate sind zusammengeführt, und alle relativen Links sind geprüft (0 kaputte Links).
- **Lernblätter:** 182 IT-Blätter und 46 WiSo-Blätter mit einheitlichem Aufbau. Rund 90 Blätter sind neu, unter anderem zu Backup, Verfügbarkeit, ARP, STP, VRRP, PoE, Verkabelung, DMZ, TLS, Zero Trust, ISMS, AVV, ER-Modell, Normalisierung, Vorgehensmodellen, ITIL, Pflegeversicherung, Gewerkschaften und Globalisierung.
- **Fachliche Korrekturen:** Fehler in den Lernblättern, in der Prüfungsanalyse und in mehreren Lösungsschlüsseln sind korrigiert. Korrekturen an WiSo-Lösungsschlüsseln sind mit *(Korrigiert: …)* markiert.
- **Stand der Zahlen:** Beitragssätze, Grenzwerte, Mindestlohn und Ähnliches gelten für 2026 (Grundsicherungsgeld ab 1.7.2026, Amtsgericht bis 10.000 €, Aktivrente). Jedes Blatt nennt seinen Stand.
