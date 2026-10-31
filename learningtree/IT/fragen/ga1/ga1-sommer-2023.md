```markdown
## 1. Aufgabe (Server & Security)

### a) Serverkonfiguration (4 Bereiche, 2 Einstellungen)
- **Server-Hardware:** Boot-Reihenfolge ändern (Netzwerk vor HDD), Secure-Boot aktivieren, BIOS/UEFI-Passwort setzen, USB-Ports deaktivieren.
- **Installation und Konfiguration des Betriebssystems:** Nur benötigte Rollen/Features installieren, starkes Admin-Passwort setzen, UAC aktivieren, Updates sofort einspielen.
- **Dienste und Features des Servers:** Unnötige Dienste deaktivieren (SMBv1, Telnet), Nur benötigte Ports in Firewall öffnen, Dienstkonten mit Least Privilege.
- **Anmelden am Server:** Account-Lockout-Policy (z.B. 5 Fehlversuche), Passwortkomplexität erzwingen, MFA für Admin-Zugang, interaktive Anmeldung für Nicht-Admins verbieten.
- **Administrieren des Servers:** Remote-Admin nur über VPN/jump host, Audit-Logging aktivieren, Trennung von Admin- und User-Konto, regelmäßige Berechtigungsprüfung.

### ba) Asymmetrische Verschlüsselung – Funktionsweise
- **Schlüsselpaar:** Jeder Teilnehmer besitzt einen öffentlichen Schlüssel (public key) zum Verschlüsseln und einen privaten Schlüssel (private key) zum Entschlüsseln.
- **Verteilung:** Der Public Key wird frei verteilt; der Private Key bleibt geheim.
- **Verschlüsselung:** Sender verschlüsselt Nachricht mit dem Public Key des Empfängers. Nur der Empfänger kann mit seinem Private Key entschlüsseln.
- **Analogie OSI Layer 6:** Ähnlich wie ein Briefkasten – Einwurfschlitz (public) für alle, Öffnung (private) nur mit Schlüssel.
- **Nachteil:** Langsam, daher nur für Schlüsselaustausch (hybrid) oder Signaturen.

### c) Server mit öffentlich bestätigtem Zertifikat absichern (4 Schritte)
- **Schritt 1 – Keypair erzeugen:** Server generiert privaten Schlüssel und CSR (Certificate Signing Request) mit öffentlichem Schlüssel und FQDN.
- **Schritt 2 – CSR an CA senden:** CSR wird an eine vertrauenswürdige Certificate Authority (z.B. Let's Encrypt, DigiCert) übermittelt.
- **Schritt 3 – Validierung:** CA prüft Domain-Inhaberschaft (DNS-TXT, HTTP-Challenge oder E-Mail).
- **Schritt 4 – Zertifikat installieren:** CA signiert CSR, Server installiert das signierte Zertifikat inkl. Intermediate-Kette.
- **Schritt 5 – Binden an Dienst:** Zertifikat wird im Webserver (IIS, Apache, nginx) für Port 443 gebunden; Clients prüfen Chain of Trust.

### d) E-Mail-Sicherheitsmaßnahmen (2)
- **SPF/DKIM/DMARC:** Verhindert Spoofing. SPF listet autorisierte Mailserver im DNS. DKIM signiert Mails kryptografisch. DMARC definiert Policy bei Verstößen.
- **Attachment-Filtering / Sandboxing:** Ausführbare Anhänge (.exe, .js, .docm) blockieren oder in Sandbox detonieren; Makros standardmäßig deaktivieren.
- **Alternative:** Link-Isolation/URL-Rewriting (Mail-Gateway prüft Links in Echtzeit) und Anti-Phishing-Training.

### e) Symptome einer Systemkompromittierung (2)
- **Ungewöhnliche Netzwerkaktivität:** Hohe ausgehende Traffic-Mengen zu unbekannten IPs/Ports (z.B. C2-Server), DNS-Anfragen zu DGA-Domains.
- **Unerklärliche Systemänderungen:** Neue Admin-Konten, geänderte Firewall-Regeln, deaktivierter Virenscanner, neue Autostart-Einträge.
- **Performance-Einbruch:** CPU/RAM dauerhaft hoch ohne sichtbaren Prozess (Rootkit) oder unerklärliche Festplattenaktivität (Ransomware-Verschlüsselung).

---

## 2. Aufgabe (Verfügbarkeit & Skalierung)

### a) USV-Schaltskizzen
- **Linke Skizze (Bypass, Batterie im Normalbetrieb über Wechselrichter):** **Online-USV (VFI)** – Last wird permanent über Wechselrichter versorgt; Bypass nur bei Fehler.
- **Rechte Skizze (Last direkt am Eingang, Umschalter auf Wechselrichter bei Fehler):** **Offline-USV (VFD)** – Last hängt normal am Netz; bei Stromausfall Umschaltung auf Batterie/Wechselrichter.

### b) USV-Abkürzungen
- **Online:** VFI = **V**oltage and **F**requency **I**ndependent (Spannung und Frequenz unabhängig).
- **Offline:** VFD = **V**oltage and **F**requency **D**ependent (Spannung und Frequenz abhängig vom Netz).
- **Line-Interactive:** VI = **V**oltage **I**ndependent (Spannung unabhängig, Frequenz abhängig).

### c) Line-Interactive vs. Offline – Vor-/Nachteile
- **Vorteil 1:** AVR (Automatic Voltage Regulation) gleicht Spannungsschwankungen ohne Batterieeinsatz aus; schont Batterie und verlängert Laufzeit.
- **Vorteil 2:** Kürzere Umschaltzeit (< 4 ms) gegenüber Offline (5–10 ms); weniger Risiko für empfindliche Netzteile.
- **Nachteil 1:** Teurer als Offline-USV bei gleicher Leistung.
- **Nachteil 2:** Bei Frequenzschwankungen kein Schutz (nur V, nicht F); für kritische Server weniger geeignet als Online.

### b) Skalierung
- **Horizontale Skalierung (Scale out):** Hinzufügen weiterer Server/Instanzen zu einem Pool. Last wird über Load Balancer verteilt. Beispiel: Zusätzliche Web-Server-Instanz im Cluster. Vorteil: nahezu unbegrenzt, ausfallsicher.
- **Vertikale Skalierung (Scale up):** Erhöhung der Ressourcen eines einzelnen Servers (mehr CPU-Cores, RAM, schnellere SSD). Vorteil: einfach, keine Code-Änderung. Nachteil: physische Grenzen, Single Point of Failure.

### c) Blue Green Deployment
- **Grundprinzip:** Zwei identische Umgebungen (Blue = live, Green = staging/neue Version).
- **Before Deployment:** Load Balancer zeigt auf Blue (Version 1.1 live). Green (Version 1.0) ist Staging.
- **Deployment:** Neue Version 1.0 wird auf Green installiert und getestet.
- **After Deployment:** Load Balancer wird auf Green umgeschaltet. Blue wird zum Staging für den nächsten Rollback.
- **Vorteil:** Zero-Downtime, sofortiger Rollback durch Zurückschalten des LB.

---

## 3. Aufgabe (Programmierung & UML)

### a) MONCPU – Programmverweiterung
- **Logik:** Array `usedCPU` mit 24 Werten durchlaufen. Zähle Stunden, in denen Wert > 80. Wenn Zähler > 18, MessageBox anzeigen.
- **Code:**
```csharp
int sumstd = 0;
for (int i = 0; i < usedCPU.Length; i++)
{
    if (usedCPU[i] > 80)
    {
        sumstd = sumstd + 1;
    }
}
if (sumstd > 18)
{
    MessageBox.Show(message, title);
}
```
- **Hinweis:** `cpuutil`/`cpulimit` sind Deklarationen, werden hier nicht zwingend benötigt; `sumstd` ist der Zähler.

### b) Aggregation vs. Komposition
- **Aggregation:** Schwache Beziehung. Teil kann ohne Ganzes existieren. Beispiel: Team (Ganzes) und Spieler (Teil). Spieler existiert auch ohne Team. UML: leere Raute.
- **Komposition:** Starke Beziehung. Teil kann nicht ohne Ganzes existieren; Lebenszyklus gebunden. Beispiel: Haus (Ganzes) und Raum (Teil). Raum existiert nicht ohne Haus. UML: gefüllte Raute.

### c) SCHTASKS-Aufruf
```
SCHTASKS /Create /TN CPUMON /TR "d:\util\MONCPU.EXE" /SC DAILY /ST 16:10
```
- **/TN** = Taskname CPUMON.
- **/TR** = auszuführendes Programm mit vollem Pfad.
- **/SC DAILY** = täglich.
- **/ST 16:10** = Startzeit im 24h-Format.

---

## 4. Aufgabe (Backup & SAN)

### a) Differenzielle Sicherung – Datenvolumen
- **Gegeben (inkrementell):** Mo 25, Di 10, Mi 10, Do 20, Fr 10, Sa 5 GB. Vollsicherung So (nicht dargestellt).
- **Differenziell:** Jeder Tag sichert alle Änderungen seit dem letzten **Vollbackup** (Sonntag).
- **Berechnung kumulativ:**
  - Mo: 25 GB (Änderungen seit So)
  - Di: 25 + 10 = 35 GB
  - Mi: 35 + 10 = 45 GB
  - Do: 45 + 20 = 65 GB
  - Fr: 65 + 10 = 75 GB
  - Sa: 75 + 5 = 80 GB
- **Balkendiagramm:** Mo 25, Di 35, Mi 45, Do 65, Fr 75, Sa 80 (kumulativ steigend).

### ba) Benötigte Nettokapazität (5 Jahre)
- **Altdatenbestand:** 8 TiB * 90% = 7,2 TiB (belegte Daten).
- **Jährlicher Zuwachs:** 450 GiB = 450 / 1024 ≈ 0,4395 TiB.
- **5 Jahre Zuwachs:** 5 * 0,4395 = 2,1975 TiB.
- **Gesamt:** 7,2 + 2,1975 = **9,3975 TiB** ≈ **9,4 TiB**.
- **Hinweis:** Alternative 9,7 TiB laut Aufgabenstellung.

### bb) Anzahl Festplatten für RAID 6
- **RAID 6:** Nettokapazität = (N - 2) * 1 TiB. Benötigt: 9,4 TiB (oder 9,7 TiB).
- **Rechnung:** N - 2 >= 9,4 → N >= 11,4 → **N = 12 Platten**.
- **Mit alternativem Wert 9,7:** N - 2 >= 9,7 → N >= 11,7 → **N = 12 Platten**.
- **Ergebnis:** 12 Festplatten (10 Daten + 2 Parity).

### bc) Hot-Spare-Festplatte
- **Funktion:** Reserve-Festplatte im RAID-Verbund, die im Normalbetrieb nicht genutzt wird (kein Datenzugriff).
- **Automatischer Rebuild:** Bei Ausfall einer Platte übernimmt die Hot Spare sofort den Platz und wird rekonstruiert (Rebuild). Kein manueller Eingriff nötig.
- **Vorteil:** Reduziert die Wiederherstellungszeit (RTO) und das Risiko eines zweiten Ausfalls während des Rebuilds.

### c) RTO vs. RPO
- **RTO (Recovery Time Objective):** Maximale **Zeitdauer**, die ein System nach einem Ausfall ausfallen darf, bis es wieder läuft. Umfasst Fehlersuche, Reparatur, Wiederherstellung, Kommunikation. Beispiel: "System muss in 4 Stunden wieder online sein."
- **RPO (Recovery Point Objective):** Maximaler **Datenverlust** in Zeit gemessen. Definiert das Alter des letzten Backups, das akzeptabel ist. Beispiel: "RPO = 1 Stunde" bedeutet, maximal 1 Stunde Datenverlust erlaubt. Bestimmt Backup-Intervall.

---

**Merksatz:** IHK-Prüfungen testen **Kausalität und Normen** – wer USV-Typen (VFI/VFD/VI), RAID-Parity und Backup-Kumulation nicht als Vokabeln parat hat, verliert Punkte an der Oberfläche. Lerne die Standards (Ports, RAID-Level, DSGVO, OSI) als **Hard-Facts**, nicht als Prosa.
```
