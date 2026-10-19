# OSI

## OSI-Modell – 7 Schichten (IHK)

| Nr. | Schicht | Aufgabe | Protokolle / Beispiele | Geräte / Adressen |
|---:|---|---|---|---|
| 7 | Anwendung | Stellt Netzwerkdienste für Anwendungen bereit. | HTTP(S), DNS, DHCP, SMTP, FTP | Proxy, Application Gateway |
| 6 | Darstellung | Datenformate, Zeichencodierung, Kompression und ggf. Verschlüsselung. | JPEG, PNG, UTF-8, JSON, TLS* | – |
| 5 | Sitzung | Verwaltet Sitzungen: Aufbau, Aufrechterhaltung und Beendigung. | RPC, NetBIOS, Sitzungsfunktionen von Anwendungen | – |
| 4 | Transport | Ende-zu-Ende-Transport, Ports, Segmentierung und ggf. Zuverlässigkeit. | TCP, UDP | TCP-/UDP-Ports |
| 3 | Vermittlung | Logische Adressierung und Routing zwischen Netzwerken. | IPv4, IPv6, ICMP, IPsec | Router, IP-Adresse |
| 2 | Sicherung | Übertragung innerhalb eines lokalen Netzes, Frames und MAC-Adressierung; Fehlererkennung. | Ethernet, WLAN (MAC), PPP | Switch, MAC-Adresse |
| 1 | Bitübertragung | Überträgt Bits als elektrische, optische oder Funksignale. | Ethernet-PHY, WLAN-PHY | Kabel, Glasfaser, Funk, Repeater |

*TLS wird im OSI-Modell nicht eindeutig einer einzelnen Schicht zugeordnet; häufig wird es zwischen Schicht 4 und 7 eingeordnet.

## Prüfungserklärungen in 3 Sätzen

**1. Bitübertragungsschicht**
- Überträgt Bits als elektrische Signale, Lichtimpulse oder Funkwellen.
- Definiert unter anderem Signalpegel, Frequenzen und Übertragungsmedien.
- Beispiele: Kupferkabel, Glasfaser und WLAN-Funk.

**2. Sicherungsschicht**
- Überträgt Frames innerhalb eines lokalen Netzsegments.
- MAC-Adressen identifizieren Netzwerkschnittstellen; Prüfsummen erkennen Übertragungsfehler.
- Beispiele: Ethernet, WLAN und Switches.

**3. Vermittlungsschicht**
- Adressiert Datenpakete logisch und vermittelt sie zwischen Netzwerken.
- IP-Adressen identifizieren Quell- und Zieladressen; Router leiten Pakete weiter.
- Beispiele: IPv4, IPv6 und ICMP.

**4. Transportschicht**
- Ermöglicht die Kommunikation zwischen Anwendungen über Portnummern.
- TCP bietet unter anderem zuverlässige, geordnete Übertragung; UDP bietet verbindungslosen Transport ohne Zustellgarantie.
- Beispiele: TCP 443 für HTTPS, UDP 53 für DNS.

**5. Sitzungsschicht**
- Verwaltet logische Kommunikationssitzungen zwischen Anwendungen.
- Unterstützt je nach Protokoll den Aufbau, die Steuerung und die Beendigung einer Sitzung.
- Sitzungsfunktionen können auch in höheren Protokollen implementiert sein.

**6. Darstellungsschicht**
- Definiert, wie Anwendungsdaten codiert und strukturiert werden.
- Formate wie JPEG, PNG und JSON ermöglichen die Interpretation der Daten.
- Kompression und Verschlüsselung werden dem Modell häufig hier zugeordnet, liegen praktisch aber nicht immer auf dieser Schicht.

**7. Anwendungsschicht**
- Stellt Netzwerkdienste für Anwendungen bereit.
- HTTP(S) dient der Webkommunikation, DNS der Namensauflösung und SMTP dem E-Mail-Versand.
- DHCP konfiguriert IP-Parameter; FTP überträgt Dateien.

## Wichtige Korrekturen

- **MAC-Adressen machen die Übertragung nicht automatisch sicher.** Sie dienen der Adressierung innerhalb eines lokalen Netzsegments.
- **TCP garantiert keine fehlerfreie Gesamtanwendung.** Es gewährleistet zuverlässige, geordnete Byteübertragung zwischen TCP-Endpunkten, sofern die Verbindung erfolgreich bleibt.
- **UDP ist nicht automatisch schneller.** Es hat weniger Transportmechanismen, aber die tatsächliche Geschwindigkeit hängt vom Anwendungsfall ab.
- **DNS, DHCP und HTTP gehören zur Anwendungsschicht**, auch wenn sie UDP oder TCP verwenden.
- **Sitzungen sind nicht ausschließlich Schicht 5.** Moderne Protokolle verteilen Sitzungsfunktionen oft auf mehrere Schichten.
- **JSON ist ein Datenformat, kein Netzwerkprotokoll.** PNG und JPEG sind Bildformate; UTF-8 ist eine Zeichencodierung.

## Merkhilfe

| Schicht | Leitfrage |
|---|---|
| 7 Anwendung | Welcher Netzwerkdienst wird benötigt? |
| 6 Darstellung | Wie sind die Daten codiert? |
| 5 Sitzung | Wie wird die Sitzung verwaltet? |
| 4 Transport | Welcher Prozess empfängt die Daten? |
| 3 Vermittlung | In welches Netzwerk muss das Paket? |
| 2 Sicherung | Welches Gerät im lokalen Netz ist das Ziel? |
| 1 Bitübertragung | Wie werden die Bits physisch übertragen? |

**Datenkapselung beim Senden:**

Anwendungsdaten → TCP-Segment/UDP-Datagramm → IP-Paket → Ethernet-Frame → Bits

**Wichtige Adressen:** MAC = Schicht 2, IP = Schicht 3, Port = Schicht 4.
