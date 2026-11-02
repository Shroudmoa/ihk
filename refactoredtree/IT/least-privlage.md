# least_privlage

**Least-Privilege-Prinzip – 3-zeiliger IHK-Check**

1. **Definition:** Gewähren Sie jedem User/Minimalaccount genau die Rechte, um die benötigten Aufgaben auszuführen. Keine überflüssigen Berechtigungen → kein unnötiges Risiko.

2. **AD-Umsetzung:**
   – Use groups für Berechtigungen (SG_ReadFin, SG_AdminFin).
   – ACLs auf Objekte (File-System, AD-Objekte).
   – Token Rights: USER_RIGHTS, SeTcbPrivilege (teilweise eingeschränkt).
   – UAC-Prompt, Admin Approval über consent, migrieren GPOs mit „Require Admin Approval“.

3. **Pro/Con + DSGVO + OSI:**
   Pros: geringe Blast-Radius, DSGVO Art.5 Minimi. („Datenminimierung“) & Art.32 („Sicherheit“).
   Cons: Überverwaltung, Berechtigungsvererbung, Aufwand für Audit.
   OSI: L7 LDAP/Kerberos → Auth, L6 TLS (LDAPS) → Datensicherung, L4 TCP 389/636/88 → über die Wire, L5 Session → RBAC-Rollen. DSGVO: Art.25 Privacy by Design, Art.32 Verschlüsselung, Art.5 Minimierung.

**3 verwandte Themen:**
1. Privileged Access Management (PAM/LAPS) – gesperrte, zeitlich begrenzte Admin-Rechte.
2. Just-In-Time (JIT) Access – dynamische Elevation mit Check-In/Check-Out.
3. Application Whitelisting (AppLocker) – erzwingt nur erlaubte Binaries, begrenzt Ausbreitung.
