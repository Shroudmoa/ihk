# Auflösung und Speicherbedarf (Bild, Video, Druck)

## In 30 Sekunden

- **Unkomprimiertes Bild:** Breite · Höhe · Farbtiefe (Bit) / 8 = Byte.
- **Unkomprimiertes Video:** Breite · Höhe · Farbtiefe · fps · Sekunden / 8 = Byte.
- **Komprimiertes Video:** Bitrate · Sekunden / 8 = Byte (8 Mbit/s → 3,6 GB pro Stunde).
- **Druck/Scan:** Pixel = Zoll · dpi, 1 Zoll = 2,54 cm. A4 bei 300 dpi ≈ 2.480 × 3.508 Pixel.
- Für MiB durch 1.024² teilen, für MB durch 1.000². Die Aufgabe sagt, welche Einheit gilt.
- Geprüft: Webcam-Datenmenge pro Minute (1920 × 1080, 16 Bit, 10 fps) 2022S. Bandbreiten-Rechnungen in GA2.

## Kernwissen

### Auflösungen

| Name | Pixel | Megapixel |
|---|---|---:|
| HD (720p) | 1.280 × 720 | 0,9 |
| Full HD (1080p) | 1.920 × 1.080 | 2,1 |
| QHD (1440p) | 2.560 × 1.440 | 3,7 |
| 4K UHD (2160p) | 3.840 × 2.160 | 8,3 |
| 8K UHD | 7.680 × 4.320 | 33,2 |

### Farbtiefe

| Bit pro Pixel | Farben | Einsatz |
|---:|---:|---|
| 1 | 2 (Schwarz/Weiß) | Fax, Strichzeichnung |
| 8 | 256 | Graustufen, GIF |
| 16 | 65.536 (High Color) | einfache Kameras |
| 24 | 16,7 Mio. (True Color, 3 · 8 Bit RGB) | Fotos, Monitor |
| 32 | 16,7 Mio. + 8 Bit Transparenz (Alpha) | Grafik, PNG |

### Rechenweg 1: Webcam pro Minute (Muster 2022S)

1.920 × 1.080 Pixel, 16 Bit Farbtiefe, 10 Bilder/s, unkomprimiert, 1 Minute:

1. Bits: 1.920 · 1.080 · 16 · 10 · 60 = 19.906.560.000 Bit
2. Byte: / 8 = 2.488.320.000 Byte
3. MiB: / 1.024 / 1.024 ≈ **2.373 MiB pro Minute** (≈ 2,32 GiB)

Weitere Angaben wie „Aufzeichnung 5 Stunden“ sind für „pro Minute“ irrelevant. Hochrechnen nur, wenn danach gefragt ist: 5 h = 300 min → ≈ 695 GiB.

### Rechenweg 2: Foto

24-Megapixel-Foto (6.000 × 4.000), 24 Bit:
6.000 · 4.000 · 24 / 8 = 72.000.000 Byte ≈ **68,7 MiB** unkomprimiert. Als JPEG (≈ 1:10) etwa 7 MB.

### Rechenweg 3: Druck und Scan (dpi)

- **Pixel = Größe in Zoll · dpi** bzw. **Größe in cm = Pixel / dpi · 2,54**
- A4 = 21,0 × 29,7 cm = 8,27 × 11,69 Zoll. Bei 300 dpi: 8,27 · 300 ≈ 2.480 und 11,69 · 300 ≈ 3.508 Pixel.
- Scan A4, 300 dpi, 24 Bit: 2.480 · 3.508 · 3 Byte = 26.099.520 Byte ≈ **24,9 MiB** unkomprimiert.
- Wie groß lässt sich das 24-MP-Foto bei 300 dpi drucken? 6.000 / 300 · 2,54 = **50,8 cm** × 4.000 / 300 · 2,54 ≈ **33,9 cm**.
- Faustwerte: 300 dpi für Fotodruck, 150 dpi für Poster aus Entfernung, 72–96 dpi am Bildschirm.

### Rechenweg 4: Komprimiertes Video über die Bitrate

Speicher = Bitrate · Zeit / 8

| Stream | Bitrate (typisch) | pro Stunde | pro Tag (24 h) |
|---|---:|---:|---:|
| Full HD, H.264, 25 fps | 4 Mbit/s | 1,8 GB | 43,2 GB |
| Full HD, H.265 | 2 Mbit/s | 0,9 GB | 21,6 GB |
| 4K, H.265 | 12 Mbit/s | 5,4 GB | 129,6 GB |

Beispiel: 8 Kameras Full HD H.264 (4 Mbit/s), 24/7, Aufbewahrung 3 Tage (Löschfrist):
8 · 43,2 GB · 3 = **1.036,8 GB ≈ 1,04 TB**. Benötigte Bandbreite zum Rekorder: 8 · 4 = 32 Mbit/s.

### Rechenweg 5: Audio (PCM, CD-Qualität)

44.100 Hz · 16 Bit · 2 Kanäle = 1.411.200 Bit/s ≈ 1,41 Mbit/s → pro Minute 1.411.200 · 60 / 8 = 10.584.000 Byte ≈ **10,1 MiB**.

## Prüfungsfrage

**Frage:** Eine Webcam liefert 1.280 × 720 Pixel mit 24 Bit Farbtiefe und 15 Bildern pro Sekunde (unkomprimiert). Berechnen Sie die Datenmenge pro Minute in MiB.

**Musterantwort:** 1.280 · 720 · 24 Bit · 15 · 60 = 19.906.560.000 Bit. Geteilt durch 8 ergibt das 2.488.320.000 Byte, geteilt durch 1.024² ≈ **2.373 MiB pro Minute**. (Zufällig derselbe Wert wie bei 1.920 × 1.080 mit 16 Bit und 10 fps, denn beide Produkte sind gleich groß.)

## Falle

- Durch 8 teilen vergessen (Bit → Byte).
- MB und MiB mischen: 1 MiB = 1.048.576 Byte.
- Die Zeit nicht in Sekunden umrechnen.
- Bei Druckaufgaben cm direkt mit dpi multiplizieren. Erst in Zoll umrechnen (/ 2,54).
- Unkomprimierte Rechnung für Speicherplanung echter Kameras nehmen. Real wird über die Bitrate des Codecs gerechnet.
- Irrelevante Angaben (Aufzeichnungsdauer, Anzahl Kameras) einrechnen, obwohl nach einem Einzelwert gefragt ist.

## Merksatz

**„Pixel mal Pixel mal Bit mal fps mal Sekunden, durch 8, durch 1.024 zum Quadrat.“**

## Verwandt

- [Video- & Fotoformate](video-photo-formats.md) – Codecs und Kompression
- [Begriffe Kamera & Netzwerk](begriffe.md) – PoE, Bewegungserkennung
- [Bit-Byte](../FiSi-Administrator/OSI-IP-mac-bit-byte/bit-byte.md) – Einheiten und Präfixe
- [Kapazitätsplanung](../Komponenten/Storage/kapazitaetsplanung.md) – Speicherbedarf hochrechnen
- [DAS-NAS-SAN](../Komponenten/Storage/das-nas-san.md) – Ablage der Aufnahmen
- [Lektion L28 GA1 Praxisthemen](../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md)
- [Lektion 04 Programmierung](../analyse-der-exams/04_Programmierung_Grundlagen.md) – Bit-Bedarf berechnen
