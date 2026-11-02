# Firewall-Generationen und Sicherheitskonzepte

> Abgrenzung: Dieses Blatt erklärt die **Entwicklung der Firewall-Typen** (Paketfilter → SPI → Proxy → NGFW) und das Zusammenspiel mit Virenschutz, IDS/IPS und DMZ. **Regeln lesen und schreiben** steht in [Firewall](firewall.md).

## In 30 Sekunden

- **1. Generation – Paketfilter:** prüft jedes Paket einzeln nach IP, Port, Protokoll (OSI 3/4), zustandslos.
- **2. Generation – Stateful Packet Inspection (SPI):** merkt sich Verbindungen in einer **State Table** und lässt Antworten automatisch zurück.
- **3. Generation – Application-Level-Gateway/Proxy:** versteht das Anwendungsprotokoll (OSI 7) und terminiert die Verbindung.
- **NGFW (Next Generation Firewall):** SPI + **Anwendungserkennung**, **IPS**, URL-Filter, **TLS-Inspection**, Benutzerbezug.
- Keine Firewall ersetzt **Virenschutz/EDR, Patches, Backups und Awareness** – Sicherheit entsteht durch mehrere Schichten (**Defense in Depth**).

## Kernwissen

| Generation | OSI-Schicht | Entscheidet nach | Vorteil | Grenze |
|---|---|---|---|---|
| **Paketfilter** | 3/4 | Quell-/Ziel-IP, Port, Protokoll, je Paket | schnell, einfach (z. B. ACL im Router) | kein Verbindungszusammenhang, Rückweg-Regeln nötig, leicht zu täuschen |
| **SPI** | 3/4 + Zustand | zusätzlich Verbindungszustand (SYN, ESTABLISHED) | Antworten automatisch erlaubt, kein „Öffnen“ hoher Ports nötig | sieht keine Inhalte, Malware über Port 443 bleibt unerkannt |
| **Proxy / ALG** | 7 | Inhalte des Protokolls (HTTP, SMTP, FTP) | Inhaltsprüfung, Caching, verbirgt Clients | je Protokoll ein Proxy, langsamer, Flaschenhals |
| **NGFW** | 3–7 | Anwendung, Benutzer, Inhalt, Reputation | App-Control („YouTube ja, Upload nein“), IPS, Sandbox, TLS-Inspection | teuer, komplex, Rechenleistung, Datenschutzfragen bei TLS-Inspection |
| **WAF** | 7 | HTTP-Anfragen an Webanwendungen | schützt vor SQL-Injection, XSS | ersetzt keine sichere Programmierung |
| **UTM** | 3–7 | Bündel aus Firewall, VPN, AV, Spamfilter | „alles in einer Box“ für KMU | Single Point of Failure |

**Prüfungsklassiker SPI vs. NGFW (2026S GA2 ac, 2025S ac)**
- SPI arbeitet auf **Layer 3/4** mit State Table, erkennt **keine** Anwendungen.
- NGFW arbeitet bis **Layer 7**: Deep Packet Inspection, Anwendungserkennung unabhängig vom Port, integriertes **IPS**, URL-Filter, Benutzer-Identität.

**TLS-Inspection (2022S, 2024W):** Die NGFW bricht HTTPS als gewollter **Man-in-the-Middle** auf, prüft den Klartext und verschlüsselt neu. Voraussetzung: Das CA-Zertifikat der Firewall ist auf allen Clients als vertrauenswürdig verteilt (GPO/MDM), sonst erscheint z. B. `SEC_ERROR_UNKNOWN_ISSUER`. Organisatorisch: Betriebsrat (§ 87 Abs. 1 Nr. 6 BetrVG) und Datenschutz einbinden, Ausnahmen für Banking/Gesundheit.

**Sicherheitskonzepte rund um die Firewall**

| Konzept | Aufgabe | Details |
|---|---|---|
| **DMZ** | öffentlich erreichbare Server vom LAN trennen | [DMZ und Sicherheitszonen](dmz-sicherheitszonen.md) |
| **IDS/IPS** | Angriffsmuster erkennen bzw. blockieren | [IDS und IPS](ids-ips.md) |
| **Virenschutz / EDR** | Schadcode auf dem Endgerät erkennen (Signatur, Heuristik, Verhalten) | [Malware und Ransomware](../hacking/malware-ransomware.md) |
| **Segmentierung / VLAN** | Ausbreitung im internen Netz begrenzen | [Zero Trust](../zero-trust.md) |
| **Host-Firewall** | schützt das einzelne Gerät, auch im eigenen LAN | Windows-Firewall, nftables |

**Was eine Firewall nicht abwehrt (2026S GA2 ab):** Phishing und Social Engineering, Innentäter mit gültigen Rechten, Malware über USB-Stick, bereits infizierte Clients im LAN (Ost-West-Verkehr), Diebstahl von Geräten.

## Prüfungsfrage

**Frage:** Ein Unternehmen ersetzt seine Stateful-Packet-Inspection-Firewall durch eine Next Generation Firewall. Nennen Sie zwei Vorteile der NGFW und einen Nachteil.

**Musterantwort:** Erstens erkennt die NGFW Anwendungen unabhängig vom Port und kann sie gezielt erlauben oder sperren, etwa einen Cloud-Speicher-Upload über HTTPS, den eine SPI-Firewall nur als Port 443 sieht. Zweitens bringt sie ein integriertes IPS mit, das Angriffsmuster und Exploits im Datenstrom erkennt und blockiert. Nachteil: Für die Prüfung verschlüsselter Verbindungen ist TLS-Inspection nötig; das kostet Rechenleistung, erfordert die Verteilung eines eigenen CA-Zertifikats und wirft Datenschutz- und Mitbestimmungsfragen auf.

## Falle

- Paketfilter und SPI verwechseln: nur SPI merkt sich den **Verbindungszustand**.
- „Stateful prüft Inhalte“ → nein, Inhalte prüfen Proxy und NGFW.
- NGFW als Ersatz für Virenschutz verkaufen → Endgeräteschutz bleibt nötig (USB, verschlüsselte Mails, Offline-Geräte).
- TLS-Inspection ohne Hinweis auf CA-Verteilung und Betriebsvereinbarung beschreiben → Punktverlust.

## Merksatz

**„Paket – Zustand – Anwendung: je höher die Generation, desto tiefer der Blick.“**

## Verwandt

- [Firewall – Regeln lesen und schreiben](firewall.md)
- [DMZ und Sicherheitszonen](dmz-sicherheitszonen.md)
- [IDS und IPS](ids-ips.md)
- [NAT und PAT](nat-pat.md) – Adressübersetzung ist keine Filterung
- [TLS und HTTPS](../Verschluesselung/tls-https.md) – TLS-Inspection im Detail
- [Proxy](../../Netzwerk-Administration/proxy.md)
- [OSI-Modell](../../OSI-IP-mac-bit-byte/osi.md)
- [Lektion 16 Firewall](../../../analyse-der-exams/L16_Firewall.md)
- [Lektion 27 PKI, TLS, VPN](../../../analyse-der-exams/L27_PKI_TLS_Verschluesselung_VPN.md)
