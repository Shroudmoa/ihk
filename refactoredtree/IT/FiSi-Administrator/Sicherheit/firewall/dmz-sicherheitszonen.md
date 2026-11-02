# DMZ und Sicherheitszonen

## In 30 Sekunden

- Die **DMZ** (demilitarisierte Zone) ist ein eigenes Netzsegment zwischen Internet und internem LAN für **Server, die von außen erreichbar sein müssen** (Web, Mail-Gateway, Reverse Proxy, VPN-Gateway).
- Wird ein DMZ-Server kompromittiert, steht der Angreifer **nicht** im internen Netz – die Firewall zwischen DMZ und LAN bremst ihn.
- Grundregel: Internet → DMZ nur die veröffentlichten Dienste, DMZ → LAN **so gut wie nichts**, Internet → LAN **nie** direkt.
- Bauformen: **einstufig** (eine Firewall mit drei Schnittstellen) oder **zweistufig** (zwei Firewalls, DMZ dazwischen).
- Klassiker in GA2 (2021W, 2022S, 2022W, 2023W, 2025S, 2026S): „Welcher Server gehört in die DMZ? Begründen Sie.“

## Kernwissen

**Was gehört wohin?**

| Zone | Typische Systeme | Begründung |
|---|---|---|
| **Internet** (untrusted) | Kunden, Angreifer | kein Vertrauen |
| **DMZ** (semi-trusted) | Webserver, Reverse Proxy/WAF, Mail-Relay/Spamfilter, VPN-Gateway, öffentlicher DNS | muss von außen erreichbar sein, hohes Angriffsrisiko |
| **LAN / interne Zone** (trusted) | Active Directory/Domain Controller, Datenbank-, File-, ERP-Server, Clients | sensible Daten, **nie** direkt aus dem Internet erreichbar |
| **Weitere Zonen** | Gast-WLAN, Management-Netz, Server-Netz, IoT/Produktion | jeweils eigenes VLAN/Subnetz mit eigenen Regeln |

**Bauformen**

| Variante | Aufbau | Vorteil | Nachteil |
|---|---|---|---|
| **Einstufig** („Three-legged“) | eine Firewall mit Schnittstellen WAN, DMZ, LAN | günstig, einfach | eine Fehlkonfiguration oder Lücke öffnet beide Übergänge |
| **Zweistufig** (Screened Subnet) | äußere Firewall Internet↔DMZ, innere Firewall DMZ↔LAN | doppelte Hürde; mit **zwei Herstellern** wirkt eine Lücke nicht auf beide | teurer, mehr Verwaltungsaufwand |

**Regelwerk (Beispiel Webshop)**

| # | Quelle | Ziel | Dienst | Aktion | Zweck |
|---:|---|---|---|---|---|
| 1 | Internet | Webserver (DMZ) | TCP 443 | allow | Shop veröffentlichen |
| 2 | Webserver (DMZ) | DB-Server (LAN) | TCP 3306 | allow | nur diese eine Verbindung |
| 3 | Admin-Netz (LAN) | DMZ | TCP 22 | allow | Wartung von innen |
| 4 | DMZ | LAN | any | deny | Lateral Movement verhindern |
| 5 | Internet | LAN | any | deny | interne Zone schützen |
| 6 | any | any | any | deny | Default Deny |

**Typische Detailfragen**
- **DMZ-Server mit privaten IPs erreichbar machen (2023W):** über **Destination-NAT / Port-Forwarding** der öffentlichen IP auf die interne DMZ-Adresse.
- **SSH aus dem Internet in die DMZ (2023W):** besser nicht offen ins Internet, sondern **über VPN**; falls doch: nur Schlüssel-Login, Quell-IP einschränken, Fail2ban/Rate-Limit.
- **Reverse Proxy (2022S):** steht in der DMZ, terminiert TLS, verbirgt die Backend-Server und filtert als WAF Angriffe wie SQL-Injection.
- **IPv6-DMZ (2025S):** ein /64 je Segment; NAT entfällt, die Firewall-Regeln übernehmen die Trennung allein.

## Prüfungsfrage

**Frage:** Ein Unternehmen betreibt einen Webserver, einen Datenbankserver und einen Domain Controller. Ordnen Sie die Server der DMZ oder dem internen Netz zu und begründen Sie, warum der Webserver nicht im internen Netz stehen sollte.

**Musterantwort:** Der Webserver gehört in die DMZ, weil er aus dem Internet erreichbar sein muss. Datenbankserver und Domain Controller gehören ins interne Netz, weil sie sensible Daten und Identitäten enthalten und nie direkt aus dem Internet erreichbar sein dürfen; der Webserver darf nur über eine eng gefasste Firewall-Regel auf den Datenbankport zugreifen. Stünde der Webserver im internen Netz, hätte ein Angreifer nach einer erfolgreichen Kompromittierung sofort Zugriff auf alle internen Systeme. Die DMZ begrenzt diesen Schaden, weil eine zweite Firewall-Grenze überwunden werden muss.

## Falle

- Datenbank „mit in die DMZ“ stellen, damit der Webserver schneller zugreift → falsch, sensible Daten bleiben intern.
- DMZ mit „Bereich ohne Firewall“ verwechseln – die DMZ ist **stark gefiltert**, nicht offen.
- Pauschal „DMZ → LAN allow“ → dann ist die DMZ wirkungslos.
- Den „Exposed Host“/„DMZ-Host“ von Heimroutern für eine echte DMZ halten: der leitet nur alle Ports an einen Rechner im LAN weiter.

## Merksatz

**„Was von außen erreichbar sein muss, kommt in die DMZ – was wertvoll ist, bleibt drinnen.“**

## Verwandt

- [Firewall – Regeln lesen und schreiben](firewall.md)
- [Firewall-Generationen und Sicherheitskonzepte](firewalls-alle-generationen-sicherheitskonzepte-antivirus.md)
- [NAT und PAT](nat-pat.md) – Port-Forwarding in die DMZ
- [Proxy und Reverse Proxy](../../Netzwerk-Administration/proxy.md)
- [Zero Trust](../zero-trust.md) – Segmentierung weitergedacht
- [VLAN](../../Netzwerk-Administration/vlan.md) – Zonen technisch trennen
- [Lektion 16 Firewall](../../../analyse-der-exams/L16_Firewall.md)
- [Lektion 29 GA2-Praxisthemen (DMZ)](../../../analyse-der-exams/L29_GA2_Sonstiges_Praxisthemen.md)
