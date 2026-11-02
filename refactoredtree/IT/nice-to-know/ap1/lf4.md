# Klausurvorbereitung LF4 – Kann-Liste

## Informationssicherheit, IT-Sicherheit und gesetzliche Anforderungen

### IT-Sicherheit vs. Informationssicherheit
*   **IT-Sicherheit:** Schutz der IT-Infrastruktur (Hardware/Software) vor technischen Bedrohungen. Fokus auf Vertraulichkeit, Integrität, Verfügbarkeit von IT-Systemen.
*   **Informationssicherheit:** Ganzheitlicher Ansatz. Schützt **alle** Informationen (auch physisch: Papierarchive, Kopierräume) vor einer breiten Palette von Bedrohungen.
*   **TOM (Technisch-Organisatorische Maßnahmen):** Durch DSGVO vorgeschriebene Maßnahmen zur Sicherheit personenbezogener Daten.

### BSI (Bundesamt für Sicherheit in der Informationstechnik)
*   Zentrale Behörde in Deutschland für Informationssicherheit.
*   **Aufgaben:** Standards/Richtlinien entwickeln, Zertifizierung/Akkreditierung, Beratung, Frühwarnsystem/Incident Response, Sensibilisierung/Schulung.

### Gesetzliche Anforderungen
*   **Telemedien-Dienstanbieter (TKG):** Stand der Technik, Nutzer über Schadprogramme informieren, erhebliche Störungen an BSI und Bundesnetzagentur melden.
*   **Kritische Infrastruktur (BSIG):** IT-Systeme nach Stand der Technik absichern, Informationssicherheit mind. alle 2 Jahre prüfen lassen, erhebliche Störungen an BSI melden.
*   **Anbieter digitaler Dienste:** Geeignete technische/organisatorische Maßnahmen treffen, um Risiken zu vermeiden/minimieren.

### Folgen von Informationsverlust
*   Vertrauensverlust bei Kunden.
*   Betriebsstörungen (Prozesse laufen nicht).
*   Wirtschaftliche Schäden (Wettbewerbsnachteile, Kosten).
*   Rechtliche Konsequenzen (Bußgelder, Strafverfahren).
*   Verlust geistigen Eigentums.

### Verschlüsselung
*   **Symmetrisch:** Ein gemeinsamer Schlüssel für Ver- und Entschlüsselung. (Schnell, aber Schlüsselaustausch problematisch).
*   **Asymmetrisch (Public-Key):** Zwei Schlüssel (Public zum Verschlüsseln, Private zum Entschlüsseln). Löst Schlüsselaustauschproblem, ermöglicht digitale Signaturen.
*   **Hybrid:** Kombination. Asymmetrisch für sicheren Schlüsselaustausch, symmetrisch für die eigentliche Datenübertragung.

### Digitale Signatur
*   Gewährleistet **Integrität, Authentizität, Nichtabstreitbarkeit**.
*   Stellt sicher, dass Nachricht unverändert ist und vom angegebenen Absender stammt.

### Schutzziele der Informationssicherheit (CIA-Triade)
1.  **Vertraulichkeit:** Nur Berechtigte haben Zugriff (z.B. Verschlüsselung, Zugriffskontrolle).
2.  **Integrität:** Daten sind echt, unverändert und vollständig (z.B. Signaturen, Prüfsummen).
3.  **Verfügbarkeit:** Daten/Systeme sind für Berechtigte zum richtigen Zeitpunkt nutzbar (z.B. Redundanz, USV, Backups).

### Geistiges Eigentum
*   **Urheberrecht:** Entsteht automatisch, schützt kreative Werke (Literatur, Software). Schützt die konkrete Ausdrucksform, nicht die Idee. Voraussetzungen: Originalität, feste Form, persönlicher Beitrag.
*   **Markenrecht:** Schützt Namen, Logos, Kennzeichen (Registrierung nötig).
*   **Patentrecht:** Schützt Erfindungen/technische Innovationen (Registrierung nötig).
*   **Lizenzrecht:** Einräumung von Nutzungsrechten durch den Inhaber an Dritte.
*   *Registerrechte:* Marken-, Patent-, Designrecht (müssen registriert werden).
*   *Urheberrecht:* Kein Registerrecht (entsteht automatisch).

## Bedrohungen und Angriffe

### Angriffsarten
*   **Passiv:** Abhören/Abfangen von Daten (z.B. Packet Sniffing). System bleibt funktionsfähig. Ziel: Vertrauliche Infos.
*   **Aktiv:** Direkter Eingriff ins System (z.B. DoS, Manipulation). Ziel: Schaden, unautorisierter Zugriff.

### Schadsoftware
*   **Viren:** Hängen sich an Dateien, verbreiten sich bei Ausführung.
*   **Trojaner:** Tarnen sich als nützliche Software, öffnen Hintertüren.
*   **Ransomware:** Verschlüsselt Daten, fordert Lösegeld.
*   **Spyware/Keylogger:** Spähen Daten/Verhalten aus, zeichnen Tastatureingaben auf.
*   **Würmer:** Verbreiten sich selbstständig über Netzwerke (ohne Wirtsdatei).
*   **Adware:** Zeigt unerwünschte Werbung.
*   *Verbreitungswege:* E-Mail-Anhänge, Drive-by-Downloads, USB-Geräte, unsichere Software-Quellen, Social Engineering.

### Softwarebasierte Verwundbarkeit
*   Sicherheitslücken durch Programmierfehler, unsichere Implementierung. (z.B. XSS: Einschleusen von JavaScript in unsichere Eingabefelder).

## Schutzmaßnahmen

### Passwörter
*   Individuell pro Account, ausreichend lang, alle Zeichen (Groß/Klein, Ziffern, Sonderzeichen), keine Wörterbuchwörter, keine persönlichen Daten, keine Muster, regelmäßig ändern.

### Firewalls
*   **Personal Firewall:** Software auf einem PC. Überwacht/blockiert Datenverkehr (ein-/ausgehend).
*   **Externe Firewall (Netzwerkfirewall):** Hardware zwischen internem Netz und Internet. Analysiert Datenpakete, wendet Sicherheitsregeln an.

### Virenscanner
*   **Voraussetzungen zur Identifizierung:** Aktuelle Virendefinitionen/Signaturen, Heuristische Analyse (Verhaltensmuster), Echtzeitschutz, regelmäßige Updates, Systemscans.

### Schutzmaßnahmen des Systemadministrators
*   Zugriffskontrollen, Sicherheitsrichtlinien, Patches/Updates, Firewall, Antiviren-Software, Mitarbeiterschulungen, Sicherheitsaudits, Verschlüsselung, Netzwerküberwachung.

## Schutzbedarfsanalyse

### Schutzbedarfskategorien
*   **Normal:** Begrenzte, überschaubare Auswirkungen.
*   **Hoch:** Beträchtliche Auswirkungen.
*   **Sehr hoch:** Existenzielle, katastrophale Auswirkungen.

### Schadensszenarien
1.  **Verstoß gegen Gesetze/Vorschriften/Verträge:** (z.B. DSGVO-Verstoß).
2.  **Beeinträchtigung des informationellen Selbstbestimmungsrechts:** (z.B. Missbrauch personenbezogener Daten).
3.  **Beeinträchtigung der persönlichen Unversehrtheit:** (z.B. Fehlfunktion medizinischer Geräte).
4.  **Beeinträchtigung der Aufgabenerfüllung:** (z.B. Systemausfall verhindert Transaktionen).
5.  **Negative Innen-/Außenwirkung:** (z.B. Imageverlust, Vertrauensverlust).
6.  **Finanzielle Auswirkungen:** (z.B. Umsatzverluste, Wiederherstellungskosten, Diebstahl).

### Warum Schutzbedarfsanalyse?
*   Schutz von Systemen/Daten vor Bedrohungen.
*   Identifikation von Schwachstellen.
*   Frühzeitiges Erkennen von Risiken.
*   Gewährleistung von Vertraulichkeit, Integrität, Verfügbarkeit.
*   Erfüllung rechtlicher Vorschriften.
