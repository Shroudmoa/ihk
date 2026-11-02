# WireGuard

## Kurzprofil
*   **Typ:** Open Source, im Linux-Kernel integriert (ab Kernel 5.6).
*   **Port:** Standard **UDP** (dynamisch, oft 51820).
*   **Transport:** Ausschließlich **UDP** (kein TCP) [citation:12].

## Technische Funktionsweise
WireGuard ist bewusst minimalistisch gehalten (ca. 4.000 Zeilen Code vs. 400.000 bei OpenVPN). Es nutzt moderne Kryptographie (Noise Protocol Framework):
*   **Curve25519** (Schlüsselaustausch)
*   **ChaCha20** (Verschlüsselung)
*   **Poly1305** (Authentifizierung)
*   **BLAKE2s** (Hashing) [citation:7][citation:12].

## Vor- und Nachteile (IHK-Relevant)
*   **Vorteile:**
    *   **Performance:** Höchster Durchsatz und niedrigste Latenz im Vergleich (Kernel-Space) [citation:11].
    *   **Einfachheit:** Keine komplexe PKI nötig, nur ein Schlüsselpaar pro Peer (Public/Private Key).
    *   **Roaming:** Wechselt nahtlos zwischen Netzwerken (z.B. WLAN zu Mobilfunk) ohne Neuverbindung.
*   **Nachteile:**
    *   **UDP-Zwang:** Probleme mit restriktiven Firewalls, die nur TCP erlauben.
    *   **Kein DPI-Schutz:** Nicht so einfach als HTTPS tarntbar wie OpenVPN.

## Prüfungs-Merksatz
> „WireGuard ist der moderne Performance-Sieger. Durch die Kernel-Integration und moderne Kryptographie ist es extrem schnell und einfach zu konfigurieren, scheitert aber oft an Firewalls, die kein UDP zulassen.“
