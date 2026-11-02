# Netzwerkzugangskontrolle: 802.1X, RADIUS und Port-Security

## In 30 Sekunden

- **IEEE 802.1X** ist **portbasierte Zugangskontrolle**: Ein Gerät an einer Netzwerkdose oder im WLAN bekommt erst nach erfolgreicher Authentifizierung Zugang zum Netz.
- Drei Rollen: **Supplicant** (Client), **Authenticator** (Switch oder Access Point), **Authentication Server** (meist **RADIUS**, angebunden an AD/LDAP).
- Nach der Anmeldung kann der RADIUS-Server ein **VLAN dynamisch zuweisen** (Mitarbeiter-, Gast-, Quarantäne-VLAN) – Prüfung 2023S.
- **WPA2/WPA3-Enterprise** = 802.1X im WLAN: jeder Benutzer hat eigene Zugangsdaten oder ein eigenes Zertifikat statt eines gemeinsamen Pre-Shared Keys (2022S, 2023W).
- Einfachere Alternative am Switch: **Port-Security** (erlaubte MAC-Adressen je Port) – schwächer, weil MAC-Adressen fälschbar sind.

## Kernwissen

**Ablauf 802.1X**
1. Gerät wird angeschlossen; der Switch-Port lässt zunächst **nur EAP-Pakete** (EAPOL) durch.
2. Der Supplicant sendet seine Identität und Nachweise per **EAP** an den Authenticator.
3. Der Authenticator leitet sie per **RADIUS** an den Authentication Server weiter.
4. Der RADIUS-Server prüft gegen AD/LDAP oder das Zertifikat und antwortet mit **Access-Accept** (ggf. mit VLAN-Zuweisung) oder **Access-Reject**.
5. Der Port wird freigeschaltet bzw. bleibt gesperrt; Start und Ende der Sitzung werden protokolliert (**Accounting**).

**EAP-Methoden**

| Methode | Nachweis | Sicherheit |
|---|---|---|
| **EAP-TLS** | Zertifikate auf **Client und Server** (PKI nötig) | sehr hoch, kein Passwort |
| **PEAP / EAP-TTLS** | Server-Zertifikat + Benutzername/Passwort im TLS-Tunnel | gut, wenn Clients das Server-Zertifikat prüfen |
| **MAB** (MAC Authentication Bypass) | nur MAC-Adresse | schwach, Notlösung für Drucker/IoT ohne 802.1X-Unterstützung |

**RADIUS (AAA)**
- **Authentication** (wer?), **Authorization** (welches VLAN, welche Rechte?), **Accounting** (wann, wie lange?)
- Ports **UDP 1812** (Authentifizierung) und **UDP 1813** (Accounting)
- Beispiele: Microsoft NPS, FreeRADIUS, Cisco ISE

**RADIUS/802.1X vs. PSK (Prüfung 2022S, 2023W)**

| | Pre-Shared Key (WPA2-Personal) | 802.1X/RADIUS (Enterprise) |
|---|---|---|
| Zugangsdaten | ein gemeinsames Passwort für alle | individuell je Benutzer oder Gerät |
| Mitarbeiter scheidet aus | PSK auf allen Geräten ändern | nur dessen Konto/Zertifikat sperren |
| Nachvollziehbarkeit | nicht möglich, wer verbunden war | Accounting je Benutzer |
| VLAN-Zuweisung | fest je SSID | dynamisch nach Benutzer/Gruppe |
| Aufwand | gering | RADIUS-Server, ggf. PKI |

**Port-Security am Switch (Lektion 16)**
- nur bestimmte MAC-Adressen pro Port: **statisch** eingetragen oder **sticky** (erste gelernte MAC wird gespeichert)
- Verstoß: **shutdown** (Port deaktiviert), **restrict** (verwerfen + Log), **protect** (still verwerfen)
- Ergänzend: ungenutzte Ports deaktivieren oder in ein Parkplatz-VLAN legen, Switch-Management nur über SSH und eigenes Management-VLAN (2024W GA2: Switch-Zugriff absichern).

## Prüfungsfrage

**Frage:** In den Besprechungsräumen eines Unternehmens sind Netzwerkdosen frei zugänglich. Beschreiben Sie, wie 802.1X verhindert, dass fremde Geräte Zugriff auf das Firmennetz erhalten, und nennen Sie zwei Vorteile gegenüber einem gemeinsamen WLAN-Schlüssel.

**Musterantwort:** Mit 802.1X lässt der Switch-Port zunächst nur Authentifizierungspakete durch. Das angeschlossene Gerät muss sich mit Zertifikat oder Benutzerdaten anmelden; der Switch leitet die Anfrage an einen RADIUS-Server weiter, der gegen das Active Directory prüft. Nur bei Erfolg wird der Port freigeschaltet und dem Gerät das passende VLAN zugewiesen; unbekannte Geräte bleiben gesperrt oder landen im Gast-VLAN. Gegenüber einem gemeinsamen Schlüssel hat jeder Benutzer eigene Zugangsdaten, die einzeln gesperrt werden können, und alle Anmeldungen sind dem jeweiligen Benutzer zuordenbar protokolliert.

## Falle

- Rollen vertauschen: der **Switch/AP ist Authenticator**, nicht der Server.
- RADIUS-Ports mit LDAP (389) oder Kerberos (88) verwechseln – RADIUS nutzt **UDP 1812/1813**.
- Port-Security als gleichwertig zu 802.1X darstellen → MAC-Adressen lassen sich fälschen.
- Bei EAP-TLS vergessen, dass eine **PKI** für Client-Zertifikate nötig ist.

## Merksatz

**„Erst ausweisen, dann ins Netz – Client fragt, Switch vermittelt, RADIUS entscheidet.“**

## Verwandt

- [AAA und SIEM](aaa-siem.md) – Authentication, Authorization, Accounting
- [PKI und Zertifikate](Verschluesselung/pki.md) – EAP-TLS
- [LDAP](../Netzwerk-Administration/ldap.md) und [Active Directory](active-directory.md) – Benutzerquelle für RADIUS
- [WLAN](../Netzwerk-Administration/wlan.md) – WPA2/WPA3-Enterprise
- [VLAN](../Netzwerk-Administration/vlan.md) – dynamische VLAN-Zuweisung
- [Lektion 29 GA2-Praxisthemen (802.1X)](../../analyse-der-exams/L29_GA2_Sonstiges_Praxisthemen.md)
- [Lektion 16 Firewall und Port-Security](../../analyse-der-exams/L16_Firewall.md)
- [Lektion 09 WLAN](../../analyse-der-exams/09_WLAN.md)
