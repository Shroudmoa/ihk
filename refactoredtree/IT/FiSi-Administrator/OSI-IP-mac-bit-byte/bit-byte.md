# Bit, Byte, GB und GiB

## In 30 Sekunden

- **1 Byte = 8 Bit.** Kleines `b` = Bit, großes `B` = Byte → `Mbit/s` ≠ `MB/s` (Faktor 8).
- **Dezimal (SI):** kB, MB, GB, TB = 10^3, 10^6, 10^9, 10^12 – Hersteller von Festplatten und **alle Datenraten**.
- **Binär (IEC):** KiB, MiB, GiB, TiB = 2^10, 2^20, 2^30, 2^40 – Anzeige vieler Betriebssysteme, RAM.
- 1 GiB ≈ 1,074 GB; eine „1-TB-Platte“ zeigt Windows als ca. **931 GB** (eigentlich GiB) an.
- Bit-Bedarf: mit n Bit lassen sich **2^n** Zustände darstellen (wichtig für Subnetting und Datentypen).

## Kernwissen

### Einheiten

| Dezimal | Wert | Binär | Wert |
|---|---:|---|---:|
| 1 kB | 1.000 Byte | 1 KiB | 1.024 Byte |
| 1 MB | 1.000.000 Byte | 1 MiB | 1.048.576 Byte |
| 1 GB | 1.000.000.000 Byte | 1 GiB | 1.073.741.824 Byte |
| 1 TB | 10^12 Byte | 1 TiB | 1.099.511.627.776 Byte |

**Abweichung wächst mit der Größe:** KiB/kB = 2,4 %, MiB/MB = 4,9 %, GiB/GB = 7,4 %, TiB/TB = 10,0 %.

### Umrechnen

| Gesucht | Rechnung |
|---|---|
| Bit → Byte | ÷ 8 |
| Byte → Bit | × 8 |
| GB → GiB | ÷ 1,073741824 |
| GiB → GB | × 1,073741824 |
| TB → TiB | ÷ 1,099511627776 |

**Beispiel: Warum zeigt eine 4-TB-Platte weniger an?**
4 × 10^12 Byte ÷ 2^40 = 3,638 TiB → Windows zeigt „3,63 TB“ (meint TiB). Nichts ist verloren, nur die Einheit ist anders; zusätzlich belegt das Dateisystem etwas Platz.

### Datenrate → Durchsatz

| Leitung | theoretisch |
|---|---:|
| 8 Mbit/s | 1 MB/s |
| 100 Mbit/s | 12,5 MB/s |
| 1 Gbit/s | 125 MB/s |
| 2,5 Gbit/s | 312,5 MB/s |
| 10 Gbit/s | 1.250 MB/s |

Praktisch weniger wegen Protokoll-Overhead (Ethernet, IP, TCP) – grob 5–10 %.

### Wie viele Bit brauche ich? (2^n ≥ Anzahl)

| Bedarf | Bit | Beispiel |
|---|---:|---|
| 2 Zustände | 1 | an/aus |
| 256 Werte | 8 | ein Oktett, 0–255 |
| 4.094 VLANs | 12 | 802.1Q-VLAN-ID (2^12 = 4.096, 0 und 4095 reserviert) |
| 65.536 Werte | 16 | Portnummer, IPv6-Gruppe |
| 4,3 Mrd. | 32 | IPv4-Adressraum |

### Hexadezimal

Eine Hexziffer = **4 Bit** (Nibble), zwei Hexziffern = 1 Byte. `F` = 1111, `FF` = 255. Daher: MAC 48 Bit = 12 Hexziffern, IPv6-Gruppe 16 Bit = 4 Hexziffern.

## Prüfungsfrage

**Frage:** Ein Kunde reklamiert, dass sein neues NAS mit 4 × 8 TB im RAID 5 nur „21,8 TB“ anzeigt statt 24 TB. Erklären Sie die Differenz rechnerisch.

**Musterantwort:** RAID 5 mit 4 × 8 TB ergibt (4 − 1) × 8 TB = 24 TB = 24 × 10^12 Byte. Das NAS zeigt binäre Einheiten an: 24 × 10^12 ÷ 2^40 ≈ **21,83 TiB**. Die Kapazität fehlt also nicht; der Hersteller rechnet dezimal (TB), das System binär (TiB). Zusätzlich verringern Dateisystem-Metadaten den nutzbaren Platz geringfügig.

## Falle

- `MB/s` und `Mbit/s` gleichsetzen → Faktor 8 falsch.
- Datenraten binär rechnen (1 Mbit/s = 1.048.576 bit/s) → **falsch**, Datenraten sind immer dezimal.
- GiB-Werte als „Verlust durch Formatierung“ erklären → die Hauptursache ist die **Einheit**.
- Die Abweichung als konstant 2,4 % annehmen → sie steigt mit jeder Stufe.

## Merksatz

**„Klein-b ist Bit, Groß-B ist Byte – i heißt binär.“**

## Verwandt

- [Übertragungszeit & Datenrate](uebertragungszeit-datenrate.md) – Rechenaufgaben mit Overhead
- [Subnetting](subnetting-ipv4-ipv6.md) – Zweierpotenzen in Aktion
- [RAID](../Server/raid.md) – Kapazitäten berechnen
- [Auflösung & Speicherbedarf](../../nice-to-know/aufloesung-speicherbedarf.md) – Bild-/Videogrößen
- [Datentypen](../../UML-Code-SQL/Pseudocode/datentypen.md) – Bit-Bedarf von Datentypen
