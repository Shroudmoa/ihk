# WAN- und Internet-Anschlussarten

## In 30 Sekunden

- „Letzte Meile“ zuordnen (GA2 Sommer 2022): **VDSL = Telefonleitung (Kupferdoppelader)**, **DOCSIS = Koaxialkabel (TV-Kabel)**, **LTE/5G = Mobilfunk**, **FTTH = Glasfaser**.
- **Shared Medium** (Kabel, GPON, Mobilfunk): Bandbreite wird mit Nachbarn geteilt → zu Stoßzeiten weniger Durchsatz, keine Garantie (GA2 Sommer 2025, Winter 2025).
- **Geschäftskunden-Anschluss** vs. Privat: **SLA** (Verfügbarkeit, Entstörzeit), **feste öffentliche IPs**, oft **symmetrisch**, priorisierter Support (2025S).
- **CGNAT / DS-Lite**: Kunde hat keine eigene öffentliche IPv4 → **keine Portweiterleitung von außen** möglich (2023W, 2025W).
- Hochverfügbarkeit: **zwei Provider über unterschiedliche Medien**, Verfügbarkeit parallel: 1 − (1 − A1)(1 − A2).

## Kernwissen

### Anschlussarten

| Technik | Medium letzte Meile | Typische Raten (down/up) | Merkmale |
|---|---|---|---|
| ADSL2+ | Telefon-Kupferdoppelader | 16 / 1 Mbit/s | stark **asymmetrisch**, Rate sinkt mit Leitungslänge |
| VDSL2 / Vectoring / Supervectoring | Kupfer ab Kabelverzweiger (**FTTC**) | 50–250 / 10–40 Mbit/s | asymmetrisch, kurze Kupferstrecke |
| Kabel (DOCSIS 3.0/3.1) | **Koaxialkabel** | bis 1.000 / 50–100 Mbit/s | **Shared Medium** im Segment |
| **FTTB** | Glasfaser bis ins Gebäude, dann Kupfer | bis 1 Gbit/s | |
| **FTTH** (GPON/XGS-PON) | Glasfaser bis in die Wohnung/Firma | 1 Gbit/s+ | GPON: passive Splitter (z. B. 1:32), **geteilte** 2,5/1,25 Gbit/s |
| LTE / 5G | **Mobilfunk** | stark schwankend | Volumentarife, schwankende Latenz/Jitter, oft **CGNAT**; gut als Backup oder Übergang |
| Satellit (GEO / LEO) | Funk | GEO ~600 ms Latenz, LEO ~30–60 ms | abgelegene Standorte, wetterabhängig |
| Standleitung / Ethernet-Anschluss | dedizierte Glasfaser | symmetrisch, garantiert (z. B. 1 Gbit/s) | **nicht geteilt**, SLA, teuer |
| MPLS / SD-WAN | über Providernetz bzw. mehrere Leitungen | – | Standortvernetzung mit QoS bzw. zentraler Steuerung |

**FTTx-Merkhilfe:** **C**urb (Kabelverzweiger am Straßenrand) → **B**uilding (Keller) → **H**ome (Wohnung).

### Geschäfts- vs. Privatanschluss (GA2 Sommer 2025, „nenne drei“)

| Merkmal | Privat | Geschäft |
|---|---|---|
| Bandbreite | „bis zu“, Best Effort | zugesicherte Mindestbandbreite |
| Verfügbarkeit | keine Zusage | **SLA**, z. B. 99,5 % |
| Entstörung | Werktage, Tage | z. B. **8 h**, 24/7 |
| IP-Adressen | dynamisch, ggf. CGNAT | **feste öffentliche IPv4/IPv6-Präfixe** |
| Upload | gering | höher oder symmetrisch (VPN, Server, Cloud-Backup) |
| Support | Hotline | bevorzugt, fester Ansprechpartner |

### Shared Medium – warum problematisch?

Bei Kabel, GPON und Mobilfunk teilen sich mehrere Kunden die Kapazität eines Segments. Nutzen Nachbarn viel Bandbreite (abends), sinkt der eigene Durchsatz, Latenz und Jitter steigen – kritisch für Kassensysteme, VPN, VoIP und Videokonferenzen. Abhilfe: Geschäftstarif mit garantierter Bandbreite oder dedizierte Leitung.

### CGNAT und DS-Lite (GA2 Winter 2023, Winter 2025)

- **CGNAT:** Der Provider vergibt private bzw. Shared Addresses (`100.64.0.0/10` oder `10.x.x.x`) und übersetzt viele Kunden auf wenige öffentliche IPv4-Adressen.
- **DS-Lite:** Kunde bekommt natives **IPv6**, IPv4 wird getunnelt und beim Provider per CGNAT übersetzt.
- **Folge:** Eingehende IPv4-Verbindungen (Portweiterleitung, eigener VPN-Server, Kamera-Zugriff) sind **nicht möglich**, da die öffentliche IP nicht dem Kunden gehört.
- **Lösungen:** Zugriff über **IPv6**, beim Provider **feste öffentliche IPv4** buchen (Geschäftstarif), **ausgehenden** Tunnel zu einem Server mit öffentlicher IP (Cloud-VPN-Gateway, Relay).

### Redundante Anbindung und Verfügbarkeit (GA2 Sommer 2026, Winter 2025)

Zwei Provider über **unterschiedliche Medien und Trassen** (z. B. Glasfaser + LTE/DSL) vermeiden einen gemeinsamen Single Point of Failure. Gesamtverfügbarkeit parallel:

A_gesamt = 1 − (1 − A1) · (1 − A2)

| Anschluss 1 | Anschluss 2 | Gesamt |
|---|---|---|
| 99 % | 99 % | 1 − 0,01 · 0,01 = **99,99 %** |
| 99 % | 98,5 % | 1 − 0,01 · 0,015 = 99,985 % |
| 99 % | 98 % | 1 − 0,01 · 0,02 = 99,98 % |

Erlaubte Ausfallzeit bei 99 % pro Jahr: 0,01 × 8.760 h = **87,6 h**.

## Prüfungsfrage

**Frage (angelehnt an GA2 Sommer 2025):** Eine neue Filiale soll ans Internet angebunden werden. Angeboten werden ein GPON-Glasfaseranschluss für Privatkunden und – bis zur Fertigstellung – ein LTE-Router. Erläutern Sie den Begriff „Shared Medium“ am Beispiel GPON und nennen Sie zwei Aspekte, die gegen LTE als dauerhafte Lösung sprechen.

**Musterantwort:** Beim GPON wird eine Glasfaser vom Verteiler über passive optische Splitter auf viele Haushalte aufgeteilt (z. B. 1:32). Alle Teilnehmer an einem Splitter teilen sich die Kapazität der Faser (z. B. 2,5 Gbit/s Downstream). Bei hoher Last der anderen sinkt die verfügbare Bandbreite der Filiale, eine Mindestbandbreite ist nicht garantiert. Gegen LTE dauerhaft sprechen: (1) **Volumenbegrenzung** bzw. Drosselung und schwankende Bandbreite je nach Funkzellenauslastung, (2) **höhere und schwankende Latenz/Jitter**, was Kassensysteme, VPN und VoIP stört; zudem meist **CGNAT**, sodass keine eingehenden Verbindungen möglich sind.

## Falle

- VDSL der Koaxleitung zuordnen → VDSL nutzt die **Telefon-Kupferdoppelader**, DOCSIS das **Koax**.
- Bandbreiten „bis zu“ als garantiert verkaufen.
- Bei Videokonferenz/VoIP nur den Download betrachten → **Upload** asymmetrischer Anschlüsse ist der Engpass.
- Zwei Leitungen desselben Providers über dieselbe Trasse als „redundant“ bezeichnen → gemeinsamer Fehlerpunkt (Bagger).
- Verfügbarkeiten paralleler Leitungen **multiplizieren** → das gilt nur für **serielle** Ketten; parallel: Ausfallwahrscheinlichkeiten multiplizieren.

## Merksatz

**„Telefon = DSL, Koax = Kabel, Funk = LTE, Glas = FTTH – geteilt ist nicht garantiert, und CGNAT lässt niemanden rein.“**

## Verwandt

- [NAT & PAT](../Sicherheit/firewall/nat-pat.md) – Adressübersetzung, CGNAT-Grundlage
- [FHRP / VRRP](fhrp-vrrp-hsrp.md) – Gateway-Redundanz bei zwei Providern
- [VPN](../../nice-to-know/VPN/readme.md) – Standortkopplung über das Internet
- [Verkabelung](verkabelung-kupfer-lwl.md) – Glasfaser und Kupfer
- [Übertragungszeit & Datenrate](../OSI-IP-mac-bit-byte/uebertragungszeit-datenrate.md) – Upload-/Downloadzeiten
- [SLA](../Server/Cloud/sla.md) – Verfügbarkeitszusagen
- [Netzwerktypen](../netzwerktypen.md) – WAN und MAN
- [Lektion 18 – Netzwerk-Berechnungen](../../analyse-der-exams/L18_Netzwerk_Berechnungen.md)
- [Lektion 29 – GA2 Sonstiges](../../analyse-der-exams/L29_GA2_Sonstiges_Praxisthemen.md)
