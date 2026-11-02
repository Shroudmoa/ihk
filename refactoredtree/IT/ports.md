# Ports

## Wichtige Ports zum Auswendiglernen

| Port | Protokoll / Dienst |
|---:|---|
| 20/21 | FTP-Datenübertragung / FTP-Steuerung, klassischer FTP-Modus |
| 22 | SSH, SFTP |
| 25 | SMTP-Serverkommunikation, häufig zwischen Mailservern |
| 53 | DNS über UDP oder TCP |
| 67/68 | DHCPv4-Server / DHCPv4-Client |
| 80 | HTTP |
| 110 | POP3 |
| 123 | NTP |
| 143 | IMAP |
| 161/162 | SNMP-Abfragen / SNMP-Traps |
| 389 | LDAP |
| 443 | HTTPS |
| 445 | SMB |
| 465 | SMTP über implizites TLS, üblich für Mail-Submission |
| 587 | SMTP Message Submission |
| 636 | LDAPS |
| 993 | IMAP über implizites TLS |
| 995 | POP3 über implizites TLS |
| 1433 | Microsoft SQL Server, Standardport |
| 3306 | MySQL, Standardport |
| 3389 | RDP |
| 5432 | PostgreSQL, Standardport |
| 5900 | VNC, häufig verwendeter Standardport |


## Weiterführende Ports (Registriert / Spezialisiert)

| Port | Protokoll / Dienst | Typical Use (IHK-relevant) |
|------|-------------------|----------------------------|
| 23   | Telnet            | Legacy-Remotezugriff (sicherheitskritisch) |
| 88   | Kerberos          | Authentifizierung in AD-Umgebungen |
| 135  | RPC               | Microsoft-DCOM / Anmelde-Dienste |
| 137/138/139 | NetBIOS | Windows-Netzwerkb_define, Filesharing |
| 161  | SNMP (v2/v3)      | Netzwerkmanagement (DSGVO-konforme Protokollierung) |
| 179  | BGP               | Internet-Routing |
| 389  | LDAP (cleartext)  | Directory‑Services (unsicher) |
| 443  | HTTPS (variiert)   | Webdienstarbeit |
| 502  | Modbus TCP        | Industrielle Steuerungssysteme (SCADA) |
| 514  | Syslog / rsyslog  | Ereignisprotokollierung (DSGVO-Audit-Trail) |
| 623  | IPMI / Solace     | Out-of-Band-Serververwaltung |
| 873  | rsync            | Verteilte Dateisynchronisation |
| 902  | VMware‑Tools      | VM-Verwaltung |
| 1080 | SOCKS5 Proxy      | Anonymisierung & Bypass |
| 1161 | SNMPv5 (Experimentell) | Neue Management-Protokolle |
| 1701 | L2TP              | VPN-Zustandaufrechterhaltung |
| 1723 | PPTP              | Legacy-VPN (veraltet) |
| 2049 | NFS              | Netzwerk-Dateisystemfreigabe |
| 3124 | UPnP‑Control      | UPnP-Fernsteuerung (Portweiterleitung) |
| 4500 | IPsec NAT‑T       | VPN-Verschlüsselung (Strongswan, LibreSwan) |
| 5060/5061 | SIP / SIPS   | VoIP-Anrufe |
| 5900 | VNC (SSL-Variante 5901) | Remote-Desktop |
| 6379 | Redis DB          | In-Memory-Datenbank |
| 6081 | VXLAN             | Software-definiertes Netzwerk (SDN) |
| 8080 | HTTP‑ALT          | Webanwendungen hinter NAT |
| 8443 | HTTPS‑ALT         | Webanwendungen hinter Proxy |
| 9200 | Elasticsearch    | Volltextsuche / Logging (ELK-Stack) |
| 9300 | ES-Cluster-Intern | Interner Knotenkommunikation |
| 27017| MongoDB           | NoSQL-Datenbank |
| 11211| Memcached         | Caching-Service (speichert sensible Daten) |
| 21726| Windows Update    | Lokaler WSUS-Client (IPv6) |
| 4444 | RemoteShell (Metasploit) | Penetrationstests (wichtig für IHK-Prüfungen) |

---

## Statistik / „Wie viele Ports?“

- **TCP-Bereich:** 0–65535 (65 536 Werte)
- **UDP-Bereich:** 0–65535 (gleich)
- **WKP (0–1023):** 1024 Ports, davon ~16 für Application-Ausstuben relevant für die IHK-Ausbildung (die obige WKP-Tabelle enthält die wichtigsten).
- **Registrierte Ports (1024–49151):** 48 128 Ports – alles, was hier erscheint, ist für spezialisierte Dienste „registriert“.
- **Dynamische/Private Ports (49152–65535):** 16 384 Ports – werden vom Betriebssystem automatisch zugewiesen, nicht zum Auswendiglernen geeignet.

**Ziel für die IHK-Prüfung:** Beherrschen Sie die ~16 WKP + die wichtigsten registrierten Ports (≈30 insgesamt). Alles, was darüber hinausgeht, ist „Referenzwissen“ für Firewallregeln, DSGVO-konforme Protokollierung (z. B. Syslog/514) und basierte Modellüberlegungen (Layer 3/4).

---

## DSGVO & IHK-„Double-Take“

- **Syslog (Port 514)**: Protokolliert Netzwerkereignisse, Pflicht für DSGVO-Audit-Trails.
- **SNMP (161/162)**: Überwachung von Geräten – Speicherung von Community-String und Türnummern muss verschlüsselt werden (SNMPv3) oder ist DSGVO-konform.
- **NFS (2049) / SMB (445)**: Dateiserverfreigaben – Zugriffskontrolle, Verschlüsselung (SMB‑3, NFSv4 Kerberos) erforderlich, um „personenbezogene Daten“ zu schützen.

---

## Technische Pflege ( OSI‑Modell )

| OSI‑Schicht | Relevant für die von Ihnen gelernten Ports |
|-----------|--------------------------------------|
| Layer 1 (PHY) | Keine direkte Portzuordnung |
| Layer 2 (DL)  | VLAN/802.1Q, MAC-Bindings (z. B. Port 23, 445) |
| Layer 3 (IP)  | Routing, NAT (Portweiterleitung für 443, 587, 993) |
| Layer 4 (TCP/UDP) | **Alle Ports** – Transportprotokolle; Portnummern identifizieren den Endpunkt der Anwendung. |
| Layer 5 (Session) | Kerberos (88), L2TP (1701), PPTP (1723) |
| Layer 6 (Presentation) | SSL/TLS‑verschlüsselte Ports (443, 636, 993, 995) |
| Layer 7 (Application) | FTP, SMTP, DNS, LDAP, HTTP, RDP, VNC, SQL, MongoDB usw. |

**Faustregel:** Wenn Sie einen Port für die IHK kennen, notieren Sie den OSI‑Layer, den er repräsentiert – das zeigt Ihnen, welches Sicherheitsniveau (z. B. TLS am Presentation‑Layer) erforderlich ist.

---

**Zusammenfassung (savage):** Wenn Sie denken, Sie müssen sich alle 65 535 Ports merken – Sie verschwenden Zeit. Konzentrieren Sie sich auf die 16 wichtigsten WKP + 30 registrierte Ports, die in den IHK-Prüfungen auftreten werden. Alles andere ist Referenzwissen für die Sicherheitspolitiken (DSGVO, Netzwerküberwachung) und das Protokoll-Design auf Layer 4. Wenn Sie zusätzliche Ports benötigen, die nicht in der obigen Liste stehen, fügen Sie sie einfach zur „Weiterführende Ports“-Tabelle hinzu und ordnen Sie sie dem entsprechenden OSI‑Layer und der DSGVO-bezogenen Einschränkung zu.
