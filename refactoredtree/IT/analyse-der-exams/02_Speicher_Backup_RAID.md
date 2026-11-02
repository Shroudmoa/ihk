# 2. Speicher, Backup & RAID

*Quelle: GA1 Aufgabe 2/4 (2021, 2022 Sommer/Winter, 2023 Sommer, 2024
Sommer/Winter, 2025 Sommer/Winter u. a.)*

> ✅ **Verifiziert am echten Prüfungskorpus:** RAID/Speicher-Berechnungen
> kamen in **7 von 10 Jahrgängen** vor – mit fast immer einer konkreten
> Rechenaufgabe. Merke dir die **Kernformeln der IHK**:
> - **RAID 5** Nettokapazität = **(n − 1) × Kapazität pro Platte**
> - **RAID 6** Nettokapazität = **(n − 2) × Kapazität pro Platte**
> - **RAID 10** Nettokapazität = **(n / 2) × Kapazität pro Platte**
>
> Beispiele aus den Original-Prüfungen:
> - **2022S:** RAID-Liste 0,1,10,5,6,50,60 zuordnen – "höchste Schreibrate"
>   → RAID 10; "größte Kapazität bei Fehlertoleranz" → RAID 5.
> - **2022W:** RAID 6 + Hot-Spare: 5+3+5 TiB = 13 TiB benötigt;
>   13 / 2 = 6,5 → aufgerundet 7 Datenplatten; **Antwort: 10 Platten gesamt**
>   (7 Daten + 2 Parität + 1 Hot-Spare).
> - **2023S:** RAID 6, 9,4 TiB benötigt → **12 Platten** (10 Daten + 2
>   Parität à 1 TiB).
> - **2024S:** 20 TiB netto, 4-TiB-Platten: RAID 10 = **10 Platten** (5×2),
>   RAID 5 = **6 Platten** (5+1), RAID 6 = **7 Platten** (5+2).
> - **2024W:** RAID 15 (verschachtelt), 4×2,5 TiB + 2×3,5 TiB-Platten:
>   netto = (6−4)×2,5 = **5 TiB**; RAID-05-Vergleich: **12,5 TiB**; **JBOD**
>   als redundanzlose Alternative genannt.
> - **2025S:** RAID-6-Erweiterung von 80 auf 160 TiB (8-TiB-Platten):
>   **10 zusätzliche Platten** (22 statt 12 gesamt); RAID-10-Vergleich bei
>   gleicher Plattenzahl: **88 TiB**.
> - **2025W:** RAID 6, 20×8-TiB-Platten: netto = (20−2)×8 = **144 TiB**;
>   RAID-6-Vorteil ggü. RAID 5: schützt vor **2 gleichzeitigen
>   Plattenausfällen**.

## 2.1 SAN vs. NAS

- **NAS** (Network Attached Storage): Dateisystem-Ebene, über Netzwerk
  angesprochen (File-Protokolle). Einfach, kostengünstig.
- **SAN** (Storage Area Network): Block-Ebene (wie lokale Festplatte),
  sehr performant, für große zentrale Speicher.

## 2.2 Rechenaufgabe: Betriebszeit eines SAN

Beispiel (2021): SAN 20 TiB, aktuell 9 TiB belegt (90 % von altem NAS),
Zuwachs 750 GiB/Jahr, Frage: Jahre bis **> 70 %** belegt sind.

Rechenweg:
```
70 % von 20 TiB = 14 TiB frei nutzbar bis Grenze
Bereits belegt: 9 TiB
Puffer bis 14 TiB: 14 - 9 = 5 TiB = 5120 GiB
5 TiB / 750 GiB pro Jahr ≈ 6,8 Jahre → 6 volle Jahre
```
**Tipp:** Immer **Rechenweg aufschreiben** – oft gibt es Punkte schon dafür.

## 2.3 Zuordnungseinheit (Cluster-Size)

- **Kleine Cluster (z. B. 4 KiB):** sinnvoll bei vielen **kleinen Dateien**
  (weniger Verschnitt/Slack).
- **Große Cluster (z. B. 64 KiB):** sinnvoll bei **großen Dateien**
  ( Videos, Datenbanken) → weniger Verwaltungsoverhead, schnellerer Zugriff.

## 2.4 Speicherplatzoptimierung

- **Deduplizierung:** Doppelte Datenblöcke werden einmal physikalisch
  gespeichert (Beispiel: 50 identische VM-Basisimages belegen nur einmal Platz).
- **Komprimierung:** Daten werden algorithmisch verkleinert (Beispiel:
  Text-/Log-Dateien, ZIP-Archive).

## 2.5 Backup as a Service (BaaS)

Sinnvoll, weil:
- **Offsite-Kopie** automatisch (Schutz bei Brand/Diebstahl vor Ort)
- Kein eigener Backup-Server nötig
- Verschlüsselter Transfer/Store, oft versioniert

## 2.6 Revisionssichere Archivierung

Daten müssen **unveränderbar und nachweisbar** über den gesetzlichen
Aufbewahrungszeitraum aufbewahrt werden (kein Überschreiben/Löschen möglich,
Audit-Protokoll vorhanden). Beispiel: Rechnungen nach HGB/AO.

## 2.7 RAID-Level (sehr prüfungsrelevant!)

| Level | Minimum Platten | Nutzen | Parität |
|-------|-----------------|--------|---------|
| **RAID 0** | 2 | Performance (Striping), keine Ausfallsicherheit | – |
| **RAID 1** | 2 | Spiegelung (1:1) | – |
| **RAID 5** | 3 | Striping + **einfache** Parität | 1 Platte Ausfall toleriert |
| **RAID 6** | 4 | Striping + **doppelte** Parität | 2 Platten Ausfall toleriert |
| **RAID 10** | 4 | Mirror + Stripe (1+0) | pro Spiegel 1 Ausfall |
| **RAID 50** | 6 | Mehrere RAID-5-Verbünde gestriped | 1 Ausfall pro Teil-Array |
| **RAID 60** | 8 | Mehrere RAID-6-Verbünde gestriped | 2 Ausfälle pro Teil-Array |
| **RAID 15** | 6 | Verschachtelt: mehrere Spiegel + RAID-5-Parität | hohe Fehlertoleranz, hoher Plattenaufwand |
| **RAID 05** | 6 | RAID-5-Verbünde gespiegelt | hohe Fehlertoleranz, hoher Plattenaufwand |
| **JBOD** | 1 | "Just a Bunch Of Disks" – Platten einfach aneinandergehängt | **keine Redundanz**, keine Performancesteigerung |

**Rechenaufgabe (2024S):** 20 TiB Netto, Platten à 4 TiB.

- **RAID 10:** 20/4 = 5 Platten Nutzen → wegen Spiegelung ×2 = **10 Platten**
- **RAID 5:** 20/4 = 5 + 1 Parität = **6 Platten**
- **RAID 6:** 20/4 = 5 + 2 Parität = **7 Platten**

→ **RAID 5** braucht am wenigsten Platten (Antwort auf „geringste Anzahl").

**Weitere Rechenbeispiele aus echten Prüfungen:**

- **2023S (RAID 6, 9,4 TiB benötigt):** Mit 1-TiB-Platten braucht man
  10 Datenplatten (10 TiB ≥ 9,4 TiB) + 2 Paritätsplatten = **12 Platten**.
- **2025W (RAID 6, 20 Platten à 8 TiB):**
  netto = (20 − 2) × 8 TiB = **144 TiB**.
- **2025S (RAID-6-Erweiterung, 8-TiB-Platten, 80 → 160 TiB netto):**
  Ausgangs-Setup: 12 Platten gesamt (10 Daten + 2 Parität) = 80 TiB.
  Für 160 TiB werden 20 Datenplatten benötigt → **10 zusätzliche Platten**,
  insgesamt **22 Platten** (statt 12).
- **2024W (RAID 15, verschachtelt, 4×2,5 TiB + 2×3,5 TiB):**
  netto = (6 − 4) × 2,5 TiB = **5 TiB**. Vergleich RAID 05 mit denselben
  Platten: **12,5 TiB**. **JBOD** wurde als redundanzlose Alternative
  genannt (kein Schutz bei Plattenausfall).

## Typische Prüfungsaufgabe

> „Berechnen Sie die benötigte Anzahl Festplatten für RAID 10, 5 und 6."

→ Immer **Rechenweg** zeigen, Netto-Kapazität / Plattengröße zuerst!
Merke die Kernformeln: RAID 5 = (n−1)×Kapazität, RAID 6 = (n−2)×Kapazität,
RAID 10 = (n/2)×Kapazität.
