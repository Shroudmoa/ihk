# Access Control (Zugriffskontrolle)

**Definition:** Der Prozess, der festlegt und durchsetzt, **wer** (Identität) **worauf** (Ressource) **wie** (Berechtigung) zugreifen darf.

## 1. Die 3 Schritte der Zugriffskontrolle (AAA-Modell)
Jeder Zugriff durchläuft diese drei Phasen:
1.  **Authentifizierung (Wer bist du?):** Identitätsprüfung (z.B. Passwort, Fingerabdruck, Smartcard).
2.  **Autorisierung (Was darfst du?):** Zuweisung von Rechten basierend auf der Identität (z.B. Rolle, Gruppe).
3.  **Accounting / Auditing (Was hast du getan?):** Protokollierung der Zugriffe für die Nachvollziehbarkeit.

## 2. Bekannte Begriffe & Modelle (IHK-Relevant)

### CAL (Client Access License)
*   **Bedeutung:** Eine Lizenzgebühr, die Microsoft (und andere Hersteller) für den Zugriff auf Server-Dienste verlangt.
*   **Unterscheidung:** Es gibt **User CALs** (pro Nutzer, egal von wie vielen Geräten er zugreift) und **Device CALs** (pro Gerät, egal wie viele Nutzer darauf arbeiten). 
*   **Prüfungsrelevanz:** Bei der Planung von Server-Lizenzen muss berechnet werden, ob User- oder Device-CALs günstiger sind (z.B. Schichtbetrieb = Device CALs).

### RBAC (Role-Based Access Control)
*   **Bedeutung:** Rollenbasiertes Zugriffskontrollmodell.
*   **Funktion:** Rechte werden nicht einzelnen Personen zugewiesen, sondern **Rollen** (z.B. "Buchhalter", "Admin"). Nutzer werden diesen Rollen zugeordnet und erben automatisch die Rechte. Das ist der Standard in Unternehmen (z.B. Active Directory).

### ACL (Access Control List)
*   **Bedeutung:** Zugriffskontrollliste.
*   **Funktion:** Eine Liste, die an einem Objekt (Datei, Ordner, Router-Interface) hängt. Sie enthält Einträge (Regeln), wer was darf (z.B. "User Max: Read, Write", "User Gast: Deny").
*   **Praxis:** Wird bei Windows-NTFS-Rechten und Firewall-Regeln genutzt.

### MAC (Mandatory Access Control)
*   **Bedeutung:** Zwingende Zugriffskontrolle.
*   **Funktion:** Der Systemadministrator legt fest, wer was darf (z.B. Geheimhaltungsstufen: Öffentlich, Intern, Geheim). Der Nutzer kann diese Rechte **nicht** weitergeben. Sehr strikt (z.B. bei Linux SELinux).

### DAC (Discretionary Access Control)
*   **Bedeutung:** Diskrete / Eigenverantwortliche Zugriffskontrolle.
*   **Funktion:** Der **Besitzer** einer Datei entscheidet, wer Zugriff bekommt. Er kann Rechte weitergeben. Standard bei Windows/Linux-Systemen.

## 3. Weitere wichtige Fachbegriffe
*   **Least Privilege (Minimalprinzip):** Ein Nutzer bekommt nur exakt die Rechte, die er für seine Arbeit zwingend benötigt. Nichts mehr.
*   **Need-to-know-Prinzip:** Ein Nutzer erhält nur Zugriff auf Informationen, die er für seine konkrete Aufgabe wissen muss.
*   **MFA (Multi-Faktor-Authentifizierung):** Zwei oder mehr Faktoren zur Anmeldung (z.B. Passwort + SMS-Code). Schützt vor Passwortdiebstahl.
*   **SSO (Single Sign-On):** Einmal anmelden, für alle Dienste (z.B. Windows-Login gibt automatisch Zugriff auf E-Mail und Intranet).
*   **Vier-Augen-Prinzip:** Eine kritische Aktion (z.B. Überweisung, Server-Löschung) erfordert die Bestätigung von zwei Personen.
