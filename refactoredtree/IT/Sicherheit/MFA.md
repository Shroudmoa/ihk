# MFA (Multi-Faktor-Authentifizierung)

## Definition & Prinzip
MFA ist ein Sicherheitsverfahren, bei dem sich ein Nutzer mit **mindestens zwei unterschiedlichen Faktoren** (Kategorien) authentifizieren muss. 
**Das Prinzip:** Ein Angreifer müsste mehrere, voneinander unabhängige Hürden gleichzeitig überwinden (z.B. Passwort *und* Besitz des Handys), um Zugriff zu erhalten. Fällt ein Faktor (z.B. Passwort-Leak), bleibt das System geschützt.

## Die 3 Faktoren (Wissen, Besitz, Sein)
Für echtes MFA müssen die Faktoren aus **verschiedenen** Kategorien kommen. Zweimal „Wissen“ (z.B. Passwort + PIN) ist kein MFA, sondern 2FA im weiteren Sinne, bietet aber keinen Schutz gegen Phishing.

1.  **Wissen (Knowledge):** Etwas, das der Nutzer **weiß**.
    *   *Beispiele:* Passwort, PIN, Sicherheitsfrage.
2.  **Besitz (Possession):** Etwas, das der Nutzer **hat**.
    *   *Beispiele:* Smartphone (App/TAN), Hardware-Token (YubiKey), Smartcard.
3.  **Sein (Inherence):** Etwas, das der Nutzer **ist** (Biometrie).
    *   *Beispiele:* Fingerabdruck, Gesichtserkennung (FaceID), Iris-Scan.

## IHK-Merksatz
> „MFA kombiniert mindestens zwei **verschiedene** Faktoren (Wissen, Besitz, Sein). Ein Passwort plus PIN ist kein echtes MFA, da beides ‚Wissen‘ ist. Erst Passwort + Smartphone (Besitz) oder Fingerabdruck (Sein) erhöhen die Sicherheit signifikant.“
