# E-Mail-Authentifizierung: SPF, DKIM & DMARC

Diese drei Technologien bilden den Standard für die Absicherung von E-Mail-Domänen gegen Spoofing und Phishing. Sie bauen aufeinander auf.

## 1. SPF (Sender Policy Framework)
*   **Zweck:** Verhindert, dass Unbefugte E-Mails im Namen einer Domain versenden (Identitätsdiebstahl).
*   **Funktion:** Der Domain-Inhaber veröffentlicht im DNS einen **SPF-Record** (TXT-Record). Dieser listet alle IP-Adressen der Server auf, die berechtigt sind, E-Mails für diese Domain zu versenden [citation:2][citation:5].
*   **Prüfung:** Der empfangende Mailserver prüft die IP des sendenden Servers gegen den SPF-Record der Absender-Domain. Stimmt sie nicht überein, schlägt die Prüfung fehl [citation:2].
*   **Schwäche:** SPF prüft nur die "Envelope From" (technische Absenderadresse), nicht die für den Nutzer sichtbare "Header From"-Adresse. Zudem brechen Weiterleitungen die Prüfung oft.

## 2. DKIM (DomainKeys Identified Mail)
*   **Zweck:** Stellt sicher, dass die E-Mail auf dem Transportweg nicht verändert wurde (Integrität) und wirklich von der angegebenen Domain stammt.
*   **Funktion:** Der sendende Server fügt der E-Mail eine **digitale Signatur** (Private Key) hinzu. Der zugehörige **Public Key** wird im DNS der Domain veröffentlicht [citation:4][citation:12].
*   **Prüfung:** Der Empfänger verifiziert die Signatur mit dem Public Key aus dem DNS. Ist die Signatur gültig, ist die Mail unverändert und authentisch.
*   **Vorteil:** Übersteht im Gegensatz zu SPF meist Weiterleitungen unbeschadet.

## 3. DMARC (Domain-based Message Authentication, Reporting and Conformance)
*   **Zweck:** DMARC ist kein eigener Authentifizierungsmechanismus, sondern eine **Policy (Richtlinie)**. Es sagt dem Empfänger, was er tun soll, wenn SPF und DKIM fehlschlagen oder nicht zur sichtbaren Absenderadresse passen [citation:1][citation:12].
*   **Alignment (Ausrichtung):** DMARC erzwingt, dass die Domain in der sichtbaren "Header From"-Adresse mit der Domain übereinstimmt, die durch SPF oder DKIM authentifiziert wurde. Dies schließt die Lücke, die SPF und DKIM einzeln offen lassen [citation:1][citation:12].
*   **Policy (p=):** Der Domain-Inhaber legt im DMARC-Record fest, wie mit Mails umgegangen wird, die die Prüfung nicht bestehen:
    *   `p=none`: Keine Aktion (nur Monitoring/Reporting).
    *   `p=quarantine`: Mail in Quarantäne/Spam verschieben.
    *   `p=reject`: Mail komplett ablehnen [citation:8][citation:14].
*   **Reporting:** DMARC ermöglicht es dem Domain-Inhaber, Berichte (Aggregate Reports) über die Authentifizierungsergebnisse seiner Domain zu erhalten.

## Zusammenspiel (IHK-Merksatz)
SPF prüft den **Server** (IP), DKIM prüft die **Nachricht** (Signatur), DMARC prüft die **Domain** (Alignment) und legt die **Konsequenzen** fest. Für einen vollständigen Schutz sollten alle drei implementiert sein.
