# VPN (Virtual Private Network)

## Was ist ein VPN?
Ein VPN ist ein logisches, privates Netzwerk, das über ein öffentliches Netzwerk (z.B. das Internet) aufgebaut wird. Es ermöglicht eine sichere, verschlüsselte Kommunikation zwischen entfernten Standorten oder einzelnen Clients.

## Die 3 VPN-Topologien (IHK-Basiswissen)
*   **Site-to-Site (Standortkopplung):** Verbindet zwei oder mehr lokale Firmennetze (z.B. Filiale und Zentrale) fest über das Internet. Ersetzt teure Standleitungen [citation:10][citation:15].
*   **End-to-Site (Remote Access):** Ein einzelner Client (z.B. Home-Office-Mitarbeiter) baut eine Verbindung zu einem VPN-Gateway im Firmennetz auf. Der Client erhält Zugriff auf das gesamte LAN [citation:5][citation:15].
*   **End-to-End:** Zwei einzelne Clients (z.B. zwei PCs) verbinden sich direkt über einen Tunnel, ohne dass ein ganzes Firmennetz involviert ist [citation:10][citation:15].

## Die Protokolle in diesem Ordner
*   **OpenVPN:** Der flexible Klassiker (SSL/TLS-basiert).
*   **WireGuard:** Der moderne, schnelle Standard (Kernel-basiert).
*   **IPsec:** Der Standard für Site-to-Site und Enterprise-Umgebungen (IKEv2).
