# Video- und Fotoformate

## In 30 Sekunden

- **Codec** = Verfahren, das Video/Audio komprimiert (H.264, H.265, AV1). **Container** = Dateiformat, das Video, Audio und Untertitel zusammenhält (.mp4, .mkv).
- **H.265 (HEVC)** spart gegenüber **H.264 (AVC)** ca. **50 %** bei gleicher Qualität, braucht aber mehr Rechenleistung. **MJPEG** komprimiert nur Einzelbilder → sehr groß.
- Fotos: **JPEG** verlustbehaftet und klein, **PNG** verlustfrei mit Transparenz, **RAW** unbearbeitete Sensordaten, **HEIC/WebP/AVIF** moderne effiziente Formate.
- **Verlustbehaftet** verwirft Details dauerhaft, **verlustfrei** stellt das Original exakt wieder her.
- Prüfungsbezug: Kamera-/Webcam-Auswahl 2022S, Speicherbedarf von Videoaufzeichnungen → [Auflösung & Speicherbedarf](aufloesung-speicherbedarf.md).

## Kernwissen

### Video-Codecs

| Codec | Kompression | Rechenaufwand | Einsatz |
|---|---|---|---|
| **MJPEG** | jedes Bild einzeln als JPEG (nur intra-frame) | gering | ältere Kameras, sehr hoher Speicherbedarf |
| **H.264 / AVC** | Bewegungskompensation zwischen Bildern (I-, P-, B-Frames) | mittel | Standard für Streaming, Videokonferenz, Überwachung |
| **H.265 / HEVC** | ca. 50 % kleiner als H.264 | hoch | 4K, Überwachung mit langer Aufbewahrung. Lizenzgebühren |
| **VP9 / AV1** | ähnlich bzw. besser als H.265 | hoch | YouTube, Browser-Streaming. AV1 lizenzfrei |

**Inter-Frame-Kompression:** Nur Änderungen gegenüber dem Vorbild werden gespeichert. Bei ruhigen Bildern (leerer Flur) ist die Datenrate sehr gering, bei viel Bewegung (Regen, Bäume) steigt sie.

### Container

| Container | typische Inhalte | Hinweis |
|---|---|---|
| .mp4 | H.264/H.265 + AAC | universell abspielbar |
| .mkv | fast alle Codecs, mehrere Tonspuren/Untertitel | flexibel |
| .avi | ältere Codecs, MJPEG | veraltet |
| .mov | Apple QuickTime | Apple-Umfeld |

### Fotoformate

| Format | Kompression | Transparenz | Einsatz |
|---|---|---|---|
| **JPEG** | verlustbehaftet | nein | Fotos, Web. Mehrfaches Speichern verschlechtert die Qualität |
| **PNG** | verlustfrei | ja | Screenshots, Logos, Grafiken mit Text |
| **GIF** | verlustfrei, max. 256 Farben | 1 Bit | einfache Animationen |
| **TIFF** | verlustfrei oder unkomprimiert | ja | Druckvorstufe, Archivierung von Scans |
| **RAW** | unbearbeitete Sensordaten | – | professionelle Fotografie, große Dateien |
| **HEIC / WebP / AVIF** | modern, effizient | ja | Smartphones, Web |
| **SVG** | Vektorgrafik (XML) | ja | Logos, Icons, beliebig skalierbar |

## Prüfungsfrage

**Frage:** Für 12 Überwachungskameras soll der Speicherbedarf des Rekorders gesenkt werden. Nennen Sie zwei technische Maßnahmen.

**Musterantwort:** Erstens kann auf einen effizienteren Codec umgestellt werden, etwa von H.264 auf H.265. H.265 benötigt bei gleicher Bildqualität etwa die halbe Datenrate. Zweitens kann die Aufzeichnung per Bewegungserkennung gesteuert werden: Statt rund um die Uhr wird nur aufgezeichnet, wenn sich im Bild etwas bewegt. Weitere Möglichkeiten sind eine niedrigere Bildrate (z. B. 10 statt 25 fps) oder eine geringere Auflösung, wo keine Details nötig sind.

## Falle

- Codec und Container verwechseln: „.mp4 ist ein Codec“ ist falsch.
- JPEG für Screenshots mit Text. Es entstehen Artefakte, PNG ist besser.
- H.265 empfehlen, ohne zu prüfen, ob Rekorder und Clients es dekodieren können.
- Komprimierte Formate (JPEG, MP4) durch ZIP weiter verkleinern wollen. Das bringt kaum etwas.

## Merksatz

**„Codec packt, Container trägt. H.265 halbiert, MJPEG verschwendet.“**

## Verwandt

- [Auflösung & Speicherbedarf](aufloesung-speicherbedarf.md) – Rechenwege
- [Begriffe Kamera & Netzwerk](begriffe.md) – Bewegungserkennung, PoE
- [Deduplizierung & Komprimierung](../Komponenten/Storage/deduplizierung-komprimierung.md) – verlustfrei vs. verlustbehaftet
- [Archivierung revisionssicher](../FiSi-Administrator/Server/backup/archivierung-revisionssicher.md) – offene Formate (PDF/A, TIFF)
