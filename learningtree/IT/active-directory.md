# Active-Directory

Active Directory ist Microsofts integrierte Verzeichnisdienstplattform, die in Windows Server betrieben wird und es Organisationen ermöglicht, Ressourcen und Benutzerkonten auf eine zentralisierte, hierarchische und sicherheitsbewusste Weise zu verwalten.

Nachfolgend finden Sie eine prägnante, aber umfassende Übersicht über die wichtigsten Komponenten, Konzepte, Dienste und Verwaltungsmöglichkeiten von AD sowie einige praktische Tipps und häufige Fallstricke.

---

## 1. Geschichte und Zweck
| Zeitleiste | Meilenstein |
|----------|-----------|
| **2000** | Einführung von Windows 2000 Server – erste Version von AD als Verzeichnisdienst für die Verwaltung von Domänen, Benutzern und Ressourcen. |
| **2003/2008** | Erweiterte Funktionen: Gruppenrichtlinien, Vertrauen zwischen Forests, AD LDS (Lightweight Directory Services), AD FS (Active Directory Federation Services). |
| **2012/2016** | Verbesserte Cloud-Integration (Azure AD), überarbeitete Verwaltungstools, Unterstützung für die Arbeit von überall (Pass-Through-Authentifizierung, Hybrididentitäten). |
| **2019–2022** | Verbesserte Leistung, neue PowerShell-Cmdlets, Modernisierung des „AD Admin Center“, Integration von Azure AD Connect. |
| **2024+** | Kontinuierliche Weiterentwicklung (z. B. AD Trust-Updates für Passwort-Hash-Synchronisierung, „Schnelle Anmeldung“ in Azure AD). |

**Hauptzweck:**
- **Verzeichnisdienst:** Speichert Objekte (Benutzer, Computer, Gruppen, Geräte, Anwendungen) und deren Attribute in einer strukturierten Datenbank (Ntds.dit).
- **Identitäts- und Zugriffsverwaltung:** Authentifizierung (Kerrighedienste), Autorisierung (basierend auf Sicherheitsgruppen, RBAC) und Richtlinienüberwachung.
- **Ressourcenverwaltung:** Gruppenrichtlinienbereitstellung, Softwareverteilung, Zertifikatsausstellung, Geräteüberwachung.

---

## 2. Kernarchitektur

| Komponente | Was sie ist | Wichtige Merkmale |
|-----------|------------|--------------|
| **Domäne** | Logische Gruppe von Computern, die ein gemeinsames AD-Verzeichnis freigeben und gemeinsame Richtlinien befolgen. | Grundlage für die Authentifizierung; wird durch einen DNS-Domänennamen identifiziert (z. B. `contoso.com`). |
| **Domänencontroller (DC)** | Server, die einen Schreibzugriff auf die AD-Datenbank (`Ntds.dit`) haben und mit den DCs der Domäne kommunizieren. | Muss Windows Server OS mit dem „Active Directory-Domänendienste“-Feature ausführen. |
| **Forest** | Höchste logische Gruppe, die eine Sammlung von Domänen enthält, die ein gemeinsames Verzeichnis schema, Verzeichnispartitionen und Vertrauensbeziehungen teilen. | Ein Forest hat einen einzigen `schema`- und `configuration`-Container und eine `domainnaming`-Partition. |
| **Tree** | Eine oder mehrere zusammengehörende Domänen in einem Forest (z. B. `child.contoso.com` unter `contoso.com`). | Bäumliche Struktur erleichtert die logische Gruppierung von Ressourcen. |
| **Schema** | Definiert die Attributtypen und Objektklassen, die in einem Forest vorhanden sind. | Die Verwaltung des Schemas erfolgt über `Active Directory Schema`-Snap-In oder PowerShell (`Add-ADSchemaExtension`, `Remove-ADSchemaExtension`). |
| **Konfiguration** | Speichert topologische Informationen (Domänen, DCs, Vertrauensbeziehungen, Site-Topologie usw.). | Wird automatisch repliziert; wird für die Standortbasierte Vermittlung verwendet. |
| **Benutzer- und Geräteobjekte** | Konten (User, Group) und Computerobjekte, die Sicherheitsidentitäten darstellen. | Jedes Objekt hat einen `SAMAccountName` (Domänenpräfix) und einen `Distinguished Name` (DN). |
| **Attributbezeichner (OID)** | Eindeutige Ganzzahlen, die AD-Attribute identifizieren (z. B. `1.2.840.113556.1.2.79` = `member`). | Wird im Schema für jede Objektklasse registriert. |
| **DNS** | DNS ist grundlegend: Jede Domäne erhält einen DNS-Domänennamen, DCs veröffentlichen `Srv`-Resource-Records (z. B. `_ldap._tcp.dc._msdcs.<domain>.`). | Muss eine zuverlässige, authoritative DNS-Infrastruktur sein. |
| **Partitions** | Teile des Gesamtverzeichnisses (Schema, Konfiguration, Domänen-NTDS-Partikel, Anwendungspartitionen). | Die Replikation erfolgt pro Partition über Auswahlen für jedes Attribut. |

### Replikation
- **LSYNC (Link-State Change)**: DCs replizieren Änderungen an Domänenpartitionen anhand einer topologiebasierten Auswahl.
- **Konfiguration und Schema**: Repliziert in einer vollständigen, vollständigen Mesh-Konfiguration.
- **Transportprotokolle**: <br>• **DNS-SRV** (Standard, über HTTP/HTTPS) <br>• **IP-SLP** (älter) <br>• **WinRM** (optional für „improved replication“).
- **Wartung:** `repadmin /showrepl`, `Get-ADReplicationStatus` (PowerShell).

---

## 3. Hauptdienste

| Dienst | Beschreibung | Typische Anwendungsfälle |
|---------|-------------|--------------------|
| **AD DS (Domänendienste)** | Klassischer Verzeichnisdienst mit Schreibzugriff, der Benutzer, Computer und andere Objekte hostet. | On-Premises-Identität, Kerberos-Authentifizierung, Gruppenrichtlinien. |
| **AD LDS (Lightweight Directory Services)** | Schemalose oder benutzerdefinierte Verzeichnisdaten ohne Domänenintegration (wird auch als „AD LDS“ bezeichnet, früher „AD-Administratorcenter“). | Konfiguration für nicht-Domänenbezogene Verzeichnisdaten (z. B. Zertifikatsanfordert). |
| **AD FS (Active Directory Federation Services)** | SAML- und OAuth-basiert für die Single-Sign-On-Authentifizierung (SSO) über Organisationsgrenzen hinweg. | Externe Benutzer erhalten Zugriff auf Cloud-Apps (Office 365, SaaS) ohne VPN. |
| **AD CS (Active Directory Zertifikatsdienste)** | Bereitstellung von Zertifikatsvorlagen, Ausstellung und Verwaltung von Zertifikaten. | interne PKI, TLS für Websites, Code-Signatur. |
| **AD RMS (Rights Management Services)** – *veraltet*; neuerer Ansatz verwendet Azure Information Protection. | Verschlüsselung und Schutz von Daten (jetzt in Azure AD Protection integriert). | Sensible Dokumentenkontrolle. |
| **AD recycle bin** (ab Windows Server 2016) | Erlaubt das wiederherstellen gelöschter Objekte ohne Wiederherstellung des gesamten Forests. | Problembehandlung und Auditing. |
| **Azure AD** | Cloudbasierter Verzeichnisdienst (auch als „Microsoft Cloud AD“ bezeichnet). | Mitarbeiteridentität für Cloud-Apps, SaaS, B2C/ B2B. |

---

## 4. Verwaltung und Verwaltungstools

| Tool | Typ | Wichtige Aufgaben |
|------|------|-------------|
| **Active Directory-Benutzer und -Computer (ADUC)** | GUI-Snap-In (schlägt auf `dsa.msc` auf) | Benutzerverwaltung, Gruppenrichtlinien-Verknüpfung, OU-Erstellung und Zuweisung. |
| **Active Directory-Administratorcenter (ADA)** | GUI (neuer, erweiterter Zugriff) | Verbesserte Berichterstattung, „Quick-Select“, DOM-Kartierung. |
| **Windows PowerShell** | Befehlszeile / Skripting | Verwendung von `ActiveDirectory` (RSAT) oder `Add-PSSnapin` <br> **Beispiele:** <br> `Get-ADUser -Filter * -Properties *` <br> `New-ADGroup -Name "DBA" -GroupScope DomainLocal -Path "OU=Groups,DC=contoso,DC=com"` <br> `Set-ADUser -Identity jsmith -PasswordNeverExpires $true` |
| **Active Directory-Modul für PowerShell** (`Install-WindowsFeature -Name RSAT-AD-Powershell`) | Modul | Ermöglicht die remote Ausführung von Befehlen über `Enter-PSSession` mit einem DC oder über `Invoke-Command`. |
| **AD Explorer** | GUI (Sysinternals) | Baumbasierte Objektinspektion, erweiterte Filterung, Snapshot-Funktion. |
| **Mitglieder der Richtungsgruppe** | GUI (dsa.msc) | Echtzeit-Überprüfung von Gruppenmitgliedschaften, erleichtert die „was-wäre-wenn“-Analysen. |
| **Get-ADForest / Get-ADDomain / Get-ADDomainController** | PowerShell | Abfrage von Forest-, Domänen- und DC-Informationen (z. B. Rolle des letzten.Inkrementals, Betriebsmaster). |
| **Group Policy Management Console (GPMC)** | GUI | Erstellung, Bearbeitung und Bereitstellung von Richtlinien, Backups. |
| **SACL / DAC** | Erweitertes Auditing über die Sicherheitseigenschaften von Objekten (z. B. `auditpolicy /view`). |
| **Quest / Veeam / SolarWinds** | Dritte | Verbesserte Berichterstattung, AD-Gesundheitsüberwachung. |

---

## 5. Sicherheit und bewährte Praktiken

| Praxis | Warum es wichtig ist | Umsetzungs-Tipps |
|----------|----------------|------------------------|
| **Starke Passwörter und Passwortrichtlinien** | Verhindert Passwortvergabe und Angriffe mit gestohlenen Anmeldeinformationen. | Verwenden Sie `Group Policy` → **Password Settings Policy** (komplex, Länge ≥ 12, Expiration ≤ 90 Tage). |
| **Passwort-Hash-Synchronisierung (PHS) für Hybrididentitäten** | Reduziert die Notwendigkeit eines Fernzugangs, aber dennoch unsicher? | Verwenden Sie Azure AD Connect; aktivieren Sie **Pass-Through-Authentifizierung** oder **PHS** basierend auf Ihrem Sicherheitsniveau. |
| **Konten mit geringsten Berechtigungen** | Begrenzt die Ausbreitung von Schadsoftware. | Führen Sie Server als **nicht-domänenfreien** Kontos aus, deaktivieren Sie **Domänenadministrator**-Konten für die tägliche Nutzung. |
| **Sichere Kommunikationskanäle** | Verhindert Abhören und Man-in-the-Middle-Angriffe. | Enforce TLS 1.2 für AD-Abfragen, verwenden Sie LDAPS (StartTLS) über Zertifikatsbindung (`ldaps://`). |
| **Überwachung von privilegierenden Operationen** | Detektiert Ausnutzungsversuche; erfordern die Aktivierung von SACL/ Audit. | Aktivieren Sie **„Sicherheits“-Protokolle** für `Directory Service Access` und `Logon/Logoff`. |
| **Sichere replication** | AD repliziert Klartextpasswörter (gehasht). Sichere Kanäle sind unerlässlich. | Stellen Sie sicher, dass DCs kommunizieren können, verwenden Sie IP-Sec (Transport Layer Security) für die Replikation. |
| **Geplante Backups** | Wiederherstellung nach Datenverlust oder Ransomware-Angriffen. | Verwenden Sie `ntdsutil` für Volumes und erzeugen Sie eine vollständige Forest-Wiederherstellung oder verwenden Sie AD-Tombstone-Löschfristen. |
| **Überprüfung der Sicherheitsrichtlinie** | Erkennen verwaister Objekte und Anmeldeinformationen. | Führen Sie monatlich `Get-ADUser -Filter {Enabled -eq $false}` aus. |
| **Starke Schema/Operationsmaster-Sicherheit** | Die Operationsmaster-Rollen (Schema-, Domino-, RID-, PDC-, Infrastruktur-) sind Hochrisikofunktionen. | Beschränken Sie den Zugriff auf privilegiertes Personal, aktivieren Sie Multifaktor-Authentifizierung für DCs, die diese Rollen ausführen. |
| **Implementierung von AD FS / Pass-Through** | Reduziert die Exposition gegenüber Passwort-Hashes im Netzwerk. | Verwenden Sie die neuesten AD FS-Zertifikatschichtkonfigurationen und implementieren Sie „Multi-Factor Authentication“-Add-ons. |
| **Active Directory Recycle Bin** | Schnellere Wiederherstellung von Löschvorgängen, weniger Abhängigkeit von TOOT (Tail-Trigger-Overwrite-Tombstone). | Stellen Sie sicher, dass Sie Windows Server 2016+ mit aktuellen Patches verwenden, aktivieren Sie `Enable-ADOptionalFeature`. |
| **Gruppenrichtlinienbereitstellung** | Erzwingt Sicherheitsrichtlinien und Konfigurationen über die Domäne hinweg. | Testen Sie in einer OU vor der umfassenden Bereitstellung, überwachen Sie `Resultant Set of Policy (RSOP)` und verwenden Sie `gPResult`. |
| **Segmentierung und Branchesite** | Verbessert die Leistung für entfernte Standorte. | Verwenden Sie DCs, die der nächstgelegenen Site angehören; konfigurieren Sie `site-links` und `site-link-bridge`. |

---

## 6. Häufige Szenarien und Fallstricke

| Szenario | Typischer Ablauf | Typische Fallstricke / Lösungen |
|----------|----------------|---------------------------|
| **Neues Offsite-Büro einrichten** | 1️⃣ Erstellen Sie DCs, fügen Sie sie der entsprechenden Site hinzu. <br>2️⃣ Führen Sie DNS-Integration durch. <br>3️⃣ Konfigurieren Sie die Replikation (`repadmin /kcc`). <br>4️⃣ Installieren Sie den gleichen OS/Build-Standard. | Falsche GUIDs oder `Invalid Site Link`-Konfiguration. **Lösung:** Überprüfen Sie `repadmin /showrepl` und bestätigen Sie die DNS-SRV-Einträge. |
| **OU-Verschiebung (z. B. Benutzerkonten konsolidieren)** | `Move-ADObject` oder GUI; kopieren Sie `msDS-*, `pwdLastSet`, `accountExpires` usw. | Vergessen, `msDS-ManagedPassword`-Attribute zu kopieren; dies führt dazu, dass Passwörter für Wiederherstellungskonten erneuert werden. Verwenden Sie `Move-ADDirectoryPartition`? Nein, für Verschiebungen innerhalb eines Forests verwenden Sie `Move-ADObject -TargetPath`. |
| **Wiederherstellen eines gelöschten Benutzers** | Verwenden Sie die AD-Tombstone-Frist (Standard 180 Tage) oder den Papierkorb. | Nicht Aktivierung des Papierkorns → Sie müssen eine vollständige Wiederherstellung über `ntdsutil` durchführen. Verwenden Sie den Papierkorb für die meisten täglichen Wiederherstellungen. |
| **Implementierung einer neuen Zertifikatsvorlage** | Verwenden Sie den AD-CS-Verwaltungs-UI, erstellen Sie die Vorlage, verteilen Sie sie, erstellen Sie Zertifikatsanforderungen. | Vergessen der `CAIssuance`-Berechtigungen; führen Sie `certutil -setreg caServer` für den Server aus. |
| **Konfiguration von Hybrididentitäten (Azure AD)** | Installieren Sie Azure AD Connect, wählen Sie das Kopplungsmodel (Pass-Through, PHS), fügen Sie Connectors hinzu. | Falsche DNS-Konfiguration → fehlende `_ldap://`-Anmeldeinformationen. Überprüfen Sie `Test-AdConnector` und `Get-ADForest` in Azure. |
| **Bereitstellung von Gruppenrichtlinien** | Erstellen Sie eine neue GPO in der OU, verknüpfen Sie sie, erzwingen Sie `block inheritance`, fügen Sie Benutzer/Gruppen hinzu. | Richtlinienänderung wird nicht angewendet aufgrund von `Security Filtering` (`Authenticated Users`). Entfernen Sie die unerwünschten Sicherheitsfilter. |
| **Überprüfung der privilegierbaren Zugriffe** | Verwenden Sie PowerShell: `Get-ADPrincipalPolicy -PrincipalIdentity <user> -PolicyClass All`. | Ignorieren von `managedServiceAccount`-Attributen. Aktivieren Sie `msDS-AllowedToActOnS` für SPN-Vertrauen. |

---

## 7. Praktische PowerShell-Snippets für den schnellen Überblick

```powershell
# Installieren Sie das RSAT-Modul (falls noch nicht geschehen)
Install-WindowsFeature -Name RSAT-AD-PowerShell

# Verbinden Sie sich mit einem Domänencontroller (optional)
$dc = "DC01.contoso.com"
$session = New-PSSession -ComputerName $dc -Credential (Get-Credential)

# Alle Domänencontroller für eine Domäne anzeigen
Get-ADDomainController -Filter * | Select-Object Name, Site, OperatingSystem, IsGlobalCatalog

# Anzeige der Replikationskonnektoren für einen DC
repadmin /showrepl $dc

# Erstellung einer neuen Sicherheitsgruppe in einer OU
New-ADGroup -Name "HR-Readers" -GroupScope DomainLocal `
    -Path "OU=Groups,DC=contoso,DC=com" `
    -Description "Kann HR-Datenbankberichte lesen"

# Zuweisung von Mitgliedern mithilfe von CSV (Laden Sie eine Datei members.csv mit den Spalten Name, Group)
$csv = Import-Csv C:\temp\members.csv
foreach($row in $csv){
    Add-ADGroupMember -Identity $row.Group -Members $row.Name
}

# Exportieren aller Benutzerkonten, die in den letzten 90 Tagen ablaufen (CSV)
$expiryDate = (Get-Date).AddDays(90)
Get-ADUser -Filter {AccountExpirationDate -like "*"} `
    -Properties AccountExpirationDate, EmailAddress, Department |
Where-Object {$_.AccountExpirationDate -le $expiryDate} |
Select-Object Name, EmailAddress, Department, AccountExpirationDate |
Export-Csv C:\temp\ExpiringUsers.csv -NoTypeInformation

# Anzeigen des resultierenden Satzes der Gruppenrichtlinie für einen Benutzer
$user = "jsmith"
$rsop = Get-RSOP -User $user
$rsop | Format-Table -Property Path, WMI

# Löschen eines gelöschten Benutzers (falls Sie ihn wiederherstellen möchten)
$deletedUser = Get-ADObject -Filter {Name -eq "jdoe"} -IncludeDeletedObject -Properties Name, LastKnownParent
Restore-ADObject -PrevServer $dc -Object $deletedUser
```

*Alle Befehle werden im interaktiven Modus von Windows PowerShell oder in einem Verwaltungs-Master einer Domäne (der die Berechtigung zum Ausführen von AD-Cmdlets benötigt) ausgeführt.*

---

## 8. Neue Funktionen und zukünftige Entwicklung

| Funktion | Was sie bietet | Wie sie AD verändert |
|---------|-----------------|------------------------|
| **Azure AD Domain Services (AAD DS)** | Vollständig verwaltete, domänenbeitretende Umgebung ohne Notwendigkeit eines eigenen DCs. | Ermöglicht die Migration von traditionellen Windows-Clients zu Cloud-Only ohne physische DCs. |
| **Pass-Through-Authentifizierung** | Streamlined-Kombinationsverfahren ohne Passwort-Hash. | Reduziert die Belastung durch Azure AD Connect und verbessert die Sicherheit. |
| **Schnelle Anmeldung** | Automatisierte einmalige Anmeldung (SSO) für benutzerdefinierte Apps. | Reduziert den manuellen Aufwand für die Zuordnung von Kennungen. |
| **Hybrid Azure AD Join** | Geräte können sich sowohl mit AAD als auch mit AD verbinden. | BietetFlexibilität für hybride Umgebungen. |
| **Integrierte RBAC für AD** | `Get-ADPermission`/`Set-ADPermission` für rollenspezifische Verwaltung. | Ermöglicht eine detailliertere Zugriffskontrolle auf administrative Aufgaben. |
| **Erweiterte Sign-in-Analyse über „Azure AD Conditional Access“** | Integration mit AD für bedingte Benutzeranmeldungsprozesse. | Zeigt an, wann MFA erzwungen werden soll, wenn ein Benutzer von einem „Risky-IP“-Bereich aus anmeldet. |

---

## 9. Kurzes Cheat Sheet für eine typische Bereitstellung (Beispiel)

1. **Installieren Sie den DNS-Domänendienst** (authoritativ für Ihre Domäne) und stellen Sie sicher, dass `Forwarders` angemessen konfiguriert sind.
2. **Installieren Sie ADDS** auf einem Server, der zu den Sites passt, und benennen Sie die Domäne auf `contoso.com` um.
3. **Installieren Sie Azure AD Connect** (`express`) für die Cloud-Identität; wählen Sie **Pass-Through-Authentifizierung** oder **PHS** basierend auf der Sicherheitsrichtlinie.
4. **Erstellen Sie die OU-Struktur** (`OU=Users`, `OU=Computers`, `OU=Servers`).
5. **Erstellen Sie die Gruppenrichtlinienobjekte** (Domänenrichtlinie, Sicherheitsrichtlinie, Softwareverteilung).
6. **Aktivieren Sie die AD-Zertifikatsdienste** zur Ausstellung interner Zertifikate.
7. **Aktivieren Sie den Active Directory Recycle Bin** (`Enable-ADOptionalFeature`).
8. **Implementieren Sie das Monitoring** (`Performance Monitor`, `Event Log`, `Azure Monitor` für AD DS-Ereignisse).
9. **Dokumentieren Sie wichtige Änderungen** in einem Update-Tracker (z. B. Confluence), erfassen Sie Änderungen am Schema und an den Vertrauensbeziehungen.

---

## 10. Kurze FAQ (am häufigsten gefragt)

| Frage | Kurze Antwort |
|----------|--------------|
| **Kann ich eine einzelne Domäne aus einem Forest entfernen?** | Nein – Sie können eine Domäne nicht entfernen, ohne den gesamten Forest zu löschen. Verwenden Sie `Remove-ADDomain` nach Abschluss der Bereinigung. |
| **Was ist der Unterschied zwischen `domaintrust` und `foresttrust`?** | Domänenvertrauen verknüpfen zwei Domänen miteinander; Forestvertrauen verknüpfen zwei Forests miteinander (und sind standardmäßig eingeschränkt). |
| **Wo werden Passwort-Hashes in AD gespeichert?** | In den `unicodepwattributes`-Attributen jedes Benutzerobjekts (`user` → `unicodePwd` – bereinigter Wert; `ntPwdHistory`, `lmPwdHistory`). |
| **Ich habe einen gelöschten AD-Benutzer verloren – kann ich ihn wiederherstellen?** | Wenn der Papierkorb aktiviert ist, können Sie den Benutzer wiederherstellen (obwohl Sie den Papierkorb wieder aktivieren müssen). Andernfalls sind Sie von der Tombstone-Löschfrist (180 Tage) abhängig und müssen eine vollständige Forest-Wiederherstellung verwenden. |
| **Kann ich AD auch über einen Proxy verwalten?** | Ja – konfigurieren Sie den Proxy für `Enter-PSSession` oder `Invoke-Command` oder verwenden Sie die `Active Directory Module` über den **Web-Powershell**-Endpunkt (`Get-PSSessionConfiguration`). |
| **Wie sehe ich die letzten veränderten Attribute für ein Objekt?** | Verwenden Sie `Get-ADObject -Identity <distinguishedName> -Properties * -Server <DC> | Select-Object *`. Die Filterung erfolgt über das `msDS-ChangesSince`-Attribut oder `Get-ADChange` (erfordert AD-Powershell 6+). |
| **Ich bin eine AD-„Owner“-berechtigt – was kann ich tun?** | Sie haben Kontrolle über den Sicherheitsdeskriptor für Objekte (z. B. Berechtigungen). Verwenden Sie `Get-Acl` und `Set-Acl`, um Berechtigungen zu ändern. |
| **Gibt es eine Möglichkeit, die LDAP-Abfrage zu beschleunigen?** | Verwenden Sie spezifische Attribute (`Properties`), vermeiden Sie `*` (ALL), verwenden Sie `Filter`, speichern Sie Abfragen, verwenden Sie `LDAP`-Bindungen über `GC://` für globale Katalogsuchen. |

---

## 11. Ressourcen und Weiterbildung

| Ressource | Link |
|----------|------|
| **Microsoft Docs – Active Directory** | https://learn.microsoft.com/en-us/windows-server/identity/ad-ds/ |
| **AD DS PowerShell Module Docs** | https://learn.microsoft.com/en-us/powershell/module/activedirectory/ |
| **Die Offizielle AD-Website von Microsoft** | https://learn.microsoft.com/en-us/windows-server/identity/ |
| **Blogs von Microsoft MVPs** (z. B. **Brian surgen** – „Active Directory Deep Dives“) | https://4sysops.com, https://www.a11.SysNetSecurity.de |
| **Best-Practice-Checkliste (Microsoft)** | https://learn.microsoft.com/en-us/previous-versions/windows/it-pro/windows-server-2012-r2-and-2012/hh770349(v=ws.11) |
| **PowerShell-Hacks** | „PowerShell für AD-Administratoren“ (David Mengel) – Kurs auf Pluralsight. |

---

### Abschließender Gedanke

Active Directory ist ein komplexes, aber äußerst leistungsstarkes Ökosystem, das Identitäten im gesamten Windows-Ökosystem ermöglicht (und auch Brücken zu Azure/AD-Benutzern schlägt). Eine solide Grundlagenkenntnis – Objekte, Replikation, Vertrauensbeziehungen, Schema – ermöglicht eine effektive Planung und Verwaltung. Behandeln Sie es **als Lebenszyklus** (Erstellung → Betrieb → Überwachung → Wiederherstellung), und Sie vermeiden viele der klassischen Fallstricke, die dazu führen, dass AD zu einer Schwachstelle und nicht zu einer Stärke wird.

Wenn Sie ein bestimmtes Szenario benötigen (z. B. ein Skript zur Bereitstellung von DCs, zur Konfiguration von Hybrididentitäten oder zur Behebung eines AD-Fails), teilen Sie mir einfach Ihre Anforderungen mit, und ich werde Ihnen eine konkrete Schritt-für-Schritt-Anleitung erstellen.
