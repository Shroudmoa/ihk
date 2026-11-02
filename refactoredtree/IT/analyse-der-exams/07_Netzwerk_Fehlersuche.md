# 7. Netzwerk-Fehlersuche

*Quelle: GA2 Aufgabe 2 (2024 Sommer)*

> ✅ **Verifiziert am echten Prüfungskorpus:** Fehlersuche ist neben
> IPv4-Subnetting das häufigste GA2-Thema – **in jedem der 10 Jahrgänge**
> in unterschiedlicher Ausprägung:
> - **APIPA (169.254.x.x):** 2023S, 2024S, 2024W
> - **ICMP/Ping blockiert, HTTP geht trotzdem:** 2021W und 2024W (fast
>   identisch) – `ping` läuft in Timeout, aber Port 80 ist per `telnet`
>   bzw. `Test-NetConnection` erreichbar → Firewall blockiert nur ICMP
> - **DNS falsch/tot:** 2023W (nslookup-Timeout wegen falschem
>   VLAN-Routing), 2024S (fest eingetragener `8.8.8.8` verhindert interne
>   Namensauflösung), 2026S (DHCP liefert DNS `192.168.5.250`, aber
>   `ipconfig` zeigt `192.168.5.1` – Diskrepanz durch alten DHCP-Lease)
> - **nslookup-Ausgaben erklären:** 2021W und 2024W (fast identisch) –
>   Server-Name, IPv6-DNS-Adresse, „nicht autorisierende Antwort",
>   AAAA-/A-Records unterscheiden
> - **tracert:** 2021W (Zeile mit nur `*` / Timeout ohne IP = Hop hat TTL
>   verarbeitet, aber keine Rückantwort gesendet), 2023S (Umgehung des
>   Proxys über einen IPv6-Breakout)
> - **DHCP-Fehlkonfiguration:** 2023S (DHCP-Relay zwischen VLANs fehlt),
>   2024W (falscher Adressbereich, Scope-Erschöpfung), 2026S (kompletter
>   DHCP-**DORA**-Mitschnitt inkl. Wireshark-Hexdump)
> - **Sonstiges:** 2024W (fehlerhafter **CNAME** → falscher FTP-Server,
>   „404 File not found"), 2024S (Link zeigt nur 100 Mbit/s statt Gigabit –
>   Ursache Duplex-/Kabel-/Port-Fehlkonfiguration)

## 7.1 `ipconfig` lesen können

Wichtige Felder:
- **IPv4-Adresse** (Auto. Konfiguration = APIPA, siehe unten)
- **Subnetzmaske**
- **Standardgateway**
- **DNS-Server**
- **Verbindungslokale IPv6-Adresse** (`fe80::/10`)

## 7.2 APIPA / 169.254.x.x

**Symptom:** Client hat IPv4-Adresse `169.254.93.253`, kein Gateway.

**Grund:** DHCP ist aktiv, aber **kein DHCP-Server** erreichbar → Windows
vergibt automatisch eine **APIPA**-Adresse aus `169.254.0.0/16`.
Folge: Nur Kommunikation im lokalen Segment, kein Internet.

**Lösung:**
- DHCP-Server verfügbar? → `ipconfig /renew`
- oder manuelle Konfiguration (IP, Maske, Gateway, DNS)

## 7.3 IPv6 link-local (`fe80::`)

`fe80::4257:a46c:5622:bb4`
- `fe80` = link-local (nur im Segment gültig)
- Interface Identifier wird aus MAC generiert oder zufällig

## 7.4 DNS-Fehler (FQDN nicht auflösbar, IP geht)

**Symptom:** Internet klappt, aber `fileserver.ihk.local` per Name nicht
erreichbar; `ipconfig` zeigt DNS `8.8.8.8` (öffentlich) statt DC.

**Ursache:** Öffentlicher DNS kennt interne Namen nicht.

**Lösung:** DNS-Server auf DC (z. B. `172.16.1.11`) setzen bzw. „automatisch
beziehen" (DHCP) aktivieren, falschen DNS (8.8.8.8) entfernen.

**Weitere DNS-Fehler-Varianten aus dem Korpus:**
- **2023W:** `nslookup` liefert Timeout, weil das **VLAN-Routing** falsch
  konfiguriert ist und der DNS-Server dadurch gar nicht erreichbar ist.
- **2026S:** `ipconfig` zeigt DNS-Server `192.168.5.1`, obwohl der
  DHCP-Server laut Konfiguration `192.168.5.250` ausliefert. **Ursache:**
  Client nutzt noch einen **alten DHCP-Lease** und hat die neue
  DNS-Einstellung noch nicht per Renew übernommen.

**`nslookup`-Ausgabe erklären (2021W, 2024W – fast identisch):**
- Zeile „Server: ..." → Name/IP des befragten DNS-Servers
- „Address: ...%<Zahl>" → **IPv6-Adresse** des DNS-Servers (Zone-ID nach
  `%`)
- „Nicht autorisierende Antwort" → Antwort kommt aus dem **Cache** eines
  Servers, der nicht die **Autorität (SOA)** für die Zone besitzt
- **A-Record** = IPv4-Adresse, **AAAA-Record** = IPv6-Adresse zum
  angefragten Namen

## 7.5 ICMP blockiert, aber Dienst erreichbar (2021W / 2024W)

**Symptom:** `ping server.ihk.local` läuft in den Timeout, aber der
Webserver ist im Browser bzw. per `telnet server 80` /
`Test-NetConnection server -Port 80` erreichbar.

**Ursache:** Firewall blockiert nur **ICMP (Ping)**, TCP-Port 80 ist
weiterhin freigegeben. Ping-Ausfall bedeutet **nicht automatisch**, dass
der Dienst nicht erreichbar ist!

## 7.6 tracert / traceroute

- **2021W:** Eine Zeile zeigt nur `*` statt einer IP-Adresse.
  → Der Hop hat das Paket **verarbeitet** (TTL abgelaufen), aber **keine
  ICMP-Time-Exceeded-Antwort** zurückgesendet (z. B. Firewall-Regel) –
  das heißt nicht, dass der Hop unerreichbar ist.
- **2023S:** `tracert` zeigt, wie der konfigurierte Proxy über einen
  **IPv6-Breakout** umgangen werden kann, weil die Proxy-Regel nur für
  IPv4 greift.

## 7.7 DHCP-Fehlkonfiguration

- **2023S:** Clients in einem VLAN bekommen keine IP, weil das
  **DHCP-Relay** (IP-Helper-Adresse) zwischen den VLANs fehlt und der
  Broadcast des DHCP-Discover den Server im anderen VLAN nicht erreicht.
- **2024W:** Falsch konfigurierter Adressbereich bzw. **Scope-Erschöpfung**
  (alle Adressen im Pool vergeben) → neue Clients bekommen keine IP mehr
  (fallen auf APIPA zurück).
- **2026S:** Vollständiger **DORA-Mitschnitt** (Discover – Offer – Request
  – Acknowledge) im Wireshark-Hexdump analysieren – jeden der 4 Schritte
  benennen und im Hexdump wiederfinden können.

## 7.8 CNAME-Fehlkonfiguration (2024W)

**Symptom:** Aufruf von `ftp.ihk.local` liefert „**404 File not found**"
oder landet auf dem falschen Server.

**Ursache:** Der **CNAME-Eintrag** zeigt auf den falschen Ziel-Hostnamen
(z. B. auf den Webserver statt den FTP-Server).

## 7.9 Geschwindigkeitsproblem: 100 Mbit/s statt Gigabit (2024S)

**Symptom:** Verbindung läuft nur mit 100 Mbit/s statt der erwarteten
1 Gbit/s.

**Mögliche Ursachen (nennen):**
- **Duplex-Fehlkonfiguration** (Auto-Negotiation schlägt fehl)
- **Kabel** entspricht nicht Cat5e/Cat6 (nur 2 statt 4 Adernpaare)
- **Portkonfiguration** am Switch (Port fest auf 100 Mbit/s eingestellt)

## 7.10 Mail-Client manuell einrichten (Telnet-Test)

Telnet-Ausgabe liefert die nötigen Parameter:
```
* OK [CAPABILITY IMAP4rev1 ... STARTTLS AUTH=PLAIN AUTH=LOGIN] Dovecot ready
```
→ **Protokoll:** IMAP
→ **Verbindungssicherheit:** STARTTLS
→ **Authentifizierung:** Passwort, normal

Ports merken:
- IMAP(S): 143 / 993
- POP3(S): 110 / 995
- SMTP(S): 25 / 465 (SMTPS) / 587

## Typische Prüfungsaufgabe

> „Erläutern Sie anhand der Konsolenausgabe die Fehlerursache."

→ Immer **konkret auf das gezeigte Feld** beziehen (z. B. „DNS steht auf
8.8.8.8 statt DC-IP").
