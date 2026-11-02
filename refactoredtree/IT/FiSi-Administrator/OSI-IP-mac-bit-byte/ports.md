# Ports

## In 30 Sekunden

- Ein **Port** (16 Bit, 0–65535) adressiert auf **Schicht 4** die Anwendung auf einem Host; **Socket = IP + Protokoll + Port**.
- Bereiche: **0–1023 Well-Known**, **1024–49151 Registered**, **49152–65535 dynamisch/ephemeral** (Quellports der Clients).
- In fast jeder GA2 müssen Ports in **Firewallregeln** oder **Portweiterleitungen** eingetragen werden (Web 80/443, Mail 25/465/587/993, VPN 500/4500/1194).
- Immer auch das **Transportprotokoll** angeben: DNS UDP/TCP 53, DHCP UDP 67/68, SNMP UDP 161/162.
- Protokolle **ohne Port**: ICMP (Protokoll 1), ESP (50), AH (51), ARP (gar kein IP).

## Kernwissen

### Pflichtliste (prüfungsrelevant)

| Port | Proto | Dienst | Hinweis |
|---:|---|---|---|
| 20 / 21 | TCP | FTP Daten / Steuerung | unverschlüsselt |
| 22 | TCP | SSH, SFTP, SCP | sichere Fernwartung |
| 23 | TCP | Telnet | unverschlüsselt – nur noch zum Port-Test |
| 25 | TCP | SMTP (Server ↔ Server) | eingehende Mail |
| 53 | UDP/TCP | DNS | TCP für Zonentransfer, große Antworten |
| 67 / 68 | UDP | DHCPv4 Server / Client | |
| 69 | UDP | TFTP | PXE-Boot, Firmware |
| 80 | TCP | HTTP | |
| 88 | TCP/UDP | Kerberos | AD-Anmeldung |
| 110 | TCP | POP3 | |
| 123 | UDP | NTP | Zeitsynchronisation |
| 143 | TCP | IMAP | mit STARTTLS möglich |
| 161 / 162 | UDP | SNMP Abfrage / Trap | |
| 389 | TCP/UDP | LDAP | |
| 443 | TCP (UDP bei QUIC) | HTTPS | auch SSL-VPN |
| 445 | TCP | SMB | Dateifreigaben |
| 465 | TCP | SMTPS (implizites TLS) | Mail-Einlieferung |
| 500 | UDP | IKE (IPsec) | |
| 514 | UDP | Syslog | TLS-Variante: TCP 6514 |
| 546 / 547 | UDP | DHCPv6 Client / Server | |
| 587 | TCP | SMTP Submission (STARTTLS) | Mail-Client → Server |
| 636 | TCP | LDAPS | |
| 853 | TCP | DNS over TLS | |
| 993 | TCP | IMAPS | |
| 995 | TCP | POP3S | |
| 1194 | UDP (TCP) | OpenVPN | |
| 1433 | TCP | Microsoft SQL Server | |
| 1812 / 1813 | UDP | RADIUS Auth / Accounting | 802.1X, WPA-Enterprise |
| 1883 / 8883 | TCP | MQTT / MQTT über TLS | IoT |
| 3128 | TCP | Squid-Proxy | |
| 3306 | TCP | MySQL / MariaDB | |
| 3389 | TCP/UDP | RDP | nie direkt ins Internet |
| 4500 | UDP | IPsec NAT-Traversal | |
| 5060 / 5061 | UDP/TCP / TCP | SIP / SIP über TLS | VoIP-Signalisierung |
| 5432 | TCP | PostgreSQL | |
| 5900 | TCP | VNC | |
| 8080 / 8443 | TCP | HTTP/HTTPS alternativ | Proxy, Admin-Oberflächen |
| 51820 | UDP | WireGuard | Standard, frei wählbar |

### Mail-Ports sortiert (GA2 2021W, 2022W, 2024S, 2025S)

| Zweck | Klartext / STARTTLS | implizites TLS |
|---|---|---|
| Server ↔ Server (MTA) | 25 | – |
| Client → Server (Senden) | 587 | 465 |
| Abrufen IMAP | 143 | 993 |
| Abrufen POP3 | 110 | 995 |

**STARTTLS** = Verbindung startet im Klartext und wird per Befehl auf TLS umgeschaltet (z. B. IMAP 143). **Implizites TLS** = TLS ab dem ersten Byte (993, 465).

### Quell- und Zielport

- Der **Server** lauscht auf einem festen **Zielport** (z. B. 443).
- Der **Client** wählt einen zufälligen **Quellport** aus dem dynamischen Bereich (z. B. 51515).
- Bei **PAT** ersetzt der Router Quell-IP **und** Quellport (z. B. 10.0.0.1:45123 → 31.101.17.41:50001) – so ordnet er Antworten wieder zu (GA2 Winter 2025).
- In Firewallregeln ist der **Quellport** daher fast immer `any`.

### Ports ohne Port – IP-Protokollnummern

| Protokoll | Nummer | Bemerkung |
|---|---:|---|
| ICMP | 1 | Ping, Traceroute – Firewall erlaubt/sperrt „ICMP“, nicht einen Port |
| TCP | 6 | |
| UDP | 17 | |
| ESP | 50 | IPsec-Nutzdaten; hat keine Ports → hinter NAT nötig: **NAT-T über UDP 4500** (GA2 Sommer 2023) |
| AH | 51 | IPsec-Authentifizierung, nicht NAT-fähig |

### Port testen

| Befehl | Zweck |
|---|---|
| `Test-NetConnection server -Port 443` | PowerShell: TCP-Port erreichbar? |
| `telnet server 25` | Banner eines TCP-Dienstes lesen (z. B. IMAP CAPABILITY, 2024S) |
| `nc -vz server 22` | Linux netcat |
| `netstat -ano` / `ss -tulpn` | eigene offene Ports und Prozesse anzeigen |
| `nmap -sS server` | Portscan (nur mit Erlaubnis!) |

## Prüfungsfrage

**Frage (angelehnt an GA2 Winter 2021 / Sommer 2024):** Der Webserver `10.10.0.10` und der Mailserver `10.10.0.11` in der DMZ sollen aus dem Internet erreichbar sein. Die Mitarbeiter sollen Mails nur verschlüsselt senden und abrufen. Nennen Sie die nötigen Portweiterleitungen mit Protokoll.

**Musterantwort:**
- Web: TCP **443** → `10.10.0.10:443` (optional TCP 80 nur für die Weiterleitung auf HTTPS).
- Mail-Empfang von fremden Servern: TCP **25** → `10.10.0.11:25`.
- Mail-Versand durch Mitarbeiter: TCP **465** (implizites TLS) oder **587** (STARTTLS) → `10.10.0.11`.
- Mail-Abruf: TCP **993** (IMAPS) → `10.10.0.11:993`.
Alle übrigen eingehenden Verbindungen werden verworfen.

## Falle

- Protokoll vergessen: „Port 53“ allein reicht nicht – **UDP und TCP 53**.
- DHCP-Ports vertauschen: **Server 67, Client 68**.
- SNMP-Trap auf 161 → Traps gehen an **162**.
- ICMP oder ESP einen Port zuordnen → beide haben **keine** Ports.
- Telnet als Fernwartung empfehlen → unverschlüsselt; **SSH 22** verwenden.
- RDP 3389 oder SMB 445 ins Internet freigeben → klassisches Einfallstor, nur über VPN.

## Merksatz

**„22 sicher rein, 25 Mail rein, 53 Namen, 80 offen, 443 sicher, 993 Postfach sicher.“** Und: immer TCP oder UDP dazuschreiben.

## Verwandt

- [TCP vs. UDP](tcp-udp.md) – Transportprotokolle hinter den Ports
- [Firewall](../Sicherheit/firewall/firewall.md) – Regeln mit Ports formulieren
- [NAT & PAT](../Sicherheit/firewall/nat-pat.md) – Portweiterleitung und PAT
- [Proxy](../Netzwerk-Administration/proxy.md) – Squid 3128, Reverse Proxy 443
- [E-Mail-Authentifizierung](../../nice-to-know/email-auth.md) – SPF, DKIM, DMARC
- [VPN](../../nice-to-know/VPN/readme.md) – Ports von IPsec, OpenVPN, WireGuard
- [Lektion 16 – Firewall](../../analyse-der-exams/L16_Firewall.md)
