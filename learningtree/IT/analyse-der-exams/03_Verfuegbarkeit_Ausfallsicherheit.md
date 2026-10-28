# 3. Verfügbarkeit & Ausfallsicherheit

*Quelle: GA1 Aufgabe 3/4 (2021 Winter, 2022 Winter, 2023 Sommer, 2025
Sommer/Winter, 2026 Sommer u. a.)*

> ✅ **Verifiziert am echten Prüfungskorpus:** Verfügbarkeit/MTBF/USV kam in
> **6 von 10 Jahrgängen** vor.
> - **2021W:** MTBF-Rechnung (Plattenverbund).
> - **2022W:** RAM-Kennwerte DDR4-3200 (effektiver Takt, Datenrate,
>   CAS Latency) im Kontext Server-Hardware/Performance.
> - **2023S:** USV-Typen Online/Offline/Line-Interactive (VFI/VFD/VI),
>   RTO/RPO-Definitionen.
> - **2025S:** USV-Berechnung mit Notstromaggregat-Anlaufzeit.
> - **2025W:** Badewannenkurve, MTBF vs. MTTF, "Mix-and-Match"-Prinzip.
> - **2026S:** SLA-Verfügbarkeitsrechnung (Ausfallzeit/Jahr, Kostenverlust).

## 3.1 Schutzziele der Informationssicherheit

Merke dir die **CIA-Triade**:

- **Confidentiality (Vertraulichkeit):** Schutz vor unberechtigtem Zugriff/
  Weitergabe.
- **Integrity (Integrität):** Schutz vor absichtlicher/versehentlicher
  Veränderung.
- **Availability (Verfügbarkeit):** Dienste/Daten zur zugesicherten Zeit
  verfügbar.

## 3.2 Reliability vs. Availability

- **Reliability (Zuverlässigkeit):** Wahrscheinlichkeit, dass ein System
  fehlerfrei arbeitet (z. B. Kasse gibt korrekte Beträge aus).
- **Availability (Verfügbarkeit):** System ist zum Benutzen da
  (z. B. Kasse ist zur Ladenöffnung erreichbar).

## 3.3 MTBF – Mean Time Between Failures

Erwartete Zeit zwischen zwei Ausfällen. Beispiel (2021W):

> 16 gleiche Platten, jede MTBF = 800.000 h. Gesamt-MTBF des Verbunds?

**Rechenweg (verifiziert):** Da jede der 16 Platten unabhängig ausfallen
kann (Verbund = Reihenschaltung aus Zuverlässigkeitssicht), teilt man die
Einzel-MTBF durch die Plattenanzahl:

```
800.000 h / 16 Platten = 50.000 h Gesamt-MTBF des Verbunds
50.000 h / (24 h × 365 Tage) ≈ 5,7 Jahre
```

→ Im Schnitt fällt alle **5,7 Jahre** eine Platte im Verbund aus.
**Merke:** Mehr Platten = mehr potenzielle Fehlerquellen = kürzere
Gesamt-MTBF. Rechenweg immer explizit hinschreiben (24/7 = 8.760 h/Jahr).

## 3.3a MTBF vs. MTTF (2025W)

- **MTBF (Mean Time Between Failures):** für **reparierbare** Systeme/
  Komponenten (z. B. Festplatte im RAID wird getauscht und der Verbund
  läuft weiter).
- **MTTF (Mean Time To Failure):** für **nicht reparierbare**
  Wegwerf-Komponenten (z. B. Sicherung, LED) – Kennwert für die
  durchschnittliche Lebensdauer bis zum endgültigen Ausfall.

### Badewannenkurve (2025W)

Die Ausfallrate von Hardware über die Zeit verläuft in **3 Phasen**:

1. **Frühausfälle (Infant Mortality):** hohe Ausfallrate durch
   Fertigungsfehler, fällt schnell ab (Burn-in-Tests sollen das abfangen).
2. **Normalbetrieb (Nutzungsphase):** konstante, niedrige Ausfallrate über
   den Großteil der Lebensdauer.
3. **Verschleißphase (Wear-out):** steigende Ausfallrate durch
   Materialermüdung am Lebensende.

**"Mix-and-Match"-Prinzip (2025W):** Beim Aufbau eines RAID-Verbunds
bewusst Platten **unterschiedlicher Hersteller/Chargen** mischen, damit
nicht alle Platten gleichzeitig in die Verschleißphase kommen und ggf.
zeitgleich ausfallen (Risikostreuung).

## 3.3b USV-Typen und RTO/RPO (2023S)

| USV-Typ | Norm-Kürzel | Funktionsweise |
|---|---|---|
| **Offline-USV** | VFD (Voltage and Frequency Dependent) | Schaltet erst bei Netzausfall auf Akku um (kurze Umschaltzeit) |
| **Line-Interactive-USV** | VI (Voltage Independent) | Gleicht Spannungsschwankungen aktiv aus, schaltet bei Ausfall um |
| **Online-USV** | VFI (Voltage and Frequency Independent) | Last läuft dauerhaft über den Wechselrichter/Akku – **unterbrechungsfrei**, keine Umschaltzeit |

- **RTO (Recovery Time Objective):** maximal tolerierbare **Zeitspanne bis
  zur Wiederherstellung** eines Systems nach einem Ausfall.
- **RPO (Recovery Point Objective):** maximal tolerierbarer **Datenverlust**,
  gemessen als Zeitspanne seit dem letzten Backup (bestimmt die nötige
  Backup-Frequenz).

## 3.3c USV-Berechnung mit Notstromaggregat (2025S)

Beispiel: Ein Notstromaggregat benötigt **3 Minuten Anlaufzeit**. Die USV
soll bei **30 % Akkuladung** eine Warnung/Zielschwelle erreichen, muss aber
zusätzlich die Überbrückung der 3-minütigen Anlaufzeit sicherstellen
(zusätzlicher Puffer von **5 %**).

```
Ziel-Reserve:       30 %
+ Puffer für 3 Min. Anlaufzeit:  5 %
= Start des Notstromaggregats bei 35 % Akkuladung
```

**Merke:** Bei USV-Aufgaben immer prüfen, ob zusätzlich zur Ziel-Reserve
noch eine Zeitreserve für die Anlaufzeit externer Notstromquellen
eingerechnet werden muss.

## 3.3d SLA-Verfügbarkeitsrechnung (2026S)

Beispiel: SLA garantiert **99,9 % Verfügbarkeit**. Ausfallzeit pro Jahr und
Kostenverlust bei 500 €/h Ausfallkosten?

```
Ausfallanteil: 100 % - 99,9 % = 0,1 %
Stunden pro Jahr: 24 h × 365 Tage = 8.760 h
Ausfallzeit: 8.760 h × 0,1 % = 8,76 h/Jahr
Kostenverlust: 8,76 h × 500 €/h = 4.380 €/Jahr
```

## 3.3e RAM-Kennwerte am Beispiel DDR4-3200 (2022W)

Auch Arbeitsspeicher-Kennwerte wurden im Kontext Server-Performance/
Verfügbarkeit abgefragt:

- **Effektiver Takt:** 3.200 MHz
- **Datenrate bei 64-Bit-Bus:** 25,6 GB/s
- **CAS Latency (CL):** 16 Taktzyklen

## 3.4 Technische Maßnahmen für hohe Ausfallsicherheit

Nenne mindestens drei (2024 Musterlösung):

- **USV** (unterbrechungsfreie Stromversorgung)
- **Hochverfügbarkeitscluster** (Failover)
- **Load Balancing** (Lastverteilung)
- **RAID / SAN** mit Redundanz (Crossover/Failover bei Speicher)
- Redundante Netzanbindung (mehrere Uplinks)

## 3.5 Ausfallsicherer Server-Aufbau

Neben Festplatten-RAID:
- Redundante **Netzteile** (2x PSU)
- Redundante **Netzwerkkarten** (Teaming)
- **ECC-RAM**, doppelte **Netzanbindung**

## Typische Prüfungsaufgabe

> „Erläutern Sie am Beispiel des Kassensystems je eine Anforderung bezüglich
> Zuverlässigkeit und Verfügbarkeit."

→ Konkretes Beispiel pro Begriff, nicht nur Definition!
