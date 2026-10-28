# RBAC

**AD-RBAC in 3 Zeilen Prüfungsantwort**

**Struktur:**
OU-Hierarchie (Abteilungs-OUs) + Sicherheitsgruppen (z.B. SG_Finance_Read, SG_IT_Admin) + GPOs zur Delegierung. Jede Abteilung bekommt eigene OU + zugehörige Groups. Domain Controller via FSMO-Rollen verteilen (1x PDC, 1x weitere DC für Replikation & Last).

**Rollen & Geräte:**
Client (Win10/11) → Switch (L2/L3) → Firewall/UTM (L3-7) → DC1 (PDC, DNS, DHCP) → DC2 (Backup DC, Global Catalog). Rollen: Domain Admins (voll), Account Operators (lokale Rechte), Server Operators (Dienste).

**DSGVO-Bezug (Art. 25, 5):**
Privilegieren Zugriff durch RBAC = „Privacy by Design“ (Art. 25). Dokumentation aller Rollen & Zugriffe = Auditspflicht (Art. 30). Trennung von „Need-to-know“ = Datenminimierung (Art. 5 Abs. 1 lit. c). Rollenwechsel/Löschung bei Kündigung = Löschkonzept einhalten.

**OSI-Mapping:**
L7: LDAP/Kerberos | L6: SSL/TLS (LDAPS) | L5: Session (Kerberos-TGT) | L4: TCP (389/636/3268) | L3: IP | L2: Ethernet/VLANs (Trunk zur DC-Infrastruktur).

---
**AD RBAC – ausführlicher 3-Zeilen-Check**

1. **Struktur:** RBAC = Rollen, keine User-Rechte. AD baut Rollen über OU-Hierarchie + Sicherheitsgruppen.
   Beispiel: `OU_Finance` → `SG_Finance_Read`, `SG_Finance_Admin`. User werden in SGs aufgenommen. Admins in Admin-SG. GPO/ACL steuert Zugriff.

2. **Plan/DC:** 3 Abteilungen → 3 OUs. Jede OU eigene Gruppen. 2 DCs. FSMO/Rollen: PDC, Schema, Domain Naming, RID, Infrastructure. Global Catalog. Replikation. Rollen: Domain Admins, Account Operators, Server Operators.

3. **Exam/Mapping:** OSI: L7 LDAP/Kerberos, L6 TLS, L5 Session, L4 TCP 389/636/3268, L3 IP, L2 Switch/VLAN.
   DSGVO: Art. 25 Privacy by Design, Art. 5 Datenminimierung, Art. 32 Vertraulichkeit. Ports: LDAP 389, LDAPS 636, Kerberos 88, SMB 445.


RBAC trennt Lese/Schreib/Admin. OU je Abteilung. SGs. GPO. Gruppenüberschneidungen vermeiden. DSGVO via Need-to-know + TLS. OSI sauber mappen.

**Pros/Cons:**
Pros: zentral, auditierbar, DSGVO-konform.
Cons: Admin of Admin, fehlende PAM, Gruppenüberschneidungen.

**3 Themen, die du ergänzen solltest:**
1. Kerberos Delegation – unconstrained/constrained/resource-based.
2. AdminSDHolder + Privileged Access Management (PAM) / LAPS.
3. ADFS/SSO/SAML/OAuth.

