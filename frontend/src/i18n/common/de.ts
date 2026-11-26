import { type I18n } from './interface';

export const I18nDe: I18n = {
    lang: 'de',
    common: {
        accept: 'Akzeptieren',
        activeTheme: 'Aktives Farbschema',
        authenticate: 'Authentifizieren',
        cancel: 'Abbrechen',
        changeTheme: 'Farbschema wechseln - Aktiv: {{ CURRENT }}',
        close: 'Schließen',
        copyToClip: 'Wert in Zwischenablage kopieren',
        delete: 'Löschen',
        details: 'Details',
        edit: 'Bearbeiten',
        email: 'E-Mail',
        errTooShort: 'Eingabe zu kurz',
        errTooLong: 'Eingabe zu lang',
        expandContent: 'Inhalt ausklappen',
        hide: 'Verbergen',
        hours: 'Stunden',
        invalidInput: 'Ungültige Eingabe',
        legend: 'Legende',
        maxFileSize: 'Maximale Dateigröße',
        minutes: 'Minuten',
        month: 'Monat',
        months: [
            'Januar',
            'Februar',
            'März',
            'April',
            'Mai',
            'Juni',
            'Juli',
            'August',
            'September',
            'Oktober',
            'November',
            'Dezember',
        ],
        never: 'Niemals',
        password: 'Passwort',
        refresh: 'Erneuern',
        required: 'Notwendig',
        save: 'Speichern',
        search: 'Suchen',
        seconds: 'Sekunden',
        selectI18n: 'Sprache wählen',
        show: 'Anzeigen',
        summary: 'Zusammenfassung',
        timezone: 'Zeitzone',
        weekDaysShort: ['Mo', 'Di', 'Mi', 'Do', 'Fr', 'Sa', 'So'],
        year: 'Jahr',
    },
    account: {
        account: 'Benutzer Account',
        accType: 'Account Typ',
        accTypePasskeyText1: `Dies ist ein "Passkey-Only" Account. Das bedeutet, dass\ndieser 
            Account kein Passwort hat und auch keines benötigt.`,
        accTypePasskeyText2: `Der Account kann in einen Passwort-Account umgewandelt\nwerden. Das 
            würde allerdings bedeuten, dass ein Login auf einem neuen Gerät ohne vorherige, 
            zumindest\neinmalige zusätzliche Verifizierung des Passwortes nicht mehr möglich sein 
            wird.`,
        accTypePasskeyText3: 'Soll dieser Account gewandelt und ein Passwort hinzugefügt werden?',
        accessExp: 'Zugang erlischt',
        accessRenew: 'Zugang erneuerbar bis',
        accessRenewDelete: 'Möglichkeit zur Erneuerung löschen',
        birthdate: 'Geburtsdatum',
        canModifyFor: 'MFA können modifiziert werden für:',
        city: 'Stadt',
        changePassword: 'Passwort wechseln',
        convertAccount: 'Account Umwandeln',
        convertAccountP1: `Dieser Account kann in einen Passkey-Only Account umgewandelt\nwerden. 
            Diese Umwandlung löscht das Passwort und erlaubt den alleinigen Login mit den 
            registrierten\nPasskeys. Nur Passkeys mit zusätzlicher Benutzerverifizierung werden 
            akzeptiert. Diese sind auf der\n'MFA' Seite durch das zusätzliche Symbol hinter dem 
            Passkey Namen gekennzeichnet.`,
        country: 'Land',
        deleteAccount: {
            deleteAccount: 'Account Löschen',
            deleteAccountDesc: `ACHTUNG: Das Löschen dieses Accounts ist unwiderruflich! Zur 
            Bestätigung, bitte eingeben:`,
        },
        deviceId: 'ID',
        deviceName: 'Name',
        devices: 'Geräte',
        devicesDesc: 'Mit diesem Account verknüpfte Geräte',
        emailUpdateConfirm: `Die E-Mail Adresse wurde noch nicht aktualisiert. Eine Nachricht\nmit 
            einem Bestätigungslink wurde an die neue Adresse geschickt. Das Update muss über den
            \nenthaltenen Link bestätigt werden. Nach der Bestätigung wird die neue Adresse gesetzt.`,
        emailVerified: 'E-Mail verifiziert',
        familyName: 'Nachname',
        federatedConvertPassword1: `Dies ist ein verknüpfter Account. Das bedeutet, dass\nder 
            Login via externem Provider zur Authentifizierung geschieht. Der derzeitige Provider 
            ist:`,
        federatedConvertPassword2: `Es kann ein Passwort Reset via E-Mail angefordert\nwerden. Das 
            würde nach Abschluss diesem Account ein lokales Passwort hinzufügen. Danach wäre der\n
            Login entweder per externem Provider oder lokalem Passwort möglich. Passwort Reset 
            anfordern?`,
        generateRandom: 'Zufällig generiert',
        givenName: 'Vorname',
        groups: 'Gruppen',
        key: 'Schlüssel',
        keyUnique: 'Schlüssel muss einzigartig sein',
        lastLogin: 'Letzter Login',
        mfaActivated: 'MFA aktiviert',
        navInfo: 'Info',
        navEdit: 'Editieren',
        navMfa: 'MFA',
        navLogout: 'Logout',
        navBackToAdmin: 'Admin-UI',
        other: 'Sonstiges',
        pam: {
            addSshKey: 'Neuen Key Hinzufügen',
            comment: 'Kommentar',
            generatePassword: 'Neues Passwort',
            keyAdded: 'Hinzugefügt',
            keyType: 'Typ',
            username: 'Benutzername',
            validFor: 'Passwort gültig für {{ secs }} Sekunden',
        },
        passkeys: {
            missingAttestation:
                'Der Passkey entspricht nicht den geforderten Sicherheitsstandards.',
            type: 'Passkey Typ',
            types: ['Normal', 'Verknüpft'],
            rkWarning: `ACHTUNG: Ein verknüpfter Passkey (Resident Key) kann zwar zum direkten Login 
                ohne die Eingabe der E-Mail Adresse genutzt werden, aber es wird auch ein 
                zusätzlicher Speicherplatz für jede Anwendung gebraucht. Es muss dafür gesorgt 
                werden, dass genug Speicher vorhanden ist.
                Um einen bestehenden Passkey zu einem verknüpften zu machen, muss er zuerst
                gelöscht und dann neu hinzugefügt werden.`,
            // [Default, Resident Key]
            typesDesc: [
                'Vor dem Login mit diesem Passkey muss die E-Mail Adresse eingegeben werden.',
                `Funktioniert mit dem Passkey Button auf der Login Seite, ohne Eingabe der
                    E-Mail Adresse.`,
            ],
        },
        passwordConfirm: 'Passwort bestätigen',
        passwordCurr: 'Derzeitiges Passwort',
        passwordCurrReq: 'Derzeitiges Passwort ist notwendig',
        passwordNew: 'Neues Passwort',
        passwordNewReq: 'Neues Passwort ist notwendig',
        passwordNoMatch: 'Passwörter stimmen nicht überein',
        passwordExpiry: 'Passwort Ablauf',
        passwordPolicyFollow: 'Die Passwort Regeln müssen eingehalten werden',
        passwordReset: 'Passwort Reset',
        phone: 'Telefon',
        preferredUsername: {
            desc: `Manche Anwendungen greifen auf diesen Benutzernamen zurück. Welche 
				Anwendung davon Gebrauch macht, kann allerdings nicht kontrolliert werden.`,
            forceOverwrite: 'Überschreiben Erzwingen',
            immutable: 'ACHTUNG: Der Benutzername kann nur einmalig gesetzt werden!',
            immutableInfo: 'Der Benutzername kann nicht geändert werden.',
            notAvailable: 'Dieser Name ist nicht verfügbar.',
            preferredUsername: 'Bevorzugter Benutzername',
        },
        privacy: {
            clientId: 'Client ID',
            connectedApps: 'Verbundene Apps',
            connectedAppsDesc: 'Anwendungen, denen Zugriff auf den Account gewährt wurde.',
            granted: 'Erteilt',
            logoutWarning:
                'Dadurch werden außerdem alle Sitzungen auf allen Geräten beendet. Ein erneuter Login ist notwendig.',
            noConnectedApps: 'Bisher wurde keiner Anwendung Zugriff gewährt.',
            privacy: 'Datenschutz',
            resource: 'Ressource',
            revoke: 'Zugriff widerrufen',
            revokeAll: 'Alle Zugriffe widerrufen',
            revokeAllDesc:
                'Widerruft die Zustimmung für alle hier aufgeführten Anwendungen. Jede Anwendung muss erneut um Zustimmung bitten.',
            revokeDesc:
                'Der Widerruf entfernt die Zustimmung und die Anwendung kann ihren Zugriff nicht mehr erneuern. Access Tokens, die mit den gelöschten Refresh Tokens ausgestellt wurden, werden bei Userinfo und Introspection widerrufen. Anwendungen, die ihre Tokens selbst validieren, können sie bis zu ihrem Ablauf weiterhin akzeptieren. Die Anwendung muss erneut um Zustimmung bitten.',
            scopes: 'Scopes',
            updated: 'Aktualisiert',
            userAgent: 'User Agent',
        },
        providerLink: 'Account Verbinden',
        providerLinkDesc: `Dieser Account kann mit einem der folgenden Login Provider\nverbunden 
            werden. Nach der Aktivierung des Prozesses wird eine Weiterleitung auf die Login Seite\n
            des gewählten Providers ausgelöst. Nach erfolgreichem Login und bei Übereinstimmung der 
            E-Mail\nAdressen wird dieser Account verknüpft.`,
        providerUnlink: 'Verbindung Trennen',
        providerUnlinkDesc: `Nur wenn mindestens ein Passwort oder ein Passkey für diesen\nAccount 
            gesetzt ist, kann die Verbindung zum Provider gelöst werden.`,
        regDate: 'Datum der Registrierung',
        regIp: 'Registrierung von IP',
        roles: 'Rollen',
        street: 'Straße',
        user: 'Benutzer',
        userCreated: 'Benutzer erstellt',
        userEnabled: 'Benutzer Aktiviert',
        userExpiry: 'Benutzer Ablauf',
        userVerifiedTooltip: 'Abgesichert durch Fingerabdruck oder PIN',
        residentKeyTooltip: `Dies ist ein Resident Key. Er kann für direkten Login ohne Eingabe der E-Mail verwendet 
            werden.`,
        aaguidTooltip: 'Zertifizierter Passkey',
        webIdDesc: `Hier können die Felder festgelegt werden, die über die WebID veröffentlicht\n
            werden. Dies ist ein Feature, was von manchen Netzwerken für dezentrale Logins genutzt 
            wird.\nIst die WebID nicht bekannt, wird sie höchstwahrscheinlich nicht 
            benötigt.`,
        webIdDescData:
            'Eigene Daten können in gültigem FOAF Vokabular zur WebID\nhinzugefügt werden:',
        webIdExpertMode: 'Expertenmodus aktivieren',
        zip: 'PLZ',
    },
    authorize: {
        busy: 'Der Server ist gerade ausgelastet. Bitte gleich noch einmal versuchen.',
        clientForceMfa: `Dieser Login setzt MFA voraus für eine erhöhte Sicherheit.\nUm Zugang zu 
            bekommen, ist ein Login in den Account notwendig und mindestens ein Passkey\n
            muss hinzugefügt werden.`,
        clientGroupPrefixForbidden: 'Fehlende Gruppenzugehörigkeit für diesen Login',
        consent: {
            agree: 'Ich stimme der [Übermittlung meiner Daten] zu',
            changed: 'Der Inhalt der Zustimmung hat sich geändert, bitte erneut prüfen.',
            continue: 'Weiter',
            details: 'Details',
            intro: '{{ CLIENT }} bittet um Zugriff auf:',
            introResource: '{{ CLIENT }} bittet um Zugriff auf {{ RESOURCE }}:',
            required: 'immer',
            scopes: {
                address: 'Postanschrift',
                email: 'E-Mail-Adresse',
                groups: 'Gruppenzugehörigkeiten',
                openid: 'Benutzer-ID und Rollen für die Anmeldung',
                phone: 'Telefonnummer',
                profile:
                    'Name, Benutzername (kann die E-Mail-Adresse sein), Bild, Sprache, Geburtsdatum und Zeitzone',
                webid: 'WebID',
            },
            terms: 'Bedingungen',
            title: 'Zustimmung',
            warning: 'Bitte sorgfältig lesen',
        },
        email: 'E-Mail',
        emailBadFormat: 'Inkorrektes E-Mail Format',
        emailRequired: 'E-Mail ist notwendig',
        emailSentMsg: 'Sollte die Adresse registriert sein, wurde eine Nachricht versandt',
        expectingPasskey: 'Erwarte Authentifizierung per Passkey',
        expectingOtp: 'Erwarte Authentifizierung per Einmalpasswort',
        http429: 'Zu viele ungültige Versuche. Gesperrt bis:',
        invalidCredentials: 'Ungültige Zugangsdaten',
        invalidKeyUsed: 'Ungültiger Sicherheitsschlüssel',
        login: 'Login',
        mfaAck: 'Bestätigt',
        mfaOpenApp: 'App öffnen, um fortzufahren',
        mfaSelectionDesc: 'Bestätigungsmethode wählen:',
        usePasskey: 'Passkey verwenden',
        useEmailCode: 'E-Mail-Code verwenden',
        emailCodeDesc: 'Einen einmaligen Code per E-Mail erhalten',
        navigateToAccount: 'Account Dashboard',
        needsUserUpdate: `Es fehlen Informationen zum Account. Diese benötigen ein Update, bevor
            mit dem Login fortgefahren werden kann.`,
        orLoginWith: 'oder einloggen mit',
        passkey: 'Passkey',
        passkeyNotDiscovered: `Passkeys, die nicht verknüpft (als Resident Key) hinzugefügt wurden,
            können so nicht gefunden werden. Bitte stattdessen die E-Mail Adresse eingeben.`,
        password: 'Passwort',
        passwordExpired:
            'Passwort ist abgelaufen. Eine E-Mail mit einem Reset-Link wurde verschickt.',
        passwordForgotten: 'Passwort vergessen?',
        passwordRequest: 'Anfordern',
        passwordRequired: 'Passwort ist notwendig',
        passwordResetDesc: `Bitte E-Mail Adresse angeben, um einen Passwort Reset Link anzufordern. 
            Sollte die Adresse in der Datenbank existieren, wird an diese ein Link verschickt.`,
        passwordResetSuccess: 'Anfrage erhalten. Dieses Fenster kann nun geschlossen werden.',
        requestExpires: 'Anfrage läuft ab',
        requestExpired: 'Anfrage ist abgelaufen',
        signUp: 'Benutzer Registrierung',
        validEmail: 'Gültige E-Mail Adresse angeben',
    },
    device: {
        accept: 'Akzeptieren',
        autoRedirectAccount: 'Automatische Weiterleitung zum Account folgt',
        closeWindow: 'Dieses Fenster kann nun geschlossen werden.',
        decline: 'Ablehnen',
        desc: 'Bitte den {{count}}-stelligen vom Gerät angezeigten Benutzer Code eingeben.',
        descScopes: 'Das Gerät fragt Zugang an zu:',
        isAccepted: 'Die Anfrage wurde akzeptiert',
        isDeclined: 'Die Anfrage wurde abgewiesen',
        submit: 'Absenden',
        title: 'Gerät Authorisierung',
        userCode: 'Benutzer Code',
        wrongOrExpired: 'Ungültiger oder abgelaufener Code',
    },
    emailChange: {
        title: 'E-Mail Wechsel bestätigt',
        textChanged: 'Die E-Mail Adresse wurde erfolgreich geändert von',
        textLogin: 'Ein Login mit der neuen Adresse ist jetzt möglich.',
        to: 'zu',
    },
    error: {
        // errorText: "Die angeforderte Seite konnte nicht gefunden werden.",
        details: 'Details Anzeigen',
        // detailsText: undefined,
    },
    index: {
        register: 'Registrieren',
        accountLogin: 'Account',
        adminLogin: 'Admin',
    },
    logout: {
        logout: 'Logout',
        confirmMsg: 'Soll der Login beendet und die Session geschlossen werden?',
        cancel: 'Abbrechen',
    },
    mfa: {
        webauthn: {
            title: 'Passkeys',
            p1: `Bei paralleler Nutzung mehrerer Systeme, wie z.B. Windows und Android, sollte 
                die Registrierung mit Android durchgeführt werden.`,
            p2: `Android ist diejenige Plattform, die derzeit die wenigsten Features der passwortlosen 
                Technologie unterstützt. Schlüssel, die dort registriert werden, funktionieren auf 
                anderen Geräten gleichermaßen. Dies gilt jedoch nicht andersherum.`,
            p3: 'Für weitere Informationen, siehe',
            registerNew: 'Neuen Passkey Registrieren',
            docLinkText: 'die Dokumentation zu Passkeys',
            unsupportedText: 'Dieser Browser ist zu alt und unterstützt kein Webauthn.',
        },
        otp: {
            title: 'Einmalpasswort',
            titleEmail: 'Email-basierte Einmalpasswörter',
            activationCode: 'Bitte gib das per E-Mail gesendete Einmalpasswort ein.',
            code: 'Einmalpasswort',
            registerNew: 'Neuen OTP Registrieren',
            resendOtp: 'Neues Einmalpasswort Senden',
            sessionExpiresIn: 'Einmalpasswort läuft ab in:',
            sessionExpired: 'Einmalpasswort abgelaufen',
        },
        requestCanceled: 'Anfrage abgebrochen',
        errorReg: 'Fehler beim Starten der Registrierung',
        lastUsed: 'Zuletzt genutzt',
        noKey: 'Es wurde in diesem Speicher noch kein Sicherheitsschlüssel registriert',
        reAuthenticatePasskey: `Bevor Passkeys bearbeitet werden können, muss zuerst einer der 
            bestehenden überprüft werden:`,
        reAuthenticatePwd: `Bevor Passkeys bearbeitet werden können, ist eine erneute Bestätigung 
            des Passworts notwendig.`,
        reAuthenticateOtp: `Bevor Einmalpasswort bearbeitet werden können, muss zuerst einer der 
            bestehenden überprüft werden:`,
        register: 'Registrieren',
        registered: 'Registriert',
        registeredKeys: 'Registrierte Passkeys',
        registeredOtps: 'Registrierte Einmalpasswörter',
        passkeyName: 'Passkey Name',
        passkeyNameErr: '2 - 32 Buchstaben, keine Sonderzeichen',
        passwordInvalid: 'Ungültiges Passwort',
        test: 'Test',
        testError: 'Fehler beim Starten des Tests',
        testSuccess: 'Test erfolgreich',
    },
    pagination: {
        entries: 'Einträge',
        gotoPage: 'Gehe zu Seite',
        gotoPagePrev: 'Gehe zu vorheriger Seite',
        gotoPageNext: 'Gehe zu nächster Seite',
        pagination: 'Seitennummerierung',
        showCount: 'Anzahl anzeigen',
        total: 'Gesamt',
    },
    passwordPolicy: {
        passwordPolicy: 'Passwort Regeln',
        lengthMin: 'Länge min',
        lengthMax: 'Länge max',
        lowercaseMin: 'Kleinbuchstaben min',
        uppercaseMin: 'Großbuchstaben min',
        digitsMin: 'Ziffern min',
        specialMin: 'Sonderzeichen min',
        notRecent: 'Keines der letzten Passwörter',
    },
    passwordReset: {
        accountLogin: 'Account Login',
        badFormat: 'Ungültiges Format',
        continueSignIn: 'Weiter zur Anmeldung',
        fidoLink: 'https://fidoalliance.org/fido2',
        generate: 'Generieren',
        newAccDesc1: `Es gibt die Option zwischen zwei Account Typen zu wählen: Passwortlos 
            oder\ntraditionelles Passwort.`,
        newAccDesc2: `Der passwortlose Account Typ ist immer zu bevorzugen. Er bietet einen\nsehr 
            viel höheren Sicherheitsstandard als traditionelle Passwörter und gleichzeitig einen 
            einfacheren\nund schnelleren Login.\nDazu wird mindestens ein Passkey (Yubikey, Apple 
            Touch ID, Windows Hello, ...) benötigt, welcher dem\nFIDO2 Standard gerecht wird. Für 
            weitere Informationen kann diesem Link gefolgt werden: `,
        newAccount: 'Neuer Account',
        passwordReset: 'Passwort Zurücksetzen',
        password: 'Passwort',
        passwordless: 'Passkey',
        passwordConfirm: 'Passwort bestätigen',
        passwordNoMatch: 'Passwörter stimmen nicht überein',
        required: 'Notwendig',
        save: 'Speichern',
        success1: 'Das Passwort wurde erfolgreich zurückgesetzt.',
        success2: 'Weiterleitung in {{ secs }} Sekunden.',
        success3: 'Sollte keine Weiterleitung erfolgen, bitte hier klicken:',
        successPasskey1: 'Der neue Passkey wurde erfolgreich registriert.',
        successPasskey2: `Bitte direkt in den Account einloggen und mindestens einen weiteren\nBackup 
            Passkey registrieren. Ein passwortloser Account kann nicht den Passwort\nReset via 
            E-Mail nutzen für den Fall, dass der derzeitige Passkey abhanden 
            kommt.`,
    },
    register: {
        alreadyRegisteredEmail: 'E-Mail ist bereits registriert',
        alreadyRegisteredUsername: 'Benutzername ist bereits vergeben',
        domainAllowed: 'Erlaubte Domain:',
        domainErr: 'E-Mail Domain ist nicht erlaubt',
        domainRestricted: 'E-Mail Domains sind beschränkt',
        email: 'E-Mail',
        emailBadFormat: 'Ungültiges E-Mail Format',
        emailCheck: 'Bitte den E-Mail Posteingang prüfen',
        orRegisterWith: 'oder registrieren mit',
        regexName: 'Name mit 2 - 32 Buchstaben ohne Sonderzeichen',
        register: 'Registrieren',
        success: 'Registrierung erfolgreich',
        userReg: 'Benutzer Registrierung',
    },
    tos: {
        acceptOptUntil: 'Akzeptieren ist optional bis:',
        deny: 'Verweigern',
        tos: 'Allgemeine Geschäftsbedingungen',
    },
    userRevoke: {
        title: 'Logins widerrufen',
        desc1: `Sämtliche Logins und Sessions für diesen Benutzer wurden soweit wie möglich 
            widerrufen.`,
        desc2: 'Passwörter sollten auf der Stelle erneuert werden!',
    },
};
