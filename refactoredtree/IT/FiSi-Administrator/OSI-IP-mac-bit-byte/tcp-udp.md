# TCP vs. UDP

## In 30 Sekunden

- Beide arbeiten auf **OSI-Schicht 4** und adressieren Anwendungen über **Ports** (Socket = IP-Adresse + Port).
- **TCP:** verbindungsorientiert, **zuverlässig** (Bestätigungen, Wiederholung, Reihenfolge), Flusskontrolle – dafür mehr Overhead und Latenz.
- **UDP:** verbindungslos, **kein** Handshake, keine Zustellgarantie – schnell und schlank, ideal für Echtzeit (VoIP, Video, DNS-Anfragen).
- TCP-Verbindungsaufbau: **3-Way-Handshake SYN → SYN/ACK → ACK**; Abbau mit FIN/ACK.
- Prüfungsklassiker: „Warum nutzt MQTT TCP?“ (GA2 Sommer 2025), „Welches Protokoll nutzt VoIP für Sprache?“ (RTP über UDP).

## Kernwissen

### Vergleich

| Merkmal | TCP | UDP |
|---|---|---|
| Verbindung | verbindungsorientiert (Handshake) | verbindungslos |
| Zuverlässigkeit | ACKs, Retransmission, Prüfsumme | nur Prüfsumme, kein Nachsenden |
| Reihenfolge | Sequenznummern sichern Reihenfolge | keine Garantie |
| Fluss-/Staukontrolle | ja (Window, Slow Start) | nein |
| Header | mind. **20 Byte** | **8 Byte** |
| Geschwindigkeit/Latenz | höher durch Bestätigungen | gering |
| Broadcast/Multicast | nein | ja |
| Typische Dienste | HTTP(S), SSH, SMTP, IMAP, FTP, SMB, RDP, MQTT | DNS-Anfragen, DHCP, NTP, SNMP, Syslog, TFTP, RTP (VoIP), WireGuard, QUIC |
| PDU | Segment | Datagramm |

### 3-Way-Handshake

```
Client                                   Server (Port 443)
  | --- SYN      Seq=100 ----------------> |   "Ich will verbinden"
  | <-- SYN/ACK  Seq=300, Ack=101 -------- |   "OK, ich auch"
  | --- ACK      Seq=101, Ack=301 -------> |   Verbindung steht (ESTABLISHED)
```
- **Abbau:** FIN → ACK, FIN → ACK (4 Schritte, jede Richtung einzeln) oder sofort per **RST**.
- **SYN-Flood:** Angreifer sendet massenhaft SYN ohne abschließendes ACK → halboffene Verbindungen füllen den Speicher des Servers (DoS). Schutz: SYN-Cookies, Firewall/IPS.
- Eine **SPI-Firewall** verfolgt genau diese Zustände in ihrer State Table.

### TCP-Flags

| Flag | Bedeutung |
|---|---|
| SYN | Verbindungsaufbau, Sequenznummern synchronisieren |
| ACK | Bestätigung empfangener Daten |
| FIN | geordneter Verbindungsabbau |
| RST | Verbindung sofort abbrechen (z. B. Port geschlossen) |
| PSH | Daten sofort an die Anwendung |
| URG | dringende Daten |

### Zuverlässigkeit bei TCP

- **Sequenz- und Bestätigungsnummern:** Jedes Byte wird gezählt; fehlt eine Bestätigung, wird nach Timeout **neu gesendet**.
- **Sliding Window:** Empfänger meldet, wie viele Byte er noch aufnehmen kann (Flusskontrolle).
- **Staukontrolle:** Slow Start, Congestion Avoidance – bei Paketverlust wird die Senderate reduziert.

### Wann welches Protokoll?

| Anforderung | Wahl | Begründung |
|---|---|---|
| Datei, Webseite, E-Mail, Datenbank | TCP | jedes Byte muss korrekt ankommen |
| Sprache/Video in Echtzeit | UDP | verspätete Pakete sind wertlos, Nachsenden würde stören |
| Kurze Abfrage (DNS, NTP) | UDP | ein Paket hin, eins zurück – Handshake wäre teurer als die Nutzlast |
| IoT-Telemetrie mit Zustellgarantie (MQTT) | TCP | zuverlässige, geordnete Zustellung; persistente Sitzung |
| Moderne Web-Transporte (QUIC/HTTP/3) | UDP | Zuverlässigkeit wird in der Anwendung umgesetzt |

## Prüfungsfrage

**Frage (angelehnt an GA2 Sommer 2025):** Die Sensoren einer Halle übertragen Messwerte per MQTT an einen Broker. Begründen Sie, warum MQTT auf TCP statt UDP aufsetzt, und nennen Sie einen Nachteil.

**Musterantwort:** MQTT benötigt eine zuverlässige, geordnete Zustellung: TCP bestätigt empfangene Segmente, wiederholt verlorene und sorgt über Sequenznummern für die richtige Reihenfolge. Außerdem nutzt MQTT eine dauerhafte Sitzung (CONNECT/CONNACK), die TCP als verbindungsorientiertes Protokoll bereitstellt. Die MQTT-QoS-Stufen (0, 1, 2) bauen darauf auf. Nachteil: Durch Handshake, Bestätigungen und den größeren Header entstehen mehr Overhead und Latenz als bei UDP, was bei sehr vielen kleinen Nachrichten oder schlechten Funkverbindungen ins Gewicht fällt.

## Falle

- „UDP ist unsicher“ → UDP ist **unzuverlässig** (keine Zustellgarantie), nicht automatisch unsicher; Sicherheit regelt z. B. TLS/DTLS.
- DNS als „nur UDP“ bezeichnen → Zonentransfer und große Antworten laufen über **TCP 53**.
- Handshake mit vier Schritten angeben → **Aufbau 3**, Abbau 4 Schritte.
- ICMP (Ping) als TCP oder UDP einordnen → ICMP ist ein eigenes Protokoll auf **Schicht 3** (Protokollnummer 1), ohne Ports.
- „UDP hat keine Prüfsumme“ → doch, nur keine Wiederholung.

## Merksatz

**„TCP ist das Einschreiben mit Rückschein, UDP die Postkarte.“** SYN, SYN-ACK, ACK – dann wird geredet.

## Verwandt

- [Ports](ports.md) – welche Dienste auf welchem Port und Protokoll
- [OSI](osi.md) – Transportschicht
- [VoIP & QoS](../Netzwerk-Administration/voip-qos.md) – RTP über UDP, Jitter
- [Firewall](../Sicherheit/firewall/firewall.md) – SPI verfolgt TCP-Zustände
- [DDoS](../Sicherheit/hacking/ddos.md) – SYN-Flood
- [Netzwerk-Befehle](../Netzwerk-Administration/netzwerk-befehle.md) – `netstat`, `Test-NetConnection`
- [Lektion 29 – GA2 Sonstiges (MQTT)](../../analyse-der-exams/L29_GA2_Sonstiges_Praxisthemen.md)
