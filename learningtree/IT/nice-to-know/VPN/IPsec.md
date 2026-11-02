# IPsec (Internet Protocol Security)

## Kurzprofil
*   **Typ:** Standard-Suite (IETF), oft in Hardware-Routern/Firewalls integriert.
*   **Ports:** **UDP 500** (IKE) und **UDP 4500** (NAT-T) [citation:3].
*   **Transport:** IP-basiert (Layer 3).

## Die zwei Modi (IHK-Kernwissen)
IPsec kennt zwei Betriebsmodi, die man auseinanderhalten muss:
1.  **Transport Mode:** Nur die Nutzdaten (Payload) werden verschlüsselt. Der originale IP-Header bleibt sichtbar. Wird für **Host-zu-Host** (End-to-End) Verbindungen genutzt [citation:3][citation:13].
2.  **Tunnel Mode:** Das **gesamte ursprüngliche IP-Paket** (inkl. Header) wird verschlüsselt und in ein neues IP-Paket verpackt. Wird für **Site-to-Site** (Gateway-zu-Gateway) und Remote Access genutzt [citation:3][citation:13].

## Schlüsselaustausch: IKEv2
IPsec nutzt **IKEv2 (Internet Key Exchange)**, um die Sicherheitsparameter (Algorithmen, Schlüssel) dynamisch auszuhandeln. Es können Zertifikate (PKI) oder Pre-Shared Keys (PSK) genutzt werden. PSK ist unsicherer und sollte nur im Test genutzt werden [citation:8].

## Vor- und Nachteile (IHK-Relevant)
*   **Vorteile:**
    *   **Standard:** In fast allen Enterprise-Firewalls und Routern nativ unterstützt.
    *   **Performance:** Sehr gut, oft Hardware-beschleunigt (ASIC) .
    *   **Sicherheit:** Ausgereift, bietet Perfect Forward Secrecy (PFS).
*   **Nachteile:**
    *   **Komplexität:** Die Konfiguration (Phase 1/Phase 2, IKEv2, Proposals) ist sehr komplex und fehleranfällig.
    *   **NAT-Probleme:** Benötigt NAT-Traversal (Port 4500), was manchmal Probleme macht.

## Prüfungs-Merksatz
> „IPsec ist der Enterprise-Standard. Im Tunnel-Mode versteckt es das komplette Original-Paket (Site-to-Site), im Transport-Mode nur die Nutzdaten (Host-to-Host). Es ist komplex, aber weit verbreitet und performant.“
