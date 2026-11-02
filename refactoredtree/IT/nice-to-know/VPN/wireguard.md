# WireGuard

## In 30 Sekunden

- **WireGuard** ist ein schlankes Open-Source-VPN (ca. 4.000 Zeilen Code), seit Linux-Kernel **5.6** im Kernel, verfügbar für alle gängigen Systeme.
- Transport **ausschließlich UDP**, Standardport **51820** (frei wählbar).
- Feste, moderne Kryptografie ohne Aushandlung: **Curve25519** (Schlüsselaustausch), **ChaCha20-Poly1305** (Verschlüsselung + Integrität), **BLAKE2s** (Hash).
- Authentifizierung über **statische Schlüsselpaare** je Peer – keine Zertifikate, keine CA, **kein eingebauter Widerruf**.
- Stärken: hoher Durchsatz, geringe Latenz, einfache Konfiguration, nahtloses Roaming; Schwächen: UDP-only, Schlüsselverwaltung bei vielen Nutzern.

## Kernwissen

### Funktionsweise

| Begriff | Bedeutung |
|---|---|
| Interface `wg0` | virtuelles Layer-3-Interface |
| `PrivateKey` / `PublicKey` | jeder Peer erzeugt ein Schlüsselpaar; Public Keys werden ausgetauscht |
| `AllowedIPs` | **Cryptokey Routing**: welche Ziel-IPs über welchen Peer laufen und von welchem Peer welche Quell-IPs akzeptiert werden |
| `Endpoint` | öffentliche IP:Port der Gegenstelle |
| `PersistentKeepalive` | hält NAT-Einträge offen (z. B. alle 25 s) |

```
[Interface]
PrivateKey = <privat>
Address    = 10.99.0.2/32
DNS        = 10.0.0.53

[Peer]
PublicKey  = <öffentlicher Schlüssel des Gateways>
Endpoint   = vpn.firma.de:51820
AllowedIPs = 10.0.0.0/16          # Split Tunnel; 0.0.0.0/0 = Full Tunnel
```

WireGuard ist **zustandsarm**: Es gibt keine „Verbindung“ im klassischen Sinn – ein Handshake erfolgt bei Bedarf und regelmäßig neu (Schlüsselrotation nach ca. 2 Minuten → Forward Secrecy). Wechselt der Client die IP (WLAN → Mobilfunk), läuft der Tunnel einfach weiter (**Roaming**).

### Vor- und Nachteile

| Vorteile | Nachteile |
|---|---|
| sehr hoher Durchsatz, geringe Latenz (Kernel) | **nur UDP** – scheitert an Netzen, die nur TCP 80/443 erlauben |
| kleiner, gut prüfbarer Code → geringe Angriffsfläche | **keine PKI**: Public Keys müssen manuell oder per Zusatzsoftware verteilt werden |
| einfache Konfiguration ohne Proposals | kein eingebauter **Widerruf**, keine Benutzer-/MFA-Anbindung |
| keine Antwort auf unauthentifizierte Pakete („unsichtbar“ für Scanner) | keine dynamische Adressvergabe eingebaut (feste IP je Peer) |
| nahtloses Roaming | für Compliance (Benutzerprotokoll) Zusatzwerkzeuge nötig |

### Einordnung (GA2 Sommer 2025)

Bei der Auswahl zwischen einer VPN-Software mit **X.509-Zertifikaten** und einer mit **manuell hinterlegten Public Keys** (wie WireGuard) spricht für viele Benutzer die Zertifikatslösung: zentrale CA, **Widerruf per CRL/OCSP**, automatisierbare Verteilung. Für wenige feste Standorte oder Admin-Zugänge ist WireGuard wegen Einfachheit und Leistung sehr gut geeignet; für große Nutzerzahlen kommen Management-Lösungen hinzu.

## Prüfungsfrage

**Frage:** Ein Unternehmen will 300 Außendienstmitarbeiter per VPN anbinden. Zur Wahl stehen WireGuard und eine IPsec-Lösung mit Zertifikaten aus der firmeneigenen CA. Nennen Sie je einen Vor- und Nachteil von WireGuard für dieses Szenario.

**Musterantwort:** Vorteil: WireGuard bietet einen sehr hohen Durchsatz bei geringer Latenz und unterstützt nahtloses Roaming zwischen WLAN und Mobilfunk – angenehm für mobile Mitarbeiter. Nachteil: WireGuard authentifiziert über statisch hinterlegte Public Keys ohne CA; bei 300 Benutzern müssen Schlüssel manuell verteilt und bei Verlust eines Geräts auf dem Gateway gelöscht werden – einen zentralen Widerruf wie bei Zertifikaten (CRL/OCSP) und eine Anbindung an AD/MFA gibt es nicht ohne Zusatzwerkzeuge. Außerdem funktioniert es nur über UDP und scheitert in Netzen, die nur TCP 443 erlauben.

## Falle

- „WireGuard nutzt TLS“ → nein, eigenes Protokoll auf Basis des Noise-Frameworks.
- TCP-Port angeben → WireGuard ist **nur UDP**.
- „Keine Zertifikate = unsicher“ → die Kryptografie ist stark; die Schwäche liegt in der **Schlüsselverwaltung**.
- `AllowedIPs` nur als Firewall verstehen → es steuert auch das **Routing** in den Tunnel.

## Merksatz

**„WireGuard: schnell, schlank, nur UDP – Schlüssel statt Zertifikate, also auch Schlüssel selbst verwalten.“**

## Verwandt

- [VPN – Überblick](readme.md) – Vergleich und Fehlerbilder
- [IPsec](IPsec.md) – zertifikatsbasierte Alternative
- [OpenVPN](openvpn.md) – TLS-basierte Alternative mit TCP-443-Option
- [Symmetrische und asymmetrische Verschlüsselung](../../FiSi-Administrator/Sicherheit/Verschluesselung/symmetrische-und-asymmetrische.md) – Schlüsselpaare
- [Ports](../../FiSi-Administrator/OSI-IP-mac-bit-byte/ports.md) – UDP 51820
- [Lektion 27 – PKI, TLS & VPN](../../analyse-der-exams/L27_PKI_TLS_Verschluesselung_VPN.md)
