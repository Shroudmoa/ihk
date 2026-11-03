# Lizenzierung (CAL, Core, Subscription)

## In 30 Sekunden

- Serverprodukte wie Windows Server werden **zweiteilig** lizenziert: die **Serverlizenz** (nach CPU-Kernen) plus **Zugriffslizenzen (CALs)** für jeden zugreifenden Benutzer oder jedes Gerät.
- **User CAL** = pro Person, beliebig viele Geräte → bei **mehr Geräten als Personen** (Roaming, Laptop + Handy + Homeoffice).
- **Device CAL** = pro Gerät, beliebig viele Personen → bei **mehr Personen als Geräten** (Schichtbetrieb, Kiosk, Terminal).
- **Core-Lizenz:** alle physischen Kerne lizenzieren, mindestens 8 pro CPU und 16 pro Server. Standard-Edition = 2 VMs, Datacenter = unbegrenzt viele VMs.
- **Update** (Patch, gleiche Version) ist meist kostenlos, ein **Upgrade** (neue Hauptversion) meist kostenpflichtig.
- Geprüft: CAL 2024S, 2025S. Update/Upgrade 2023W, 2025S. Nicht überpriorisieren, aber es sind sichere Punkte.

## Kernwissen

### CAL-Modelle

| Modell | Lizenziert wird | Sinnvoll wenn | Beispiel |
|---|---|---|---|
| **User CAL** | jede Person mit Zugriff | Personen < Geräte | 20 Außendienstler mit Laptop, Tablet, Smartphone → 20 User CALs statt 60 Device CALs |
| **Device CAL** | jedes zugreifende Gerät | Geräte < Personen | 3 Schichten à 30 Personen an 30 Produktions-PCs → 30 Device CALs statt 90 User CALs |
| **Mischbetrieb** | beides kombiniert | gemischte Abteilungen | Verwaltung mit User CALs, Lager-Terminals mit Device CALs |
| **RDS CAL** | **zusätzlich** für Remote Desktop Services | Terminalserver/VDI | Windows Server CAL **und** RDS CAL nötig |
| **External Connector** | unbegrenzt viele **externe** Nutzer | Kundenportal, Partnerzugang | statt tausender CALs für Externe |

**Rechenweg CAL:** Personen und Geräte je Bereich zählen und das jeweils kleinere Modell wählen.
- Verwaltung: 10 Personen mit je 3 Geräten (30 Geräte) → **10 User CALs**.
- Produktion: 35 Personen teilen sich 15 PCs → **15 Device CALs**.
- Gesamt 25 CALs. Nur User CALs wären 45, nur Device CALs ebenfalls 45.

### Core-basierte Serverlizenz (Windows Server ab 2016)

| Regel | Wert |
|---|---|
| zu lizenzieren | **alle physischen Kerne** des Hosts |
| Minimum je CPU | 8 Kerne |
| Minimum je Server | 16 Kerne |
| Verkaufseinheit | 2-Kern-Pakete (oder 16-Kern-Pakete) |
| **Standard** | berechtigt zu **2 VMs** (OSE); für 2 weitere VMs alle Kerne erneut lizenzieren |
| **Datacenter** | **unbegrenzt viele VMs** auf dem lizenzierten Host |

**Rechenweg:** Host mit 2 CPUs à 12 Kernen = 24 Kerne (≥ 16) → 24 Core-Lizenzen = 12 Zwei-Kern-Pakete. Sollen 6 Windows-VMs laufen: Standard bräuchte 3 · 24 = 72 Kern-Lizenzen, Datacenter nur 24. Ab einer gewissen VM-Zahl ist Datacenter günstiger.
Kleiner Server mit 1 CPU à 4 Kernen → trotzdem **16** Kern-Lizenzen (Minimum).

### Lizenzarten und Vertriebsformen

| Begriff | Bedeutung |
|---|---|
| **Kauflizenz (perpetual)** | einmal zahlen, zeitlich unbegrenzt nutzen (Version bleibt) – CAPEX |
| **Subscription / Abo** | laufende Gebühr pro Monat/Jahr, inkl. Updates und Upgrades, endet mit der Zahlung – OPEX, z. B. Microsoft 365 |
| **OEM** | an die Hardware gebunden, nicht auf neue Hardware übertragbar |
| **Retail / Box** | Einzelhandelslizenz, übertragbar |
| **Volumenlizenz** | Mengenvertrag für Organisationen, zentraler Schlüssel (KMS/MAK) |
| **Named User** | fest an eine Person gebunden |
| **Concurrent / Floating** | maximale Zahl **gleichzeitiger** Nutzer (Lizenzserver zählt) |
| **Software Assurance / Wartung** | Zusatzvertrag: Anspruch auf neue Versionen, Support |
| **Open Source** | Quellcode offen. **Copyleft** (GPL): Änderungen müssen unter gleicher Lizenz weitergegeben werden. **Permissiv** (MIT, BSD, Apache): kaum Auflagen |
| **Freeware / Shareware** | kostenlos nutzbar (ohne Quellcode) / kostenlos testen, dann zahlen |

### Update vs. Upgrade (2023W, 2025S)

| | Update / Patch | Upgrade |
|---|---|---|
| Inhalt | Fehlerbehebung, Sicherheitslücken, kleine Verbesserungen | neue Hauptversion, neue Funktionen |
| Versionssprung | 2.1 → 2.2 | 2.x → 3.0 |
| Kosten | meist kostenlos (in Lizenz/Wartung enthalten) | meist kostenpflichtig, neue Lizenz oder Upgrade-Preis |
| Risiko | gering, trotzdem testen | Migration, Inkompatibilitäten, Schulung |

## Prüfungsfrage

**Frage:** Für den neuen MDM-Server werden Zugriffslizenzen benötigt. Erläutern Sie für User CAL und Device CAL je einen empfohlenen Einsatzfall.

**Musterantwort:** Eine User CAL lizenziert eine Person, egal mit wie vielen Geräten sie zugreift. Sie lohnt sich, wenn es mehr Geräte als Benutzer gibt, z. B. wenn jede Führungskraft mit Smartphone, Tablet und Notebook auf den MDM-Server zugreift. Eine Device CAL lizenziert ein Gerät, egal wie viele Personen es nutzen. Sie lohnt sich, wenn es mehr Benutzer als Geräte gibt, z. B. bei Scannern im Lager, die von drei Schichten nacheinander verwendet werden.

## Falle

- Nur „pro User“/„pro Gerät“ hinschreiben, ohne Einsatzfall. Gefragt ist immer ein **konkretes Szenario**.
- Die Serverlizenz vergessen: CALs ersetzen sie nicht, beides ist nötig.
- RDS-Zugriff nur mit normaler CAL lizenzieren. Es braucht zusätzlich eine RDS CAL.
- Bei Core-Lizenzen nur die genutzten vCPUs zählen. Lizenziert werden die **physischen** Kerne des Hosts.
- OEM-Lizenz beim Hardwaretausch auf den neuen Server mitnehmen. Das ist nicht erlaubt.
- Open Source mit „frei von Pflichten“ gleichsetzen. Die GPL hat Weitergabepflichten.

## Merksatz

**„Mehr Geräte als Köpfe → User CAL. Mehr Köpfe als Geräte → Device CAL.“**

## Verwandt

- [Lektion 10 Lizenzierung](../../analyse-der-exams/10_Lizenzierung.md)
- [Hypervisor-Typen](virtualisierung/hypervisor-typen.md) – Lizenzierung bei VMs
- [VDI & Thin Client](virtualisierung/vdi-thin-client.md) – RDS CAL
- [Cloud Computing](Cloud/cloud-computing.md) – Subscription statt Kauf
- [CAPEX-OPEX](../../Management/capex-opex.md) – Kauf vs. Abo
- [TCO](../../Management/tco.md) – Lizenzkosten über die Laufzeit
- [Patchmanagement](../Netzwerk-Administration/patchmanagement-tu-su-pu.md) – Updates verteilen
- [Urheberrecht & Compliance](../../../Wirtschaft/Recht/urheberrecht-compliance.md)
