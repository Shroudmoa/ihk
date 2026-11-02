# Netzplan

## In 30 Sekunden

- Ein **Netzplan** stellt die Vorgänge eines Projekts mit ihren **Abhängigkeiten** grafisch dar und berechnet Termine, Puffer und den **kritischen Pfad** (DIN 69900).
- Üblich ist der **Vorgangsknotennetzplan (VKN):** jeder Vorgang ist ein Kasten mit FAZ, FEZ, SAZ, SEZ, Dauer, GP und FP.
- **Vorwärtsrechnung** liefert die frühesten Termine (FAZ, FEZ), **Rückwärtsrechnung** die spätesten (SAZ, SEZ).
- **Gesamtpuffer GP = SAZ − FAZ** (bzw. SEZ − FEZ): Verschiebung ohne Verzögerung des Projektendes.
- **Freier Puffer FP = FAZ des frühesten Nachfolgers − FEZ:** Verschiebung ohne Auswirkung auf Nachfolger.
- **Kritischer Pfad** = Kette der Vorgänge mit **GP = 0**; er bestimmt die minimale Projektdauer.

## Kernwissen

### Aufbau eines Vorgangsknotens

```
┌───────┬────────┬───────┐
│  FAZ  │ Dauer  │  FEZ  │
├───────┴────────┴───────┤
│  Nr.  Vorgangsname     │
├───────┬────────┬───────┤
│  SAZ  │ GP  FP │  SEZ  │
└───────┴────────┴───────┘
```

| Kürzel | Bedeutung | Formel (Zählweise ab Zeitpunkt 0) |
|---|---|---|
| FAZ | frühester Anfangszeitpunkt | größter FEZ aller Vorgänger |
| FEZ | frühester Endzeitpunkt | FAZ + Dauer |
| SEZ | spätester Endzeitpunkt | kleinster SAZ aller Nachfolger |
| SAZ | spätester Anfangszeitpunkt | SEZ − Dauer |
| GP | Gesamtpuffer | SAZ − FAZ |
| FP | freier Puffer | min. FAZ der Nachfolger − FEZ |

Bei der Zählweise "ab Tag 1" gilt FEZ = FAZ + Dauer − 1 und der Nachfolger beginnt bei FEZ + 1. In der Prüfung die vorgegebene Zählweise verwenden und nicht mischen.

### Rechenbeispiel: Umzug des Serverraums

| Nr. | Vorgang | Dauer (Tage) | Vorgänger |
|---|---|---:|---|
| A | Planung | 3 | – |
| B | neuen Raum vorbereiten (Strom, Klima) | 6 | A |
| C | Netzwerkverkabelung | 4 | A |
| D | Backup und Abbau | 2 | C |
| E | Umzug und Aufbau | 3 | B, D |
| F | Test und Dokumentation | 2 | E |

**Vorwärtsrechnung**

| Vorgang | FAZ | FEZ |
|---|---:|---:|
| A | 0 | 3 |
| B | 3 | 9 |
| C | 3 | 7 |
| D | 7 | 9 |
| E | max(9; 9) = 9 | 12 |
| F | 12 | 14 |

Projektdauer: **14 Tage**.

**Rückwärtsrechnung** (SEZ des letzten Vorgangs = 14)

| Vorgang | SEZ | SAZ | GP = SAZ − FAZ | FP |
|---|---:|---:|---:|---:|
| F | 14 | 12 | 0 | 0 |
| E | 12 | 9 | 0 | 0 |
| D | 9 | 7 | 0 | 0 |
| B | 9 | 3 | 0 | 0 |
| C | 7 | 3 | 0 | 0 |
| A | min(3; 3) = 3 | 0 | 0 | 0 |

Hier gibt es **zwei kritische Pfade**: A–B–E–F und A–C–D–E–F (beide 14 Tage). Verlängert sich die Verkabelung C um einen Tag, verschiebt sich das Projektende auf Tag 15.

**Variante:** Dauert B nur 4 Tage, gilt FEZ(B) = 7, SAZ(B) = 9 − 4 = 5 → **GP(B) = 2**, FP(B) = FAZ(E) − FEZ(B) = 9 − 7 = **2**. Kritisch ist dann nur noch A–C–D–E–F.

### Kritischer Pfad und Gegenmaßnahmen

- Eine Verzögerung auf dem kritischen Pfad verschiebt das Projektende um denselben Betrag.
- Nicht kritische Vorgänge können ihren Puffer nutzen.
- **Crashing:** mehr Ressourcen für kritische Vorgänge (teurer).
- **Fast Tracking:** Vorgänge überlappen (z. B. EA − 1), erhöht das Risiko von Nacharbeit.
- **Umplanung:** Reihenfolge, Umfang oder Endtermin anpassen.

## Prüfungsfrage

**Frage:** Ermitteln Sie für das obige Beispiel mit verkürzter Dauer von B (4 Tage) die Projektdauer, den kritischen Pfad und die Puffer von B. Welche Auswirkung hat es, wenn sich B um 3 Tage verzögert?

**Musterantwort:** Vorwärtsrechnung: A 0–3, B 3–7, C 3–7, D 7–9, E 9–12, F 12–14 → Projektdauer **14 Tage**. Rückwärtsrechnung ergibt für B: SEZ = 9, SAZ = 5, **GP = 2**, **FP = 2**. Kritischer Pfad: **A–C–D–E–F** (alle GP = 0). Verzögert sich B um 3 Tage, endet B bei Tag 10. Das übersteigt den Gesamtpuffer um 1 Tag: E kann erst bei Tag 10 beginnen, das **Projektende verschiebt sich auf Tag 15**, und B wird Teil des neuen kritischen Pfads.

## Falle

- **FAZ des Nachfolgers bei mehreren Vorgängern:** Immer den **größten** FEZ nehmen.
- **SEZ bei mehreren Nachfolgern:** Immer den **kleinsten** SAZ nehmen.
- **Gesamtpuffer und freier Puffer verwechseln:** GP bezieht sich auf das Projektende, FP auf den direkten Nachfolger.
- **Zählweisen mischen** (ab 0 vs. ab Tag 1).
- **Kritischer Pfad = längster Einzelvorgang:** Er ist die längste **Kette**.

## Merksatz

**"Vorwärts das Maximum, rückwärts das Minimum – Puffer null heißt kritisch."**

## Verwandt

- [AOB – Anordnungsbeziehungen](aob.md) – EA, AA, EE, AE
- [Gantt-Diagramm](gantt-diagramm.md) – die Termine auf der Zeitachse
- [PERT](pert.md) – Dauern unter Unsicherheit
- [Projektstrukturplan (PSP)](projektstrukturplan-psp.md) – liefert die Vorgänge
- [Ressourcenplan](ressourcenplan.md) – Puffer für Nivellierung nutzen
- [Soll-Ist-Vergleich](soll-ist-vergleich.md) – Verzögerungen erkennen
