# IT

> Lernblätter für IHK AP2 FiSi AO2020

**Hinweis:** Die Prüfungen selber + Antworten sind unter [https://github.com/Shroudmoa/IHK-Abschlusspruefungen-AP2-FiSi-AO2020](https://github.com/Shroudmoa/IHK-Abschlusspruefungen-AP2-FiSi-AO2020) erhältlich.

---

## Die Geschichte: Ein Server für Rheinblick

> **So liest du dieses Kapitel:** Die Geschichte folgt einem typischen AP2-Projekt von der ersten Anfrage bis zur Abnahme. Jede Station verlinkt auf die passenden Lernblätter. Die Tabellen weiter unten sind deine Schnellreferenz – die Geschichte zeigt dir, **wie die Themen in der Prüfung zusammenhängen**.

### Vorgeschichte: Der Anruf

Du bist im dritten Ausbildungsjahr bei der **Nordlicht Systemhaus GmbH**, einem kleinen Systemintegrator. An einem Montagmorgen klingelt das Telefon: Die **Rheinblick Logistik GmbH** braucht Hilfe. 120 Mitarbeiter, ein Bürogebäude, drei Lagerhallen und 30 Mitarbeiter im Homeoffice. Der alte Server läuft seit 2014, der Herstellersupport ist abgelaufen, gesichert wird auf eine einzelne USB-Festplatte, das WLAN ist offen und nächsten Monat steht ein **DSGVO-Audit** an.

Dein Ausbilder legt dir den Auftrag auf den Tisch und sagt nur zwei Sätze:

> „Du übernimmst die technische Planung. Aber erst rechnen, dann schrauben – und begründe jede Entscheidung so, dass ein Prüfer sie versteht."

Das ist dein Projekt. Und genau so läuft auch die AP2: Du bekommst eine Ausgangssituation, musst sie strukturieren, technisch lösen, wirtschaftlich bewerten und deine Entscheidung begründen.

### Station 1 – Projektauftrag: Erst planen, dann bestellen

Bevor du irgendeinen Server konfigurierst, zerlegst du den Auftrag in Arbeitspakete: Bestandsaufnahme, Anforderungsanalyse, Hardware, Netzwerk, Sicherheit, Migration, Schulung, Abnahme. Daraus entsteht ein **Projektstrukturplan (PSP)**. Die Arbeitspakete bekommen Abhängigkeiten und landen im **Netzplan**; der **kritische Pfad** zeigt dir, welche Verzögerung das ganze Projekt verschiebt. Für die Kundenkommunikation malst du einen **Gantt**-Balkenplan, für unsichere Zeiten nutzt du **PERT**, für die Personal- und Materialplanung den **Ressourcenplan** und für die Kosten die **Aufwandsschätzung** sowie die **AOB**.

Der Kunde will außerdem wissen: „Cloud oder eigener Server?" Du antwortest nicht mit Bauchgefühl, sondern mit Zahlen. **CAPEX** (einmalige Investition) gegen **OPEX** (laufende Betriebskosten), dazu **TCO** über fünf Jahre, **Amortisation** und **Break-Even-Point**. Und weil der Kunde eine Verfügbarkeit von 99,9 % verlangt, liest du genau nach, was im **SLA** steht – und was eben nicht.

> **Lernblätter dieser Station:** [Projektstrukturplan (PSP)](IT/projektstruckturplan-psp.md) · [Netzplan](IT/netzplan.md) · [Gantt-Diagramm](IT/gantt-diagramm.md) · [PERT](IT/pert.md) · [Aufwandsschätzung](IT/aufwandsschaetzung.md) · [Ressourcenplan](IT/ressourcenplan.md) · [AOB](IT/aob.md) · [Soll-Ist-Vergleich](IT/soll-ist-vergleich.md) · [Stundensatz intern](IT/stundensatz-intern.md) · [CAPEX-OPEX](IT/capex-opex.md) · [TCO](IT/tco.md) · [Amortisation](IT/amortisation.md) · [BEP](IT/bep.md) · [SLA](IT/sla.md)

**Prüfungsbrücke:** *Warum kommt der PSP vor dem Netzplan?* – Der PSP definiert erst die Arbeitspakete; der Netzplan braucht diese Pakete, um Dauer und Abhängigkeiten darzustellen. Ohne PSP kein belastbarer kritischer Pfad.

### Station 2 – Was muss eigentlich geschützt werden?

Bei der Bestandsaufnahme siehst du sofort die Probleme: offenes WLAN, ein einziges Admin-Passwort für alle, keine Verschlüsselung, kein Konzept für Homeoffice. Du beginnst nicht mit Technik, sondern mit den **Schutzzielen**: Vertraulichkeit, Integrität, Verfügbarkeit – kurz **CIA**. Jede spätere Maßnahme musst du einem dieser Ziele zuordnen können.

Dann kommt der Datenschutz. Welche personenbezogenen Daten liegen im Lagerverwaltungssystem? Wer darf sie sehen? Wie lange? Du leitest die **TOMs** nach Art. 32 DSGVO ab, klärst die Rolle des **Datenschutzbeauftragten** und weißt: Eine **Datenpanne** muss innerhalb von 72 Stunden gemeldet werden. Für die Lagerkameras prüfst du **Video- und Fotoformate**, **Auflösung und Speicherbedarf** und Begriffe wie **PoE** und **Vandalismusschutz**. Homeoffice und private Smartphones bringst du über **MDM und BYOD**-Regeln in den Griff. Und weil der Kunde „KI für die Lagerprognose" ins Gespräch bringt, vergleichst du den Nutzen mit klassischer Automatisierung – inklusive Risiken.

> **Lernblätter dieser Station:** [Schutzziele](IT/schutzziele.md) · [DSGVO & TOM](IT/dsgvo-tom.md) · [TOMs](IT/toms.md) · [Datenschutzbeauftragter](IT/dbs-datenschutzbeauftragten.md) · [Datenpanne & SCC](IT/scc-datenpanne.md) · [MDM & BYOD](IT/mdm-byod.md) · [KI in der IT](IT/ki-in-it-vergleich-mit-klassisch.md) · [OS-Sicherheitsmerkmale](IT/nice-to-know/OS-sicherheitsmerkmale.md) · [Hardware- und Netzwerkbegriffe](IT/nice-to-know/begriffe.md) · [Video- und Fotoformate](IT/nice-to-know/video-photo-formats.md) · [Auflösung & Speicherbedarf](IT/nice-to-know/Resoulotion.md)

**Prüfungsbrücke:** *Ein Kameraausfall – welches Schutzziel ist betroffen?* – In der Regel die **Verfügbarkeit**. Fällt die Aufzeichnung aus, fehlt der Nachweis. Erst wenn Aufnahmen abfließen, kommt **Vertraulichkeit** dazu. Immer sauber trennen und begründen.

### Station 3 – Die Hardware: Rechnen statt raten

Jetzt dimensionierst du den Server. Du schaust auf **CPU** (Kerne, Threads, Takt, Cache), **Speicherhierarchie** und **Festplatten**: HDD, SSD oder NVMe? Dazu **Schnittstellen und Kabel**, und du rechnest sauber mit **Bit und Byte** – denn genau dort verlierst du in der Prüfung die einfachsten Punkte.

Der Server bekommt lokalen Speicher und ein zentrales **NAS** für Dateien; für die Virtualisierung planst du ein **SAN** oder zumindest zentralen Block-Storage. Für die Datenplatten wählst du **RAID 10** für die Datenbank und **RAID 6** für das Backup-Ziel. Du kennst die nutzbare Kapazität, die Ausfalltoleranz und die Rechenwege. Beim **RAID-Controller** achtest du auf Cache, Cache-Schutz und Transferrate – genau die Merkmale, die in Übungsaufgaben gefragt sind.

Weil ein Server allein kein **HA** ist, planst du Redundanz: zwei Netzteile, zwei Switches, **Cluster** und einen **Load Balancer** für die Webanwendung. Die USV schützt vor kurzen Ausfällen, das Notstromkonzept vor langen. Und bei der Entsorgung der alten Hardware denkst du an **Umweltschutz** und **Arbeitsschutz** – Akkus, Kältemittel, schwere Racks.

> **Lernblätter dieser Station:** [CPU](IT/cpu.md) · [Speicherhierarchie](IT/speicherhirachie.md) · [USB, SSD, HDD](IT/usb-ssd-hdd.md) · [Bit & Byte](IT/bit-byte.md) · [DAS-NAS-SAN](IT/das-nas-san.md) · [RAID](IT/raid.md) · [Übungsaufgabe RAID-Controller](IT/fragen/raid.md) · [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md) · [Cluster](IT/cluster.md) · [Load Balancer](IT/load-balancer.md)

**Prüfungsbrücke:** *Vier Platten à 8 TB im RAID 5 – wie viel ist nutzbar?* – (n − 1) × Plattengröße = 3 × 8 TB = **24 TB**. Beim RAID 6 sind es (n − 2) × Größe = **16 TB**. Immer erst Level, dann Formel, dann Einheit hinschreiben.

### Station 4 – Betriebssystem, Boot und Härtung

Die Server bekommen ein aktuelles Linux und Windows Server. Du prüfst **BIOS/UEFI**, aktivierst **Secure Boot** und nutzt **TPM 2.0** für **BitLocker**. Du verstehst die Bootkette: UEFI → Bootloader → Kernel → Userspace. Nur so kannst du später erklären, warum Secure Boot eine manipulierte Boot-Partition erkennt.

Danach wird gehärtet: unnötige Dienste aus, **Least Privilege**, starke Authentifizierung, Host-Firewall an, **Patchmanagement** mit getrennten Test-, Staging- und Produktionsumgebungen (TU/SU/PU). Auf den Linux-Servern setzt du Dateirechte mit **chmod** – 750 für Verzeichnisse, 640 für Konfigurationsdateien, niemals 777 „weil es dann geht". Die Sicherheitsmerkmale des Betriebssystems (Benutzerkonten, Rechte, Updates, Logging) dokumentierst du für das Audit.

> **Lernblätter dieser Station:** [BIOS-UEFI](IT/bios-uefi.md) · [BIOS/UEFI & Kernel](IT/bios-uefi-kernel.md) · [OS-Härtung](IT/os-haertung.md) · [Patchmanagement TU/SU/PU](IT/patchmanagment-tu-su-pu.md) · [chmod](IT/chmod.md) · [BitLocker](IT/bitlocker.md)

**Prüfungsbrücke:** *Warum braucht BitLocker UEFI und TPM?* – Das TPM speichert den Schlüssel hardwaregebunden und misst den Bootvorgang. Ändert sich etwas, verweigert es die Freigabe. Secure Boot verhindert zusätzlich, dass unsignierte Bootloader starten.

### Station 5 – Virtualisierung und Cloud

Der Kunde braucht nicht fünf einzelne Server, sondern eine virtuelle Umgebung. Du vergleichst **Hypervisor Typ 1** (produktiv, direkt auf der Hardware) und **Typ 2** (Test/Entwicklung). Du entscheidest dich für einen Typ-1-Cluster mit drei Hosts. Darauf laufen VMs für Active Directory, Fileserver, Datenbank und die Lageranwendung. Die Webanwendung wird als **Container** betrieben – dichter, schneller gestartet, aber mit geteiltem Kernel.

**Live-Migration** erlaubt dir Wartung ohne Ausfall. **Skalierbarkeit** (Scale-Up/Out) und **Elastizität** (automatisches Hoch- und Runterfahren) planst du für Spitzen im Weihnachtsgeschäft. Für Updates der Anwendung nutzt du **Blue-Green Deployment**: grün wird getestet, dann schaltet der Load Balancer um, bei Problemen zurück auf blau.

Die Cloud prüfst du als Option: **IaaS/PaaS/SaaS** und die weiteren Modelle, Public/Private/Hybrid. Am Ende bleibt die Kernanwendung lokal, das Backup und die Testumgebung gehen in die Cloud. Wirtschaftlich ist das nur, weil du **CAPEX/OPEX** und **TCO** sauber verglichen hast. Bei den Lizenzen klärst du **User- und Device-CALs**, bevor du bestellst.

> **Lernblätter dieser Station:** [Virtualisierung](IT/virtualisierung.md) · [Hypervisor-Typen](IT/hypervisor-typen.md) · [VM, Container, Server](IT/vm-container-server.md) · [Live-Migration](IT/live-migration-vms.md) · [Cloud Computing](IT/cloud-computing.md) · [Skalierbarkeit & Elastizität](IT/skalierbarkeit-elastizitaet.md) · [Blue-Green Deployment](IT/blue-green-deployment.md) · [Cluster](IT/cluster.md) · [Load Balancer](IT/load-balancer.md) · [Lizenzierung](IT/analyse-der-exams/10_Lizenzierung.md) · [Virtualisierung in der Prüfung](IT/analyse-der-exams/L13_Virtualisierung.md)

**Prüfungsbrücke:** *VM oder Container?* – Die VM virtualisiert Hardware über einen Hypervisor und bringt einen eigenen Kernel mit. Der Container virtualisiert das Betriebssystem und teilt den Kernel des Hosts. Deshalb ist der Container leichter, aber weniger isoliert.

### Station 6 – Das Netzwerk: Vom OSI-Modell zur Verkabelung

Jetzt wird es ernst. Du zeichnest das Netz und nutzt das **OSI-Modell** als Landkarte: Kabel und WLAN auf Schicht 1, **MAC** und Switching auf Schicht 2, **IPv4/IPv6** und **Routing** auf Schicht 3, **TCP/UDP und Ports** auf Schicht 4, die Anwendungen darüber. So findest du später jeden Fehler systematisch.

Du planst **VLANs**: 10 Büro, 20 Lager, 30 Server, 40 Gäste, 50 Management. Der Switch macht **Access**-Ports für Endgeräte und **Trunk**-Ports zum Router; **802.1Q** taggt die Frames. Für die Lagerhalle brauchst du **WLAN** mit **WPA3-Enterprise** und **802.1X**; Gäste kommen in ein eigenes VLAN ohne Zugriff aufs Firmennetz.

Die Adressierung rechnest du durch: Büro `192.168.10.0/24` (254 Hosts), Lager `192.168.20.0/23` (510 Hosts), Server `192.168.30.0/28` (14 Hosts). **DHCP** verteilt die Adressen per **DORA**, **DNS** löst Namen auf, **NAT/PAT** übersetzt den Internetzugang. Der **Router** routet zwischen den VLANs, statische Routen für die Filiale, OSPF dort, wo es wächst. **IPv6** planst du gleich mit – **SLAAC**, **NDP**, kein NAT als Sicherheitsersatz.

Nach außen schützt eine **Firewall** mit Zonen und **DMZ**; der **Proxy** filtert und cached; **VPN** bringt die Homeoffice-Mitarbeiter sicher ins Netz. Gegen **DDoS** auf den Webshop helfen Load Balancer, Rate Limits und ein upstream-Dienst. Und wenn doch etwas klemmt, arbeitest du mit **ping**, **traceroute**, **nslookup** und **Wireshark** – erst Schicht für Schicht, dann Theorie.

> **Lernblätter dieser Station:** [OSI](IT/osi.md) · [MAC](IT/mac.md) · [Netzwerktypen](IT/netzwerktypen.md) · [IPv4](IT/ipv4.md) · [Subnetting IPv4 & IPv6](IT/subnetting-ipv4-ipv6.md) · [IPv6](IT/ipv6.md) · [DHCP](IT/dhcp.md) · [DNS](IT/dns.md) · [Ports](IT/ports.md) · [Routing](IT/routing.md) · [Switches & VLANs](IT/switches-und-vlans.md) · [VLAN](IT/vlan.md) · [WLAN](IT/wlan.md) · [Firewall](IT/firewall.md) · [Firewall-Generationen & Antivirus](IT/firewalls-alle-generationen-sicherheitskonzepte-antivirus.md) · [Proxy](IT/proxy.md) · [NAT & PAT](IT/nat-pat.md) · [VPN](IT/vpn.md) · [DDoS](IT/ddos.md) · [Netzwerkanalyse & Troubleshooting](IT/netzwerkanalyse-und-troubleshooting.md) · [Load Balancer](IT/load-balancer.md) · [IPv4 & Routing in der Prüfung](IT/analyse-der-exams/06_IPv4_Adressierung_Routing.md) · [Netzwerk-Fehlersuche](IT/analyse-der-exams/07_Netzwerk_Fehlersuche.md) · [IPv6 in der Prüfung](IT/analyse-der-exams/08_IPv6.md) · [WLAN in der Prüfung](IT/analyse-der-exams/09_WLAN.md) · [VLAN & Routing](IT/analyse-der-exams/L15_VLAN_Routing.md) · [Firewall](IT/analyse-der-exams/L16_Firewall.md) · [DHCP & DNS](IT/analyse-der-exams/L17_DHCP_DNS.md) · [Netzwerk-Berechnungen](IT/analyse-der-exams/L18_Netzwerk_Berechnungen.md)

**Prüfungsbrücke:** *Wie viele Hosts hat ein /26?* – 2⁶ − 2 = **62**. Zieh die Netz- und Broadcastadresse ab. Bei IPv6 gibt es keine Broadcastadresse – dort rechnest du mit 2⁶⁴ Adressen pro /64 und denkst an SLAAC.

### Station 7 – Identität und Rechte

Die Mitarbeiter brauchen Konten. Du baust **Active Directory** mit **LDAP** als Verzeichnisstandard, **Kerberos** für die Anmeldung und **GPOs** für zentrale Einstellungen. Rechte vergibst du über **RBAC**: Rollen wie „Lager", „Buchhaltung", „IT-Admin" – nicht über einzelne Benutzer. Es gilt **Least Privilege**: Der Lager-Mitarbeiter braucht keine Domänen-Admin-Rechte, nur weil er ein neues Notebook bekommt.

Für Homeoffice und Admins aktivierst du **MFA**. Anmeldungen, Änderungen und Zugriffe landen im **AAA/SIEM**; dort fallen Brute-Force-Versuche und ungewöhnliche Login-Zeiten auf. Beim Offboarding deaktivierst du das Konto sofort, entziehst Gruppen und prüfst die Geräte. Genau diese Nachvollziehbarkeit will der Prüfer hören.

> **Lernblätter dieser Station:** [LDAP](IT/ldap.md) · [Active Directory](IT/active-directory.md) · [Kerberos](IT/kerberos.md) · [GPO](IT/gpo.md) · [RBAC](IT/rbac.md) · [Least Privilege](IT/least-privlage.md) · [Authentifizierung, Autorisierung & MFA](IT/authentifizierung-autoritisierung-mfa.md) · [AAA & SIEM](IT/aaa-siem.md) · [chmod](IT/chmod.md)

**Prüfungsbrücke:** *Authentifizierung oder Autorisierung?* – Authentifizierung beantwortet „Wer bist du?", Autorisierung „Was darfst du?". MFA gehört zur Authentifizierung, RBAC zur Autorisierung, Logging/Accounting zur Nachvollziehbarkeit.

### Station 8 – Kryptografie im Alltag

Der Kunde will verschlüsselte E-Mails und eine sichere Anmeldung am Webshop. Du erklärst den Unterschied zwischen **symmetrischer** und **asymmetrischer** Verschlüsselung: symmetrisch ist schnell (AES) und schützt die Daten, asymmetrisch (RSA/ECC) löst das Schlüsselaustauschproblem. **TLS** kombiniert beides – und genau das ist eine beliebte Prüfungsfrage.

Für die internen Dienste baust du eine **PKI** mit Root-CA und ausstellender CA auf. Zertifikate für Intranet, VPN und Server werden darüber verwaltet. **Hashfunktionen** (SHA-256) sichern die Integrität, **PGP** verschlüsselt E-Mails Ende-zu-Ende und signiert sie. **BitLocker** schützt die Notebooks, **VPN** mit IPSec oder TLS schützt die Leitung ins Homeoffice. Du weißt: Verschlüsselung ersetzt keine Firewall, und ein Zertifikat ist kein Backup.

> **Lernblätter dieser Station:** [Hash](IT/hash.md) · [Symmetrisch & asymmetrisch](IT/symmetrische-und-asymmetrische.md) · [PKI](IT/pki.md) · [PGP](IT/pgp.md) · [BitLocker](IT/bitlocker.md) · [VPN](IT/vpn.md) · [PKI, TLS & VPN in der Prüfung](IT/analyse-der-exams/L27_PKI_TLS_Verschluesselung_VPN.md)

**Prüfungsbrücke:** *Warum nutzt TLS nicht nur asymmetrische Kryptografie?* – Asymmetrisch ist rechenintensiv. TLS nutzt es nur für den Schlüsselaustausch und die Authentifizierung; die eigentlichen Daten werden symmetrisch verschlüsselt. Hashfunktionen sichern die Integrität.

### Station 9 – Backup, Storage und der Ernstfall

Der alte Server hatte „Backup" auf einer USB-Platte. Du baust das neu: **3-2-1-1-0** – drei Kopien, zwei Medien, eine extern, eine offline/unveränderlich, null Fehler bei der Wiederherstellung. Du unterscheidest **Voll-, differenzielle und inkrementelle** Sicherung, planst die Aufbewahrung und testest die **Restore**-Zeit. **Snapshots** sind praktisch für Updates, aber **kein Backup** – sie liegen auf demselben System.

**RAID** schützt gegen Plattenausfall, nicht gegen Löschen, Ransomware oder Standortschaden. Deshalb kommen **NAS/SAN**, externe Medien und ein Cloud-Ziel dazu. Du definierst **RTO** (Wie lange darf die Wiederherstellung dauern?) und **RPO** (Wie viel Datenverlust ist akzeptabel?). Für die Buchhaltung planst du ein **Cluster** mit **HA** und einen **Load Balancer**; die **SLA**-Werte prüfst du gegen die echten Messungen.

> **Lernblätter dieser Station:** [Snapshots & Backups](IT/snapshots-backups.md) · [RAID](IT/raid.md) · [DAS-NAS-SAN](IT/das-nas-san.md) · [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md) · [Cluster](IT/cluster.md) · [SLA](IT/sla.md) · [Backup-Strategien](IT/analyse-der-exams/L12_Backup_Strategien.md) · [Speicher, Backup & RAID](IT/analyse-der-exams/02_Speicher_Backup_RAID.md) · [Verfügbarkeit & Ausfallsicherheit](IT/analyse-der-exams/03_Verfuegbarkeit_Ausfallsicherheit.md)

**Prüfungsbrücke:** *RTO und RPO?* – RTO ist die maximale Wiederherstellungszeit, RPO der maximale Datenverlust seit der letzten Sicherung. Ein RAID 1 hat RTO nahe null bei Plattenausfall, aber RPO ungleich null bei versehentlichem Löschen – deshalb braucht es beides.

### Station 10 – Die Anwendung und ihre Daten

Die Lagerverwaltung soll einen neuen Report bekommen: durchschnittlicher Bestand, höchster Bestand, sortierte Liste. Du schreibst **Pseudocode**, testest mit einem **Schreibtischtest** und findest den klassischen Fehler `i <= arr.Length`. Danach geht es an die Datenbank: Tabellen für Artikel, Lager und Bewegungen. Du entwirfst das Schema, schreibst **DDL** und **DML**, fragst mit **SELECT**, **GROUP BY** und **JOIN** ab und kennst den Unterschied zwischen **INNER** und **LEFT JOIN**.

Für die Dokumentation zeichnest du **UML**: **Klassendiagramm** für die Domäne, **Use-Case** für die Akteure, **Aktivitätsdiagramm** für den Wareneingang, **Sequenzdiagramm** für die Buchung. Den Geschäftsprozess „Wareneingang" modellierst du zusätzlich in **BPMN**, weil der Kunde ihn fachlich lesen will. Am Ende prüfst du das ER-Modell auf Normalisierung und die Abfragen auf NULL-Fallen.

> **Lernblätter dieser Station:** [Pseudocode: Einleitung](IT/Pseudocode/00_Einleitung.md) · [Grundlagen & Syntax](IT/Pseudocode/01_Grundlagen_Syntax.md) · [Arrays, Mittelwert, Max](IT/Pseudocode/02_Arrays_Mittelwert_Max.md) · [Schleifen & Fehlersuche](IT/Pseudocode/03_Schleifen_Fehlersuche.md) · [Praxistests & Musterlösungen](IT/Pseudocode/04_Praxistests_Musterloesungen.md) · [SQL: Übersicht](IT/sql/readme.md) · [SQL-Grundlagen](IT/sql/grundlagen.md) · [DDL & DML](IT/sql/ddl-dml.md) · [SQL-Joins](IT/sql/joins.md) · [UML: Übersicht](IT/UML/readme.md) · [Klassendiagramm](IT/UML/Klassendiagram.md) · [Use-Case-Diagramm](IT/UML/Anwendungsfalldiagramm-Use-Case-Diagramm.md) · [Aktivitätsdiagramm](IT/UML/Aktivitätsdiagramm.md) · [Sequenzdiagramm](IT/UML/Sequenzdiagramm) · [BPMN](IT/UML/BPMN.md) · [Programmierung in der Prüfung](IT/analyse-der-exams/04_Programmierung_Grundlagen.md) · [Datenmodellierung, SQL & UML](IT/analyse-der-exams/L14_Datenmodellierung_SQL_UML.md)

**Prüfungsbrücke:** *INNER JOIN oder LEFT JOIN?* – INNER liefert nur Treffer in beiden Tabellen. LEFT liefert alle Zeilen der linken Tabelle, auch ohne Partner – dort erscheint dann NULL. Genau danach wird in Aufgaben zur Datenmodellierung gefragt.

### Station 11 – Der Mensch im Projekt

Technik ist nur die halbe Prüfung. Das Projekt hat ein Team, einen Zeitplan und ein Budget. Du kennst die **Sozialversicherung**, das **Betriebsrat**-Mitbestimmungsrecht (besonders bei Leistungs- und Verhaltenskontrolle), die **Unternehmensorganisation** und die **Kostenrechnung**. Bei Verträgen und Steuern weißt du, wann du die Fachabteilung holst.

Im Lager achtest du auf **Arbeitsschutz**, **Brandschutz** und **Erste Hilfe**; bei der Entsorgung auf **Umweltschutz und Nachhaltigkeit**. Du ordnest das Unternehmen den **Wirtschaftssektoren** zu und verstehst **Konjunktur** und **Marktformen**, weil der Kunde im Logistikmarkt unterwegs ist. Die WiSo-Themen vertiefst du im [Wirtschafts-Lerntree](Wirtschaft.md).

> **Lernblätter dieser Station:** [Sozialversicherung](IT/analyse-der-exams/L19_Sozialversicherung.md) · [Betriebsrat & Mitbestimmung](IT/analyse-der-exams/L20_Betriebsrat_Mitbestimmung.md) · [Unternehmensorganisation](IT/analyse-der-exams/L21_Unternehmensorganisation.md) · [Kostenrechnung](IT/analyse-der-exams/L22_Kostenrechnung.md) · [Recht & Steuern](IT/analyse-der-exams/L23_WISO_Recht_Steuern.md) · [Arbeitsschutz, Brandschutz & Erste Hilfe](IT/analyse-der-exams/L24_Arbeitsschutz_Brandschutz_ErsteHilfe.md) · [Umweltschutz & Nachhaltigkeit](IT/analyse-der-exams/L25_Umweltschutz_Nachhaltigkeit.md) · [Wirtschaftssektoren, Konjunktur & Marktformen](IT/analyse-der-exams/L26_Wirtschaftssektoren_Konjunktur_Marktformen.md) · [WiSo-Grundlagen](IT/analyse-der-exams/11_WiSo_Grundlagen.md)

**Prüfungsbrücke:** *Der Kunde will Login-Zeiten auswerten – was sagst du?* – Das ist eine Leistungs- und Verhaltenskontrolle. Der Betriebsrat muss mitbestimmen; außerdem brauchst du eine Rechtsgrundlage nach DSGVO und musst Zweckbindung und Datenminimierung beachten.

### Station 12 – Generalprobe: Prüfungssimulation

Das Projekt ist abgenommen. Jetzt bereitest du dich auf die echte AP2 vor – mit den **Analysen der Altprüfungen 2021–2026** und den Übungsfragen. Du arbeitest zuerst die 🔴-Themen ab, dann 🟡, und simulierst pro Tag eine komplette Prüfung: GA1, GA2 und WiSo. Anschließend vergleichst du mit den Musterlösungen und schließt die Lücken.

Für GA1 wiederholst du **Cloud**, **Speicher/Backup/RAID**, **Verfügbarkeit**, **Programmierung**, **Datenschutz** und die **GA1-Sonstiges**-Themen. Für GA2 sind **IPv4/Routing**, **Fehlersuche**, **IPv6**, **WLAN**, **VLAN**, **Firewall**, **DHCP/DNS** und **Netzwerkberechnungen** Pflicht. Für WiSo kommen die Lektionen 19–26 dazu. Mit **[Abkürzungen](IT/abkuerzungen.md)** und **[Fachbegriffen](IT/fachbegriffe.md)** schärfst du die Sprache; die **AP1-Kann-Listen** nutzt du als Grundlagen-Check.

> **Lernblätter dieser Station:** [Prüfungsanalyse: Überblick](IT/analyse-der-exams/README.md) · [Index & Prüfungsstruktur](IT/analyse-der-exams/00_Index.md) · [Cloud Computing](IT/analyse-der-exams/01_Cloud_Computing.md) · [Speicher, Backup & RAID](IT/analyse-der-exams/02_Speicher_Backup_RAID.md) · [Verfügbarkeit & Ausfallsicherheit](IT/analyse-der-exams/03_Verfuegbarkeit_Ausfallsicherheit.md) · [Programmierung](IT/analyse-der-exams/04_Programmierung_Grundlagen.md) · [Datenschutz & Informationssicherheit](IT/analyse-der-exams/05_Datenschutz_Informationssicherheit.md) · [IPv4 & Routing](IT/analyse-der-exams/06_IPv4_Adressierung_Routing.md) · [Netzwerk-Fehlersuche](IT/analyse-der-exams/07_Netzwerk_Fehlersuche.md) · [IPv6](IT/analyse-der-exams/08_IPv6.md) · [WLAN](IT/analyse-der-exams/09_WLAN.md) · [Lizenzierung](IT/analyse-der-exams/10_Lizenzierung.md) · [WiSo-Grundlagen](IT/analyse-der-exams/11_WiSo_Grundlagen.md) · [L12 Backup-Strategien](IT/analyse-der-exams/L12_Backup_Strategien.md) · [L13 Virtualisierung](IT/analyse-der-exams/L13_Virtualisierung.md) · [L14 Datenmodellierung, SQL & UML](IT/analyse-der-exams/L14_Datenmodellierung_SQL_UML.md) · [L15 VLAN & Routing](IT/analyse-der-exams/L15_VLAN_Routing.md) · [L16 Firewall](IT/analyse-der-exams/L16_Firewall.md) · [L17 DHCP & DNS](IT/analyse-der-exams/L17_DHCP_DNS.md) · [L18 Netzwerk-Berechnungen](IT/analyse-der-exams/L18_Netzwerk_Berechnungen.md) · [L19 Sozialversicherung](IT/analyse-der-exams/L19_Sozialversicherung.md) · [L20 Betriebsrat & Mitbestimmung](IT/analyse-der-exams/L20_Betriebsrat_Mitbestimmung.md) · [L21 Unternehmensorganisation](IT/analyse-der-exams/L21_Unternehmensorganisation.md) · [L22 Kostenrechnung](IT/analyse-der-exams/L22_Kostenrechnung.md) · [L23 WiSo Recht & Steuern](IT/analyse-der-exams/L23_WISO_Recht_Steuern.md) · [L24 Arbeitsschutz, Brandschutz & Erste Hilfe](IT/analyse-der-exams/L24_Arbeitsschutz_Brandschutz_ErsteHilfe.md) · [L25 Umweltschutz & Nachhaltigkeit](IT/analyse-der-exams/L25_Umweltschutz_Nachhaltigkeit.md) · [L26 Wirtschaftssektoren, Konjunktur & Marktformen](IT/analyse-der-exams/L26_Wirtschaftssektoren_Konjunktur_Marktformen.md) · [L27 PKI, TLS, Verschlüsselung & VPN](IT/analyse-der-exams/L27_PKI_TLS_Verschluesselung_VPN.md) · [L28 GA1-Sonstiges](IT/analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md) · [L29 GA2-Sonstiges](IT/analyse-der-exams/L29_GA2_Sonstiges_Praxisthemen.md) · [Abkürzungen](IT/abkuerzungen.md) · [Fachbegriffe](IT/fachbegriffe.md)

**Übungsfragen GA1:** [Winter 2021](IT/fragen/ga1/ga1-winter-2021.md) · [Sommer 2022](IT/fragen/ga1/ga1-sommer-2022.md) · [Winter 2022](IT/fragen/ga1/ga1-winter-2022.md) · [Sommer 2023](IT/fragen/ga1/ga1-sommer-2023.md) · [Winter 2023](IT/fragen/ga1/ga1-winter-2023.md) · [Sommer 2024](IT/fragen/ga1/ga1-sommer-2024.md) · [Winter 2024](IT/fragen/ga1/ga1-winter-2024.md) · [Sommer 2025](IT/fragen/ga1/ga1-sommer-2025.md) · [Winter 2025](IT/fragen/ga1/ga1-winter-2025.md) · [Sommer 2026](IT/fragen/ga1/ga1-sommer-2026.md)

**Übungsfragen GA2:** [Winter 2021](IT/fragen/ga2/ga2-winter-2021.md) · [Sommer 2022](IT/fragen/ga2/ga2-sommer-2022.md) · [Winter 2022](IT/fragen/ga2/ga2-winter-2022.md) · [Sommer 2023](IT/fragen/ga2/ga2-sommer-2023.md) · [Winter 2023](IT/fragen/ga2/ga2-winter-2023.md) · [Sommer 2024](IT/fragen/ga2/ga2-sommer-2024.md) · [Winter 2024](IT/fragen/ga2/ga2-winter-2024.md) · [Sommer 2025](IT/fragen/ga2/ga2-sommer-2025.md) · [Winter 2025](IT/fragen/ga2/ga2-winter-2025.md) · [Sommer 2026](IT/fragen/ga2/ga2-sommer-2026.md)

**AP1-Grundlagencheck:** [LF1](IT/nice-to-know/ap1/lf1.md) · [LF2](IT/nice-to-know/ap1/lf2.md) · [LF3](IT/nice-to-know/ap1/lf3.md) · [LF4](IT/nice-to-know/ap1/lf4.md) · [LF5](IT/nice-to-know/ap1/lf5.md) · [LF6](IT/nice-to-know/ap1/lf6.md)

### Epilog: Deine Checkliste für die Prüfung

1. **Lesen:** Aufgabe zweimal lesen, Operatoren markieren (nennen, beschreiben, erläutern, berechnen).
2. **Strukturieren:** Erst die Lösung skizzieren, dann schreiben – Anzahl der geforderten Punkte einhalten.
3. **Rechnen:** Einheiten, Bit/Byte, 2ⁿ-Regeln und RAID-Formeln hinschreiben; Zwischenergebnisse runden.
4. **Begründen:** Jede technische Entscheidung mit Schutzziel, Kosten oder Betrieb verbinden.
5. **Verknüpfen:** Immer die Nachbarthemen mitdenken – RAID ≠ Backup, NAT ≠ Firewall, Authentifizierung ≠ Autorisierung.
6. **Zeit:** pro Aufgabe grob 20 Minuten, Rest für Kontrolle; keine Aufgabe leer lassen.

Wenn du diese Geschichte einmal komplett durchgearbeitet hast, hast du alle Lernblätter unter `IT/` mindestens einmal angefasst – und genau das ist der Unterschied zwischen „irgendwas auswendig gelernt" und „die Prüfung verstanden".

---

## Grundlagen & Hardware (9)

| Thema | In einem Satz | Verknüpfung |
|---|---|---|
| [CPU](IT/cpu.md) | Kerne, Threads, Takt, Cache – warum GHz allein nichts aussagt. | [Speicherhirachie](IT/speicherhirachie.md), [Virtualisierung](IT/virtualisierung.md) |
| [USB-SSD-HDD](IT/usb-ssd-hdd.md) | Schnittstellen, Geschwindigkeit und Einsatz von HDD, SSD und NVMe. | [Speicherhirachie](IT/speicherhirachie.md), [DAS-NAS-SAN](IT/das-nas-san.md) |
| [Bit-Byte](IT/bit-byte.md) | Bit, Byte, KB vs. KiB – die Umrechnungsfalle in Aufgaben. | [Abkürzungen](IT/abkuerzungen.md) |
| [Speicherhirachie](IT/speicherhirachie.md) | Register bis Band: Geschwindigkeit, Kosten, Flüchtigkeit. | [CPU](IT/cpu.md), [USB-SSD-HDD](IT/usb-ssd-hdd.md) |
| [chmod](IT/chmod.md) | rwx, Oktalzahlen und die 755/644-Frage. | [Least Privilege](IT/least-privlage.md), [OS-Härtung](IT/os-haertung.md) |
| [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md) | RTO/RPO, USV-Typen, HA vs DR, SPOF vermeiden. | [RAID](IT/raid.md), [Snapshots-Backups](IT/snapshots-backups.md), [Load Balancer](IT/load-balancer.md), [Cluster](IT/cluster.md) |
| [LDAP](IT/ldap.md) | Lightweight Directory Access Protocol für zentrale Verzeichnisdienste. | [Active-Directory](IT/active-directory.md), [Authentifizierung-Autoritisierung-MFA](IT/authentifizierung-autoritisierung-mfa.md), [Kerberos](IT/kerberos.md) |
| [BIOS-UEFI](IT/bios-uefi.md) | Bootvorgang, Secure Boot, Firmware vs UEFI. | [BIOS/UEFI-Kernel](IT/bios-uefi-kernel.md), [Bitlocker](IT/bitlocker.md) |
| [Virtualisierung](IT/virtualisierung.md) | Grundlagen der Virtualisierung, Vorteile/Nachteile. | [Hypervisor-Typen](IT/hypervisor-typen.md), [VM-Container-Server](IT/vm-container-server.md) |

## Netzwerk (23)

| Thema | In einem Satz | Verknüpfung |
|---|---|---|
| [OSI](IT/osi.md) | Die 7 Schichten und wo Switch, Router und Proxy hingehören. | [MAC](IT/mac.md), [IPv4](IT/ipv4.md), [Ports](IT/ports.md), [Firewall](IT/firewall.md), [Proxy](IT/proxy.md) |
| [MAC](IT/mac.md) | 48 Bit, OUI, ARP – und warum MAC-Filter keine Sicherheit sind. | [OSI](IT/osi.md), [Netzwerktypen](IT/netzwerktypen.md), [Switches und VLANs](IT/switches-und-vlans.md) |
| [Netzwerktypen](IT/netzwerktypen.md) | PAN bis GAN und welche Risiken pro Größe dazukommen. | [OSI](IT/osi.md), [Firewall](IT/firewall.md), [DAS-NAS-SAN](IT/das-nas-san.md), [WLAN](IT/wlan.md) |
| [Firewall](IT/firewall.md) | Regelwerke lesen und schreiben, Stateful vs. NGFW, DMZ. | [OSI](IT/osi.md), [Proxy](IT/proxy.md), [Netzwerktypen](IT/netzwerktypen.md), [DDoS](IT/ddos.md) |
| [IPv6](IT/ipv6.md) | 128 Bit, SLAAC, NDP – und was du von IPv4 nicht übernehmen darfst. | [IPv4](IT/ipv4.md), [OSI](IT/osi.md), [NAT/PAT](IT/nat-pat.md), [Subnetting IPv4 & IPv6](IT/subnetting-ipv4-ipv6.md) |
| [Proxy](IT/proxy.md) | Forward, Reverse und Port Forwarding in einer Tabelle. | [Firewall](IT/firewall.md), [OSI](IT/osi.md), [DMZ](IT/firewall.md) |
| [IPv4](IT/ipv4.md) | Subnetting, CIDR und die 2-Host-Regel – kommt in jeder GA2. | [IPv6](IT/ipv6.md), [OSI](IT/osi.md), [Switches und VLANs](IT/switches-und-vlans.md), [NAT/PAT](IT/nat-pat.md), [Subnetting IPv4 & IPv6](IT/subnetting-ipv4-ipv6.md) |
| [Active-Directory](IT/active-directory.md) | Microsoft-Verzeichnisdienst, Kerberos, Gruppenrichtlinien. | [LDAP](IT/ldap.md), [RBAC](IT/rbac.md), [Kerberos](IT/kerberos.md), [GPO](IT/gpo.md) |
| [RBAC](IT/rbac.md) | Role-Based Access Control – Rollen statt direkter Berechtigungen. | [Least Privilege](IT/least-privlage.md), [Active-Directory](IT/active-directory.md), [LDAP](IT/ldap.md) |
| [Ports](IT/ports.md) | Wichtige Ports, Protokolle und Dienste. | [Firewall](IT/firewall.md), [OSI](IT/osi.md), [IPv4](IT/ipv4.md), [DNS](IT/dns.md) |
| [Subnetting IPv4 & IPv6](IT/subnetting-ipv4-ipv6.md) | CIDR, Blockgrößen, SLAAC und IPv6-Adressierung. | [IPv4](IT/ipv4.md), [IPv6](IT/ipv6.md), [Routing](IT/routing.md) |
| [DHCP](IT/dhcp.md) | DORA-Vorgang, Lease, DHCP-Relay, DHCP-Snooping. | [DNS](IT/dns.md), [Subnetting IPv4 & IPv6](IT/subnetting-ipv4-ipv6.md), [Switches und VLANs](IT/switches-und-vlans.md) |
| [DNS](IT/dns.md) | Namensauflösung, A/AAAA/MX/CNAME, DNSSEC, DoH/DoT. | [DHCP](IT/dhcp.md), [Ports](IT/ports.md), [Firewall](IT/firewall.md) |
| [Routing](IT/routing.md) | Statisch/Dynamisch, OSPF/RIP/BGP, Routingtabelle, Longest Prefix Match. | [Subnetting IPv4 & IPv6](IT/subnetting-ipv4-ipv6.md), [OSI](IT/osi.md), [Firewall](IT/firewall.md), [Load Balancer](IT/load-balancer.md) |
| [Switches und VLANs](IT/switches-und-vlans.md) | Access/Trunk, 802.1Q, Inter-VLAN-Routing, STP/RSTP. | [Routing](IT/routing.md), [MAC](IT/mac.md), [Netzwerktypen](IT/netzwerktypen.md) |
| [VLAN](IT/vlan.md) | VLAN-Grundlagen kompakt: Segmentierung, Trunk, Inter-VLAN-Routing. | [Switches und VLANs](IT/switches-und-vlans.md), [Routing](IT/routing.md), [Firewall](IT/firewall.md) |
| [Firewalls (alle Generationen) & Sicherheitskonzepte (Antivirus)](IT/firewalls-alle-generationen-sicherheitskonzepte-antivirus.md) | Paketfilter bis NGFW, Zonen, DMZ, IDS/IPS, EDR. | [Firewall](IT/firewall.md), [VPN](IT/vpn.md), [OS-Härtung](IT/os-haertung.md), [DDoS](IT/ddos.md) |
| [VPN](IT/vpn.md) | Remote-Access/Site-to-Site, IPSec, SSL/TLS-VPN, WireGuard. | [Firewall](IT/firewall.md), [Routing](IT/routing.md), [PKI](IT/pki.md), [WLAN](IT/wlan.md) |
| [WLAN](IT/wlan.md) | 802.11-Standards, 2,4/5/6GHz, WPA2/WPA3, 802.1X. | [Netzwerktypen](IT/netzwerktypen.md), [Authentifizierung-Autoritisierung-MFA](IT/authentifizierung-autoritisierung-mfa.md), [VPN](IT/vpn.md) |
| [Netzwerkanalyse und Troubleshooting](IT/netzwerkanalyse-und-troubleshooting.md) | OSI-orientiert, Ping/Traceroute/NSLookup/Wireshark, systematisch. | [OSI](IT/osi.md), [Routing](IT/routing.md), [DNS](IT/dns.md), [DHCP](IT/dhcp.md), [Ports](IT/ports.md) |
| [Load Balancer](IT/load-balancer.md) | Lastverteilung, Algorithmen, Layer 4/7, Health Checks, VIP. | [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md), [Routing](IT/routing.md), [Firewall](IT/firewall.md), [DDoS](IT/ddos.md), [Skalierbarkeit-Elastizität](IT/skalierbarkeit-elastizitaet.md) |
| [DDoS](IT/ddos.md) | DoS/DDoS, Angriffsarten, Botnet, Gegenmaßnahmen – Verfügbarkeitsangriff. | [Load Balancer](IT/load-balancer.md), [Firewall](IT/firewall.md), [Schutzziele](IT/schutzziele.md), [Netzwerkanalyse und Troubleshooting](IT/netzwerkanalyse-und-troubleshooting.md) |
| [Blue-Green Deployment](IT/blue-green-deployment.md) | Zero-Downtime-Deployment, Umschaltung, schneller Rollback. | [Load Balancer](IT/load-balancer.md), [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md), [Skalierbarkeit-Elastizität](IT/skalierbarkeit-elastizitaet.md) |

## Sicherheit & Datenschutz (18)

| Thema | In einem Satz | Verknüpfung |
|---|---|---|
| [DSGVO & TOM](IT/dsgvo-tom.md) | Rechtsgrundlagen, Betroffenenrechte und die TOM, die die Prüfung sehen will. | [TOMs](IT/toms.md), [DBS-Datenschutzbeauftragten](IT/dbs-datenschutzbeauftragten.md), [SCC-Datenpanne](IT/scc-datenpanne.md) |
| [Schutzziele](IT/schutzziele.md) | Vertraulichkeit, Integrität, Verfügbarkeit – CIA. | [RAID](IT/raid.md), [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md), [DDoS](IT/ddos.md) |
| [hackers-can](IT/hackers-can.md) | Angriffe aus Prüfungsszenarien mit passenden Gegenmaßnahmen. | [Firewalls (alle Generationen) & Sicherheitskonzepte (Antivirus)](IT/firewalls-alle-generationen-sicherheitskonzepte-antivirus.md), [OS-Härtung](IT/os-haertung.md), [DDoS](IT/ddos.md) |
| [Symmetrische-und-Asymmetrische](IT/symmetrische-und-asymmetrische.md) | Wann symmetrisch, wann asymmetrisch – warum TLS beides nutzt. | [PKI](IT/pki.md), [Hash](IT/hash.md), [PGP](IT/pgp.md) |
| [Least Privilege](IT/least-privlage.md) | Minimalrechtprinzip – nur benötigte Rechte. | [RBAC](IT/rbac.md), [chmod](IT/chmod.md), [OS-Härtung](IT/os-haertung.md) |
| [Authentifizierung-Autoritisierung-MFA](IT/authentifizierung-autoritisierung-mfa.md) | Authentifizierung vs. Autorisierung, MFA-Varianten. | [LDAP](IT/ldap.md), [Active-Directory](IT/active-directory.md), [Kerberos](IT/kerberos.md), [RBAC](IT/rbac.md) |
| [OS-Härtung](IT/os-haertung.md) | Betriebssystem absichern – Updates, Rechte, Firewalls. | [Least Privilege](IT/least-privlage.md), [Patchmanagment-TU-SU-PU](IT/patchmanagment-tu-su-pu.md), [hackers-can](IT/hackers-can.md) |
| [Patchmanagment-TU-SU-PU](IT/patchmanagment-tu-su-pu.md) | Test-/Sicherheits-/Produktionsumfeld, Patch-Prozesse. | [OS-Härtung](IT/os-haertung.md), [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md) |
| [AAA-SIEM](IT/aaa-siem.md) | Authentication, Authorization, Accounting & Sicherheitsüberwachung. | [Authentifizierung-Autoritisierung-MFA](IT/authentifizierung-autoritisierung-mfa.md), [Firewalls (alle Generationen) & Sicherheitskonzepte (Antivirus)](IT/firewalls-alle-generationen-sicherheitskonzepte-antivirus.md) |
| [Bitlocker](IT/bitlocker.md) | TPM-basierte Festplattenverschlüsselung. | [BIOS-UEFI](IT/bios-uefi.md), [Schutzziele](IT/schutzziele.md) |
| [Hash](IT/hash.md) | Einwegfunktionen, Hashwerte, Integritätsprüfung. | [Symmetrische-und-Asymmetrische](IT/symmetrische-und-asymmetrische.md), [PKI](IT/pki.md), [PGP](IT/pgp.md) |
| [PKI](IT/pki.md) | Public Key Infrastructure, Zertifikate, CAs, TLS. | [Symmetrische-und-Asymmetrische](IT/symmetrische-und-asymmetrische.md), [Hash](IT/hash.md), [VPN](IT/vpn.md) |
| [PGP](IT/pgp.md) | Pretty Good Privacy – E-Mail-Verschlüsselung & Signaturen. | [Symmetrische-und-Asymmetrische](IT/symmetrische-und-asymmetrische.md), [PKI](IT/pki.md), [Hash](IT/hash.md) |
| [TOMs](IT/toms.md) | Technische und organisatorische Maßnahmen nach Art. 32 DSGVO. | [DSGVO & TOM](IT/dsgvo-tom.md), [Schutzziele](IT/schutzziele.md) |
| [DBS-Datenschutzbeauftragten](IT/dbs-datenschutzbeauftragten.md) | Aufgaben, Benennungspflicht, Stellung des DSB. | [DSGVO & TOM](IT/dsgvo-tom.md), [TOMs](IT/toms.md) |
| [SCC-Datenpanne](IT/scc-datenpanne.md) | Datenpanne, Meldepflicht 72h, Standard Contractual Clauses. | [DSGVO & TOM](IT/dsgvo-tom.md), [DBS-Datenschutzbeauftragten](IT/dbs-datenschutzbeauftragten.md) |
| [KI in IT - Vergleich mit klassisch](IT/ki-in-it-vergleich-mit-klassisch.md) | Einsatz, Chancen, Risiken, verantwortungsvoller Einsatz. | [Netzwerkanalyse und Troubleshooting](IT/netzwerkanalyse-und-troubleshooting.md), [OS-Härtung](IT/os-haertung.md) |
| [MDM, BYOD & Mobile Device Management](IT/mdm-byod.md) | BYOD/CYOD/COPE, Containerisierung, Full/Selective Wipe. | [Authentifizierung-Autoritisierung-MFA](IT/authentifizierung-autoritisierung-mfa.md), [Least Privilege](IT/least-privlage.md), [VPN](IT/vpn.md), [DSGVO & TOM](IT/dsgvo-tom.md) |

## Virtualisierung & Server (5)

| Thema | In einem Satz | Verknüpfung |
|---|---|---|
| [Virtualisierung](IT/virtualisierung.md) | Grundlagen, Ziele, Vorteile/Nachteile der Virtualisierung. | [Hypervisor-Typen](IT/hypervisor-typen.md), [VM-Container-Server](IT/vm-container-server.md) |
| [VM-Container-Server](IT/vm-container-server.md) | Bare Metal, VM, Container: Isolation, Dichte, Startzeit, Kernel-Unterschied. | [Hypervisor-Typen](IT/hypervisor-typen.md), [Virtualisierung](IT/virtualisierung.md), [Skalierbarkeit-Elastizität](IT/skalierbarkeit-elastizitaet.md) |
| [Hypervisor-Typen](IT/hypervisor-typen.md) | Typ 1 (Bare-Metal) vs. Typ 2 (Hosted) – Einsatz & Overhead. | [Virtualisierung](IT/virtualisierung.md), [VM-Container-Server](IT/vm-container-server.md), [Live-Migration-VMs](IT/live-migration-vms.md) |
| [BIOS/UEFI-Kernel](IT/bios-uefi-kernel.md) | Bootkette, Kernel Space vs. User Space, System Calls. | [BIOS-UEFI](IT/bios-uefi.md), [OS-Härtung](IT/os-haertung.md) |
| [Live-Migration-VMs](IT/live-migration-vms.md) | VMs ohne Ausfall verschieben: Ablauf, Voraussetzungen, Grenzen. | [Hypervisor-Typen](IT/hypervisor-typen.md), [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md), [Cluster](IT/cluster.md) |

## Storage & Backup (3)

| Thema | In einem Satz | Verknüpfung |
|---|---|---|
| [Snapshots-Backups](IT/snapshots-backups.md) | 3-2-1-1-0, Voll/Differenziell/Inkrementell, warum Snapshots kein Backup sind. | [RAID](IT/raid.md), [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md), [DAS-NAS-SAN](IT/das-nas-san.md) |
| [RAID](IT/raid.md) | Level 0/1/5/6/10, nutzbare Kapazität, Ausfalltoleranz, Rechenweg. | [Snapshots-Backups](IT/snapshots-backups.md), [Schutzziele](IT/schutzziele.md), [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md), [DAS-NAS-SAN](IT/das-nas-san.md) |
| [DAS-NAS-SAN](IT/das-nas-san.md) | Block/Datei/Objekt, Direkt-/Netzwerkspeicher, Einsatzgebiete. | [RAID](IT/raid.md), [Snapshots-Backups](IT/snapshots-backups.md), [Netzwerktypen](IT/netzwerktypen.md) |

## Cloud & Wirtschaftlichkeit (4)

| Thema | In einem Satz | Verknüpfung |
|---|---|---|
| [Cloud_computing](IT/cloud-computing.md) | IaaS/PaaS/SaaS + FaaS/CaaS/DBaaS/STaaS/NaaS, Public/Private/Hybrid, CAPEX→OPEX. | [Virtualisierung](IT/virtualisierung.md), [Skalierbarkeit-Elastizität](IT/skalierbarkeit-elastizitaet.md), [CAPEX-OPEX](IT/capex-opex.md), [SLA](IT/sla.md) |
| [CAPEX-OPEX](IT/capex-opex.md) | Investition vs. Betriebskosten – die Rechnung hinter Cloud-Entscheidungen. | [Cloud_computing](IT/cloud-computing.md), [TCO](IT/tco.md), [Amortisation](IT/amortisation.md) |
| [SLA](IT/sla.md) | Verfügbarkeit in Prozent, Reaktionszeit und was ein SLA nicht verspricht. | [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md), [Cloud_computing](IT/cloud-computing.md), [TCO](IT/tco.md) |
| [Skalierbarkeit-Elastizität](IT/skalierbarkeit-elastizitaet.md) | Vertikal vs. horizontal, Scale-Up/Out/In/Down – und was elastisch wirklich heißt. | [Cloud_computing](IT/cloud-computing.md), [Load Balancer](IT/load-balancer.md), [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md), [VM-Container-Server](IT/vm-container-server.md) |

## Referenz – Projektmanagement & Wirtschaft (16)

| Thema | In einem Satz | Verknüpfung |
|---|---|---|
| [Abkürzungen](IT/abkuerzungen.md) | Nachschlagen, nicht lernen: die Abkürzungen der Prüfung. | Alle Themen |
| [Fachbegriffe](IT/fachbegriffe.md) | Slang und Architektur-Begriffe, die in Aufgaben vorkommen. | Alle Themen |
| [Pseudocode](IT/Pseudocode/readme.md) | Prüfungsvorbereitung für Pseudocode/C#-Aufgaben (Arrays, Schleifen, Fehlersuche). | [Analyse Prüfungen - Programmierung](IT/analyse-der-exams/04_Programmierung_Grundlagen.md) |
| [UML](IT/UML/readme.md) | UML-Diagramme für AP2 - Klassendiagramm, Use-Case, Aktivitätsdiagramm, Sequenzdiagramm. | [Analyse Prüfungen - Datenmodellierung/SQL/UML](IT/analyse-der-exams/L14_Datenmodellierung_SQL_UML.md) |
| [AOB](IT/aob.md) | Aufwandsorientierte Budgetierung. | [Aufwandsschätzung](IT/aufwandsschaetzung.md), [Ressourcenplan](IT/ressourcenplan.md) |
| [Netzplan](IT/netzplan.md) | Vorgangspfeil-Netzplan, kritischer Pfad. | [PERT](IT/pert.md), [Gantt-Diagramm](IT/gantt-diagramm.md) |
| [Gantt-Diagramm](IT/gantt-diagramm.md) | Balkendiagramm zur Terminplanung. | [Netzplan](IT/netzplan.md), [Ressourcenplan](IT/ressourcenplan.md) |
| [Projektstruckturplan-PSP](IT/projektstruckturplan-psp.md) | Arbeitspakete, Gliederungstiefe. | [Aufwandsschätzung](IT/aufwandsschaetzung.md), [Netzplan](IT/netzplan.md) |
| [Aufwandsschätzung](IT/aufwandsschaetzung.md) | Schätzverfahren für Projekte. | [PERT](IT/pert.md), [Projektstruckturplan-PSP](IT/projektstruckturplan-psp.md), [AOB](IT/aob.md) |
| [PERT](IT/pert.md) | Program Evaluation Review Technique – Drei-Zeiten-Methode. | [Netzplan](IT/netzplan.md), [Aufwandsschätzung](IT/aufwandsschaetzung.md) |
| [Ressourcenplan](IT/ressourcenplan.md) | Personelle und materielle Ressourcen. | [Gantt-Diagramm](IT/gantt-diagramm.md), [AOB](IT/aob.md) |
| [Soll-Ist-Vergleich](IT/soll-ist-vergleich.md) | Abweichungsanalyse im Projektcontrolling. | [Ressourcenplan](IT/ressourcenplan.md), [Amortisation](IT/amortisation.md) |
| [Amortisation](IT/amortisation.md) | Amortisationszeit einer Investition. | [CAPEX-OPEX](IT/capex-opex.md), [TCO](IT/tco.md), [BEP](IT/bep.md) |
| [TCO](IT/tco.md) | Total Cost of Ownership – Gesamtkosten über Lebenszyklus. | [CAPEX-OPEX](IT/capex-opex.md), [Cloud_computing](IT/cloud-computing.md), [Amortisation](IT/amortisation.md) |
| [BEP](IT/bep.md) | Break-Even-Point – Gewinnschwelle. | [Amortisation](IT/amortisation.md), [TCO](IT/tco.md) |
| [Stundensatz-(intern)](IT/stundensatz-intern.md) | Kalkulation interner Stundensätze. | [CAPEX-OPEX](IT/capex-opex.md), [Aufwandsschätzung](IT/aufwandsschaetzung.md) |

## Vertiefung – Code, Daten & Modelle

| Thema | In einem Satz | Verknüpfung |
|---|---|---|
| [Pseudocode: Einleitung](IT/Pseudocode/00_Einleitung.md) | Aufgabentypen und Prüfungsumfeld. | [Grundlagen & Syntax](IT/Pseudocode/01_Grundlagen_Syntax.md) |
| [Pseudocode: Grundlagen & Syntax](IT/Pseudocode/01_Grundlagen_Syntax.md) | Datentypen, Arrays, Schleifen, if/else. | [Arrays, Mittelwert, Max](IT/Pseudocode/02_Arrays_Mittelwert_Max.md) |
| [Pseudocode: Arrays, Mittelwert, Max](IT/Pseudocode/02_Arrays_Mittelwert_Max.md) | Mittelwert, Maximum, Minimum, zweithöchster Wert. | [Schleifen & Fehlersuche](IT/Pseudocode/03_Schleifen_Fehlersuche.md) |
| [Pseudocode: Schleifen & Fehlersuche](IT/Pseudocode/03_Schleifen_Fehlersuche.md) | Bubblesort, Schreibtischtest, Fehler finden. | [Praxistests](IT/Pseudocode/04_Praxistests_Musterloesungen.md) |
| [Pseudocode: Praxistests](IT/Pseudocode/04_Praxistests_Musterloesungen.md) | Originalnahe Übungstests mit Musterlösungen. | [Programmierung in der Prüfung](IT/analyse-der-exams/04_Programmierung_Grundlagen.md) |
| [SQL: Übersicht](IT/sql/readme.md) | DDL, DML, DQL, Joins, Views, Indizes in einer Zusammenfassung. | [SQL-Grundlagen](IT/sql/grundlagen.md) |
| [SQL-Grundlagen](IT/sql/grundlagen.md) | Datenbank vs. DBMS, Schlüssel, ACID, Normalisierung. | [DDL & DML](IT/sql/ddl-dml.md) |
| [DDL & DML](IT/sql/ddl-dml.md) | CREATE/ALTER/DROP und INSERT/UPDATE/DELETE. | [SQL-Joins](IT/sql/joins.md) |
| [SQL-Joins](IT/sql/joins.md) | INNER, LEFT, RIGHT, FULL, CROSS und SELF JOIN. | [Datenmodellierung, SQL & UML](IT/analyse-der-exams/L14_Datenmodellierung_SQL_UML.md) |
| [UML: Übersicht](IT/UML/readme.md) | Statische und dynamische Sicht, Prüfungsrelevanz. | [Klassendiagramm](IT/UML/Klassendiagram.md) |
| [Klassendiagramm](IT/UML/Klassendiagram.md) | Klassen, Attribute, Methoden, Vererbung, Assoziation. | [Use-Case-Diagramm](IT/UML/Anwendungsfalldiagramm-Use-Case-Diagramm.md) |
| [Use-Case-Diagramm](IT/UML/Anwendungsfalldiagramm-Use-Case-Diagramm.md) | Akteure, Systemgrenze, Anforderungen. | [Aktivitätsdiagramm](IT/UML/Aktivitätsdiagramm.md) |
| [Aktivitätsdiagramm](IT/UML/Aktivitätsdiagramm.md) | Abläufe, Entscheidungen, Verantwortlichkeiten. | [Sequenzdiagramm](IT/UML/Sequenzdiagramm) |
| [Sequenzdiagramm](IT/UML/Sequenzdiagramm) | Nachrichten zwischen Objekten über die Zeit. | [BPMN](IT/UML/BPMN.md) |
| [BPMN](IT/UML/BPMN.md) | Geschäftsprozesse mit Ereignissen, Gateways und Pools. | [Aktivitätsdiagramm](IT/UML/Aktivitätsdiagramm.md) |

## Prüfungsanalyse & Übungsfragen

| Thema | In einem Satz | Verknüpfung |
|---|---|---|
| [Prüfungsanalyse: Überblick](IT/analyse-der-exams/README.md) | Master-Index mit Prioritäten und 4-Wochen-Lernplan. | [Index & Prüfungsstruktur](IT/analyse-der-exams/00_Index.md) |
| [Index & Prüfungsstruktur](IT/analyse-der-exams/00_Index.md) | AP2-Aufbau, Notenschlüssel, Bearbeitungsregeln. | [Alle Lektionen](IT/analyse-der-exams/README.md) |
| [01 Cloud Computing](IT/analyse-der-exams/01_Cloud_Computing.md) | IaaS/PaaS/SaaS, Cloud-Modelle und Kosten. | [Cloud Computing](IT/cloud-computing.md) |
| [02 Speicher, Backup & RAID](IT/analyse-der-exams/02_Speicher_Backup_RAID.md) | RAID-Berechnungen und Speicher-Basics. | [RAID](IT/raid.md) |
| [03 Verfügbarkeit & Ausfallsicherheit](IT/analyse-der-exams/03_Verfuegbarkeit_Ausfallsicherheit.md) | USV, MTBF, Cluster und Redundanz. | [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md) |
| [04 Programmierung (Grundlagen)](IT/analyse-der-exams/04_Programmierung_Grundlagen.md) | Datentypen, Arrays, Struktogramme, Bit-Bedarf. | [Pseudocode](IT/Pseudocode/readme.md) |
| [05 Datenschutz & Informationssicherheit](IT/analyse-der-exams/05_Datenschutz_Informationssicherheit.md) | DSGVO, TOM, Malware und Phishing. | [DSGVO & TOM](IT/dsgvo-tom.md) |
| [06 IPv4-Adressierung & Routing](IT/analyse-der-exams/06_IPv4_Adressierung_Routing.md) | Subnetting, Broadcast, ARP und NAT. | [IPv4](IT/ipv4.md) |
| [07 Netzwerk-Fehlersuche](IT/analyse-der-exams/07_Netzwerk_Fehlersuche.md) | Systematische Fehlersuche mit ipconfig, ping und DNS. | [Netzwerkanalyse & Troubleshooting](IT/netzwerkanalyse-und-troubleshooting.md) |
| [08 IPv6](IT/analyse-der-exams/08_IPv6.md) | IPv6-Subnetting, SLAAC und DHCPv6. | [IPv6](IT/ipv6.md) |
| [09 WLAN](IT/analyse-der-exams/09_WLAN.md) | WPA2/WPA3, Gast-VLAN und WLAN-Scan. | [WLAN](IT/wlan.md) |
| [10 Lizenzierung (CALs)](IT/analyse-der-exams/10_Lizenzierung.md) | User- und Device-CALs richtig zuordnen. | [Cloud Computing](IT/cloud-computing.md) |
| [11 WiSo-Grundlagen](IT/analyse-der-exams/11_WiSo_Grundlagen.md) | Überblick über Recht, Betrieb und Wirtschaft. | [Wirtschaft](Wirtschaft.md) |
| [L12 Backup-Strategien](IT/analyse-der-exams/L12_Backup_Strategien.md) | Voll/Differenziell/Inkrementell, GVS, 3-2-1, MTBF. | [Snapshots & Backups](IT/snapshots-backups.md) |
| [L13 Virtualisierung](IT/analyse-der-exams/L13_Virtualisierung.md) | Hypervisor Typ 1/2 und Container. | [Virtualisierung](IT/virtualisierung.md) |
| [L14 Datenmodellierung, SQL & UML](IT/analyse-der-exams/L14_Datenmodellierung_SQL_UML.md) | ER-Modell, SQL und UML-Diagramme. | [SQL](IT/sql/readme.md), [UML](IT/UML/readme.md) |
| [L15 VLAN & Routing](IT/analyse-der-exams/L15_VLAN_Routing.md) | Access/Trunk, Router-on-a-Stick, STP und FHRP. | [Switches & VLANs](IT/switches-und-vlans.md) |
| [L16 Firewall](IT/analyse-der-exams/L16_Firewall.md) | Paketfilter, SPI/NGFW, Regeln und Port-Security. | [Firewall](IT/firewall.md) |
| [L17 DHCP & DNS](IT/analyse-der-exams/L17_DHCP_DNS.md) | DORA, Lease, Namensauflösung und Split-Horizon. | [DHCP](IT/dhcp.md), [DNS](IT/dns.md) |
| [L18 Netzwerk-Berechnungen](IT/analyse-der-exams/L18_Netzwerk_Berechnungen.md) | Verfügbarkeit und Redundanz berechnen. | [HA-Hochverfügbarkeit](IT/ha-hochverfuegbarkeit.md) |
| [L19 Sozialversicherung](IT/analyse-der-exams/L19_Sozialversicherung.md) | Die fünf Zweige und ihre Beiträge. | [Wirtschaft](Wirtschaft.md) |
| [L20 Betriebsrat & Mitbestimmung](IT/analyse-der-exams/L20_Betriebsrat_Mitbestimmung.md) | JAV, Betriebsvereinbarung und Mitbestimmungsrechte. | [Wirtschaft](Wirtschaft.md) |
| [L21 Unternehmensorganisation](IT/analyse-der-exams/L21_Unternehmensorganisation.md) | Leitungssysteme, Sektoren und Aufbauorganisation. | [Wirtschaft](Wirtschaft.md) |
| [L22 Kostenrechnung](IT/analyse-der-exams/L22_Kostenrechnung.md) | Deckungsbeitrag, Amortisation und Wirtschaftlichkeit. | [Amortisation](IT/amortisation.md) |
| [L23 WiSo Recht & Steuern](IT/analyse-der-exams/L23_WISO_Recht_Steuern.md) | Kaufrecht, Gesellschaftsformen, Steuern und AGG. | [Wirtschaft](Wirtschaft.md) |
| [L24 Arbeitsschutz, Brandschutz & Erste Hilfe](IT/analyse-der-exams/L24_Arbeitsschutz_Brandschutz_ErsteHilfe.md) | Pflichten im Betrieb und Notfallmaßnahmen. | [Wirtschaft](Wirtschaft.md) |
| [L25 Umweltschutz & Nachhaltigkeit](IT/analyse-der-exams/L25_Umweltschutz_Nachhaltigkeit.md) | Recycling, Energie und nachhaltige Beschaffung. | [Wirtschaft](Wirtschaft.md) |
| [L26 Wirtschaftssektoren, Konjunktur & Marktformen](IT/analyse-der-exams/L26_Wirtschaftssektoren_Konjunktur_Marktformen.md) | Sektoren, Konjunkturphasen und Wettbewerb. | [Wirtschaft](Wirtschaft.md) |
| [L27 PKI, TLS, Verschlüsselung & VPN](IT/analyse-der-exams/L27_PKI_TLS_Verschluesselung_VPN.md) | Zertifikate, TLS-Handshake und VPN-Varianten. | [PKI](IT/pki.md), [VPN](IT/vpn.md) |
| [L28 GA1: Sonstiges](IT/analyse-der-exams/L28_GA1_Sonstiges_Praxisthemen.md) | Weitere wiederkehrende Praxisthemen für GA1. | [GA1-Übungsfragen](IT/fragen/ga1/ga1-sommer-2026.md) |
| [L29 GA2: Sonstiges](IT/analyse-der-exams/L29_GA2_Sonstiges_Praxisthemen.md) | Weitere wiederkehrende Praxisthemen für GA2. | [GA2-Übungsfragen](IT/fragen/ga2/ga2-sommer-2026.md) |
| [GA1-Übungsfragen 2021–2026](IT/fragen/ga1/ga1-winter-2021.md) | Zehn komplette GA1-Aufgaben mit Lösungen. | [GA1 Winter 2021](IT/fragen/ga1/ga1-winter-2021.md), [Sommer 2022](IT/fragen/ga1/ga1-sommer-2022.md), [Winter 2022](IT/fragen/ga1/ga1-winter-2022.md), [Sommer 2023](IT/fragen/ga1/ga1-sommer-2023.md), [Winter 2023](IT/fragen/ga1/ga1-winter-2023.md), [Sommer 2024](IT/fragen/ga1/ga1-sommer-2024.md), [Winter 2024](IT/fragen/ga1/ga1-winter-2024.md), [Sommer 2025](IT/fragen/ga1/ga1-sommer-2025.md), [Winter 2025](IT/fragen/ga1/ga1-winter-2025.md), [Sommer 2026](IT/fragen/ga1/ga1-sommer-2026.md) |
| [GA2-Übungsfragen 2021–2026](IT/fragen/ga2/ga2-winter-2021.md) | Zehn komplette GA2-Aufgaben mit Lösungen. | [GA2 Winter 2021](IT/fragen/ga2/ga2-winter-2021.md), [Sommer 2022](IT/fragen/ga2/ga2-sommer-2022.md), [Winter 2022](IT/fragen/ga2/ga2-winter-2022.md), [Sommer 2023](IT/fragen/ga2/ga2-sommer-2023.md), [Winter 2023](IT/fragen/ga2/ga2-winter-2023.md), [Sommer 2024](IT/fragen/ga2/ga2-sommer-2024.md), [Winter 2024](IT/fragen/ga2/ga2-winter-2024.md), [Sommer 2025](IT/fragen/ga2/ga2-sommer-2025.md), [Winter 2025](IT/fragen/ga2/ga2-winter-2025.md), [Sommer 2026](IT/fragen/ga2/ga2-sommer-2026.md) |
| [AP1-Kann-Listen LF1–LF6](IT/nice-to-know/ap1/lf1.md) | Grundlagencheck für die AP1. | [LF1](IT/nice-to-know/ap1/lf1.md), [LF2](IT/nice-to-know/ap1/lf2.md), [LF3](IT/nice-to-know/ap1/lf3.md), [LF4](IT/nice-to-know/ap1/lf4.md), [LF5](IT/nice-to-know/ap1/lf5.md), [LF6](IT/nice-to-know/ap1/lf6.md) |
| [Übungsaufgabe RAID-Controller](IT/fragen/raid.md) | Prüfungsnahe Aufgabe mit Lösung und Erklärung. | [RAID](IT/raid.md) |

> **Tipp:** Die Analyse-Lektionen sind nach Prüfungsterminen sortiert (2021–2026). Arbeite zuerst die 🔴-Themen ab und nutze die Übungsfragen als Generalprobe unter Zeitdruck. Die vollständigen Prüfungen liegen im [verlinkten Repository](https://github.com/Shroudmoa/IHK-Abschlusspruefungen-AP2-FiSi-AO2020).
