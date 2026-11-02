# Energieeffizienz und Stromkosten (Green IT)

## In 30 Sekunden

- **Energie (kWh) = Leistung (kW) · Zeit (h)**. **Kosten = kWh · Preis pro kWh**. Dauerbetrieb = 24 · 365 = **8.760 h/Jahr**.
- **Wirkungsgrad η** des Netzteils: Aufnahme aus der Steckdose = abgegebene Leistung / η. Mit 90 % braucht ein 360-W-Server 400 W aus dem Netz.
- **Netzteil dimensionieren (2026S):** Summe der Komponenten + Puffer, dann auf die nächste marktübliche Größe aufrunden.
- **PUE** = Gesamtenergie des Rechenzentrums / Energie der IT. 1,0 wäre ideal, 1,5 heißt 50 % Zusatz für Kühlung, USV-Verluste und Licht.
- Green IT: **Konsolidierung durch Virtualisierung**, effiziente Netzteile (80 PLUS), Energiesparfunktionen, Kalt-/Warmgang, Abwärmenutzung, lange Nutzungsdauer.
- Geprüft: Netzteil 2026S, Stromkosten Server-Cluster 2023W, Green IT als Virtualisierungsvorteil 2025S.

## Kernwissen

### Formeln

| Größe | Formel | Einheit |
|---|---|---|
| Leistung | P = U · I | W |
| Energie | E = P · t | Wh (/1.000 → kWh) |
| Kosten | K = E · Preis | € |
| Netzaufnahme | P_Netz = P_Ausgang / η | W |
| Wirkleistung (USV) | P [W] = S [VA] · cos φ | W |
| PUE | E_gesamt / E_IT | ohne Einheit |

### Rechenweg 1: Stromkosten eines Clusters

4 Server à 350 W, Dauerbetrieb, Netzteil-Wirkungsgrad 92 %, Strompreis 0,30 €/kWh.

1. Ausgangsleistung: 4 · 350 W = 1.400 W
2. Netzaufnahme: 1.400 W / 0,92 ≈ 1.521,7 W ≈ 1,522 kW
3. Energie: 1,522 kW · 8.760 h ≈ **13.333 kWh/Jahr**
4. Kosten: 13.333 kWh · 0,30 €/kWh ≈ **4.000 €/Jahr**
5. Mit PUE 1,6 (Kühlung usw.): 4.000 € · 1,6 ≈ **6.400 €/Jahr**

### Rechenweg 2: Netzteil dimensionieren (Muster 2026S)

| Komponente | Anzahl · Leistung | Summe |
|---|---|---:|
| Mainboard | 1 · 40 W | 40 W |
| CPU | 2 · 90 W | 180 W |
| RAM-Module | 8 · 4 W | 32 W |
| HDD | 4 · 10 W | 40 W |
| Netzwerkkarten | 2 · 6 W | 12 W |
| **Summe** | | **304 W** |

- Puffer 20 %: 304 W · 1,2 = **364,8 W**
- Wurde ein Wirkungsgrad von 90 % vorgegeben und soll er eingerechnet werden (so in der Musterlösung): 364,8 W / 0,9 ≈ **405,3 W** → nächste Größe im 50-W-Raster: **450 W**.
- Fachlich ist die Nennleistung eines Netzteils seine **Ausgangs**leistung. Der Wirkungsgrad bestimmt vor allem die Aufnahme aus der Steckdose, also Stromkosten und Abwärme. Die Aufgabenstellung genau lesen und den gewählten Rechenweg hinschreiben.

### Rechenweg 3: Ersparnis durch Konsolidierung

10 alte Server à 300 W werden durch 2 Virtualisierungshosts à 600 W ersetzt.
- Vorher 3.000 W, nachher 1.200 W → Ersparnis 1.800 W = 1,8 kW
- 1,8 kW · 8.760 h = **15.768 kWh/Jahr** → bei 0,30 €/kWh ≈ **4.730 €/Jahr** (ohne Kühlung)
- Bei Investitionskosten von 14.000 € amortisiert sich die Umstellung über den Strom allein in ≈ 3 Jahren → [Amortisation](../../Management/amortisation.md).

### Maßnahmen der Green IT

| Bereich | Maßnahme |
|---|---|
| Server | Virtualisierung/Konsolidierung, höhere Auslastung, Abschalten ungenutzter Systeme, Energiesparprofile im BIOS/UEFI |
| Netzteile | 80-PLUS-Zertifizierung (Bronze bis Titanium), redundante Netzteile richtig dimensionieren |
| Storage | SSD statt HDD bei Zugriffslast, Tiering, Dedup, Archiv auf Band (braucht im Lager keinen Strom) |
| Rechenzentrum | Kalt-/Warmgang-Einhausung, höhere Zulufttemperatur, freie Kühlung, Abwärmenutzung, Ökostrom, PUE messen |
| Arbeitsplatz | Thin Clients, Energiesparmodus, Monitore mit Energielabel, Geräte länger nutzen und refurbished kaufen |
| Organisation | Beschaffungsrichtlinie (Blauer Engel, EU-Energielabel, EPEAT), fachgerechtes Recycling (ElektroG, WEEE) |

Rechtlicher Rahmen: Das **Energieeffizienzgesetz (EnEfG)** schreibt Rechenzentren in Deutschland u. a. PUE-Grenzwerte, Abwärmenutzung und Energiemanagement vor.

## Prüfungsfrage

**Frage:** Ein Server nimmt im Mittel 480 W aus dem Netz auf und läuft rund um die Uhr. Berechnen Sie die jährlichen Stromkosten bei 0,32 €/kWh und nennen Sie zwei Maßnahmen, um sie zu senken.

**Musterantwort:** 0,48 kW · 8.760 h = 4.204,8 kWh pro Jahr. 4.204,8 kWh · 0,32 €/kWh ≈ **1.345,54 €** pro Jahr. Senken lassen sich die Kosten erstens durch Konsolidierung: Die Dienste laufen als VMs auf einem gemeinsamen Host, und Altgeräte werden abgeschaltet. Zweitens durch effizientere Hardware, z. B. ein Netzteil mit 80 PLUS Platinum (weniger Umwandlungsverluste) und aktivierte Energiesparfunktionen.

## Falle

- W und kWh verwechseln. Leistung ist ein Momentanwert, Energie ist Leistung mal Zeit.
- Durch 1.000 teilen vergessen (Wh → kWh).
- Wirkungsgrad falsch herum anwenden. Die Aufnahme aus dem Netz ist **größer** als die Abgabe, also durch η teilen, nicht malnehmen.
- Netzteil exakt auf die Summe auslegen. Ohne Puffer läuft es am Limit (Einschaltströme, Erweiterungen, schlechterer Wirkungsgrad nahe Volllast).
- VA und W bei USVs gleichsetzen. W = VA · Leistungsfaktor.

## Merksatz

**„Watt mal Stunden durch tausend mal Preis. Und was aus der Steckdose kommt, ist immer mehr, als beim Server ankommt.“**

## Verwandt

- [Virtualisierung](virtualisierung/virtualisierung.md) – Konsolidierung
- [VDI & Thin Client](virtualisierung/vdi-thin-client.md) – sparsame Arbeitsplätze
- [USV](usv.md) – VA, W, Laufzeit
- [Server-Hardware](../../Komponenten/server-hardware.md) – Netzteile, Komponentenauswahl
- [TCO](../../Management/tco.md) – Betriebskosten über die Laufzeit
- [Amortisation](../../Management/amortisation.md)
- [Umwelt & Nachhaltigkeit](../../../Wirtschaft/umwelt-nachhaltigkeit.md)
- [Lektion L28 GA1 Praxisthemen](../../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md)
- [Lektion L25 Umweltschutz](../../analyse-der-exams/L25_Umweltschutz_Nachhaltigkeit.md)
