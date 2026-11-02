# Netzwerk-Befehle (ipconfig, ping, tracert, nslookup & Co.)

## In 30 Sekunden

- `ipconfig /all` zeigt IP, Maske, Gateway, DNS, DHCP-Server, Lease, MAC – **die** Grundlage jeder Fehleranalyse.
- `ping` testet die **Erreichbarkeit per ICMP Echo** (Schicht 3) und liefert Laufzeit und TTL – Ping-Ausfall heißt nicht, dass der Dienst aus ist.
- `tracert`/`traceroute` zeigt den **Weg Hop für Hop**, indem die TTL schrittweise erhöht wird.
- `nslookup`/`dig` prüft die **Namensauflösung**; `Test-NetConnection -Port` bzw. `telnet` prüft einen **TCP-Port**.
- Ausgaben lesen können ist Pflicht: GA2 2021W, 2022W, 2023S, 2024S, 2024W, 2025W, 2026S.

## Kernwissen

### Übersicht Windows / Linux / Cisco

| Zweck | Windows | Linux | Cisco IOS |
|---|---|---|---|
| IP-Konfiguration | `ipconfig /all` | `ip a` (alt: `ifconfig`) | `show ip interface brief` |
| DHCP neu holen | `ipconfig /release` + `/renew` | `dhclient -r` + `dhclient` | – |
| DNS-Cache | `ipconfig /displaydns`, `/flushdns` | `resolvectl flush-caches` | – |
| Erreichbarkeit | `ping`, `Test-Connection` | `ping` | `ping` |
| Weg | `tracert`, `pathping` | `traceroute`, `mtr`, `tracepath` | `traceroute` |
| Namensauflösung | `nslookup`, `Resolve-DnsName` | `dig`, `host`, `nslookup` | – |
| ARP-/Nachbartabelle | `arp -a` | `ip neigh` | `show arp` |
| Routingtabelle | `route print` | `ip route` | `show ip route` |
| Offene Verbindungen/Ports | `netstat -ano`, `Get-NetTCPConnection` | `ss -tulpn` | – |
| TCP-Port testen | `Test-NetConnection srv -Port 443` | `nc -vz srv 443`, `curl -v` | `telnet srv 443` |
| MAC-Tabelle | – | – | `show mac address-table` |

### `ipconfig /all` lesen (GA2 Sommer 2024, Sommer 2026)

| Feld | Was prüfen? |
|---|---|
| DHCP aktiviert | Ja/Nein – bei Nein und falscher Konfiguration: manuelle Fehlkonfiguration |
| IPv4-Adresse | `169.254.x.x` = APIPA; „(Dupliziert)“ = Konflikt; „Bevorzugt“ = ok |
| Subnetzmaske | passt sie zum Netzplan? |
| Standardgateway | leer → kein Verkehr in fremde Netze; im eigenen Subnetz? |
| DHCP-Server | welcher Server hat geantwortet (Rogue-DHCP?) |
| DNS-Server | interner DNS/DC oder öffentlicher (`8.8.8.8`)? |
| Lease erhalten / läuft ab | Lease-Dauer berechnen (z. B. 09:35:36 → 09:45:31 ≈ 10 min) |
| Verbindungslokale IPv6 | `fe80::…%12` – `%12` ist die Zonen-/Interface-ID |
| Physische Adresse | MAC des Adapters |

### `ping`

```
C:\> ping -4 www.ihk.de
Ping wird ausgeführt für www.ihk.de [51.38.104.145] mit 32 Bytes Daten:
Antwort von 51.38.104.145: Bytes=32 Zeit=14ms TTL=52
...
Pakete: Gesendet = 4, Empfangen = 4, Verloren = 0 (0% Verlust)
```
- Name wurde aufgelöst → **DNS funktioniert** (auch wenn danach Zeitüberschreitungen kommen!).
- `Zeit` = Round Trip Time; `TTL` = verbleibende TTL der Antwort → Hop-Schätzung (64 − 52 = 12).
- „Zeitüberschreitung der Anforderung“ = keine Antwort (Ziel aus, **ICMP gefiltert**, Rückroute fehlt).
- „Zielhost nicht erreichbar“ von einem Router = dieser kennt keine Route.

| Option (Windows) | Wirkung |
|---|---|
| `-t` | dauerhaft pingen |
| `-n 10` | Anzahl Pakete |
| `-4` / `-6` | IPv4/IPv6 erzwingen |
| `-l 1472 -f` | Paketgröße, nicht fragmentieren → MTU testen |

### `tracert` / `traceroute`

**Prinzip:** Erstes Paket TTL = 1 → erster Router verwirft es und sendet **ICMP Time Exceeded** zurück; dann TTL = 2 usw., bis das Ziel antwortet. Pro Hop drei Messungen.
- Windows `tracert` nutzt **ICMP Echo**, Linux `traceroute` standardmäßig **UDP** (Option `-I` für ICMP).
- `*  *  *  Zeitüberschreitung` = dieser Hop sendet keine Time-Exceeded-Meldung (Firewall/Rate-Limit). Antworten spätere Hops, ist der Weg trotzdem intakt (2021W).
- Plötzlich IPv6-Adressen im Pfad → Verkehr geht über IPv6 (Proxy-Umgehung, 2023S).
- `-d` (Windows) verhindert die Namensauflösung der Hops und beschleunigt die Ausgabe.

### `nslookup`

```
nslookup www.google.de            -> A/AAAA über den Standard-DNS
nslookup www.google.de 1.1.1.1    -> gezielt anderen Server fragen (Vergleichstest)
nslookup -type=mx firma.de        -> Mailserver der Domain
nslookup 203.0.113.10             -> Reverse Lookup (PTR)
```
„Nicht autorisierende Antwort“ = aus dem Cache; „DNS request timed out“ = DNS-Server nicht erreichbar (Routing/VLAN/Firewall UDP 53); „Non-existent domain“ = Name existiert in dieser Zone nicht.

### Port testen statt pingen (GA2 Winter 2021, Winter 2024)

```
PS> Test-NetConnection www.ihk.de -Port 80
ComputerName     : www.ihk.de
RemoteAddress    : 141.88.222.152
PingSucceeded    : False
TcpTestSucceeded : True
```
→ ICMP wird gefiltert, der Webserver ist trotzdem erreichbar.

### `telnet` als Protokoll-Test (GA2 Sommer 2024)

`telnet mail.firma.de 143` liefert das Banner `* OK [CAPABILITY IMAP4rev1 STARTTLS AUTH=PLAIN AUTH=LOGIN] Dovecot ready` → Protokoll **IMAP**, Verbindungssicherheit **STARTTLS**, Authentifizierung **Passwort normal**.

### `netstat -ano` / `ss -tulpn`

Zeigt lokale/entfernte Adressen und Ports, Zustand (`LISTENING`, `ESTABLISHED`, `TIME_WAIT`) und Prozess-ID → „Lauscht mein Dienst überhaupt?“, „Wer baut Verbindungen zu fremden IPs auf?“ (Malware-Verdacht).

## Prüfungsfrage

**Frage (angelehnt an GA2 Winter 2021):** `ping www.ihk.de` endet mit „Zeitüberschreitung der Anforderung“, zeigt aber vorher die IP `141.88.222.152`. Im Browser ist die Seite erreichbar. Erläutern Sie das Ergebnis und nennen Sie einen Befehl, mit dem die Erreichbarkeit des Webservers sauber geprüft werden kann.

**Musterantwort:** Da `ping` die IP-Adresse anzeigt, hat die **DNS-Auflösung funktioniert**. Die Zeitüberschreitung bedeutet nur, dass keine ICMP-Echo-Antwort zurückkam – vermutlich filtert eine Firewall beim Ziel oder auf dem Weg ICMP. Der Webdienst selbst ist erreichbar, weil er über TCP 80/443 läuft. Prüfen lässt sich das z. B. mit `Test-NetConnection www.ihk.de -Port 443` (TcpTestSucceeded: True) oder `telnet www.ihk.de 80`.

## Falle

- `ping` als Beweis für einen Dienstausfall nutzen → nur ICMP getestet.
- `tracert` als TCP-Werkzeug beschreiben → Windows: ICMP, Linux: UDP.
- `ipconfig /renew` bei statischer IP empfehlen → wirkungslos.
- Aus `nslookup`-Timeout auf „Name existiert nicht“ schließen → Server nicht erreichbar ist etwas anderes als NXDOMAIN.
- `%12` hinter einer `fe80`-Adresse für einen Fehler halten → das ist die Zonen-ID des Interfaces.

## Merksatz

**„ipconfig zeigt, ping fragt, tracert folgt, nslookup übersetzt, Test-NetConnection klopft an der Tür.“**

## Verwandt

- [Netzwerkanalyse & Troubleshooting](netzwerkanalyse-und-troubleshooting.md) – Vorgehen und Symptom-Tabelle
- [DNS](../OSI-IP-mac-bit-byte/dns.md) – Record-Typen und Auflösung
- [DHCP](../OSI-IP-mac-bit-byte/dhcp.md) – Lease und Renew
- [ARP](../OSI-IP-mac-bit-byte/arp.md) – `arp -a`
- [Routing](routing.md) – `route print`, `ip route`
- [Ports](../OSI-IP-mac-bit-byte/ports.md) – welche Ports zu testen sind
- [Lektion 07 – Netzwerk-Fehlersuche](../../analyse-der-exams/07_Netzwerk_Fehlersuche.md)
