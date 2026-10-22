# Lektion 2 – Verschlüsselung & Hashing

**Bereich:** GA1 (Konzeption und Administration von IT-Systemen)
**Relevanz:** hoch – Datenschutz/IT-Sicherheit sind Dauerbrenner-Themen
**Lernzeit:** ca. 2h

---

## 1. Warum das wichtig ist

Fast jede GA1-Klausur hat mindestens eine Teilaufgabe zu Datenschutz oder Verschlüsselung – oft eingebettet in ein Szenario ("Ein Betrieb möchte Kundendaten schützen…"). Wer die Grundbegriffe sicher unterscheiden kann, holt sich hier zuverlässig Punkte.

## 2. Die drei Grundkonzepte

### a) Symmetrische Verschlüsselung
- Ein einziger Schlüssel für Ver- und Entschlüsselung
- Vorteil: schnell, geringer Rechenaufwand
- Nachteil: Schlüsselaustausch ist das Problem – wie bekommt der Empfänger den Schlüssel sicher?
- Beispiele: **AES-128, AES-256**, (veraltet: DES, 3DES)

### b) Asymmetrische Verschlüsselung
- Schlüsselpaar: **öffentlicher Schlüssel** (verschlüsselt, darf jeder haben) + **privater Schlüssel** (entschlüsselt, bleibt geheim)
- Vorteil: kein Schlüsselaustauschproblem
- Nachteil: deutlich langsamer/rechenintensiver als symmetrisch
- Beispiele: **RSA**, Elliptische-Kurven-Kryptografie (ECC)

### c) Hashing
- **Keine** Verschlüsselung im eigentlichen Sinn – Einwegfunktion, nicht umkehrbar
- Erzeugt aus beliebig langen Daten einen Wert fester Länge (Prüfsumme)
- Verwendung: Passwortspeicherung, Integritätsprüfung (wurde eine Datei verändert?)
- Beispiele: **SHA-256** (sicher), **MD5** (gilt als unsicher/veraltet, Kollisionen bekannt)

## 3. Die Tabelle, die in Klausuren oft (sinngemäß) verlangt wird

| Algorithmus | Typ | Einsatzbereich | Sicherheitsstatus |
|---|---|---|---|
| AES-128 | symmetrisch | Verschlüsseln großer Datenmengen | aktuell als sicher eingestuft |
| AES-256 | symmetrisch | wie AES-128, höhere Schlüssellänge | aktuell als sicher eingestuft, höherer Rechenaufwand |
| RSA | asymmetrisch | Schlüsselaustausch, digitale Signaturen | sicher bei ausreichender Schlüssellänge (≥2048 Bit) |
| MD5 | Hash | historisch: Prüfsummen | **unsicher**, Kollisionen praktisch erzeugbar |
| SHA-256 | Hash | Passwort-Hashing, Integritätsprüfung | aktuell als sicher eingestuft |

⚠️ **Klassischer Prüfungsfehler:** Hashing als "Verschlüsselung" bezeichnen und behaupten, man könne den Hash wieder entschlüsseln. Ein Hash ist grundsätzlich nicht umkehrbar!

## 4. In der Praxis kombiniert: Hybridverfahren

Fast alle echten Systeme (TLS/HTTPS, WLAN WPA2/3) kombinieren beide Welten:

1. **Asymmetrisch** wird genutzt, um sich zu authentifizieren und einen gemeinsamen Sitzungsschlüssel sicher auszutauschen
2. **Symmetrisch** (z. B. AES) wird dann für die eigentliche, schnelle Datenübertragung verwendet

Das ist ein beliebtes Prüfungsthema, z. B. im Kontext WLAN-Sicherheit (WPA2/3): Authentifizierung läuft anders ab als die eigentliche Verschlüsselung des Datenverkehrs.

## 5. Datenschutz-Kontext (oft mit Verschlüsselung kombiniert abgefragt)

Typisches Szenario in Altklausuren: Ein Unternehmen speichert personenbezogene Daten (Name, Kundennummer, Geburtsdatum …) und es kommt zu einem unbefugten Zugriff. Dann wird gefragt nach:

- **Informationspflichten** bei einer Datenpanne (wer muss informiert werden, in welcher Frist)
- **Technische Maßnahmen**, um zukünftigen unbefugten Zugriff zu verhindern (Verschlüsselung, Zugriffskontrolle, Protokollierung)
- Warum reines **Löschen/Formatieren** aus Datenschutzsicht nicht ausreicht (Daten sind oft noch physisch auf dem Datenträger vorhanden und wiederherstellbar – deshalb sicheres Löschen/Überschreiben nötig)

## 6. Übungsaufgabe (im Prüfungsstil, selbst erstellt)

> Ein mittelständisches Unternehmen verschlüsselt seine Kundendatenbank derzeit mit 3DES und speichert Passwörter als MD5-Hash.
>
> a) Bewerten Sie beide Verfahren hinsichtlich ihrer aktuellen Sicherheit und schlagen Sie jeweils eine zeitgemäße Alternative vor.
> b) Erläutern Sie den grundsätzlichen Unterschied zwischen einer Verschlüsselung und einem Hashverfahren.
> c) Warum ist es sinnvoll, Passwörter zu hashen statt sie verschlüsselt zu speichern?

<details>
<summary>Lösung (aufklappen)</summary>

a) 3DES gilt als veraltet/unsicher (kleine effektive Schlüssellänge, langsam) → Alternative: **AES-256**. MD5 gilt als unsicher wegen bekannter Kollisionsangriffe → Alternative: **SHA-256** (idealerweise mit Salt kombiniert).

b) Verschlüsselung ist **umkehrbar** (mit dem passenden Schlüssel lässt sich der Klartext wiederherstellen). Hashing ist eine **Einwegfunktion** – aus dem Hashwert lässt sich der Ausgangswert nicht zurückrechnen.

c) Selbst wenn die Datenbank kompromittiert wird, erhält der Angreifer nur die Hashwerte, nicht die Klartext-Passwörter. Da Hashing nicht umkehrbar ist, kann das Passwort daraus nicht direkt rekonstruiert werden (im Gegensatz zu einer Verschlüsselung, bei der ein gestohlener Schlüssel sofort alle Passwörter offenlegen würde).

</details>

## 7. Selbstcheck – kannst du das?

- [ ] Ich kann symmetrische und asymmetrische Verschlüsselung mit Vor-/Nachteilen erklären
- [ ] Ich kann erklären, warum Hashing keine Verschlüsselung ist
- [ ] Ich kenne mindestens 2 Beispielalgorithmen pro Kategorie und ihren aktuellen Sicherheitsstatus
- [ ] Ich kann erklären, warum Löschen/Formatieren datenschutzrechtlich oft nicht ausreicht

## 8. Weiterführende Ressourcen

- Podcast/Themenübersicht **Stefan Macke – IT-Berufe-Podcast** (fasst häufig abgefragte AP1/AP2-Themen zusammen, inkl. Kryptografie)
- **fachinformatiker.de** – Forum mit "AP2 FISI Masterplan", der Security-Themen priorisiert auflistet
- Suche in deinem Repo gezielt nach "Verschlüsselung" oder "Datenschutz" in den GA1-Musterlösungen, um zu sehen, wie knapp/stichwortartig die IHK Antworten erwartet

---
*Nächste Lektion: WISO – Kündigungsfristen & Arbeitsrecht*
