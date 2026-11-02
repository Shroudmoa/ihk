# Verfügbarkeit und Kennzahlen (MTBF, MTTR, MTTF)

## In 30 Sekunden

- **Verfügbarkeit A = MTBF / (MTBF + MTTR)** – Anteil der Zeit, in der das System läuft.
- **Ausfallzeit = Bezugszeit · (1 − A)**. Ein Jahr hat **8.760 h** (365 · 24).
- **Reihe** (alles muss laufen): A = A₁ · A₂ · … → wird **schlechter** als das schwächste Glied.
- **Parallel** (eins reicht): A = 1 − (1 − A₁) · (1 − A₂) → wird **besser**.
- **MTBF** gilt für reparierbare Systeme, **MTTF** für nicht reparierbare Teile, **MTTR** ist die mittlere Reparaturzeit.
- Viele gleiche Teile ohne Redundanz: MTBF_ges = MTBF / n (2021W: 800.000 h / 16 = 50.000 h ≈ 5,7 Jahre).
- Geprüft: 2021W, 2025W (MTBF/MTTF, Badewannenkurve), 2026S (SLA-Rechnung), GA2 2025W (Parallelschaltung).

## Kernwissen

### Begriffe

| Kennzahl | Bedeutung | Gilt für | Beispiel |
|---|---|---|---|
| **MTBF** – Mean Time Between Failures | mittlere Betriebszeit zwischen zwei Ausfällen | **reparierbare** Systeme | Server, RAID-Verbund, Backup-System |
| **MTTF** – Mean Time To Failure | mittlere Zeit bis zum (endgültigen) Ausfall | **nicht reparierbare** Teile | Festplatte, SSD, Lüfter, Netzteil als Tauschteil |
| **MTTR** – Mean Time To Repair/Restore | mittlere Zeit bis zur Wiederherstellung | Service, Wartung | Techniker + Ersatzteil + Rebuild = 6 h |
| **AFR** – Annualized Failure Rate | erwartete Ausfallquote pro Jahr | Datenträger | AFR ≈ 8.760 h / MTBF |
| **Verfügbarkeit** | Anteil der Betriebszeit am Gesamtzeitraum | System, Dienst, SLA | 99,9 % |

- **Zuverlässigkeit** (Reliability): Das System arbeitet über einen Zeitraum **fehlerfrei** (z. B. die Kasse rechnet immer richtig, stürzt nicht ab).
- **Verfügbarkeit** (Availability): Das System ist zum geforderten Zeitpunkt **nutzbar** (z. B. die Kasse läuft während der Öffnungszeiten).

### Tabelle der „Nines“ (Jahr = 8.760 h)

| Verfügbarkeit | Ausfall pro Jahr | pro Monat (30 Tage) | pro Woche |
|---:|---:|---:|---:|
| 99 % | 87,6 h (3,65 Tage) | 7,2 h | 1,68 h |
| 99,5 % | 43,8 h | 3,6 h | 50,4 min |
| 99,9 % | 8,76 h | 43,2 min | 10,1 min |
| 99,99 % | 52,56 min | 4,32 min | 1,01 min |
| 99,999 % | 5,26 min | 25,9 s | 6 s |

**Rückwärts:** Höchstens 4 h Ausfall pro Jahr erlaubt → A = 1 − 4 / 8.760 = 0,99954 → **99,954 %**.

### Rechenwege

**1. Verfügbarkeit aus MTBF und MTTR**
MTBF = 2.000 h, MTTR = 4 h → A = 2.000 / 2.004 = 0,998 → **99,80 %**.
Folgerung: Ein schnellerer Service (MTTR 1 h) ergibt 2.000 / 2.001 = 99,95 %. Ein Wartungsvertrag mit kurzer Reaktionszeit verbessert die Verfügbarkeit ohne neue Hardware.

**2. Reihenschaltung (Kette: Server → Switch → Storage)**
0,999 · 0,9995 · 0,999 ≈ 0,99750 → **99,75 %** → 8.760 h · 0,0025 ≈ **21,9 h** Ausfall pro Jahr.

**3. Parallelschaltung (zwei Internetleitungen à 99 %)**
1 − (0,01 · 0,01) = 1 − 0,0001 = **99,99 %**.
GA2 2025W: Leitung A hat 99 %, gesucht ist ein zweiter Anbieter für ≥ 99,99 %. Mit 98,5 % ergibt sich 1 − 0,01 · 0,015 = 99,985 % (reicht nicht), mit 99 % genau 99,99 % (reicht).

**4. Gemischt (zwei redundante Server hinter einem Switch)**
Server parallel: 1 − 0,01² = 0,9999. In Reihe mit dem Switch (99,9 %): 0,9999 · 0,999 ≈ **99,89 %**. Der einzelne Switch ist der **Single Point of Failure** und bestimmt das Ergebnis.

**5. MTBF eines Verbunds ohne Redundanz (2021W)**
16 Platten mit je MTBF 800.000 h. Jede Platte kann ausfallen, die Ausfallraten addieren sich:
MTBF_ges = 800.000 h / 16 = **50.000 h** → 50.000 / 8.760 ≈ **5,7 Jahre** bis zum nächsten Plattenausfall im Verbund.
Allgemein: 1 / MTBF_ges = 1 / MTBF₁ + 1 / MTBF₂ + …

**6. SLA-Kosten (2026S)**
99,9 % → 8,76 h/Jahr erlaubt. Bei 500 €/h Ausfallkosten → 8,76 · 500 = **4.380 €** pro Jahr.

### Badewannenkurve (2025W)

| Phase | Ausfallrate | Ursache | Maßnahme |
|---|---|---|---|
| I – Frühausfälle | hoch, fallend | Fertigungsfehler | Burn-in-Test, Garantie, erste Wochen überwachen |
| II – Nutzungsphase | niedrig, konstant (zufällig) | Zufallsausfälle | RAID, Hot Spare, Monitoring (SMART) |
| III – Verschleiß | steigend | Alterung, Abnutzung | vorbeugender Austausch, Lebenszyklus planen |

Die MTBF-Angabe des Herstellers bezieht sich auf **Phase II**. 800.000 h (91 Jahre) heißt nicht, dass eine Platte 91 Jahre hält. Es heißt: Bei vielen Platten fällt pro Jahr etwa 1,1 % aus (AFR = 8.760 / 800.000).

**Mix-and-Match (2025W):** Platten verschiedener Hersteller oder Chargen in einem RAID mischen. Dann geraten nicht alle gleichzeitig in Phase III, und ein Serienfehler (Bad Batch) trifft nicht alle Platten zugleich.

## Prüfungsfrage

**Frage:** Ein Webdienst läuft auf zwei Servern mit je 99,5 % Verfügbarkeit hinter einem Load Balancer (99,95 %). Berechnen Sie die Gesamtverfügbarkeit und die jährliche Ausfallzeit. Welche Komponente sollten Sie zuerst verbessern?

**Musterantwort:** Die beiden Server sind parallel geschaltet: 1 − (0,005 · 0,005) = 0,999975. In Reihe mit dem Load Balancer: 0,999975 · 0,9995 ≈ 0,99948 → **99,948 %**. Die Ausfallzeit beträgt 8.760 h · 0,00052 ≈ **4,6 h pro Jahr**. Der Load Balancer ist ein Single Point of Failure und begrenzt das Ergebnis. Er sollte zuerst redundant ausgelegt werden (zweiter Load Balancer im Active/Passive-Betrieb).

## Falle

- Parallel- und Reihenformel vertauschen. Redundanz (parallel) **verbessert**, eine Kette (Reihe) **verschlechtert**.
- Prozent nicht in Dezimalzahlen umrechnen: 99,9 % = 0,999, Ausfallanteil = 0,001.
- „Mehr Platten = höhere MTBF“. Ohne Redundanz sinkt die MTBF des Verbunds. Erst RAID macht den Ausfall einer Platte überlebbar.
- MTBF und MTTF vertauschen: **B**etween = reparierbar, **T**o = Wegwerfteil.
- 365 · 24 = 8.760, nicht 8.700. Bei Schaltjahren nur rechnen, wenn es verlangt ist.
- Ein SLA mit 99,9 % garantiert nicht, dass der Dienst nie länger als 8,76 h am Stück ausfällt. Es begrenzt die **Summe** im Messzeitraum.

## Merksatz

**„Reihe multipliziert die Verfügbarkeiten, parallel multipliziert die Ausfälle.“**

## Verwandt

- [HA-Hochverfügbarkeit](ha-hochverfuegbarkeit.md) – Maßnahmen, SPOF
- [SLA](Cloud/sla.md) – vertragliche Verfügbarkeit
- [Cluster](cluster.md) – Redundanz durch mehrere Knoten
- [USV](usv.md) – Stromversorgung absichern
- [RAID](raid.md) – Redundanz bei Datenträgern
- [Disaster Recovery & Notfallkonzept](backup/disaster-recovery-notfallkonzept.md) – RTO/RPO
- [Monitoring](monitoring.md) – SMART und Schwellenwerte
- [Lektion 03 Verfügbarkeit](../../analyse-der-exams/03_Verfuegbarkeit_Ausfallsicherheit.md)
- [Lektion L18 Verfügbarkeit berechnen](../../analyse-der-exams/L18_Netzwerk_Berechnungen.md)
