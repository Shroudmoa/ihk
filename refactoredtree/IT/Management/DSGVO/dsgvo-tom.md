# DSGVO & TOM

## In 30 Sekunden

- Personenbezogene Daten sind alles, was sich auf eine Person beziehen lässt – auch IP-Adresse und Personalnummer.
- Jede Verarbeitung braucht eine **Rechtsgrundlage** nach Art. 6 DSGVO.
- **TOM** (Art. 32) sind technische und organisatorische Maßnahmen – die Prüfung will konkrete Beispiele, keine Floskeln.
- Betroffenenrechte: Auskunft, Berichtigung, Löschung, Einschränkung, Datenübertragbarkeit, Widerspruch.
- Datenschutzverletzung mit Risiko: Meldung an die Aufsichtsbehörde **innerhalb von 72 Stunden**.

## Kernwissen

**Rechtsgrundlagen nach Art. 6**
| Grundlage | Wann | Beispiel |
|---|---|---|
| Einwilligung (lit. a) | freiwillig, informiert, widerrufbar | Newsletter |
| Vertrag (lit. b) | für die Vertragserfüllung nötig | Lieferadresse |
| Rechtliche Pflicht (lit. c) | Gesetz verlangt es | Rechnungsaufbewahrung |
| Berechtigtes Interesse (lit. f) | Abwägung nötig | Videoüberwachung |

# DSGVO – Kernpunkte

**Löschfristen**
*   **Pflicht:** Löschung, sobald Rechtsgrundlage entfällt (Widerruf) oder Zweck erfüllt ist.
*   **Art. 17 DSGVO:** „Recht auf Vergessenwerden“.
*   **Planung:** Lebenszyklus im Verarbeitungsverzeichnis festhalten und Fristen definieren.

**Persönlichkeitsrechte (Bild/Video)**
*   **Opt-in:** Aktive Einwilligung (elektronisch, Papier oder konkludent) zwingend erforderlich.
*   **Transparenz:** Klare Datenschutzerklärung und Hinweise auf Überwachung.

**Zweckbindung**
*   **Definition:** Verarbeitung nur zur Erfüllung rechtlicher Pflichten oder des festgelegten Zwecks.
*   **Datenminimierung:** Nur notwendige Daten erheben.
*   **Löschung:** Nach Zweckerreichung sofortige Löschpflicht.


# Datenschutzkonzept (DSGVO)

Ein Datenschutzkonzept ist das interne Regelwerk und die Dokumentation eines Unternehmens zur Einhaltung der DSGVO. Es dient dem Nachweis der gesetzeskonformen Datenverarbeitung gegenüber Aufsichtsbehörden.

## Inhaltliche Anforderungen (4 Punkte für die IHK)

Nenne vier der folgenden Punkte:

1.  **Verzeichnis von Verarbeitungstätigkeiten (VVT):** Dokumentation, welche personenbezogenen Daten zu welchem Zweck, wo und von wem verarbeitet werden.
2.  **Technische und Organisatorische Maßnahmen (TOM):** Nachweis der Datensicherheit (z.B. Zutritts-, Zugangs- und Zugriffskontrolle, Verschlüsselung).
3.  **Löschkonzept:** Genaue Definition von Fristen und Prozessen zur Vernichtung personenbezogener Daten (Recht auf Vergessenwerden).
4.  **Datenschutz-Folgenabschätzung (DSFA):** Risikobewertung für Verarbeitungen mit hohem Risiko für die Rechte und Freiheiten natürlicher Personen.
5.  **Betroffenenrechte:** Prozesse zur Bearbeitung von Auskunfts-, Berichtigungs-, Lösch- und Widerspruchsanfragen.
6.  **Auftragsverarbeitung (AVV):** Liste und Verträge mit externen Dienstleistern (z.B. Cloud-Anbieter), die Zugriff auf Daten haben.
7.  **Notfall- / Data-Breach-Konzept:** Vorgehen bei Datenschutzverletzungen (Meldepflichten an die Aufsichtsbehörde).
8.  **Schulungskonzept:** Nachweis der regelmäßigen Sensibilisierung und Schulung der Mitarbeiter.
9.  **Verantwortliche Stelle** für den Datenschutz in der Firma (Nennung der Beauftragten)



**Die TOM, die in der Prüfung zählen**
| Maßnahme | Konkret |
|---|---|
| Zutrittskontrolle | Wer kommt ins Rechenzentrum? Karte, Protokoll |
| Zugangskontrolle | Wer darf sich anmelden? Passwort, MFA |
| Zugriffskontrolle | Wer darf welche Daten? Rollen, ACL |
| Weitergabekontrolle | Was verlässt das Netz? TLS, VPN |
| Eingabekontrolle | Wer hat was geändert? Logs, Vier-Augen-Prinzip |
| Auftragskontrolle | Externer Dienstleister? AVV nach Art. 28 |
| Verfügbarkeitskontrolle | Backup, RAID, USV |
| Trennungskontrolle | Mandanten, Test und Produktion getrennt |

**Rollen**
- **Verantwortlicher:** entscheidet über Zweck und Mittel der Verarbeitung.
- **Auftragsverarbeiter:** verarbeitet weisungsgebunden, braucht einen AVV.
- **Datenschutzbeauftragter:** berät und überwacht intern, ist nicht die Aufsichtsbehörde.

**Wichtig:** Eine Einwilligung ist nicht immer nötig. Es muss nur eine passende Rechtsgrundlage geben.

## Prüfungsfrage

**Frage:** Ein Dienstleister soll für ein Unternehmen E-Mails archivieren. Nenne zwei DSGVO-Pflichten und je eine passende TOM.

**Musterantwort:** Es braucht einen Auftragsverarbeitungsvertrag nach Art. 28 und eine Rechtsgrundlage für die Verarbeitung. Technisch: Verschlüsselung bei Übertragung und Speicherung als Weitergabekontrolle. Organisatorisch: Rollenkonzept mit minimalen Rechten und Protokollierung als Zugriffskontrolle.

## Falle

- „Einwilligung ist immer nötig“ → falsch, eine andere Rechtsgrundlage reicht.
- TOM nur mit „Firewall“ beantworten → organisatorische Maßnahmen fehlen, das gibt Punktabzug.
- Datenschutzbeauftragter und Aufsichtsbehörde verwechseln.
- DSGVO und BDSG gleichsetzen: die DSGVO gilt EU-weit, das BDSG ergänzt national.

## Merksatz

**„Rechtsgrundlage zuerst, TOM danach.“** Und die acht Kontrollen: **Zutritt, Zugang, Zugriff, Weitergabe, Eingabe, Auftrag, Verfügbarkeit, Trennung.**

## Verwandt

- [Schutzziele](../../FiSi-Administrator/Sicherheit/schutzziele.md) – Vertraulichkeit, Integrität, Verfügbarkeit
- [hackers-can](../../FiSi-Administrator/Sicherheit/hacking/hackers-can.md) – typische Angriffe auf diese Schutzziele
- [Symmetrische-und-Asymmetrische](../../FiSi-Administrator/Sicherheit/Verschluesselung/symmetrische-und-asymmetrische.md) – Verschlüsselung als TOM






-------------------

# Zutrittskontrolle (DSGVO / TOM)

**Definition:** Physische Maßnahmen, die sicherstellen, dass nur befugte Personen Zutritt zu Bereichen haben, in denen personenbezogene Daten verarbeitet werden (z.B. Rechenzentrum, Serverraum, Aktenschränke).

## Maßnahmen & konkrete Umsetzung

| Maßnahme | Konkrete Umsetzung (Technisch / Organisatorisch) |
| :--- | :--- |
| **Bauliche Sicherung** | Stahltüren, vergitterte Fenster, geschlossene Wände, Sicherheitsschlösser. |
| **Zugangskontrolle (Wer?)** | Elektronische Zutrittskarten (RFID), Fingerabdruckscanner, PIN-Codes, Schlüsselverwaltung. |
| **Protokollierung** | Elektronisches Zutrittsprotokoll (Wer war wann wo?), Videoüberwachung im Eingangsbereich, manuelles Wachbuch. |
| **Besucherregelung** | Besucherausweise, Begleitpflicht durch Mitarbeiter, getrennte Wartebereiche. |
| **Alarmanlagen** | Einbruchmeldeanlagen (EMA), Bewegungsmelder, Türkontaktschalter. |
| **Personelle Maßnahmen** | Zuverlässigkeitsprüfung des Sicherheitspersonals, klare Dienstanweisungen. |

## IHK-Merksatz
> „Zutrittskontrolle = Wer kommt ins Gebäude/den Raum? Es geht um die physische Absicherung der Räumlichkeiten, in denen Daten verarbeitet werden oder Datenträger lagern.“


---
# Rechtsgrundlagen der DSGVO (Art. 6)
*Eine Einwilligung ist nicht immer nötig. Es muss nur eine passende Rechtsgrundlage geben.*

| Rechtsgrundlage | Erklärung | Praxisbeispiel |
| :--- | :--- | :--- |
| **1. Vertragserfüllung** (Art. 6 Abs. 1 lit. b) | Verarbeitung ist nötig, um einen Vertrag mit der Person zu erfüllen oder vorzubereiten. | Ein Online-Shop speichert die Lieferadresse, um das bestellte Paket zu verschicken. Ein Bankhaus speichert Kontodaten für Überweisungen. |
| **2. Rechtliche Verpflichtung** (Art. 6 Abs. 1 lit. c) | Verarbeitung ist gesetzlich vorgeschrieben. | Ein Bankhaus muss Kundendaten für 10 Jahre aufbewahren (Geldwäschegesetz, HGB). Ein Arbeitgeber muss Löhne an das Finanzamt melden. |
| **3. Berechtigtes Interesse** (Art. 6 Abs. 1 lit. f) | Verarbeitung dient einem legitimen Interesse des Verantwortlichen, sofern die Rechte der Person nicht überwiegen. | Videoüberwachung zur Diebstahlprävention im Laden. Speicherung der IP-Adresse im Webserver-Log zur Abwehr von Hackerangriffen. |
| **4. Schutz lebenswichtiger Interessen** (Art. 6 Abs. 1 lit. d) | Verarbeitung ist nötig, um das Leben einer Person zu schützen. | Ein bewusstloser Patient im Krankenhaus bekommt eine Notfallbehandlung; sein Allergiedatensatz wird abgerufen, obwohl keine Einwilligung vorliegt. |
| **5. Öffentliches Interesse** (Art. 6 Abs. 1 lit. e) | Verarbeitung dient der Wahrnehmung einer öffentlichen Aufgabe. | Eine Meldebehörde verarbeitet Daten für das Melderegister. Eine Schule verarbeitet Schülerdaten für die Zeugniserstellung. |
| **6. Einwilligung** (Art. 6 Abs. 1 lit. a) | Die Person hat freiwillig, informiert und eindeutig zugestimmt. | Ein Newsletter-Abo per Double-Opt-In. Eine Fotoaufnahme für die Firmenwebsite (Opt-in). |
