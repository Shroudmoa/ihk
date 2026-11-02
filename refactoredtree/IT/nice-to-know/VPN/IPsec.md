# IPsec (Internet Protocol Security)

## In 30 Sekunden

- **IPsec** ist eine IETF-Protokollfamilie, die IP-Pakete auf **Schicht 3** verschlüsselt und authentifiziert – Standard für **Site-to-Site-VPNs** zwischen Firewalls/Routern.
- Zwei Modi: **Transport Mode** (nur Nutzdaten geschützt, Original-IP-Header bleibt; Host ↔ Host) und **Tunnel Mode** (gesamtes Originalpaket wird verpackt; Gateway ↔ Gateway).
- Zwei Protokolle: **ESP** (Verschlüsselung + Integrität, IP-Protokoll **50**) und **AH** (nur Integrität/Authentizität, IP-Protokoll 51, nicht NAT-fähig).
- Schlüsselaustausch per **IKEv2** über **UDP 500**; hinter NAT wird ESP per **NAT-Traversal in UDP 4500** gekapselt (GA2 Sommer 2023).
- Authentifizierung per **Pre-Shared Key** oder **X.509-Zertifikaten** – Zertifikate sind skalierbar und widerrufbar.

## Kernwissen

### Modi

| | Transport Mode | Tunnel Mode |
|---|---|---|
| Geschützt | Nutzdaten (Payload) | **komplettes Original-IP-Paket** inkl. Header |
| Neuer IP-Header | nein, Original bleibt sichtbar | ja (Gateway-Adressen außen) |
| Einsatz | Host ↔ Host (End-to-End), L2TP/IPsec | **Site-to-Site**, Remote Access |
| Sichtbar von außen | echte Quell-/Ziel-IP | nur die öffentlichen IPs der Gateways |

```
Tunnel Mode:  [neuer IP-Header][ESP-Header][ Original-IP-Header | TCP | Daten ][ESP-Trailer][ICV]
                                            |<------------ verschlüsselt ------------>|
```

### ESP vs. AH

| | ESP | AH |
|---|---|---|
| Vertraulichkeit (Verschlüsselung) | **ja** | nein |
| Integrität / Authentizität | ja | ja (inkl. äußerer IP-Header) |
| NAT-fähig | mit NAT-T | **nein** (NAT verändert den geschützten Header) |
| IP-Protokollnummer | 50 | 51 |
| Praxis | Standard | kaum noch genutzt |

### IKEv2 – Ablauf

1. **IKE_SA_INIT** (Phase 1): Algorithmen aushandeln, **Diffie-Hellman**-Schlüsselaustausch → verschlüsselter Steuerkanal (IKE-SA).
2. **IKE_AUTH**: Gegenstellen authentifizieren sich (PSK oder Zertifikat), erste **Child-SA** (Phase 2) für den Nutzverkehr wird erstellt.
3. Regelmäßiges **Rekeying**; mit **PFS** (Perfect Forward Secrecy) neuer DH-Austausch je Schlüssel → ein später erbeuteter Schlüssel entschlüsselt keinen alten Verkehr.

Beide Seiten müssen identische **Proposals** haben (z. B. AES-256-GCM, SHA-256, DH-Gruppe 19/20/21, Lebensdauern) – Abweichungen sind die häufigste Ursache, wenn ein Tunnel nicht hochkommt.

### NAT-Traversal (GA2 Sommer 2023)

ESP hat **keine Portnummern**. Ein PAT-Router (z. B. im Hotel-WLAN) kann ESP-Pakete daher keiner internen Verbindung zuordnen. **NAT-T** erkennt NAT während IKE und kapselt ESP zusätzlich in **UDP 4500** → PAT funktioniert. Firewallregeln für IPsec: **UDP 500, UDP 4500** und ggf. **ESP (Protokoll 50)**.

### PSK vs. Zertifikate

| | Pre-Shared Key | X.509-Zertifikate |
|---|---|---|
| Einrichtung | einfach | PKI/CA nötig |
| Skalierung | ein Geheimnis pro Tunnel/Gruppe | jede Gegenstelle mit eigener Identität |
| Kompromittierung | Schlüssel überall tauschen | einzelnes Zertifikat **widerrufen** (CRL/OCSP) |
| Bewertung | Test, kleine Umgebungen (starker, langer PSK!) | Unternehmen |

### Vor- und Nachteile

- **Vorteile:** offener Standard, herstellerübergreifend, in nahezu jeder Firewall/jedem Router integriert, oft hardwarebeschleunigt, ausgereift, PFS.
- **Nachteile:** komplexe Konfiguration (Proposals, Lebensdauern, Traffic-Selektoren), Probleme mit NAT und restriktiven Hotspots (nur UDP), Fehlersuche aufwendig.

## Prüfungsfrage

**Frage (angelehnt an GA2 Sommer 2023):** Ein Außendienstmitarbeiter kann im Hotel-WLAN keine IPsec-Verbindung (ESP) zur Zentrale aufbauen. Erläutern Sie eine mögliche Ursache im Zusammenhang mit NAT und eine Lösung.

**Musterantwort:** Das Hotel-WLAN nutzt NAT/PAT: Viele Gäste teilen sich eine öffentliche IP, unterschieden werden sie über Portnummern. ESP besitzt jedoch keine Ports, sodass der NAT-Router die ESP-Pakete keinem Gast zuordnen kann und sie verwirft; AH würde zusätzlich an der Änderung des IP-Headers scheitern. Lösung: **NAT-Traversal** auf Client und VPN-Gateway aktivieren – ESP wird dann in UDP 4500 gekapselt und kann übersetzt werden. Sind auch UDP 500/4500 gesperrt, hilft nur ein VPN über TCP 443 (z. B. SSL-VPN/OpenVPN).

## Falle

- Transport und Tunnel vertauschen → **Tunnel = ganzes Paket = Site-to-Site**.
- AH als Verschlüsselung bezeichnen → AH **verschlüsselt nicht**.
- ESP „Port 50“ → **IP-Protokoll 50**, kein Port.
- PSK als gleichwertig zu Zertifikaten darstellen → kein individueller Widerruf.
- IPsec auf Schicht 7 einordnen → IPsec arbeitet auf **Schicht 3**.

## Merksatz

**„Tunnel packt alles ein, Transport nur den Inhalt; ESP verschlüsselt, AH nur unterschreibt; 500 verhandelt, 4500 rettet hinter NAT.“**

## Verwandt

- [VPN – Überblick](readme.md) – Topologien, Protokollvergleich, Fehlerbilder
- [OpenVPN](openvpn.md) – Alternative über TLS
- [WireGuard](wireguard.md) – schlanke Alternative
- [PKI](../../FiSi-Administrator/Sicherheit/Verschluesselung/pki.md) – Zertifikate und Widerruf
- [NAT & PAT](../../FiSi-Administrator/Sicherheit/firewall/nat-pat.md) – Portübersetzung
- [Ports](../../FiSi-Administrator/OSI-IP-mac-bit-byte/ports.md) – 500, 4500, Protokoll 50/51
- [Lektion 27 – PKI, TLS & VPN](../../analyse-der-exams/L27_PKI_TLS_Verschluesselung_VPN.md)
