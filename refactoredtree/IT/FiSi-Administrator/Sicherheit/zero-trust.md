# Zero Trust

## In 30 Sekunden

- **Zero Trust** heißt: **„Never trust, always verify“** – kein Vertrauen allein deshalb, weil ein Gerät im internen Netz steht.
- Jeder Zugriff wird **einzeln** geprüft: Identität (MFA), Gerätezustand, Kontext (Ort, Zeit, Risiko) – und nur auf die konkret benötigte Ressource erlaubt.
- Drei Grundsätze: **explizit verifizieren**, **minimale Rechte** (Least Privilege), **von einem Einbruch ausgehen** (Assume Breach).
- Technik: MFA überall, **Mikrosegmentierung**, ZTNA statt pauschalem VPN, verschlüsselte Verbindungen auch intern, laufendes Monitoring.
- Prüfung 2025S: **Least Privilege vs. Zero Trust** unterscheiden und je eine Maßnahme nennen.

## Kernwissen

**Klassisches Perimeter-Modell vs. Zero Trust**

| | Perimeter („Burg und Graben“) | Zero Trust |
|---|---|---|
| Vertrauensbasis | Standort: innen = vertrauenswürdig | Identität + Gerät + Kontext, bei **jedem** Zugriff |
| Schutzschicht | eine starke Grenze (Firewall) | viele kleine Grenzen um jede Ressource |
| Folge eines Einbruchs | Angreifer bewegt sich frei im LAN (Lateral Movement) | Schaden bleibt auf ein Segment/eine Ressource begrenzt |
| Fernzugriff | VPN ins ganze Netz | ZTNA: Zugriff nur auf freigegebene Anwendungen |

**Bausteine**

| Baustein | Beispiel |
|---|---|
| Starke Identität | MFA für alle Konten, zentrales Identity Management (AD/Entra ID), SSO |
| Gerätezustand (Device Posture) | nur verwaltete, gepatchte, verschlüsselte Geräte (MDM-Compliance) |
| Richtlinien je Zugriff | Conditional Access: z. B. Zugriff aus dem Ausland nur mit Hardware-Token |
| Mikrosegmentierung | Server-, Client-, IoT-, Management-Netze getrennt; Regeln zwischen einzelnen Workloads |
| Least Privilege / Just in Time | Admin-Rechte nur zeitlich begrenzt und auf Antrag |
| Verschlüsselung | TLS auch für interne Dienste |
| Überwachung | zentrale Protokollierung, SIEM, Anomalieerkennung |

Architektur nach NIST SP 800-207: ein **Policy Decision Point** entscheidet über den Zugriff, ein **Policy Enforcement Point** (z. B. Gateway, Proxy) setzt die Entscheidung durch.

**Least Privilege vs. Zero Trust**
- **Least Privilege** begrenzt den **Umfang** der Rechte: Wer darf was? Nur das Nötigste.
- **Zero Trust** hinterfragt die **Legitimität jedes einzelnen Zugriffs**: Ist das wirklich dieser Benutzer, auf einem vertrauenswürdigen Gerät, in plausiblem Kontext?
- Least Privilege ist ein **Bestandteil** von Zero Trust.

## Prüfungsfrage

**Frage:** Erläutern Sie das Least-Privilege-Prinzip und das Zero-Trust-Prinzip und nennen Sie zu jedem eine konkrete Umsetzungsmaßnahme.

**Musterantwort:** Nach dem Least-Privilege-Prinzip erhalten Benutzer und Prozesse nur die Rechte, die sie für ihre Aufgabe zwingend benötigen; umgesetzt wird das zum Beispiel über ein rollenbasiertes Berechtigungskonzept und getrennte Admin-Konten, die nur für Verwaltungsaufgaben genutzt werden. Nach dem Zero-Trust-Prinzip wird keinem Gerät oder Benutzer automatisch vertraut, auch nicht im internen Netz; jeder Zugriff wird anhand von Identität, Gerätezustand und Kontext neu geprüft. Eine Maßnahme ist die verpflichtende Multi-Faktor-Authentifizierung für alle Zugriffe in Kombination mit Mikrosegmentierung des Netzes.

## Falle

- Zero Trust als Produkt bezeichnen, das man „kauft“ → es ist ein **Sicherheitsmodell**, das schrittweise umgesetzt wird.
- „Zero Trust = keine Firewall mehr“ → falsch, Firewalls werden feiner eingesetzt (Segmentierung).
- Least Privilege und Zero Trust gleichsetzen – das eine betrifft den Rechteumfang, das andere die Prüfung jedes Zugriffs.

## Merksatz

**„Nicht der Ort entscheidet, sondern die geprüfte Identität – bei jedem einzelnen Zugriff.“**

## Verwandt

- [Least Privilege](least-privilege.md)
- [MFA](MFA.md)
- [DMZ und Sicherheitszonen](firewall/dmz-sicherheitszonen.md) – Segmentierung klassisch
- [RBAC](../Netzwerk-Administration/rbac.md)
- [MDM und BYOD](../Netzwerk-Administration/mdm-byod.md) – Gerätezustand prüfen
- [VPN](../../nice-to-know/VPN/readme.md)
- [Lektion 28 GA1-Praxisthemen (Least Privilege und Zero Trust)](../../analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md)
