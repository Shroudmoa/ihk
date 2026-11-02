# Übertragungszeit & Datenrate berechnen

## In 30 Sekunden

- Grundformel: **Zeit = Datenmenge in Bit ÷ Datenrate in Bit/s** – umgestellt: Datenmenge = Rate × Zeit, Rate = Menge ÷ Zeit.
- Datenraten sind **immer dezimal** (1 Mbit/s = 1.000.000 bit/s); Dateigrößen in **MiB/GiB sind binär** (1 MiB = 1.048.576 Byte).
- **Byte → Bit: × 8** – der häufigste Punktverlust.
- **Overhead** (Protokoll-Header) erhöht die zu übertragende Menge: bei 10 % Overhead **Datenmenge × 1,1**.
- Typische GA2-Aufgaben: Download-/Backupdauer (2022S, 2025W), VoIP-Bandbreite (2022W), IoT-Bandbreite (2025S), Videokonferenz (2022W).

## Kernwissen

### Einheiten

| Datenrate (dezimal) | bit/s | | Speicher (binär) | Byte |
|---|---:|---|---|---:|
| 1 kbit/s | 10^3 | | 1 KiB | 2^10 = 1.024 |
| 1 Mbit/s | 10^6 | | 1 MiB | 2^20 = 1.048.576 |
| 1 Gbit/s | 10^9 | | 1 GiB | 2^30 = 1.073.741.824 |
| 1 Tbit/s | 10^12 | | 1 TiB | 2^40 |

Steht in der Aufgabe **MB/GB** (dezimal), rechnet man mit 10^6/10^9; bei **MiB/GiB** mit 2^20/2^30. Unklar? Annahme hinschreiben.

### Rechenschema (5 Schritte)

1. Datenmenge in **Byte** umrechnen (Präfix beachten: dezimal oder binär).
2. **× 8** → Bit.
3. Overhead aufschlagen (× 1,1 bei 10 %).
4. Datenrate in **bit/s** umrechnen.
5. Teilen, Ergebnis sinnvoll umrechnen (s → min → h) und runden wie gefordert.

### Beispiel 1 – Download mit binärer Dateigröße (GA2 Sommer 2022)

**1.634 MiB** über ADSL mit **16 Mbit/s**:
1. 1.634 MiB × 1.048.576 = 1.713.373.184 Byte
2. × 8 = 13.706.985.472 bit
3. ÷ 16.000.000 bit/s = **856,7 s ≈ 14 min 17 s**

*Wer MiB wie MB behandelt (1.634 × 8 = 13.072 → ÷ 16 = 817 s ≈ 13,6 min), liegt knapp 5 % daneben – in der Prüfung den Einheitenwechsel sichtbar machen.*

### Beispiel 2 – Mit Overhead (GA2 Winter 2025)

**15 GiB** über **250 Mbit/s**, **10 % Overhead**:
1. 15 × 1.073.741.824 = 16.106.127.360 Byte
2. × 8 = 128.849.018.880 bit
3. × 1,1 = 141.733.920.768 bit
4. ÷ 250.000.000 bit/s = **566,9 s ≈ 9 min 27 s**

### Beispiel 3 – Benötigte Datenrate (Backup-Fenster)

**2 TB** (dezimal) sollen nachts in **8 h** gesichert werden:
- 2 × 10^12 Byte × 8 = 1,6 × 10^13 bit
- 8 h = 28.800 s
- 1,6 × 10^13 ÷ 28.800 ≈ 555.555.556 bit/s ≈ **556 Mbit/s** → eine 1-Gbit/s-Anbindung reicht rechnerisch, 100 Mbit/s nicht.

### Beispiel 4 – VoIP-Bandbreite (GA2 Winter 2022)

Codec **G.722 = 64 kbit/s**, **10 % Overhead**, **20 gleichzeitige Gespräche**:
- pro Gespräch: 64 × 1,1 = **70,4 kbit/s**
- gesamt: 20 × 70,4 = **1.408 kbit/s ≈ 1,41 Mbit/s** (je Richtung, Sprache ist symmetrisch)

### Beispiel 5 – Videokonferenz (GA2 Winter 2022)

Empfehlung je Teilnehmer: Audio 58/58 kbit/s, Video 2.500 kbit/s Upload / 4.000 kbit/s Download; **10 Teilnehmer** am selben Anschluss:
- Upload: 10 × (58 + 2.500) = **25.580 kbit/s ≈ 25,6 Mbit/s**
- Download: 10 × (58 + 4.000) = **40.580 kbit/s ≈ 40,6 Mbit/s**
- Ein ADSL-Anschluss mit 16 Mbit/s Down / ~1 Mbit/s Up reicht nicht – der **Upload** ist der Engpass.

### Beispiel 6 – IoT-Bandbreite (GA2 Sommer 2025)

**500 Sensoren** senden je **66 Byte pro 60 s**, verfügbar sind 10 kbit/s:
- 500 × 66 Byte = 33.000 Byte pro Minute
- ÷ 60 s = 550 Byte/s × 8 = **4.400 bit/s = 4,4 kbit/s** → passt (Reserve ca. 56 %).

### Beispiel 7 – Speicherbedarf aus Bitrate (Videoüberwachung)

Kamera streamt **4 Mbit/s** rund um die Uhr:
- 4.000.000 bit/s × 86.400 s = 3,456 × 10^11 bit ÷ 8 = 4,32 × 10^10 Byte = **43,2 GB pro Tag**; 30 Tage Aufbewahrung ≈ **1,3 TB** pro Kamera.

### Schnelltabelle 1 GB (dezimal) übertragen

| Datenrate | Dauer (ohne Overhead) |
|---|---:|
| 16 Mbit/s | 500 s ≈ 8,3 min |
| 100 Mbit/s | 80 s |
| 1 Gbit/s | 8 s |
| 10 Gbit/s | 0,8 s |

## Prüfungsfrage

**Frage:** Ein Außendienstmitarbeiter muss über LTE mit 40 Mbit/s eine Präsentation von 850 MiB auf den Firmenserver hochladen. Es wird mit 8 % Protokoll-Overhead gerechnet. Wie lange dauert der Upload mindestens? Geben Sie den Rechenweg an.

**Musterantwort:**
1. 850 MiB × 1.048.576 = 891.289.600 Byte
2. × 8 = 7.130.316.800 bit
3. × 1,08 = 7.700.742.144 bit
4. ÷ 40.000.000 bit/s = 192,5 s
**Ergebnis:** ca. **193 s ≈ 3 min 13 s**. In der Praxis länger, da die LTE-Datenrate schwankt und der Upload oft geringer ist als der Download.

## Falle

- **Byte und Bit verwechseln:** MB/s ≠ Mbit/s (Faktor 8).
- **Binär und dezimal mischen:** MiB/GiB × 2^20/2^30, Datenraten aber immer 10^6/10^9.
- Overhead **abziehen statt aufschlagen** bzw. bei der falschen Größe ansetzen – sauber formulieren: „Menge × 1,1“ oder „Nutzrate = Rate × 0,9“ (Ergebnisse unterscheiden sich leicht, Annahme nennen).
- Bei asymmetrischen Anschlüssen (ADSL, LTE, Kabel) die **Uploadrate** übersehen.
- Ergebnis ohne Einheit oder ohne Umrechnung in Minuten abgeben.

## Merksatz

**„Erst Byte, dann mal 8, dann Overhead, dann durch die Rate.“** – Leitung dezimal, Datei oft binär.

## Verwandt

- [Bit, Byte, GB und GiB](bit-byte.md) – Einheiten im Detail
- [VoIP & QoS](../Netzwerk-Administration/voip-qos.md) – Codecs und Bandbreitenbedarf
- [WAN-Anschlussarten](../Netzwerk-Administration/wan-anschlussarten.md) – asymmetrische Anschlüsse
- [Auflösung & Speicherbedarf](../../nice-to-know/aufloesung-speicherbedarf.md) – Bild- und Videogrößen
- [Lektion 29 – GA2 Sonstiges](../../analyse-der-exams/L29_GA2_Sonstiges_Praxisthemen.md)
- [Lektion 18 – Netzwerk-Berechnungen](../../analyse-der-exams/L18_Netzwerk_Berechnungen.md)
