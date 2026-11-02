# PoE (Power over Ethernet)

## In 30 Sekunden

- **PoE** überträgt **Strom und Daten über dasselbe Twisted-Pair-Kabel** – keine Steckdose am Access Point, an der Kamera oder am IP-Telefon nötig.
- Standards: **802.3af (PoE) 15,4 W**, **802.3at (PoE+) 30 W**, **802.3bt (PoE++) 60 W / 90 W** – jeweils am Switchport (PSE); am Gerät kommt weniger an.
- Der Switch hat ein **PoE-Budget** (Gesamtleistung): **Summe aller Geräte ≤ Budget** – klassische Rechenaufgabe (GA2 Winter 2021: 8 APs × 17,9 W = 143,2 W).
- Ohne PoE-Switch: **PoE-Injektor** (Midspan) zwischen Switch und Gerät.
- Vorteil für Verfügbarkeit: PoE-Switch an der **USV** → Telefone und Kameras laufen bei Stromausfall weiter.

## Kernwissen

### Standards

| Standard | Name | Typ | Leistung am Switchport (PSE) | Leistung am Gerät (PD) | genutzte Paare |
|---|---|---|---:|---:|---|
| IEEE 802.3af | PoE | Type 1 | 15,4 W | 12,95 W | 2 |
| IEEE 802.3at | PoE+ | Type 2 | 30 W | 25,5 W | 2 |
| IEEE 802.3bt | PoE++ / 4PPoE | Type 3 | 60 W | 51 W | 4 |
| IEEE 802.3bt | PoE++ / 4PPoE | Type 4 | 90 W | 71,3 W | 4 |

Die Differenz zwischen PSE und PD ist **Leitungsverlust** auf bis zu 100 m Kabel.

### Begriffe

| Begriff | Bedeutung |
|---|---|
| **PSE** (Power Sourcing Equipment) | speisendes Gerät: PoE-Switch oder Injektor |
| **PD** (Powered Device) | versorgtes Gerät: AP, IP-Telefon, Kamera, Sensor |
| **Endspan** | PoE direkt im Switch |
| **Midspan** | Injektor zwischen normalem Switch und Gerät |
| **Klassen (0–8)** | PD meldet seinen Bedarf, PSE reserviert entsprechend |
| **PoE-Budget** | maximale Gesamtleistung des Switches für alle Ports |
| **Erkennung** | PSE prüft per Widerstandsmessung, ob ein PoE-Gerät angeschlossen ist – normale Netzwerkkarten erhalten keinen Strom |

### Budget berechnen – Schema

1. Leistung je Gerät aus dem Datenblatt (oder Klasse) nehmen.
2. Mit der Anzahl multiplizieren und alles addieren.
3. Mit dem **Gesamtbudget** und der **Portklasse** vergleichen (ein PoE+-Gerät braucht einen 802.3at-Port).
4. Reserve (ca. 20 %) für Erweiterungen einplanen.

**Beispiel:** 8 APs à 17,9 W + 6 IP-Telefone à 6,5 W + 2 Kameras à 12 W
= 143,2 W + 39 W + 24 W = **206,2 W** → ein Switch mit 180 W reicht **nicht**, 370 W schon.

### Einsatz und Vorteile

| Vorteil | Erklärung |
|---|---|
| Ein Kabel statt zwei | Montage an Decke/Außenwand ohne Elektriker-Steckdose |
| Zentrale Stromversorgung | Ports per Software neu starten (Gerät „hängt“ → Port aus/an) |
| USV-Absicherung | eine USV am Switch versorgt alle PDs |
| Flexibilität | Geräte einfach versetzen |

**Zu beachten:** Erwärmung großer Kabelbündel bei hoher Leistung (802.3bt), passende Kabelkategorie (mind. Cat 5e, besser Cat 6A), Netzteil des Switches ausreichend dimensionieren, USV-Laufzeit für die Zusatzlast berechnen.

## Prüfungsfrage

**Frage (angelehnt an GA2 Winter 2021):** In einer Etage sollen 8 Access Points (je 17,9 W, PoE+) und 4 PTZ-Kameras (je 45 W) per PoE versorgt werden. Zur Auswahl stehen Switch X (24 Ports PoE+, Budget 370 W) und Switch Y (24 Ports PoE+/PoE++, Budget 370 W). Welcher Switch ist geeignet? Begründen Sie mit Rechnung.

**Musterantwort:** Leistungsbedarf: 8 × 17,9 W = 143,2 W, 4 × 45 W = 180 W, zusammen **323,2 W** – beide Budgets (370 W) reichen rechnerisch. Die Kameras benötigen jedoch je 45 W und damit mehr als PoE+ (max. 30 W am Port, 25,5 W am Gerät); sie brauchen **802.3bt (PoE++)**. Switch X liefert nur PoE+ und ist daher **ungeeignet**. Switch Y ist geeignet, weil er PoE++-Ports bietet und das Budget mit ca. 47 W Reserve ausreicht.

## Falle

- Nur das **Gesamtbudget** prüfen und die **Leistung pro Port** (Klasse) vergessen – oder umgekehrt.
- PSE- und PD-Werte verwechseln: 802.3af = 15,4 W am Port, **12,95 W** am Gerät.
- „PoE beschädigt normale PCs“ → nein, der Switch erkennt PoE-Geräte erst per Signatur.
- PoE über Glasfaser → unmöglich, Glas leitet keinen Strom.
- USV für den PoE-Switch vergessen → bei Stromausfall sind Telefone und Kameras trotzdem aus.

## Merksatz

**„af 15, at 30, bt 60 bis 90 – und die Summe muss ins Budget passen.“**

## Verwandt

- [Switches – Grundlagen](switches-und-vlans.md) – Auswahlkriterien inklusive PoE-Budget
- [WLAN](wlan.md) – Access Points als typische PoE-Geräte
- [VoIP & QoS](voip-qos.md) – IP-Telefone per PoE
- [Verkabelung](verkabelung-kupfer-lwl.md) – Kabelkategorien
- [USV](../Server/usv.md) – Stromausfall überbrücken
- [Begriffe (Hardware & Netzwerk)](../../nice-to-know/begriffe.md) – PoE-Kurzdefinition
