# Sichere Datenlöschung und Datenträgerentsorgung

## In 30 Sekunden

- **Löschen oder Formatieren reicht nicht**: Es entfernt nur die Verweise im Dateisystem, die Daten selbst bleiben lesbar und sind mit Forensik-Tools wiederherstellbar (Prüfung 2021W).
- Sicher sind: **vollständiges Überschreiben**, **herstellereigenes Secure Erase/Sanitize**, **kryptografisches Löschen** oder **physische Zerstörung**.
- **SSDs** wegen Wear-Leveling nicht einfach überschreiben, sondern Secure Erase/Sanitize, Crypto-Erase oder Zerstörung.
- **Degaussing** (Entmagnetisieren) wirkt nur bei magnetischen Datenträgern (HDD, Band), **nicht bei SSD/USB-Stick**.
- Datenschutzrechtlich gehört das zu **Art. 5 (Speicherbegrenzung), Art. 17 (Löschung) und Art. 32 DSGVO (Sicherheit)**; ein externer Entsorger braucht einen **AVV** und liefert ein **Vernichtungsprotokoll**.

## Kernwissen

| Verfahren | Geeignet für | Hinweis |
|---|---|---|
| Löschen/Schnellformatieren | – | **unsicher**, Daten wiederherstellbar |
| Vollständiges Überschreiben mit Zufallsdaten | HDD | bei modernen Festplatten genügt ein vollständiger Durchlauf; defekte Sektoren werden evtl. nicht erreicht |
| ATA Secure Erase / NVMe Sanitize | SSD, HDD | Firmware löscht alle Zellen inkl. Reservebereiche |
| Kryptografisches Löschen | verschlüsselte Datenträger (BitLocker, LUKS, Self-Encrypting Drive) | Schlüssel sicher vernichten → Daten unlesbar; setzt Verschlüsselung von Anfang an voraus |
| Degaussing | HDD, Magnetband | Datenträger danach unbrauchbar; wirkungslos bei Flash-Speicher |
| Physische Zerstörung (Schreddern, Lochen, Zerkleinern) | alle | höchste Sicherheit; Partikelgröße nach **DIN 66399** |

**DIN 66399 (Vernichtung von Datenträgern)**
- **Schutzklassen 1–3:** normaler, hoher, sehr hoher Schutzbedarf der Daten
- **Sicherheitsstufen 1–7:** je höher, desto kleiner die Reste; der Buchstabe nennt die Datenträgerart (z. B. **P** Papier, **H** Festplatte, **E** elektronische Datenträger wie SSD/USB, **O** optische Datenträger)
- personenbezogene Daten werden üblicherweise mindestens Schutzklasse 2 zugeordnet

**Organisatorisch**
- **Löschkonzept** mit Fristen und Verantwortlichen
- verschlossene Sammelbehälter für Datenträger und Akten
- zertifizierter Entsorger mit **AVV** nach Art. 28 DSGVO, **Vernichtungsprotokoll** mit Seriennummern
- Leasing-, Garantie- und Reparaturrückläufer: Datenträger vorher löschen oder einbehalten („Keep your Hard Drive“)
- Verschlüsselung von Anfang an erleichtert das spätere Löschen erheblich

## Prüfungsfrage

**Frage:** Alte Server-Festplatten mit Kundendaten sollen entsorgt werden. Begründen Sie, warum Formatieren nicht ausreicht, und nennen Sie zwei vorschriftsmäßige Verfahren.

**Musterantwort:** Beim Formatieren oder Löschen werden nur die Verwaltungsinformationen des Dateisystems entfernt; die eigentlichen Daten bleiben auf den Sektoren erhalten und lassen sich mit Wiederherstellungsprogrammen auslesen. Das wäre ein Verstoß gegen Art. 32 DSGVO. Vorschriftsmäßig ist erstens das vollständige Überschreiben bzw. bei SSDs ein Secure Erase über die Firmware und zweitens die physische Zerstörung durch einen zertifizierten Dienstleister nach DIN 66399 mit Vernichtungsprotokoll und Auftragsverarbeitungsvertrag.

## Falle

- Degaussing für SSDs empfehlen → wirkt nicht bei Flash-Speicher.
- „35-mal überschreiben“ als Pflicht nennen → bei modernen HDDs unnötig; wichtiger ist, dass **alle** Bereiche erfasst werden.
- Den Papierkorb leeren für eine Löschung halten.
- Externe Vernichtung ohne Vertrag und Nachweis beauftragen.

## Merksatz

**„Gelöscht ist nicht weg – überschreiben, sanitisieren oder schreddern, und alles protokollieren.“**

## Verwandt

- [BitLocker](../Server/bitlocker.md) – Verschlüsselung erleichtert das Löschen
- [TOM nach DSGVO](../../Management/DSGVO/toms.md)
- [Auftragsverarbeitung (AVV)](../../Management/DSGVO/auftragsverarbeitung-avv.md) – Entsorger als Auftragsverarbeiter
- [Betroffenenrechte](../../Management/DSGVO/betroffenenrechte.md) – Recht auf Löschung
- [USB, SSD, HDD](../../Komponenten/usb-ssd-hdd.md) – Aufbau der Datenträger
- [Lektion 05 Datenschutz und Informationssicherheit](../../analyse-der-exams/05_Datenschutz_Informationssicherheit.md)
