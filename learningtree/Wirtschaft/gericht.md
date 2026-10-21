# Gericht

# Gerichtsbarkeiten in Deutschland – IHK-Prüfungsrelevant

## 1. Ordentliche Gerichtsbarkeit
Zuständig für **Zivilsachen** (Streitigkeiten zwischen Privatpersonen) und **Strafsachen**. Untergliedert in Amtsgericht (AG), Landgericht (LG), Oberlandesgericht (OLG), Bundesgerichtshof (BGH).

**IHK-Bezug / IT-Vergleich:**
- **Amtsgericht (AG):** Erste Instanz bei Zivilsachen unter 5.000 € Streitwert (oder 10.000 € je nach Quelle) sowie Miet-, Familien- und Nachbarrechtssachen [citation:1][citation:3]. Vergleichbar mit einem **Layer 1 (Physical)** – die Basis, wo die meisten „einfachen" Fälle zuerst aufschlagen.
- **Landgericht (LG):** Erste Instanz bei Streitwerten über 5.000/10.000 € und bestimmten Spezialsachen (z.B. Amtshaftung). Berufungsinstanz für AG-Urteile [citation:1][citation:3]. Vergleichbar mit **Layer 2 (Data Link)** – prüft die „Übertragung" der ersten Instanz.
- **Handelskammer:** Bei LG können Kammern für Handelssachen gebildet werden – Besetzung: 1 Berufsrichter + 2 ehrenamtliche Kaufleute [citation:1][citation:3]. **IHK-relevant:** Kaufleute als Laienrichter.

**Merksatz für Prüfung:** „Bürger gegen Bürger → Ordentliches Gericht."

---

## 2. Arbeitsgerichtsbarkeit
Zuständig für **arbeitsrechtliche Streitigkeiten** zwischen Arbeitnehmer und Arbeitgeber (Kündigung, Lohn, Zeugnis, Abfindung). Erste Instanz: Arbeitsgericht (ArbG), Berufung: Landesarbeitsgericht (LAG), letzte Instanz: Bundesarbeitsgericht (BAG) [citation:2][citation:15].

**IHK-Bezug / IT-Vergleich:**
- **Urteilsfindung:** ArbG entscheidet durch Einzelrichter oder Kammer (1 Berufsrichter + 2 ehrenamtliche Beisitzer aus Arbeitgeber-/Arbeitnehmerkreisen). Vergleichbar mit **Layer 3 (Network)** – Vermittlung zwischen zwei „Endpunkten" (AN/AG).
- **Wichtig für Azubis:** Streit um Ausbildungszeugnis oder Kündigung → ArbG. **Aber:** Rehabilitanden sind keine Arbeitnehmer i.S.d. Arbeitsgerichtsgesetzes; deren Streitigkeiten gehören vor das Sozialgericht [citation:4].

**Merksatz für Prüfung:** „Arbeitnehmer gegen Arbeitgeber → Arbeitsgericht."

---

## 3. Verwaltungsgerichtsbarkeit
Zuständig für **öffentlich-rechtliche Streitigkeiten nichtverfassungsrechtlicher Art**, soweit nicht anderen Gerichten zugewiesen. Typisch: Baugenehmigung, Gewerbeerlaubnis, Ausländerrecht, Beamtenrecht, Umweltrecht. Instanzen: Verwaltungsgericht (VG), Oberverwaltungsgericht (OVG), Bundesverwaltungsgericht (BVerwG) [citation:2][citation:15].

**IHK-Bezug / IT-Vergleich:**
- **Abgrenzung zum Sozialgericht:** Beide sind „öffentlich-rechtlich", aber das Sozialrecht ist ein **Sondernetz** (eigene Protokolle/Ports). Wer zur falschen „IP-Adresse" klagt, wird verwiesen. **Prüfungsfalle:** Reha-Kosten → Sozialgericht, nicht Verwaltungsgericht.
- **Praxis:** Wenn die IHK dir die Gewerbeerlaubnis verweigert → Widerspruch → Klage vor VG.

**Merksatz für Prüfung:** „Bürger gegen Staat (allgemein) → Verwaltungsgericht."

---

## 4. Sozialgerichtsbarkeit
Zuständig für **öffentlich-rechtliche Streitigkeiten in Sozialversicherung, Arbeitsförderung, Kriegsopferversorgung, Reha-Leistungen**. Instanzen: Sozialgericht (SG), Landessozialgericht (LSG), Bundessozialgericht (BSG) [citation:2][citation:6][citation:15].

**IHK-Bezug / IT-Vergleich:**
- **Deine Aufgabe 8:** Reha-Kostenübernahme → **Sozialgericht (5)**. Begründung: § 51 SGG weist Sozialversicherungssachen den Sozialgerichten zu.
- **Vorverfahren:** Vor Klage muss **Widerspruch** bei der Behörde (Krankenkasse/Rententräger) eingelegt werden – vergleichbar mit **Retry-Mechanismus** bevor ein neuer Request (Klage) rausgeht.
- **Besetzung:** Kammer mit 1 Berufsrichter + 2 ehrenamtlichen Richtern (Versicherte/Arbeitgeber).

**Merksatz für Prüfung:** „Sozialversicherung, Reha, Rente, Krankenkasse → Sozialgericht."

---

## 5. Finanzgerichtsbarkeit
Zuständig für **Abgabenangelegenheiten** (Steuern, Zölle). Instanzen: Finanzgericht (FG), Bundesfinanzhof (BFH) [citation:2][citation:15].

**IHK-Bezug / IT-Vergleich:**
- **Prüfungsrelevant für Kaufleute:** Steuerbescheid vom Finanzamt → Einspruch → Klage vor FG. Vergleichbar mit **Layer 4 (Transport)** – regelt den „Datenfluss" zwischen Unternehmen und Staat (Steuern).
- **Kein Amtsgericht:** Steuersachen sind **ausschließlich** FG-Sache.

**Merksatz für Prüfung:** „Steuern/Zoll → Finanzgericht."

---

## 6. Verfassungsgerichtsbarkeit
**Bundesverfassungsgericht (BVerfG)** und Landesverfassungsgerichte. Zuständig für Verfassungsstreitigkeiten, Grundrechtsverletzungen (nach Erschöpfung des Rechtswegs). Steht **außerhalb** der fünf Fachgerichtsbarkeiten [citation:15].

**IHK-Bezug / IT-Vergleich:**
- **Nicht prüfungsrelevant für Standard-IHK-Fälle**, aber als „Root-Zertifizierungsstelle" im OSI-Modell denkbar: oberste Instanz, die prüft, ob alles regelkonform ist.

---

## Übersichtstabelle für die Prüfung

| Gerichtsbarkeit | Zuständig für | Erste Instanz | IHK-Stichwort |
|---|---|---|---|
| Ordentlich | Zivil/Straf | AG / LG | Miete, Kaufvertrag, Strafe |
| Arbeits | AN vs. AG | ArbG | Kündigung, Zeugnis, Lohn |
| Verwaltungs | Bürger vs. Staat (allg.) | VG | Baugenehmigung, Gewerbe |
| Sozial | Sozialversicherung | SG | **Reha, Rente, Krankenkasse** |
| Finanz | Steuern/Zoll | FG | Steuerbescheid |
| Verfassung | Grundgesetz | BVerfG | (selten IHK-relevant) |

---

## Prüfungsstrategie (3-Zeiler für die IHK)

**Frage:** „Welches Gericht ist zuständig?"
**Antwort-Schema:** „Es handelt sich um eine [öffentlich-rechtliche / zivilrechtliche / arbeitsrechtliche] Streitigkeit auf dem Gebiet [des Sozialrechts / des Arbeitsrechts / etc.]. Nach [§ 51 SGG / ArbGG / VwGO] sind daher die [Sozialgerichte / Arbeitsgerichte / Verwaltungsgerichte] zuständig. Die Klage ist beim [SG / ArbG / VG] am Sitz des Klägers/Beklagten einzureichen."

---

## DSGVO-Verknüpfung (für Kombifragen)

**Gerichte + DSGVO:** Bei Gerichtsverfahren werden personenbezogene Daten verarbeitet. Rechtsgrundlage: Art. 6 Abs. 1 lit. c DSGVO (rechtliche Verpflichtung) i.V.m. den Verfahrensordnungen (ZPO, SGG, ArbGG). **Wichtig für IHK:** Gerichte sind „öffentliche Stellen" i.S.d. DSGVO, unterliegen aber der **Justiz** – nicht der Aufsicht durch die Datenschutzaufsichtsbehörden (Art. 55 Abs. 3 DSGVO).

**IT-Vergleich:** Gerichtsverfahren = **abgeschottetes VLAN**; DSGVO = **Firewall-Regelwerk**, das den Datenfluss zwischen den VLANs regelt.
