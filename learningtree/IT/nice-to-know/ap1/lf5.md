# Klausurvorbereitung LF5

## Lektion 1 – Grundlagen: Daten, Informationen, Wissen

### Daten vs. Information
*   **Information:** Transfer von Wissen, der beim Empfänger zu einer Änderung des Wissens führt (Beseitigung von Unwissen). Immateriell, an Träger gebunden, kostet Geld, leicht übertragbar, hat einen Wert.
*   **Daten:** Wiederverwendbare Darstellung der Information in formalisierter Art (für Kommunikation, Interpretation, Prozesse). Grundlage ist der Binärcode (Bit-Folge).
*   *Interpretation:* Die Bedeutung der Daten hängt vom Bezugssystem ab (z.B. 1000001 = 65 im Dezimalsystem, 65 im Dualsystem, 'A' im ASCII).

### EVA-Prinzip
*   Eingabe -> Verarbeitung -> Ausgabe.

### Zahlensysteme
*   **Dualsystem:** Basis 2 (0, 1). Grundlage für IT.
*   **Dezimal zu Dual:** Division durch 2 (Rest aufschreiben, von unten nach oben lesen) ODER Potenzen von 2 (14 = 8+4+2+0 = 1110).
*   **Dual zu Dezimal:** Potenzen von 2 (1101 = 1*2³ + 1*2² + 0*2¹ + 1*2⁰ = 13).
*   **Ganze Zahlen mit Vorzeichen:** Zweierkomplement (erstes Bit = Vorzeichen, 0=positiv, 1=negativ).

### Zeichen und Text
*   **Zeichensatz:** Sammlung von Zeichen (ASCII, Unicode). Jedem Zeichen ist ein Binärcode zugeordnet.

### Grafische Daten
*   **Rastergrafik (Bitmap):** Folge von Bildpunkten (Pixel). Farbtiefe bestimmt Bits pro Pixel. Nicht verlustfrei skalierbar. Formate: .jpg, .png, .gif.
*   **Vektorgrafik:** Aus Linien, Kreisen, Polygonen (Parameter: Koordinaten, Stärke, Farbe). Verlustfrei skalierbar, weniger Speicherplatz. Formate: .svg, .pdf, .ai.
*   **Farbmodelle:** RGB (additiv), CMYK (subtraktiv). True Color = 24 Bit (16,7 Mio. Farben).

### Audiodaten
*   **Sampling-Rate:** Messungen pro Sekunde (z.B. CD = 44,1 kHz).
*   **Sampling-Tiefe:** Bits pro Ton (z.B. CD = 16 Bit).
*   **Tonkanäle:** Mono (1), Stereo (2), Surround (5.1).

### Algorithmus & Programm
*   **Algorithmus:** Definierte, endliche Vorgehensweise zur Problemlösung. Programmiersprachenunabhängig.
*   **Programm:** Quellcode (Befehle, Speicheradressen). Umsetzung des Algorithmus in einer Programmiersprache.

### Speicherumrechnung
*   **Dezimal (SI):** kB (10³), MB (10⁶), GB (10⁹), TB (10¹²).
*   **Binär (IEC):** KiB (2¹⁰), MiB (2²⁰), GiB (2³⁰), TiB (2⁴⁰).
*   **Speicherbedarf Bild:** Höhe * Breite * Farbkanäle * Farbtiefe (in Bit) / 8 = Byte.

---

## Lektion 2 – Einführung in Datenbanken

### Warum Datenbanken statt Excel?
*   Excel: Redundanzen, Inkonsistenzen, mangelnde Datenintegrität, eingeschränkte Skalierbarkeit, mangelnde Sicherheit, schwierige Zusammenarbeit.
*   Datenbanken: Strukturierte, zentrale Datenhaltung, Integrität, Skalierbarkeit, Sicherheit, gemeinsame Nutzung.

### Grundbegriffe
*   **Datenbank (DB):** Sammlung zusammengehöriger Daten (elektronischer Karteikasten).
*   **DBMS (Datenbankmanagementsystem):** Software zur Verwaltung der DB (Anlegen, Speichern, Ändern, Löschen, Abfragen, Benutzerverwaltung). Bsp: MySQL, Oracle, MS SQL.
*   **DBS (Datenbanksystem):** DB + DBMS.

### Schlüssel
*   **Schlüsselkandidat:** Attribut(e), die Tupel eindeutig identifizieren.
*   **Primärschlüssel:** Ausgewählter Schlüsselkandidat zur eindeutigen Identifizierung (minimale Anzahl Spalten).

### Datenunabhängigkeit (3-Ebenen-Modell nach ANSI-SPARC)
*   **Physische Datenunabhängigkeit:** Interne Ebene von konzeptioneller/externer entkoppelt. Anwendungen unabhängig von Speicherstrukturen.
*   **Logische Datenunabhängigkeit:** Konzeptionelle und externe Ebene getrennt. Änderungen an DB-Struktur haben keine Auswirkung auf Sichten/Anwendungen.

### 3-Ebenen-Modell
1.  **Externe Ebene (Sicht):** Was will wer sehen? Benutzersichten (Views).
2.  **Konzeptionelle Ebene (Logik):** Wie hängt alles zusammen? Gesamtstruktur (Tabellen, Beziehungen, Integritätsbedingungen). ER-Modell/UML.
3.  **Interne Ebene (Physisch):** Wo liegen die Daten real? Speichermedien, Zugriffsmöglichkeiten.

### Datenbankentwicklung (Schritte)
1.  **Anforderungsanalyse:** Bedarf klären, Nutzer befragen.
2.  **Konzeptioneller Entwurf:** ER-Modell (grafisch).
3.  **Logischer Entwurf:** Umsetzung in DB-Modell (Normalisierung, Redundanzen entfernen).
4.  **Physischer Entwurf (Implementierung):** Internes Schema, Speicherstrukturen, Daten einspielen.
5.  **Test & Validierung, Einsatz & Wartung.**

### ER-Modell
*   **Entität:** Konkretes/abstraktes Objekt (z.B. Kunde).
*   **Entitätstyp:** Kategorie ähnlicher Entitäten (z.B. Produkt).
*   **Beziehung:** Verbindung zwischen Entitäten (z.B. Bestellung).
*   **Beziehungstyp:** Art der Verbindung zwischen Entitätstypen (z.B. Kauf).
*   **Attribut:** Eigenschaft/Merkmal (z.B. Alter).

### ER-Modell -> Relationenschema
1.  Entitätstyp -> Tabelle.
2.  Attribut -> Spalte.
3.  1:n-Beziehung -> Fremdschlüssel auf n-Seite.
4.  1:1-Beziehung -> Fremdschlüssel in einer Tabelle.
5.  n:m-Beziehung -> Beziehungstabelle mit Fremdschlüsseln (zusammengesetzter Primärschlüssel).

### Anomalien
*   **Einfügeanomalie:** Daten können nicht eingefügt werden, weil andere fehlen.
*   **Löschanomalie:** Löschen eines Attributs löscht abhängige Attribute (Datenverlust).
*   **Aktualisierungsanomalie:** Änderungen führen zu Inkonsistenzen.
*   **Datenredundanz:** Daten mehrfach gespeichert.

### Normalformen
*   **1. Normalform (1NF):** Alle Attribute atomar (keine zusammengesetzten Informationen).
*   **2. Normalform (2NF):** 1NF + Alle Nichtschlüsselattribute hängen vollständig vom gesamten Primärschlüssel ab.
*   **3. Normalform (3NF):** 1NF + 2NF + Keine transitiven Abhängigkeiten (Nichtschlüsselattribute hängen nicht voneinander ab).

### SQL-Grundstruktur
*   `SELECT` (Spalten) `FROM` (Tabelle) `WHERE` (Bedingung) `ORDER BY` (Sortierung).
*   Boolesche Operatoren (AND, OR, NOT), Aggregatfunktionen (COUNT, SUM, AVG, MIN, MAX).
