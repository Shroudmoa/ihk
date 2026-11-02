# Proxy, Reverse Proxy & Port Forwarding

## In 30 Sekunden

- **Forward Proxy** steht **vor den Clients** und vermittelt deren Anfragen ins Internet: filtern, protokollieren, cachen, Malware scannen (z. B. Squid, Port 3128).
- **Reverse Proxy** steht **vor den Servern** und nimmt Anfragen aus dem Internet an: **TLS-Terminierung, Load Balancing, Filterung (WAF), Caching**, Backend-Adressen bleiben verborgen (nginx, HAProxy, Traefik).
- **Port Forwarding** (Destination-NAT) leitet ein Paket nur **1:1 unverändert** an eine interne IP:Port weiter – keine Inhaltsprüfung (GA2 Sommer 2026).
- Proxys arbeiten auf **Schicht 7**, Port Forwarding auf **Schicht 3/4**.
- HTTPS-Inhalte kann ein Proxy nur per **TLS-Inspection** (eigene CA, „gewollter MitM“) prüfen – Clients müssen der Proxy-CA vertrauen (GA2 Sommer 2022).

## Kernwissen

### Vergleich

| Merkmal | Forward Proxy | Reverse Proxy | Port Forwarding |
|---|---|---|---|
| Weg | Client → Proxy → Internet | Internet → Reverse Proxy → Backend | Internet → Router/Firewall → interner Server |
| schützt / vertritt | Clients | Server | – (öffnet nur einen Weg) |
| Richtung | ausgehend | eingehend | eingehend |
| OSI-Ebene | 7 | 7 | 3/4 |
| Inhaltsanalyse | ja (URL-Filter, AV, DLP) | ja (WAF, Header, Pfade) | nein |
| TLS | optional Inspection | **Terminierung** üblich | Ende-zu-Ende zum Server |
| Load Balancing | nein | häufig | nein |
| Zielauswahl | – | nach Hostname/Pfad (`/shop` → A, `/api` → B) | nach Port |

### Forward Proxy

- Aufgaben: Webzugriffe **filtern** (Kategorien, Black-/Whitelists), **protokollieren**, **cachen**, **Malware scannen**, zentrale **Authentifizierung** der Benutzer.
- Konfiguration der Clients: manuell, per **GPO**, per **PAC-Datei** (JavaScript mit Regeln) oder automatisch per **WPAD** (DNS/DHCP).
- **Transparenter Proxy:** Router leitet Port 80/443 ohne Client-Konfiguration auf den Proxy um.
- HTTPS über einen klassischen Proxy läuft per **`CONNECT`**-Tunnel – der Proxy sieht dann nur Ziel und Datenmenge, nicht den Inhalt.
- Linux-Clients: Umgebungsvariablen `HTTP_PROXY`/`HTTPS_PROXY`, z. B. `curl -x http://proxy.firma.de:3128 https://example.com`. Zugangsdaten nicht im Klartext in Variablen oder Shell-Historie ablegen.

**Proxy-Umgehung (GA2 Sommer 2023):** Proxy- und Firewallregeln nur für IPv4 – der Client nutzt IPv6 direkt (im `tracert` sichtbar) → Proxy auch für IPv6 bereitstellen bzw. direkten Ausgang ohne Proxy in der Firewall sperren.

### TLS-Inspection (GA2 Sommer 2022, Winter 2024)

1. Client baut TLS zum Proxy auf; der Proxy erzeugt **on the fly ein Zertifikat** für die Zielseite, signiert von **seiner eigenen CA**.
2. Proxy baut eine zweite TLS-Verbindung zum echten Server auf.
3. Inhalt wird entschlüsselt, gescannt (AV, IPS, DLP) und wieder verschlüsselt.

| Fehlermeldung im Browser | Ursache | Lösung |
|---|---|---|
| `NET::ERR_CERT_AUTHORITY_INVALID` / `SEC_ERROR_UNKNOWN_ISSUER` | Client kennt die Proxy-CA nicht | CA-Zertifikat per **GPO/MDM** in den Vertrauensspeicher verteilen |
| `SEC_ERROR_EXPIRED_ISSUER_CERTIFICATE` | Proxy-CA-Zertifikat abgelaufen | CA erneuern, neu verteilen |
| `SSL_ERROR_VERSION_OR_CIPHER_MISMATCH` | TLS-Version/Cipher passen nicht | TLS 1.2/1.3 und aktuelle Cipher am Proxy erlauben |

**Nachteile/Pflichten:** Datenschutz (Banking, Gesundheit – Ausnahmen definieren), **Betriebsrat/Betriebsvereinbarung** und Transparenz, Performance, Anwendungen mit **Certificate Pinning** brechen.

### Reverse Proxy

- **TLS-Terminierung/Offloading:** Zertifikat liegt zentral am Proxy, Backends werden entlastet.
- **Load Balancing** und Health Checks über mehrere Webserver.
- **Sicherheit:** Backend nicht direkt erreichbar, **WAF** filtert SQL-Injection/XSS, Rate Limiting.
- **Caching** statischer Inhalte, Kompression.
- **Routing** nach Hostname/Pfad, mehrere Dienste hinter **einer** öffentlichen IP auf Port 443.
- Typisch in der **DMZ**, die Backends dahinter im internen Segment.

### Port Forwarding

| Externer Port | Ziel intern | Zweck |
|---|---|---|
| TCP 443 | `192.168.199.10:443` | HTTPS-Webserver in der DMZ |
| TCP 22 | `192.168.199.20:22` | SSH – besser nur über VPN |

Risiken: interner Dienst direkt angreifbar, jede Schwachstelle ist von außen ausnutzbar. Schutz: nur nötige Ports, Quell-IP-Beschränkung, Patchen, für Administration VPN statt Weiterleitung.

## Prüfungsfrage

**Frage (angelehnt an GA2 Sommer 2026):** Der Webshop im Rechenzentrum ist bisher per Portweiterleitung TCP 443 erreichbar. Künftig soll ein Reverse Proxy eingesetzt werden. Grenzen Sie beide Verfahren ab und nennen Sie zwei Vorteile des Reverse Proxys.

**Musterantwort:** Bei der **Portweiterleitung** schreibt der Router nur die Zieladresse eingehender Pakete auf die interne IP des Webservers um (Destination-NAT) und leitet sie unverändert weiter; er arbeitet auf Schicht 3/4 und prüft keine Inhalte – der Webserver ist direkt dem Internet ausgesetzt. Ein **Reverse Proxy** nimmt die HTTPS-Verbindung selbst an (Schicht 7) und baut eine eigene Verbindung zum Backend auf. Vorteile: (1) **TLS-Terminierung und Filterung** – Anfragen können entschlüsselt und z. B. per WAF auf Angriffe geprüft werden, bevor sie den Shop erreichen; (2) **Load Balancing** – Anfragen lassen sich auf mehrere Webserver verteilen, was Verfügbarkeit und Leistung erhöht. Zusätzlich bleibt die interne Struktur verborgen und statische Inhalte können gecacht werden.

## Falle

- Forward und Reverse vertauschen → **Forward vor Clients**, **Reverse vor Servern**.
- „Proxy sieht automatisch HTTPS-Inhalte“ → nur mit **TLS-Inspection** und vertrauter CA.
- Port Forwarding als Sicherheitsmaßnahme darstellen → es **öffnet** einen Weg.
- Reverse Proxy mit „versteckt alles, also sicher“ begründen → das Backend muss trotzdem gepatcht und abgesichert sein.
- TLS-Inspection ohne Hinweis auf Datenschutz/Betriebsrat → Punktabzug bei „organisatorische Maßnahme“.

## Merksatz

**„Forward vor dem Client, Reverse vor dem Server, Port Forwarding nur ein Loch in der Wand.“**

## Verwandt

- [Firewall](../Sicherheit/firewall/firewall.md) – Abgrenzung Proxy vs. Paketfilter/NGFW
- [NAT & PAT](../Sicherheit/firewall/nat-pat.md) – Destination-NAT
- [Load Balancer](../Server/load-balancer.md) – Lastverteilung auf Layer 4/7
- [PKI](../Sicherheit/Verschluesselung/pki.md) – CA-Zertifikate für TLS-Inspection
- [DNS](../OSI-IP-mac-bit-byte/dns.md) – Split-Horizon zusammen mit Reverse Proxy
- [OSI](../OSI-IP-mac-bit-byte/osi.md) – Schicht 7
- [Netzwerktypen](../netzwerktypen.md) – Zugriff aus LAN/WAN nach außen
- [Lektion 17 – DHCP & DNS (Reverse Proxy)](../../analyse-der-exams/L17_DHCP_DNS.md)
- [Lektion 27 – PKI, TLS & VPN](../../analyse-der-exams/L27_PKI_TLS_Verschluesselung_VPN.md)
