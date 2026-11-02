# Begriffe: Kamera- und Netzwerk-Hardware

## In 30 Sekunden

- **PoE** (Power over Ethernet) versorgt Kameras, Access Points und Telefone über das Netzwerkkabel mit Strom. Der Switch (oder ein Injektor) muss es können.
- **Vandalismusschutz** (IK-Klasse) und **Schutzart IP** (Staub/Wasser) sind Pflichtkriterien für Außengeräte.
- **IR-Nachtsicht** ermöglicht 24-h-Überwachung, **Bewegungserkennung** spart Speicher und Bandbreite.
- Typische Aufgabe (2022S): Kamera aus einer Tabelle für einen Einsatzzweck auswählen und begründen (Außenbereich mit Publikum, Webkonferenz, Serverraum).

## Kernwissen

### PoE-Standards

| Standard | Name | Leistung am Port (PSE) | beim Gerät nutzbar | Beispiel |
|---|---|---:|---:|---|
| IEEE 802.3af | PoE | 15,4 W | 12,95 W | IP-Telefon, einfache Kamera |
| IEEE 802.3at | PoE+ | 30 W | 25,5 W | WLAN-Access-Point, PTZ-Kamera |
| IEEE 802.3bt Typ 3 | PoE++ | 60 W | 51 W | Kamera mit Heizung, Multi-Radio-AP |
| IEEE 802.3bt Typ 4 | PoE++ | 90–100 W | 71 W | Displays, Thin Clients |

**PoE-Budget:** Die Summe aller Geräte darf das Gesamtbudget des Switches nicht übersteigen. 24 Kameras à 12 W = 288 W → Switch mit mindestens 300 W (besser 370 W) PoE-Budget. Reichweite wie Ethernet: 100 m.

### Schutzarten

| Kennzeichen | Bedeutung | Beispiel |
|---|---|---|
| **IP66 / IP67** | 1. Ziffer: Staub (6 = staubdicht), 2. Ziffer: Wasser (6 = starkes Strahlwasser, 7 = zeitweiliges Untertauchen) | Außenkamera |
| **IK10** | Schlagfestigkeit (Vandalismusschutz), 20 Joule | Kamera in Reichweite von Passanten |

Umsetzung des Vandalismusschutzes: Metall- oder Polycarbonatgehäuse, Dome-Bauform, Sicherheitsschrauben, verdeckte Kabelführung.

### Kamera-Merkmale

| Begriff | Bedeutung | Nutzen |
|---|---|---|
| **IR-Nachtsicht** | Infrarot-LEDs beleuchten unsichtbar, Bild in Schwarz-Weiß | Überwachung bei Dunkelheit |
| **Bewegungserkennung** | Kamera oder Rekorder erkennt Bildänderungen (Pixelvergleich, PIR-Sensor) | nur bei Ereignissen aufzeichnen, Alarm |
| **WDR** (Wide Dynamic Range) | gleicht starke Hell-Dunkel-Kontraste aus | Eingänge mit Gegenlicht |
| **PTZ** | Pan-Tilt-Zoom, schwenk- und zoombar | großes Gelände mit einer Kamera |
| **Privatzonen-Maskierung** | Bereiche im Bild werden geschwärzt | DSGVO: öffentliche Wege, Nachbargrundstücke |
| **ONVIF** | herstellerübergreifender Standard für IP-Kameras | Kamera und Rekorder verschiedener Hersteller kombinieren |
| **WPA3 / 802.1X** | sichere WLAN- bzw. Port-Authentifizierung | Kamera im Serverraum, Schutz vor fremden Geräten |
| **fps** | Bilder pro Sekunde | flüssige Darstellung (Videokonferenz 30–60 fps), Speicherbedarf steigt mit fps |

### Datenschutz bei Videoüberwachung (Kurz)

Zweckbindung, Hinweisschild, Löschfrist (häufig 48–72 h), keine Leistungs- oder Verhaltenskontrolle der Mitarbeiter ohne Betriebsvereinbarung (§ 87 BetrVG), Interessenabwägung nach Art. 6 DSGVO → [DSGVO-TOM](../Management/DSGVO/dsgvo-tom.md).

## Prüfungsfrage

**Frage:** Für den Eingangsbereich einer Schule soll eine Kamera rund um die Uhr aufzeichnen. Nennen Sie zwei Auswahlkriterien und begründen Sie.

**Musterantwort:** Erstens braucht die Kamera **Infrarot-Nachtsicht**, weil sie auch nachts ohne zusätzliche Beleuchtung verwertbare Bilder liefern muss. Zweitens ist ein **Vandalismusschutz** (z. B. IK10, robustes Dome-Gehäuse) nötig, da sie im Publikumsbereich in Reichweite von Personen hängt und vor Beschädigung geschützt sein muss. Zusätzlich sollte sie wetterfest sein (IP66) und über PoE versorgt werden, damit kein Stromanschluss an der Außenwand nötig ist.

## Falle

- PoE ohne passenden Switch oder ohne ausreichendes PoE-Budget planen.
- PoE und PoE+ verwechseln. Ein AP mit 25 W läuft an 802.3af nicht stabil.
- IP-Schutzart mit IP-Adresse verwechseln. Es geht um „International Protection“.
- Für eine Videokonferenz die Kamera mit IR-Nachtsicht wählen. Gefragt sind dort Auflösung und Bildrate.
- Videoüberwachung ohne Datenschutzaspekte (Löschfrist, Hinweis, Betriebsrat) planen.

## Merksatz

**„Draußen: IP66 gegen Wetter, IK10 gegen Vandalen, IR gegen die Nacht, PoE gegen Kabelsalat.“**

## Verwandt

- [Auflösung & Speicherbedarf](aufloesung-speicherbedarf.md) – Datenmenge pro Kamera
- [Video- & Fotoformate](video-photo-formats.md) – Codecs
- [WLAN](../FiSi-Administrator/Netzwerk-Administration/wlan.md) – WPA3
- [Switches & VLANs](../FiSi-Administrator/Netzwerk-Administration/switches-und-vlans.md) – PoE-Switch, Kamera-VLAN
- [DSGVO-TOM](../Management/DSGVO/dsgvo-tom.md)
- [Lektion L28 GA1 Praxisthemen](../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md)
