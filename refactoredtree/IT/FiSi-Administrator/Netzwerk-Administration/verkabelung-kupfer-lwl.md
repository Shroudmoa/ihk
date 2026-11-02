# Verkabelung – Kupfer, Lichtwellenleiter, Stecker & SFP

## In 30 Sekunden

- **Twisted-Pair-Kupfer** (RJ45) reicht maximal **100 m** pro Link (90 m Installation + 10 m Patchkabel); Gigabit braucht **alle 4 Adernpaare** und mindestens **Cat 5e**, 10 Gbit/s über 100 m **Cat 6A**.
- **Lichtwellenleiter (LWL):** **Multimode** (Kern 50 µm, bis einige hundert Meter, Gebäude/RZ) und **Singlemode** (Kern 9 µm, Kilometer, zwischen Gebäuden/Standorten).
- Vorteile LWL (GA2 Winter 2023): **Reichweite, Bandbreite, unempfindlich gegen elektromagnetische Störungen, galvanische Trennung**, schwer abhörbar.
- Stecker: **RJ45** (Kupfer), **LC** (klein, an SFP-Modulen), **SC**, **ST**, **MPO** (viele Fasern).
- Transceiver: **SFP = 1 Gbit/s**, **SFP+ = 10 Gbit/s** (GA2 Sommer 2023) – Modul muss zu Fasertyp, Wellenlänge und Gegenstelle passen (GA2 Winter 2025).

## Kernwissen

### Kupfer-Kategorien

| Kategorie | Frequenz | Typische Ethernet-Nutzung | Hinweis |
|---|---:|---|---|
| Cat 5 | 100 MHz | 100 Mbit/s | veraltet; Ursache für „nur 100 Mbit/s“ |
| **Cat 5e** | 100 MHz | **1 Gbit/s** (2,5 Gbit/s) | Mindeststandard für Gigabit |
| Cat 6 | 250 MHz | 1 Gbit/s, 10 Gbit/s bis ca. 55 m | |
| **Cat 6A** | 500 MHz | **10 Gbit/s bis 100 m** | heutiger Standard für Neuinstallationen |
| Cat 7 / 7A | 600 / 1000 MHz | 10 Gbit/s | Kabelstandard, offiziell mit GG45/TERA-Steckern |
| Cat 8 | 2000 MHz | 25/40 Gbit/s bis **30 m** | Rechenzentrum, kurze Strecken |

**Ethernet-Standards:** 100BASE-TX (2 Paare), **1000BASE-T (4 Paare)**, 2,5G/5GBASE-T (Cat 5e/6), **10GBASE-T (Cat 6A)**.

**Schirmung (Bezeichnung X/YTP):** erster Buchstabe = Gesamtschirm, zweiter = Paarschirm; U = ungeschirmt, F = Folie, S = Geflecht. Beispiel **S/FTP** = Geflechtschirm gesamt + Folie je Paar (Standard für Cat 7).

**Belegung und Kabeltypen:** TIA-568A/B; **Patchkabel (1:1)** für PC ↔ Switch, **Crossover-Kabel** (gekreuzt) früher für Switch ↔ Switch oder PC ↔ PC. Heute erkennt **Auto-MDI-X** die Belegung automatisch.

### Lichtwellenleiter

| | Multimode (MM) | Singlemode (SM) |
|---|---|---|
| Kerndurchmesser | 50 µm (alt 62,5 µm) | **9 µm** |
| Lichtquelle | LED / VCSEL | Laser |
| Wellenlänge | **850 nm** (1300 nm) | **1310 / 1550 nm** |
| Reichweite 10 Gbit/s | OM3 bis 300 m, **OM4 bis 400 m** | 10 km (LR), 40 km (ER), 80 km (ZR) |
| Klassen / Mantelfarbe | OM1/OM2 orange, OM3/OM4 türkis (OM4 auch violett), OM5 limettengrün | OS1/OS2 **gelb** |
| Kosten | Transceiver günstiger | Transceiver teurer, Faser günstig |
| Einsatz | Gebäude, Rechenzentrum, Etagenverteiler | **zwischen Gebäuden**, Campus, Provider |

**Stecker**
| Stecker | Merkmal |
|---|---|
| **LC** | klein, Push-Pull mit Rastnase, Duplex – Standard an SFP/SFP+ |
| SC | quadratisch, Push-Pull, größer |
| ST | Bajonett, veraltet |
| MPO/MTP | 12/24 Fasern in einem Stecker – 40/100G im RZ |
| Schliff UPC / APC | UPC (blau bei SM) gerader Schliff; **APC (grün)** 8° schräg, geringere Rückreflexion – nicht mischen! |

### Transceiver-Module

| Modul | Datenrate | Typische Varianten |
|---|---:|---|
| **SFP** | 1 Gbit/s | 1000BASE-SX (MM, 850 nm), 1000BASE-LX (SM, 1310 nm), 1000BASE-T (RJ45) |
| **SFP+** | 10 Gbit/s | 10GBASE-SR (MM, 850 nm), 10GBASE-LR (SM, 1310 nm), DAC-Kupferkabel |
| SFP28 | 25 Gbit/s | Server-Anbindung im RZ |
| QSFP+ / QSFP28 | 40 / 100 Gbit/s | Core/Spine |

SFP+-Ports nehmen meist auch SFP-Module (dann 1 Gbit/s); umgekehrt nicht.

**Uplink funktioniert nicht (GA2 Winter 2025) – Prüfliste:** Modul passt nicht zum Fasertyp (SR an Singlemode), Wellenlänge/Typ unterschiedlich an beiden Enden, Hersteller sperrt Fremdmodule (Codierung), Rx/Tx vertauscht, verschmutzte Stecker, Port administrativ down oder falsche Geschwindigkeit konfiguriert.

### Strukturierte Verkabelung (EN 50173)

| Bereich | Verbindet | Medium |
|---|---|---|
| **Primär** (Campus) | Gebäude untereinander | LWL Singlemode |
| **Sekundär** (Steigbereich) | Gebäudeverteiler ↔ Etagenverteiler | LWL Multimode (oder Kupfer) |
| **Tertiär** (Etage) | Etagenverteiler ↔ Anschlussdose | Kupfer Cat 6A, max. 90 m |

### Kupfer vs. LWL – Entscheidung begründen

| Kriterium | Kupfer | LWL |
|---|---|---|
| Reichweite | 100 m | Hunderte Meter bis viele km |
| Bandbreite | bis 10G (Cat 6A), 40G kurz | 10/40/100G+ |
| EMV | störanfällig (Maschinen, Starkstrom) | **immun** |
| Potential | leitend → Ausgleichsströme zwischen Gebäuden | **galvanisch getrennt**, kein Blitz-/Potentialproblem |
| Abhören | Abstrahlung möglich | nur durch physischen Eingriff |
| PoE | ja | nein |
| Handhabung | robust, einfach zu konfektionieren | Biegeradius, Spleißen, Reinigung |

## Prüfungsfrage

**Frage (angelehnt an GA2 Winter 2023):** Zwei Gebäude im Abstand von 150 m sollen mit 10 Gbit/s verbunden werden. Begründen Sie, warum Glasfaser statt Kupfer gewählt wird, und schlagen Sie Fasertyp, Transceiver und Stecker vor.

**Musterantwort:** Twisted-Pair-Kupfer ist auf 100 m pro Link begrenzt, 150 m sind damit nicht normgerecht möglich. Glasfaser überbrückt die Strecke problemlos, ist unempfindlich gegen elektromagnetische Störungen und trennt die Gebäude galvanisch, sodass keine Ausgleichsströme oder Blitzschäden übertragen werden. Für 150 m mit 10 Gbit/s eignet sich Multimode OM4 mit **SFP+-Modulen 10GBASE-SR** (850 nm); zukunftssicherer zwischen Gebäuden ist Singlemode (OS2) mit **10GBASE-LR** (1310 nm). Die Module nutzen **LC-Duplex-Stecker**; auf beiden Seiten müssen gleiche Modultypen eingesetzt werden.

## Falle

- „Cat 5 reicht für Gigabit“ → Mindestnorm ist **Cat 5e**; defekte Paare führen zu 100 Mbit/s.
- 100 m als reine Kabellänge in der Wand rechnen → 90 m fest + 10 m Patchkabel.
- Multimode-Module an Singlemode-Fasern (oder umgekehrt) → kein Link.
- SFP und SFP+ verwechseln (1 vs. 10 Gbit/s).
- APC- und UPC-Stecker kombinieren → hohe Dämpfung, Steckerschaden.
- LWL „weil schneller“ begründen → Licht ist im Glas nicht schneller als das Signal im Kupfer; Vorteil ist **Bandbreite und Reichweite**.

## Merksatz

**„Kupfer: 100 Meter, 4 Paare, 6A für 10G. Glas: gelb ist Single und weit, türkis ist Multi und kurz – LC steckt im SFP.“**

## Verwandt

- [Switches – Grundlagen](switches-und-vlans.md) – Uplinks, 100-Mbit/s-Problem
- [PoE](poe.md) – Strom über Kupfer
- [Netzwerkgeräte](netzwerkgeraete.md) – Medienkonverter, Patchfeld
- [Link Aggregation (LACP)](link-aggregation-lacp.md) – mehrere Uplinks bündeln
- [WAN-Anschlussarten](wan-anschlussarten.md) – Glasfaser beim Provider (FTTH, GPON)
- [Crossover und Failover](../../nice-to-know/crossover-failover.md) – Crossover-Kabel
- [OSI](../OSI-IP-mac-bit-byte/osi.md) – Schicht 1
- [Lektion 29 – GA2 Sonstiges](../../analyse-der-exams/L29_GA2_Sonstiges_Praxisthemen.md)
