# Proxy

## Proxy, Reverse Proxy und Port Forwarding (IHK)

### 1. Vergleich

| Technik | Kommunikationsweg | Zweck | Beispiel |
|---|---|---|---|
| Forward Proxy | Client → Proxy → Internet | Kontrolliert ausgehende Client-Anfragen. | Squid |
| Reverse Proxy | Client → Reverse Proxy → Webserver | Schützt und vermittelt Anfragen an Backend-Server. | Nginx, HAProxy |
| Port Forwarding | Internet → Router/Firewall → interner Server | Leitet einen externen Port an eine interne IP und einen Port weiter. | TCP 443 → `192.168.1.10:443` |

**Merksatz:**
- Forward Proxy = vor dem Client.
- Reverse Proxy = vor dem Server.
- Port Forwarding = NAT-/Firewall-Regel zur Weiterleitung eingehender Verbindungen.

### 2. Forward Proxy

**Aufgaben**
- Webzugriffe filtern und protokollieren.
- Inhalte zwischenspeichern (Caching).
- Zentrale Zugriffskontrolle für Clients.
- Ausgehende Verbindungen über eine zentrale Stelle vermitteln.

**Beispiel:** Squid auf einer Linux-VM.

Beispiel mit curl:
`curl -x http://proxy.example.com:3128 -U max:PASSWORT https://example.com`

**Umgebungsvariablen unter Linux:**

- `HTTP_PROXY`: Proxy für HTTP-Anfragen.
- `HTTPS_PROXY`: Proxy für HTTPS-Anfragen.

Authentifizierungsdaten sollten nicht ungeschützt in Umgebungsvariablen oder Shell-Historien gespeichert werden. Die HTTPS-Verbindung zum Ziel wird bei einem gewöhnlichen HTTP-Proxy über `CONNECT` aufgebaut.

**Begriffe**
- **Squid:** Proxy-Software.
- **PAC:** JavaScript-Datei mit Regeln zur automatischen Proxy-Auswahl.
- **WPAD:** Mechanismus zur automatischen Erkennung der Proxy-Konfiguration.
- **DHCP:** Kann unter anderem IP-Adresse, Gateway und DNS-Server verteilen; WPAD kann optional über DHCP-Optionen bekannt gegeben werden.

### 3. Reverse Proxy

**Kommunikationsweg:**

`Client → Reverse Proxy → Backend-Webserver`

**Aufgaben**
- **TLS-Terminierung:** HTTPS wird am Proxy entschlüsselt; die Verbindung zum Backend kann separat verschlüsselt werden.
- **Load Balancing:** Verteilung von Anfragen auf mehrere Server.
- **Routing:** `/shop` → Server A, `/api` → Server B.
- **Caching:** Zwischenspeichern geeigneter Antworten.
- **Security:** Backend-Server müssen nicht direkt aus dem Internet erreichbar sein.

**Beispiele:** Nginx, HAProxy, Traefik, Cloudflare.

**Wichtig:** Ein Reverse Proxy versteckt interne Serveradressen gegenüber externen Clients, garantiert aber allein keine Sicherheit. Backend-Zugriffe und Proxy-Konfiguration müssen abgesichert sein.

### 4. Port Forwarding

**Kommunikationsweg:**

`Internet → Router/Firewall → interner Server`

Beispielregel:

| Externer Port | Interne Zieladresse | Zweck |
|---|---|---|
| TCP 443 | `192.168.1.10:443` | HTTPS-Webserver |
| TCP 22 | `192.168.1.20:22` | SSH-Server, möglichst nicht direkt öffentlich |

Der Router übersetzt beziehungsweise leitet eingehende Verbindungen anhand einer NAT-/Portweiterleitungsregel an den internen Server weiter.

**Risiken**
- Interne Dienste werden von außen erreichbar.
- Fehlkonfigurationen können ungewollten Zugriff ermöglichen.
- Offene Ports erhöhen die Angriffsfläche.

**Schutz:** Nur benötigte Ports freigeben, Systeme patchen, Zugriffe beschränken und möglichst VPN statt direkter Administration aus dem Internet verwenden.

### 5. IHK-Abgrenzung

| Merkmal | Forward Proxy | Reverse Proxy | Port Forwarding |
|---|---|---|---|
| Hauptziel | Clients | Server | Interner Dienst |
| Richtung | Ausgehend | Eingehend | Eingehend |
| Typische OSI-Ebene | 7 | 7 | Meist 3/4 |
| Inhaltsanalyse möglich? | Ja, abhängig von Konfiguration | Ja, abhängig von Konfiguration | Normalerweise nein |
| Load Balancing | Nicht Hauptaufgabe | Häufig | Nicht automatisch |
| NAT erforderlich? | Nicht zwingend | Nicht zwingend | Typischerweise ja |

### 6. Verbindung zu anderen IHK-Themen

- **OSI-Modell:** Proxys arbeiten typischerweise auf Layer 7; klassisches Port Forwarding basiert auf IP-Adressen und Transportports.
- **Ports:** HTTP TCP 80, HTTPS TCP 443, Squid häufig TCP 3128.
- **Firewall:** Kontrolliert Verbindungen anhand von Regeln; ein Proxy kann zusätzlich Anwendungsdaten prüfen.
- **DSGVO Art. 32:** Proxy-Logs können personenbezogene Daten enthalten und müssen angemessen geschützt werden.
- **Schutzziele:** Zugriffskontrolle unterstützt Vertraulichkeit; Logging unterstützt die Nachvollziehbarkeit; Redundanz eines Reverse Proxys kann die Verfügbarkeit erhöhen.

**Prüfungssatz:** Ein Forward Proxy vermittelt Client-Anfragen, ein Reverse Proxy vermittelt Server-Anfragen und Port Forwarding leitet eingehende Verbindungen anhand einer NAT-Regel an einen internen Dienst weiter.

## Verwandt

- [Firewall](firewall.md) – Abgrenzung Proxy vs. Firewall
- [OSI](osi.md) – Schicht 7: Anwendungsprotokolle
- [Netzwerktypen](netzwerktypen.md) – Zugriff aus LAN/WAN nach außen

