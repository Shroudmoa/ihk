# OpenVPN

## Kurzprofil
*   **Typ:** Open Source, SSL/TLS-basiert.
*   **Port:** Standard **UDP 1194** (auch TCP möglich) [citation:4].
*   **Transport:** Kann über UDP oder TCP laufen (TCP/443 ist ideal zur Umgehung von Firewalls/DPI) [citation:11].

## Technische Funktionsweise
OpenVPN nutzt die **TLS/SSL-Bibliothek (OpenSSL)** für den Schlüsselaustausch und die Authentifizierung. Es baut zwei Kanäle auf:
1.  **Control Channel:** Hier wird der TLS-Handshake durchgeführt (Zertifikate, Schlüssel).
2.  **Data Channel:** Hier fließen die eigentlichen Nutzdaten, verschlüsselt mit symmetrischen Algorithmen (z.B. AES-256) .

## Vor- und Nachteile (IHK-Relevant)
*   **Vorteile:**
    *   **Hohe Flexibilität:** Läuft auf fast jedem OS und jeder Hardware.
    *   **Firewall-Traversal:** Durch TCP/443 kann es als normaler HTTPS-Verkehr getarnt werden (Deep Packet Inspection Evasion) [citation:11].
    *   **Reife:** Sehr gut dokumentiert und weit verbreitet.
*   **Nachteile:**
    *   **Performance:** Höchster Overhead und niedrigste Durchsatzrate im Vergleich zu WireGuard und IPsec (User-Space-Implementierung) [citation:11].
    *   **Komplexität:** Die Konfiguration (Zertifikate, PKI) ist komplexer als bei WireGuard.

## Prüfungs-Merksatz
> „OpenVPN setzt auf TLS/SSL, ist extrem flexibel (UDP/TCP) und kann Firewalls durch Port 443 umgehen, hat aber den höchsten Overhead und die geringste Performance der drei großen Protokolle.“
