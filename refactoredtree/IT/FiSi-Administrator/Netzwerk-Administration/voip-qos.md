# VoIP & QoS

## In 30 Sekunden

- **VoIP** trennt **Signalisierung** (SIP, Port 5060/5061) und **Sprachdaten** (RTP über **UDP**); der Codec bestimmt die Bandbreite (G.711/G.722 = 64 kbit/s, G.729 = 8 kbit/s).
- Sprache ist **echtzeitkritisch**: Richtwerte **Latenz < 150 ms** (Einweg), **Jitter < 30 ms**, **Paketverlust < 1 %**.
- **QoS** = Verkehr klassifizieren, **markieren** (802.1p auf Schicht 2, **DSCP EF = 46** auf Schicht 3) und **priorisieren** – Sprache vor Downloads.
- Ein eigenes **Voice-VLAN** erleichtert QoS und trennt die Telefone vom Datennetz (GA2 Winter 2025).
- Rechenaufgabe (GA2 Winter 2022): **Bandbreite = Codec × (1 + Overhead) × Anzahl gleichzeitiger Gespräche**.

## Kernwissen

### Bausteine

| Komponente | Aufgabe |
|---|---|
| IP-Telefon / Softphone | Endgerät, meist per PoE versorgt |
| **SIP** (UDP/TCP 5060, TLS 5061) | Verbindungsauf- und -abbau, Registrierung an der Telefonanlage |
| **RTP** (UDP, dynamische Ports) | Transport der Sprachpakete; **SRTP** = verschlüsselt |
| TK-Anlage / IP-PBX | Rufnummern, Weiterleitungen, Voicemail (on-premises oder Cloud) |
| SIP-Trunk | Anbindung an das öffentliche Telefonnetz über den Provider |
| Session Border Controller | „Firewall“ für VoIP am Übergang ins Internet, NAT-Handling |

### Codecs

| Codec | Bitrate (Nutzdaten) | Qualität |
|---|---:|---|
| G.711 (a-law/µ-law) | 64 kbit/s | ISDN-Qualität, unkomprimiert |
| **G.722** | 64 kbit/s | **HD-Voice** (Wideband) |
| G.729 | 8 kbit/s | komprimiert, spart Bandbreite |
| Opus | 6–510 kbit/s | adaptiv, Videokonferenz-Software |

**Overhead:** Jedes Sprachpaket (alle 20 ms) trägt IP-, UDP-, RTP- und Ethernet-Header. In Prüfungen wird meist ein pauschaler Aufschlag (z. B. **10 %**) vorgegeben; real liegt G.711 über Ethernet bei rund 87 kbit/s je Richtung.

### Bandbreite berechnen (GA2 Winter 2022)

G.722 (64 kbit/s), 10 % Overhead, 5 interne + 15 externe gleichzeitige Gespräche:
1. pro Gespräch: 64 × 1,1 = **70,4 kbit/s**
2. Gespräche über den Internetanschluss: nur die **15 externen** (interne bleiben im LAN) → 15 × 70,4 = **1.056 kbit/s**
3. Werden alle 20 über die zentrale Anlage/den Anschluss geführt: 20 × 70,4 = **1.408 kbit/s**

Aufgabentext genau lesen, welche Gespräche über welche Leitung laufen – und Annahme hinschreiben. Sprache ist **symmetrisch** → gleicher Bedarf im Up- und Download.

### Qualitätsmerkmale

| Größe | Bedeutung | Richtwert | Folge bei Überschreitung |
|---|---|---|---|
| Latenz (Delay) | Laufzeit Mund → Ohr | < 150 ms einfach (ITU-T G.114) | Gesprächspartner fallen sich ins Wort |
| **Jitter** | Schwankung der Laufzeit | < 30 ms | abgehackte Sprache, Jitter-Buffer erhöht Latenz |
| Paketverlust | verlorene Pakete | < 1 % | Aussetzer, Roboterstimme |
| MOS | Mean Opinion Score 1–5 | ≥ 4 gut | subjektive Sprachqualität |

### QoS-Mechanismen

| Schritt | Umsetzung |
|---|---|
| **Klassifizieren** | Verkehr erkennen (Voice-VLAN, Ports, DSCP vom Telefon) |
| **Markieren** | Schicht 2: **802.1p / PCP** (3 Bit im VLAN-Tag, Voice = 5); Schicht 3: **DSCP** (6 Bit im IP-Header, Voice = **EF 46**, Signalisierung CS3) |
| **Warteschlangen** | Priority Queue / LLQ: Sprache wird immer zuerst gesendet |
| **Shaping / Policing** | Datenverkehr glätten bzw. begrenzen, Bandbreite für Sprache reservieren |
| **Ende-zu-Ende** | QoS wirkt nur, wo es konfiguriert ist – im Internet meist nicht; am **Engpass** (Upload des Internetanschlusses) ist es am wichtigsten |

### Eigenes VLAN für VoIP (GA2 Winter 2025, „zwei Gründe“)

- **QoS:** Sprachpakete lassen sich anhand des VLANs einfach erkennen und priorisieren.
- **Sicherheit:** Telefone (oft schlecht gepatcht) sind vom Datennetz getrennt; Mitschneiden/Angriffe auf Gespräche werden erschwert.
- **Übersicht/Betrieb:** eigener DHCP-Bereich mit TFTP-/Provisionierungs-Option, Broadcasts getrennt.

### Fax über VoIP (GA2 Winter 2022)

Fax überträgt ein **analoges Modemsignal**, das auf eine konstante, verlustfreie Leitung angewiesen ist. Komprimierende Sprachcodecs, **Jitter und Paketverlust** zerstören das Signal → Abbrüche, vor allem bei mehrseitigen Faxen. Lösung: **T.38** (Fax over IP), G.711-Passthrough ohne Kompression, oder Fax-to-Mail-Dienst.

### Videokonferenz

Bei zu wenig Bandbreite priorisiert die Software **Audio vor Video** und reduziert Auflösung/Framerate. Bandbreitenbedarf hängt ab von Auflösung, Bildrate, Anzahl sichtbarer Teilnehmer (Galerie) und Bildschirmfreigabe. Rechenbeispiel: [Übertragungszeit & Datenrate](../OSI-IP-mac-bit-byte/uebertragungszeit-datenrate.md).

## Prüfungsfrage

**Frage (angelehnt an GA2 Winter 2022):** Eine Firma führt VoIP mit dem Codec G.722 ein. Über den Internetanschluss sollen bis zu 25 Gespräche gleichzeitig möglich sein; es ist mit 10 % Overhead zu rechnen. Berechnen Sie die benötigte Bandbreite und nennen Sie zwei Maßnahmen, die die Sprachqualität sichern, wenn gleichzeitig große Downloads laufen.

**Musterantwort:** Pro Gespräch: 64 kbit/s × 1,1 = 70,4 kbit/s; 25 Gespräche: 25 × 70,4 = **1.760 kbit/s ≈ 1,76 Mbit/s**, und zwar in **beide Richtungen**, da Sprache symmetrisch ist. Maßnahmen: (1) **QoS** am Router/Firewall: Sprachpakete per DSCP EF markieren und in einer Prioritätswarteschlange bevorzugt senden, Bandbreite für Sprache reservieren. (2) **Eigenes Voice-VLAN** mit 802.1p-Priorisierung im LAN, damit Sprache auch auf den Switches Vorrang hat. Zusätzlich kann der Downloadverkehr per Traffic-Shaping begrenzt werden.

## Falle

- Nur den Download betrachten → Sprache braucht **Upload in gleicher Höhe** (ADSL-Upload ist oft der Engpass).
- Overhead vergessen oder auf die Gesamtleitung statt pro Gespräch rechnen.
- VoIP-Sprachdaten über TCP → **RTP über UDP**; nur die Signalisierung kann TCP nutzen.
- „QoS erzeugt mehr Bandbreite“ → QoS **verteilt** vorhandene Bandbreite nur anders.
- Jitter und Latenz gleichsetzen: Latenz = Verzögerung, Jitter = **Schwankung** der Verzögerung.

## Merksatz

**„SIP klingelt, RTP spricht – über UDP. QoS lässt die Sprache an der Schlange vorbei.“**

## Verwandt

- [VLAN](vlan.md) – Voice-VLAN, 802.1p im Tag
- [PoE](poe.md) – Strom für IP-Telefone
- [TCP vs. UDP](../OSI-IP-mac-bit-byte/tcp-udp.md) – warum Echtzeit UDP nutzt
- [Ports](../OSI-IP-mac-bit-byte/ports.md) – SIP 5060/5061
- [Übertragungszeit & Datenrate](../OSI-IP-mac-bit-byte/uebertragungszeit-datenrate.md) – Bandbreitenrechnungen
- [WAN-Anschlussarten](wan-anschlussarten.md) – symmetrische vs. asymmetrische Anschlüsse
- [Lektion 29 – GA2 Sonstiges (QoS, Fax)](../../analyse-der-exams/L29_GA2_Sonstiges_Praxisthemen.md)
