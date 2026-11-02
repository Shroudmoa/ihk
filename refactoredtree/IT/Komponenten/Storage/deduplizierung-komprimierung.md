# Deduplizierung und Komprimierung

## In 30 Sekunden

- **Deduplizierung** findet **identische Blöcke oder Dateien über viele Dateien hinweg** und speichert sie nur einmal. Die übrigen Vorkommen werden zu Verweisen.
- **Komprimierung** verkleinert **eine einzelne Datei bzw. einen Block** durch Kodierung von Wiederholungen (z. B. ZIP, LZ4, gzip).
- Beide sparen Speicher, Bandbreite und Kosten. Besonders stark wirken sie bei **Backups** und **VM-/VDI-Images**.
- Verschlüsselte oder bereits komprimierte Daten (JPEG, MP4, ZIP) lassen sich kaum weiter verkleinern.
- Geprüft: 2021W (Unterschied mit Beispiel), 2024W (Vorteile der Deduplizierung).

## Kernwissen

### Vergleich

| | Deduplizierung | Komprimierung |
|---|---|---|
| Wirkungsbereich | über **viele** Dateien/VMs/Backups hinweg | **innerhalb** einer Datei bzw. eines Blocks |
| Prinzip | Hash je Block → schon vorhanden? → nur Verweis speichern | Muster und Wiederholungen kürzer kodieren |
| Beispiel | 50 VMs mit gleichem Windows-Image: Systemblöcke nur einmal | Logdatei mit 1 GiB wird als `.gz` 100 MiB groß |
| typische Ersparnis | Backups 10:1 bis 30:1 | Text/Logs 70–90 %, Fotos/Videos ≈ 0 % |
| Kosten | RAM/CPU für Hash-Index, Risiko bei Index-Defekt | CPU-Last beim Packen und Entpacken |

### Varianten der Deduplizierung

| Variante | Bedeutung |
|---|---|
| **Datei-Ebene** | identische Dateien nur einmal (Single Instance Storage) |
| **Block-Ebene** | identische Blöcke, findet auch Teilgleichheiten. Wirkt stärker |
| **Inline** | vor dem Schreiben deduplizieren → sofort weniger Platz, braucht mehr Rechenleistung |
| **Post-Process** | erst schreiben, später deduplizieren → schnelleres Schreiben, braucht zeitweise vollen Platz |
| **Quellseitig** | schon am Client/Server, bevor Daten übers Netz gehen → spart Bandbreite (z. B. Backup in die Cloud) |
| **Zielseitig** | auf dem Backup-Speicher |

### Komprimierung: verlustfrei vs. verlustbehaftet

- **Verlustfrei** (ZIP, gzip, LZ4, PNG, FLAC): Original exakt wiederherstellbar. Pflicht für Programme, Datenbanken, Dokumente.
- **Verlustbehaftet** (JPEG, MP3, H.264/H.265): Details werden verworfen, sehr hohe Ersparnis. Nur für Medien → [Video- & Fotoformate](../../nice-to-know/video-photo-formats.md).

### Rechenweg: Dedup-Faktor

Wöchentlich 4 TiB Vollbackup, 8 Wochen aufbewahrt = 32 TiB logisch. Mit Dedup-Faktor 8:1 belegen sie physisch 32 / 8 = **4 TiB**.
Ersparnis in Prozent: 1 − 1/8 = **87,5 %**.

### Reihenfolge mit Verschlüsselung

**Erst deduplizieren/komprimieren, dann verschlüsseln.** Verschlüsselte Daten sehen zufällig aus, und gleiche Klartextblöcke ergeben verschiedene Chiffrate.

## Prüfungsfrage

**Frage:** Erläutern Sie den Unterschied zwischen Deduplizierung und Komprimierung jeweils an einem Beispiel.

**Musterantwort:** Deduplizierung erkennt identische Datenblöcke in verschiedenen Dateien und speichert sie nur einmal. Alle weiteren Vorkommen verweisen auf diesen Block. Beispiel: Beim Backup von 40 virtuellen Maschinen mit demselben Betriebssystem werden die gleichen Systemblöcke nur einmal abgelegt. Komprimierung verkleinert eine einzelne Datei, indem wiederkehrende Muster kürzer kodiert werden. Beispiel: Eine große Textlogdatei wird als ZIP-Archiv auf einen Bruchteil ihrer Größe gepackt und lässt sich verlustfrei wieder entpacken.

## Falle

- Beides gleichsetzen. Der Unterschied ist **über Dateien hinweg** vs. **innerhalb einer Datei**.
- Bei JPEG-Fotos oder MP4-Videos große Komprimierungsgewinne versprechen. Sie sind bereits komprimiert.
- Dedup-Speicher ohne Backup des Index. Ist der Index defekt, sind alle Verweise wertlos.
- „Komprimierungsrate 60 %“ ungeprüft rechnen. Ist die Einsparung gemeint (bleibt 40 %) oder der Rest (bleibt 60 %)? Annahme hinschreiben.

## Merksatz

**„Dedup findet Zwillinge, Komprimierung packt den Koffer.“**

## Verwandt

- [Backup-Strategien](../../FiSi-Administrator/Server/backup/backup-strategien.md)
- [Archivierung revisionssicher](../../FiSi-Administrator/Server/backup/archivierung-revisionssicher.md)
- [Kapazitätsplanung](kapazitaetsplanung.md)
- [Dateisysteme](dateisysteme.md) – ZFS/ReFS mit eingebauter Dedup
- [VDI & Thin Client](../../FiSi-Administrator/Server/virtualisierung/vdi-thin-client.md) – viele gleiche Images
- [Lektion L28 GA1 Praxisthemen](../../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md)
