# Netzwerkanalyse & Troubleshooting

## In 30 Sekunden

- Fehlersuche ist neben Subnetting das **häufigste GA2-Thema** – jeder Termin 2021–2026 enthält eine Konsolenausgabe (`ipconfig`, `ping`, `tracert`, `nslookup`) oder einen Wireshark-Mitschnitt.
- **Systematisch** vorgehen: entlang des **OSI-Modells** (bottom-up) oder „Divide and Conquer“ ab Schicht 3 mit `ping`.
- Standardkette: **Link → IP-Konfiguration → Loopback → Gateway → externe IP → DNS → Dienst-Port**.
- Antworten **immer konkret auf das gezeigte Feld beziehen** („DNS-Server steht auf 8.8.8.8 statt 172.16.1.11“).
- Wichtige Muster: **APIPA = kein DHCP**, **IP geht/Name nicht = DNS**, **Ping tot/Port offen = ICMP gefiltert**, **`* * *` im tracert = Hop antwortet nicht**.

## Kernwissen

### Vorgehensmodelle

| Methode | Vorgehen | Wann sinnvoll |
|---|---|---|
| **Bottom-up** | Schicht 1 → 7: Kabel, Link, VLAN, IP, Port, Anwendung | unklare Fehler, Hardwareverdacht |
| **Top-down** | Anwendung zuerst, dann abwärts | ein Dienst/Programm betroffen, Netz sonst ok |
| **Divide and Conquer** | mit `ping` in der Mitte (Schicht 3) beginnen, dann hoch oder runter | schnellster Standardweg |
| **Follow the Path** | Weg des Pakets Hop für Hop prüfen (`tracert`) | Fehler zwischen Standorten |
| **Vergleich / Austausch** | funktionierendes Gerät/Kabel/Port gegenprüfen | sporadische Fehler |

Immer auch fragen: **Was wurde zuletzt geändert?** (Switch-Tausch, neue VLANs, neuer AP, Firewall-Update) – und Ergebnisse dokumentieren.

### Diagnosekette mit Befehlen

| # | Prüfung | Befehl | Erkenntnis bei Fehler |
|---:|---|---|---|
| 1 | Link / Kabel / Port | LED, `ip link`, Switch-Portstatus | Schicht 1/2 defekt, Port deaktiviert, falsches VLAN |
| 2 | IP-Konfiguration | `ipconfig /all` | APIPA, falsche Maske, falsches Gateway/DNS |
| 3 | TCP/IP-Stack | `ping 127.0.0.1` | Stack/Treiber defekt |
| 4 | Default-Gateway | `ping 192.168.10.1` | lokales Netz/VLAN/Gateway-Problem |
| 5 | Internet per IP | `ping 1.1.1.1` bzw. `tracert` | Routing, NAT, Firewall, Provider |
| 6 | Namensauflösung | `nslookup www.ihk.de` | DNS-Server falsch/nicht erreichbar |
| 7 | Dienst | `Test-NetConnection srv -Port 443`, `telnet srv 80` | Dienst aus, Port gefiltert |

Befehle im Detail: [Netzwerk-Befehle](netzwerk-befehle.md).

### Symptom → Ursache (aus echten Prüfungen)

| Symptom | Wahrscheinliche Ursache | Abhilfe | Termin |
|---|---|---|---|
| IPv4 `169.254.x.x`, kein Gateway | **APIPA**: DHCP aktiv, aber kein Server erreicht (Kabel, VLAN, Relay, Server) | Link/Relay prüfen, `ipconfig /renew` | 2023S, 2024S, 2024W |
| Statischer Server plötzlich `169.254.x.x`, ARP-Antwort einer fremden MAC | **IP-Konflikt** | Doppeltes Gerät finden, IP ändern | 2023S |
| Internet geht, interne Namen nicht; DNS `8.8.8.8` | **Öffentlicher DNS** kennt interne Zone nicht | DNS auf DC setzen bzw. per DHCP beziehen, `ipconfig /flushdns` | 2024S |
| DHCP liefert neuen DNS, Client zeigt alten | **alter Lease** (oder Rogue-DHCP) | `ipconfig /release` + `/renew`, DHCP-Snooping | 2026S |
| `ping` Timeout, Web-Seite / `telnet srv 80` klappt | Firewall blockiert **nur ICMP** | kein Fehler des Dienstes; ggf. ICMP erlauben | 2021W, 2024W |
| `nslookup` Timeout im neuen VLAN | **Inter-VLAN-Routing**/Gateway fehlt, DNS nicht erreichbar | Subinterface/SVI, Trunk prüfen | 2023W |
| Clients aller VLANs außer Server-VLAN ohne IP | **DHCP-Relay** fehlt | `ip helper-address` je VLAN | 2023S, 2024W |
| Neue Clients ohne IP, alte ok | **Scope erschöpft** | Pool vergrößern, Lease verkürzen | 2024W |
| `ftp.firma.de` landet auf falschem Server, „404“ | **CNAME** zeigt auf falschen Host | CNAME korrigieren | 2024W |
| Nur 100 Mbit/s statt 1 Gbit/s | Kabel (2 Paare/Cat 5/defekt), Port fest 100, Duplex-Mismatch | Kabel tauschen, Auto-Negotiation | 2024W |
| `tracert`-Zeile `* * *` | Hop verwirft/beantwortet **ICMP Time Exceeded** nicht – Weg geht trotzdem weiter | kein Fehler, wenn spätere Hops antworten | 2021W |
| Unterschiedliche Hopzahl bei zwei `tracert` | **DNS-Load-Balancing/Anycast**: andere Ziel-IP, anderer Weg | kein Fehler | 2021W, 2022W |
| Proxy wird umgangen, `tracert` zeigt IPv6-Hops | Proxy/Firewall nur für IPv4 → **IPv6-Breakout** | IPv6 in Proxy/Firewall einbeziehen | 2023S |
| VPN im Hotel-WLAN scheitert | Hotspot blockiert UDP 500/4500/1194, NAT ohne NAT-T | VPN über TCP 443 / SSL-VPN | 2023S |
| Port mit extrem hoher Aktivität | **Schleife/Broadcast-Sturm** oder kompromittierter Client | STP prüfen, Port sperren, Client isolieren | 2025W |
| Neuer Switch, SFP-Uplink tot | **SFP-Modul inkompatibel** (SM/MM, Wellenlänge, Hersteller) oder Portkonfiguration nicht übernommen | passendes Modul, Konfiguration anpassen | 2025W |
| Eine MAC für mehrere IPs im Mitschnitt | **ARP-Poisoning** | DAI, Port-Security | 2026S |

### Wireshark – Filter und typische Auswertungen

| Filter | Zeigt |
|---|---|
| `arp` | ARP-Requests/-Replies, Konflikte, Poisoning |
| `dhcp` (ältere Versionen: `bootp`) | DORA-Ablauf, Optionen (Maske, Router, DNS, Lease) |
| `dns` | Anfragen (A/AAAA) und Antworten |
| `icmp` / `icmpv6` | Ping, Time Exceeded, NDP |
| `ip.addr == 10.0.0.5` | alles von/zu einer IP |
| `tcp.port == 443` | Verkehr eines Ports |
| `tcp.flags.syn == 1` | Verbindungsaufbauten |

**Hop-Anzahl aus einem Mitschnitt (2022W, 2023S):** Start-TTL/Hop-Limit der Gegenseite (Windows 128, Linux 64) minus empfangener Wert, z. B. 64 − 51 = **13 Hops**.

**Wo mitschneiden?** Auf dem betroffenen Host oder an einem **SPAN-/Mirror-Port** bzw. **TAP** des Switches – ein normaler Switchport sieht nur Broadcasts und eigenen Verkehr.

### Werkzeuge über die Konsole hinaus

- **Kabeltester / Qualifizierer** (Durchgang, Paare, Länge, Kategorie), **OTDR** für Glasfaser.
- **Monitoring** (SNMP, Syslog, NetFlow) für Trends und Alarme.
- **WLAN-Analyzer** für Kanäle und Signalstärke.

## Prüfungsfrage

**Frage (angelehnt an GA2 Sommer 2024):** Ein Notebook im Firmennetz zeigt bei `ipconfig /all`: IPv4 `172.16.20.57`, Maske `255.255.0.0`, Gateway `172.16.0.254`, DNS `8.8.8.8`. Webseiten im Internet funktionieren, `\\dateiserver.ihk-media.local` ist nicht erreichbar, `ping 172.16.1.20` (IP des Dateiservers) funktioniert. Analysieren Sie den Fehler und nennen Sie die Lösung.

**Musterantwort:** Schicht 1 bis 3 funktionieren: Das Notebook hat eine gültige Adresse, erreicht das Internet und den Dateiserver per IP. Der Fehler liegt in der **Namensauflösung**: Als DNS-Server ist der öffentliche Google-DNS `8.8.8.8` eingetragen, der die interne Zone `ihk-media.local` nicht kennt. Lösung: Als DNS-Server den internen DNS/Domänencontroller (z. B. `172.16.1.11`) eintragen bzw. die DNS-Adresse automatisch per DHCP beziehen, den manuellen Eintrag entfernen und mit `ipconfig /flushdns` und `nslookup dateiserver.ihk-media.local` prüfen.

## Falle

- „Ping geht nicht, also ist der Server aus“ → ICMP kann gefiltert sein; **Port testen**.
- Bei APIPA eine falsche statische IP als Ursache nennen → APIPA bedeutet **DHCP ohne Antwort**.
- `* * *` im `tracert` als Abbruch des Weges deuten, obwohl danach weitere Hops folgen.
- Lösung ohne Bezug auf die Ausgabe („Netzwerk prüfen“) → keine Punkte; das konkrete Feld nennen.
- Nur eine Ursache nennen, wenn „zwei mögliche Ursachen“ gefragt sind – unterschiedliche Schichten wählen (z. B. Kabel **und** Portkonfiguration).

## Merksatz

**„Erst Kabel, dann Adresse, dann Gateway, dann Name, dann Port – und immer: Was wurde geändert?“**

## Verwandt

- [Netzwerk-Befehle](netzwerk-befehle.md) – ipconfig, ping, tracert, nslookup, arp, netstat
- [OSI](../OSI-IP-mac-bit-byte/osi.md) – Schichtenmodell als Leitfaden
- [DHCP](../OSI-IP-mac-bit-byte/dhcp.md) – APIPA, Relay, Scope
- [DNS](../OSI-IP-mac-bit-byte/dns.md) – nslookup-Ausgaben, CNAME
- [ARP](../OSI-IP-mac-bit-byte/arp.md) – IP-Konflikt, Poisoning
- [VLAN](vlan.md) – Inter-VLAN-Fehler
- [SNMP, Syslog & NTP](snmp-monitoring-syslog-ntp.md) – Monitoring statt Zufallsfund
- [Verkabelung](verkabelung-kupfer-lwl.md) – Kabel- und SFP-Ursachen
- [Lektion 07 – Netzwerk-Fehlersuche](../../analyse-der-exams/07_Netzwerk_Fehlersuche.md)
