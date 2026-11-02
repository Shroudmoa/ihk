# DNS (Domain Name System)

## In 30 Sekunden

- DNS übersetzt **Namen in IP-Adressen** (und zurück) – hierarchisch: Root `.` → TLD `.de` → Domain `ihk.de` → Host `www.ihk.de`.
- Client fragt **rekursiv** seinen Resolver; der Resolver fragt **iterativ** Root-, TLD- und autoritativen Server (GA2 2023S, 2025S).
- Records, die immer kommen: **A, AAAA, CNAME, MX, NS, SOA, PTR, TXT** (SPF/DKIM/DMARC), **SRV** (Active Directory).
- Port **53**, meist **UDP**, **TCP** bei Zonentransfer und großen Antworten.
- **DNSSEC** sichert **Authentizität und Integrität** der Antworten – **nicht** die Vertraulichkeit.

## Kernwissen

### Record-Typen

| Typ | Bedeutung | Beispiel |
|---|---|---|
| **A** | Name → IPv4 | `www.firma.de. A 203.0.113.10` |
| **AAAA** | Name → IPv6 | `www.firma.de. AAAA 2001:db8::10` |
| **CNAME** | Alias → anderer Name | `ftp.firma.de. CNAME ftp1.firma.de.` |
| **MX** | Mailserver der Domain, mit Priorität (kleiner = bevorzugt) | `firma.de. MX 10 mail.firma.de.` |
| **NS** | zuständige Nameserver der Zone | `firma.de. NS ns1.firma.de.` |
| **SOA** | Start of Authority: Primärserver, Admin, Seriennummer, Timer | einer pro Zone |
| **PTR** | IP → Name (Reverse Lookup) | `10.113.0.203.in-addr.arpa. PTR www.firma.de.` |
| **TXT** | Freitext, u. a. **SPF, DKIM, DMARC**, Domain-Verifizierung | `v=spf1 ip4:203.0.113.0/24 -all` |
| **SRV** | Dienst + Port, z. B. Domänencontroller finden | `_ldap._tcp.dc._msdcs.firma.local` |

**TTL:** Jeder Record hat eine Lebensdauer im Cache. Lange TTL = weniger Anfragen, aber Änderungen wirken verzögert → vor Umzügen TTL senken.

### Namensauflösung Schritt für Schritt (`www.ihk.de`)

1. Client prüft **lokalen Cache** und `hosts`-Datei.
2. Client fragt seinen Resolver (z. B. Router oder DC) **rekursiv**: „Gib mir die fertige Antwort.“
3. Resolver fragt **iterativ** einen **Root-Server** → Verweis auf die `.de`-TLD-Server.
4. Resolver fragt einen **TLD-Server `.de`** → Verweis auf die autoritativen Server von `ihk.de`.
5. Resolver fragt den **autoritativen Server** von `ihk.de` → **A-/AAAA-Record**.
6. Resolver speichert die Antwort im Cache (TTL) und liefert sie dem Client.

| Begriff | Bedeutung |
|---|---|
| **Rekursiv** | Der befragte Server liefert die endgültige Antwort (oder einen Fehler) |
| **Iterativ** | Der befragte Server liefert nur einen Verweis auf den nächsten zuständigen Server |
| **Autoritativ** | Antwort vom Server, der die Zone verwaltet |
| **Nicht autorisierend** | Antwort stammt aus dem **Cache** eines anderen Servers (in `nslookup` sichtbar) |
| **Forwarder** | Interner DNS leitet Anfragen für fremde Zonen an einen festen Resolver (Provider) weiter – weniger externe Abfragen, zentraler Cache (2023S) |
| **Zone / Zonentransfer** | Verwaltungsbereich; Primär → Sekundär per AXFR/IXFR über **TCP 53** |

### `nslookup`-Ausgabe lesen (GA2 Winter 2021, Winter 2024)

```
C:\> nslookup www.google.de
Server:   ns.ihk-med.de                 <- befragter DNS-Server
Address:  2001:db8:1234::53             <- dessen IP (hier IPv6)

Nicht autorisierende Antwort:           <- aus dem Cache, nicht vom zuständigen Server
Name:     www.google.de
Addresses: 2a00:1450:4001:812::2003     <- AAAA-Record
           142.250.186.131              <- A-Record
```

### Spezielle Konzepte

| Konzept | Erklärung | Prüfung |
|---|---|---|
| **Split-Horizon / Split-Brain-DNS** | Derselbe Name liefert **intern eine private**, **extern die öffentliche** IP (getrennte Zonen/Views). Interne Nutzer gehen direkt, ohne Umweg über Firewall/NAT | 2026S |
| **DNS-Round-Robin / Load-Balancing** | Ein Name hat mehrere A/AAAA-Records; die Reihenfolge wechselt → Last verteilt. Erklärt, warum zwei Pings andere IPs zeigen | 2022W, 2025W |
| **Interne Zone** | AD-Domänen brauchen internen DNS (SRV-Records). Ein öffentlicher Resolver wie `8.8.8.8` kennt `server.firma.local` nicht | 2024S |
| **Reverse Lookup** | PTR-Records, wichtig für Mailserver (Spamprüfung) und Logs | – |

### Sicherheit

| Bedrohung / Schutz | Erklärung |
|---|---|
| **Cache Poisoning / DNS-Spoofing** | Gefälschte Antworten landen im Resolver-Cache → Nutzer werden auf falsche Server geleitet (2023S) |
| **DNSSEC** | Zonen werden signiert (RRSIG, DNSKEY, DS) → Resolver prüft **Herkunft (Authentizität)** und **Unverfälschtheit (Integrität)**; Antworten bleiben lesbar |
| **DoT / DoH** | DNS over TLS (TCP **853**) bzw. over HTTPS (**443**) → **Vertraulichkeit** der Anfragen |
| **Zonentransfer beschränken** | AXFR nur an eigene Sekundärserver erlauben |

### Befehle

| Befehl | Zweck |
|---|---|
| `nslookup name [server]` | Name auflösen, optional gegen einen bestimmten Server |
| `nslookup -type=mx firma.de` | bestimmten Record-Typ abfragen |
| `dig www.firma.de AAAA` | Linux-Werkzeug, ausführliche Ausgabe |
| `Resolve-DnsName` | PowerShell-Pendant |
| `ipconfig /displaydns` / `/flushdns` | Client-Cache anzeigen / leeren |

## Prüfungsfrage

**Frage (angelehnt an GA2 Winter 2024):** Seit dem Umzug des FTP-Dienstes auf `ftp1.ihk-med.de` (203.0.113.21) landen Nutzer bei Aufruf von `ftp.ihk-med.de` auf dem alten Webserver (203.0.113.80). In der Zone steht `ftp CNAME ihk-med.de.`. Erklären Sie den Fehler und die Korrektur.

**Musterantwort:** `ftp.ihk-med.de` ist ein Alias (CNAME) auf `ihk-med.de`. Der Resolver folgt dem Alias und liefert den A-Record von `ihk-med.de`, also 203.0.113.80 – den alten Webserver. Der CNAME muss auf den neuen Host zeigen: `ftp CNAME ftp1.ihk-med.de.` Danach löst `ftp.ihk-med.de` über `ftp1` auf 203.0.113.21 auf. Wegen der TTL wirkt die Änderung erst nach Ablauf der Cache-Zeit, Clients können mit `ipconfig /flushdns` nachhelfen.

## Falle

- **DNSSEC = Verschlüsselung** → falsch: Signatur für Authentizität und Integrität, keine Vertraulichkeit.
- Rekursiv und iterativ vertauschen: **Client → Resolver rekursiv**, **Resolver → Root/TLD/autoritativ iterativ**.
- „Nicht autorisierende Antwort“ als Fehler deuten → es ist nur eine Cache-Antwort.
- Ping per Name scheitert, per IP klappt → **DNS-Problem**, nicht Netzwerkproblem.
- CNAME auf der Zonenspitze (`firma.de`) oder zusammen mit anderen Records am selben Namen → nicht erlaubt.
- Clients mit `8.8.8.8` in einer AD-Domäne → interne Namen und Domänenanmeldung scheitern.

## Merksatz

**„A für vier, AAAA für sechs, MX für Mail, PTR rückwärts, TXT für SPF.“** Client fragt rekursiv, Resolver iterativ.

## Verwandt

- [DHCP](dhcp.md) – Option 6 verteilt den DNS-Server
- [E-Mail-Authentifizierung](../../nice-to-know/email-auth.md) – SPF, DKIM, DMARC als TXT-Records
- [Netzwerk-Befehle](../Netzwerk-Administration/netzwerk-befehle.md) – nslookup, dig, ipconfig
- [Netzwerkanalyse & Troubleshooting](../Netzwerk-Administration/netzwerkanalyse-und-troubleshooting.md) – DNS-Fehler eingrenzen
- [Proxy](../Netzwerk-Administration/proxy.md) – Reverse Proxy zusammen mit Split-Horizon
- [Active Directory](../Sicherheit/active-directory.md) – SRV-Records für Domänencontroller
- [Lektion 17 – DHCP & DNS](../../analyse-der-exams/L17_DHCP_DNS.md)
