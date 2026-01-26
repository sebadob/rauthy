import type { I18nAdmin } from './interface.ts';

export let I18nAdminDe: I18nAdmin = {
    api_key: {
        delete1: 'Soll dieser API Key wirklich gelöscht werden?',
        expires: 'Erlischt',
        generate1: 'Hier kann ein neues Secret für diesen API Key generiert werden.',
        generate2: `Das Secret wird nur einmalig direkt nach dem Generieren angezeigt.
            Wenn ein Neues generiert wurde, wird das Alte unmittelbar, permanent überschrieben.
            Diese Operation kann nicht rückgängig gemacht werden!`,
        generate3: `Der API Key muss im HTTP <code>Authorization</code> Header im folgenden
            Format mitgegeben werden:`,
        generate4:
            'Der folgende <code>curl</code> request kann zum Testen des Keys verwendet werden:',
        generate5: 'Sollte <code>jq</code> nicht installiert sein, hier eine Version ohne:',
        keyName: 'Key Name',
        limitedValidity: 'Begrenzte Gültigkeit',
    },
    attrs: {
        delete1: 'Soll dieses Attribut wirklich gelöscht werden?',
        defaultValue: 'Standard Wert',
        desc: 'Beschreibung',
        makeEditable: 'Editierbar machen',
        makeEditableP1: 'Dieses Attribut kann durch Benutzer editierbar gemacht werden.',
        makeEditableP2: `<b>ACHTUNG:</b> Diese Änderung kann niemals rückgängig gemacht werden! Jegliche Angaben durch
            einen Benutzer direkt sind immer unvalidiert und dürfen NIEMALS für irgengeine Form von Authentifizierung 
            oder Authorisierung genutzt werden!`,
        makeEditableP3: `Ein Attribut kann deshalb niemals von editierbar zu nicht-editierbar gewandelt werden, weil
            für eine gewisse Zeit, unabhängig von der Dauer, unvalidierte Eingaben erlaubt waren.`,
        name: 'Attribut Name',
        userEditable: 'Durch Benutzer Editierbar',
    },
    backup: {
        createBackup: 'Backup Erstellen',
        disabledDesc:
            'Diese Funktionen stehen nur zur Verfügung, wenn Hiqlite als Datenbank konfiguriert ist.',
        lastModified: 'Zuletzt Modifiziert',
        local: 'Lokal',
        name: 'Name',
        size: 'Größe',
    },
    clients: {
        backchannelLogout:
            'Sollte dieser client {{ OIDC_BCL }} unterstützen, kann die URI hier angegeben werden.',
        branding: {
            descHsl: `Die folgenden Werte müssen als HSL angegeben werden. Hier wird nur die Basis-Farbe
                definiert. Alpha Kanäle und andere Werte werden vom Theme dynamisch angepasst.`,
            descFullCss: `Die folgenden Werte müssen vollständig gültige Angaben für CSS <code>color</code> sein.
                Es können auch komplexe Kalkulationen oder die oben definierten CSS Variables 
                genutzt werden.`,
            descVariables: `Jede nachfolgende Beschriftung ist gleichzeitig der Name der CSS Variable. Das heisst,
                dass z.B. die freien Eingaben wiederum die Variablen referenzieren können, z.B. mit
                <code>hsla(var(--action) / .7)</code>.`,
        },
        confidential: 'Vertraulich',
        confidentialNoSecret: 'Dies is kein vertraulicher Client und hat somit kein Secret.',
        config: 'Client Konfiguration',
        delete1: 'Soll dieser Client wirklich gelöscht werden?',
        descAuthCode: `Die Gültigkeit der Auth Codes kann angepasst werden um zusätzliche Sicherheit
            zu gewinnen. Auth Codes können nur einmalig verwendet werden und sind normalerweise für 60
            Sekunden gültig. Je kürzer, desto besser, so lange der Client den Code schnell genug nutzen kann.`,
        descClientUri: `Informationen über URI und Kontakte dieses Clients zur Anzeige
            auf der Login Seite.`,
        descName: `Der Client Name kann geändert werden ohne Einfluss auf die Konfiguration.
            Er dient lediglich der Anzeige auf der Login Seite.`,
        descGroupPrefix: `Der Login für diesen Client kann limitiert werden durch ein optionales Gruppen Prefix.
            Nur Benutzer, die zur entsprechenden Gruppe gehören, dürfen sich bei diesem Client einloggen.`,
        descOrigin: `Externe, zusätzlich erlaubte Origins - normalerweise nur notwendig, wenn dieser 
            Client direkt aus dem Browser heraus Requests zu Rauthy machen muss, typischerweise SPAs.`,
        descPKCE: `Wenn der Client Support für PKCE hat, sollte zur zusätzlichen Sicherheit immer S256 
            PKCE aktiviert werden. Wenn ein nicht-vertraulicher Client (z.B. eine SPA) genutzt wird, muss
            mindestens eine PKCE Challenge aktiviert sein, um ausreichend Sicherheit bieten zu können.`,
        descPKCEEnforce: `Wenn PKCE aktiviert ist, erzwingt Rauthy die Nutzung and verweigert Logins,
            die keine korrekte Challenge bereit stellen.`,
        descUri: `Es können beliebig viele Redirect URIs angegeben werden. Am Ende einer Jeden wird 
            optional <code>*</code> als Wildcard akzeptiert.`,
        errConfidentialPKCE: `Der Client muss entweder vertraulich sein oder mindestens eine PKCE
            Challenge aktiviert haben.`,
        forceMfa: 'MFA Erzwingen',
        groupLoginPrefix: 'Login Gruppen Prefix',
        name: 'Client Name',
        scim: {
            baseUri: `Die SCIM base URI muss jene sein, von der Sub-Routen wie 
                <code>{base_uri}/Users/{id}</base_uri></code> korrekt abgeleitet werden können.`,
            desc: 'Sollte dieser client {{ SCIM_LINK }} unterstützen, kann es hier aktiviert werden.',
            enable: 'SCIMv2 aktivieren',
            groupSync: 'Gruppen synchronisieren',
            groupSyncPrefix: 'Gruppen Filter Prefix',
            groupSyncPrefixDesc: `Die zu synchronisierenden Gruppen können per optionalem Prefix gefiltert werden.
                Wenn z.B. Gruppen wie <code>app:admins</code> und <code>app:users</code> existieren, würde das
                Prefix <code>app:</code> dafür sorgen, dass nur diese Gruppen synchronisiert werden, wie auch nur
                jene Benutzer, die zu mindestens einer dieser Gruppen gehören.`,
            reqDesc: 'Es gibt folgende Bedingungen für die Kompaibilität:',
            reqLi1: 'Der client muss <code>externalId</code> korrekt handhaben.',
            reqLi2: `Mindestens die <code>/Users</code> endpunkte mit <code>filter=externalId eq "*"</code> und
                <code>filter=userName eq "*"</code> müssen unterstützt sein.`,
            reqLi3: `Wenn Gruppen sychronisiert werden sollen, so müssen unter <code>/Groups</code> zusätzlich 
                <code>filter=displayName eq "*"</code> unterstützt sein.`,
        },
        scopes: {
            allowed: 'Erlaubte Scopes',
            default: 'Standard Scopes',
            desc: `Erlaubte Scopes sind diejenigen, die der Client dynamisch beim Redirect zum Login
            im <code>authorization_code</code> flow anfordern kann. Die standard Scopes werden hingegen immer
            hinzugefügt und können Probleme lösen, wenn z.B. der <code>password</code> Flow verwendet
            wird.`,
        },
        secret: {
            doCache: 'Client Secret cachen',
            cacheDuration: 'Cache Dauer (Stunden)',
            generate: 'Neues Secret Generieren',
            rotateDesc1: `Um unterbrechungsfreie Updates durchfürhen zu können, ist es möglich, das bestehende Secret
                für eine gewisse Zeit im in-memory Cache zu behalten. Es kann ein Wert zwischen 1 und 24 Stunden 
                angegeben werden.`,
            rotateDesc2:
                'Achtung: Das derzeitige Secret sollte nicht im Cache behalten werden, wenn es ein Leak gab!',
        },
        tokenLifetime: {
            p1: `Die Token Lifetime wird auf Access und ID Tokens angewandt und wird in Sekunden angegeben.`,
            p2: `Sollte der Client EdDSA / ed25519 Algorithmen unterstützen, sollte dies die bevorzugte Wahl
                sein. RSA Algorithmen existieren lediglich aus Kompatibilitätsgründen.`,
            p3: `Der Algorithmus für Refresh Tokens kann nicht geändert werden, da diese nur von Rauthy
                genutzt werden sollten.`,
        },
    },
    common: {
        account: 'Account',
        addNew: 'Neu Hinzufügen',
        back: 'Zurück',
        caution: 'ACHTUNG',
        contact: 'Kontakt',
        copiedToClip: 'Wert wurde in die Zwischenablage kopiert',
        details: 'Details',
        edit: 'Bearbeiten',
        enabled: 'Aktiviert',
        filter: 'Filter',
        from: 'Von',
        information: 'Informationen',
        language: 'Sprache',
        loading: 'Lade',
        name: 'Name',
        nameExistsAlready: 'Name existiert bereits',
        note: 'Notiz',
        noEntries: 'Keine Einträge',
        preview: 'Vorschau',
        reset: 'Zurücksetzen',
        searchOptions: 'Suchoptionen',
        until: 'Bis',
    },
    docs: {
        book: 'Für generelle Dokumentation für Rauthy existiert das',
        encryption: 'Verschlüsselung',
        encKeys: {
            header: 'Encryption Keys',
            keyActive: 'Aktiver Key',
            keysAvailable: 'Verfügbare Keys',
            migrate: 'Migrieren',
            migrateToKey: 'Migriere alle Werte zu folgendem Encryption Key',
            p1: `Diese Schlüssel werden für die zusätzliche Verschlüsselung in verschiedenen Situationen genutzt, wie 
            z.B. gewisse Werte innerhalb der Datenbank oder Session Cookies. Sie sind statisch konfiguriert, aber können
            als best-practice manuell rotiert werden.`,
            p2: `Der aktive Schlüssel ist ebenfalls statisch im Rauthy config file gesetzt. Alle neu-verschlüsselten
            Werte werden mit dem aktiven Schlüssel verschlüsselt, während alte zur Rückwärts-Kompatibilität parallel
            existieren können.`,
            p3: `Das Migrieren aller verschlüsselten Werte an dieser Stelle kann, je nach System, einige Zeit in 
            Anspruch nehmen.`,
            pNotPossible: 'Zur Migration müssen mindestens 2 Encryption Keys vorhanden sein.',
        },
        hashing: {
            calculate: 'Berechnen',

            currValuesHead: 'Derzeitige Werte',
            currValues1: 'Die derzeitigen im Backend konfigurierten Werte sind die folgenden:',
            currValuesNote: `Notiz: Die Login Zeit vom Backend wird nur dann eine gute Richtlinie sein, nachdem 
            mindestens 5 erfolgreiche Logins seit dem letzten Neustart gemacht wurden. Der Ausgangswert ist immer 
            2000 ms und wird mit jedem erfolgreichen Login neu gemittelt.`,
            currValuesThreadsAccess: 'Threads (p_cost) die Rauthy zur Verfügung stehen',

            loginTimeHead: 'Ein paar Worte zur Login Zeit',
            loginTime1: `Generell möchten User alles so schnell wie möglich. Für eine sichere Login Prozedur jedoch
            sollte mindestens eine Dauer von 500 - 1000 ms anvisiert werden and kein Problem darstellen. Die Zeit zum
            Passwort Hashing darf nicht zu kurz gewählt werden, weil dadurch die Stärke des Hashes reduziert werden 
            würde.`,
            loginTime2: `Um standardmäßig genügend Sicherheit zu gewährleisten, erlaubt dieses Tool keine kleineren
            Werte als 500 ms für die Login Zeit.`,

            mCost1: `Die <code>m_cost</code> definiert die Menga an <b>Speicher (in kB)</b> die zum Hashing verwendet
            wird. Je höher dieser Wert, umso besser (sicherer), aber die notwendigen Ressourcen müssen natürlich 
            vorhanden sein.<br>
            Wenn z.B. 4 Passwörter zur selben Zeit gehasht werden, wird selbstverständlich <code>4 x m_cost</code>
            an Speicher benötigt, was zu jeder Zeit zur Verfügung stehen muss.`,
            mCost2: `Den "richtigen" Wert für <code>m_cost</code> zu finden ist glücklicherweise sehr einfach. Definiere
            das Maximum an Speicher, das Rauthy nutzen sollte, dividiere die Menge durch die Anzahl paralleler Logins, 
            die möglich sein sollten (<code>MAX_HASH_THREADS</code>) und ziehe hier von eine gewisse statische Menge ab.
            Die Höhe des statisch benötigten Speichers hängt von der gewählten Datenbank und Anzahl Benutzer ab, jedoch
            wird sie in den meisten Fällen im Bereich von 32 - 96 MB sein.`,

            pCost1: `<code>p_cost</code> definiert den Parallelismus fürs Hashing.<br>
            In den meisten Fällen erhöhen Werte jenseits von 8 nichts mehr die benötigte Zeit, weil der Algorithmus
            gesättigt sein wird. Dies ist auch der Standardwert für Rauthy.`,
            pCost2: `Die generelle Regel lautet:<br>
            Setze <code>p_cost</code> auf den zweifachen Wert der verfügbares CPU Kerne.<br>
            Wenn z.B. 4 Kerne zur Verfügung stehen, wäre eine <code>p_cost</code> von 8 ein guter Wert.<br>
            Der Wert muss allerdings die maximale Anzahl parallel erlaubter Logins (<code>MAX_HASH_THREADS</code>) 
            berücksichtigen und ggf. entsprechend reduziert werden.`,

            tCost1: `<code>t_cost</code> ist ein Multiplikator für die <b>Zeit</b> fürs Hashing. Dies ist der einzige
            Wert, der durch Testen auf der Zielarchitektur gefunden werden muss, weil <code>m_cost</code> und 
            <code>p_cost</code> gewissenermaßen vorgegeben sind.`,
            tCost2: `Das Finden des Wertes ist einfach: Setze <code>m_cost</code> und <code>p_cost</code> wie oben
            erklärt und erhöhe <code>t_cost</code> so lange, bis die gewünschte Login Zeit erreicht wird.`,

            utilityHead: 'Parameter Berechnungs-Werkzeug',
            utility1: `Das folgende Werkzeug kann zum Finden passender Werte für dieses Rauthy deployment genutzt 
            werden. Da die Werte von sehr vielen Faktoren abhängen, sollten dieser auf der finalen Architektur 
            eingestellt werden, am besten zu Zeiten der am höchsten erwarteten Last, um keine zu hohen Werte 
            einzustellen.`,
            utility2: `<code>m_cost</code> ist Optional und der als minimal sichere Wert von 32768 würde automatisch
            gewählt werden. Sollte <code>p_cost</code> ebenfalls nicht gegeben sein, so wird Rauthy die maximal 
            verfügbare Menge and Kernen nutzen.`,

            time: 'Zeit',
            targetTime: 'Ziel-Zeit',
            tune: 'Wichtig: Diese Werten müssen auf der finalen Architektur eingestellt werden!',
            pDetials: `Für eine detailiertere Einführung in den Argon2ID Alrogithmus stehen vielen Quellen online zur
            Verfügung. Hier werden nur ganz kurz die Werte erklärt. Die folgenden drei Werte müssen konfiguriert werden:`,
            pTune: `Die Werte können stark variieren in Abhängigkeit vom System und der generellen Systemlast. Je 
            stärker das System, desto sicherere Werte können gewählt werden.`,
            pUtility: `Dieses Werkzeug ist eine Hilfe zum Finden der besten Argon2ID Werte für das jeweilige System.
            Argon2ID is der derzeit sicherste, verfügbare Passwort Hashing Algorithmus. Um das volle Potential 
            ausschöpfen zu können, müssen die Werte allerdings auf das System angepasst werden.`,
            mCost3: 'Der minimal erlaubte Wert für <code>m_cost</code> ist <code>32768</code>.',
        },
        openapi: "Zur Integration einer externen Applikation via Rauthy's API gibt es das",
        openapiNote: `In Abhängigkeit von der Konfiguration ist das Swagger  UI nicht öffentlich zugänglich übber den
            oben genannten Link. Es ist allerdings (standardmäßig) über den internen metrics server verfügbar zur
            Reduzierung der Angriffsfläche.`,
        source: 'Der source code kann hier gefunden werden',
    },
    editor: {
        bold: 'Fett',
        code: 'Code',
        heading1: 'Überschrift 1',
        heading2: 'Überschrift 2',
        heading3: 'Überschrift 3',
        italic: 'Kursiv',
        link: 'Link',
        listBullet: 'Liste',
        listTasks: 'Aufgaben',
        listNumbered: 'Nummerierte Liste',
        paragraph: 'Absatz',
        quote: 'Zitat',
        removeFmt: 'Formatierung entfernen',
        strikeThrough: 'Durchgestrichen',
        textArea: 'Text Bearbeiten',
    },
    email: {
        cancelJob: 'Job Abbrechen',
        filterType: ['Keiner', 'In Gruppe', 'Nicht in Gruppe', 'Hat Rolle', 'Hat nicht Rolle'],
        immediate: 'Sofort',
        jobs: 'E-Mail Jobs',
        scheduled: 'Termin-Versand',
        sendAllUsers: 'Diese E-Mail wird an alle Benutzer gesendet.',
        sendAllUsersFiltered: 'Diese E-Mail wird an alle Benutzer gesendet, gefiltert durch:',
        sendMail: 'E-Mail senden',
        subject: 'Betreff',
        userFilter: 'Benutzer Filter',
    },
    error: {
        needsAdminRole: 'Um Zugriff zu erhalten ist die Rolle <b>rauthy_admin</b> notwendig.',
        noAdmin: `Für Rauthy Admin Accounts ist <b>MFA Pflicht.</b><br>
            Im <b>Account</b> kann ein Passkey hinterlegt und MFA aktiviert werden.<br>
            Danach muss ein Logout und neuer Login folgen`,
    },
    events: {
        eventLevel: 'Event Level',
        eventType: 'Event Typ',
    },
    groups: {
        delete1: 'Soll diese Gruppe wirklich gelöscht werden?',
        name: 'Gruppenname',
    },
    jwks: {
        alg: 'Algorithmus',
        p1: 'Dies sind die Json Web Keys (JWKs) die für das Signieren der Tokens genutzt werden.',
        p2: `JWKs werden standardmäßig automatisch an jedem 1. des Monats rotiert. Für alle neuen Tokens wird 
        immer die aktuellste Version eines Keys für den jeweiligen Algorithmus verwerndet. Alte Keys werden für 
        eine Weile behalten um bestehende Tokens validieren zu können und nach einer gewissen Zeit automatisch gelöscht.`,
        p3: `Die Keys können manuell rotiert werden. Abhängig von der Hardware auf der diese Rauthy Instanz läuft,
        kann dies einige Sekunden in Anspruch nehmen.`,
        type: 'Typ',
        rotateKeys: 'Keys Rotieren',
    },
    nav: {
        apiKeys: 'API Keys',
        attributes: 'Attribute',
        blacklist: 'Blacklist',
        clients: 'Clients',
        config: 'Konfiguration',
        docs: 'Dokumentation',
        events: 'Events',
        groups: 'Gruppen',
        providers: 'Provider',
        roles: 'Rollen',
        scopes: 'Scopes',
        sessions: 'Sessions',
        users: 'Benutzer',
    },
    options: {
        expires: 'Erlischt',
        lastSeen: 'Zuletzt Gesehen',
        state: 'Status',
    },
    pam: {
        addGroup: 'Neue PAM Gruppe',
        addHost: 'Neuer PAM Host',
        addUser: 'Neuer PAM Benutzer',
        deleteHost: 'Soll dieser Host wirklich gelöscht werden?',
        groupDescGeneric: `Generic Gruppen sind das Pendant zu Einträgen, die man überlicherweise in /etc/group finden
            würde. Benutzer können diesen zugewiesen werden und sie werden durch NSS Lookups ans System zurückgeliefert.`,
        groupDescHost: `Host-Gruppen dienen der Gruppierung von Hosts. NSS lookups eines Hosts innerhalb der Gruppe
            liefern als Ergebnis sämtliche anderen Hosts innerhalb dieser zurück. Benutzer können Zugriff auf Hosts
            bekommen, indem sie wiederum einer Host-Gruppe zugeordnet werden.`,
        groupDescLocal: `Local Gruppen verhalten sich fast identisch zu Generic Gruppen, mit dem Unterschied, dass sie
            zwar eine ID in der Rauthy Datenbank haben, der NSS Proxy auf dem jeweiligen Host sie aber mit einer ID
            aus /etc/group verknüpft. Auf diese Weise können Rauthy Benutzer bereits lokal existierenden Gruppen 
            zugeordnet werden.`,
        groupDescUser: `Benutzer-Gruppen sind automatisch verwaltet und eng an den Benutzer mit demselben Namen
            gekoppelt.`,
        groupDescWheel: `Diese Gruppe ist speziell. Sie ist unveränderlich und wird Benutzern dynamisch zugewiesen in
            Abhängigkeit von der Gruppenzugehörigkeit.`,
        groupName: 'Gruppenname',
        groups: 'Gruppen',
        groupType: 'Gruppen-Typ',
        hostAliases: 'Host Aliase',
        hostLocalPwdOnly: 'Lokaler Passwort Login',
        hostLocalPwdOnlyInfo: `Wenn Lokaler Passwort Login gesetzt ist, überschreibt dies ein MFA Erzwingen für lokale
            Logins. Ebenso wird (lokal) niemals ein Passkey verlangt, auch wenn der Nutzer MFA abgesichert ist. Diese
            Option sollte nur gesetzt werden, wenn absolut notwendig, wie z.B. wenn MFA-gesicherte Nutzer lokale Logins
            machen sollen, aber keine Hardware Passkeys verwenden.`,
        ipAddresses: 'IP Adressen',
        member: 'Mitglied',
        nameExistsAlready: 'Name is bereits vergeben',
        notes: 'Notizen',
        secretShow: 'Secret Anzeigen',
        secretRotate: 'Secret Rotieren',
        userEmail: 'Verlinkte Benutzer E-Mail',
        username: 'Benutzername',
        usernameNewDesc: `Der Benutzername sollte sorgfältig gewählt werden. Er lässt sich aus sicherheitstechnischen
            Gründen nachträglich nicht einfach ändern.`,
    },
    passwordPolicy: {
        configDesc: 'Regeln für neue Passwörter.',
        resetSet0: 'Der Wert 0 deaktiviert die Bedingung.',
        validForDays: 'Gültigkeit Tage',
        validityNew: 'Gültigkeit für neue Passwörter.',
    },
    providers: {
        config: {
            allowInsecureTls: 'Erlaube unsicheres TLS',
            autoLink: 'Auto-Link Benutzer',
            autoLinkDesc1: `Wenn Auto-Link Benutzer aktiviert ist, wird beim Login über diesen Provider automatisch
                ein eventuell existierender, nicht-verlinkter Benutzer mit diesem Provider verbunden.`,
            autoLinkDesc2: `ACHTUNG: Diese Option kann sehr gefährlich sein und zur Account-Übernahme führen, wenn der
                Provider keine vollständige E-Mail Überprüfung durchführt und es möglich macht eine fremde Adresse
                für einen Benutzer einzutragen! Darf in einem solchen Fall NIEMALS verwendet werden!`,
            clientName: 'Client Name',
            custRootCa: 'Eigenes Root CA PEM',
            descAuthMethod: `Die Authentication Method, welche für den <code>/token</code> Endpunkt genutzt werden soll.
                Die meisten Provider sollten mit <code>basic</code> funktionieren, manche jedoch nur mit 
                <code>post</code>. In seltenen Fällen müssen beide Optionen aktiviert werden, auch wenn es gegen das
                RFC verstößt.`,
            descClientId: 'Client ID, vom Auth Provider vorgegeben.',
            descClientName: 'Client Name der auf der Rauthy Login Form angezeigt werden soll.',
            descClientSecret: `Client Secret, vom Auth Provider vorgegeben.
                Es muss mindestens ein Secret gegeben, oder PKCE aktiviert sein.`,
            descScope: `Der scope der beim Redirect zum Login genutzt werden soll. Werte müssen durch Leerzeichen
                getrennt angegeben werden`,
            errNoAuthMethod: 'Ein client secret existiert, jedoch ist keine auth Methode aktiv',
            errConfidential:
                'Es muss mindestens entweder ein client secret existieren oder PKCE aktiv sein.',
            jsonPath: {
                p1: 'Werte aus dem ID Token nach einem erfolgreichen Upstream Login können automatisch gemapped werden.',
                p2: `Der <code>Pfad</code> muss in korrekter Regex Syntax angegeben werden. Er kann auf einzelne JSON
                    Werte verweise, oder komplexe Sturkturen wie Arrays oder Objects`,
                p3: '<code>$.</code> markiert den Beginn eines JSON Objects',
                p4: '<code>*</code> kann als Wildcard innerhalb des Pfads genutzt werden',
                p5: '<code>$.roles</code> hätte als Ziel den Wert <code>&#123;"roles": "value"&#125;</code>',
                p6: `<code>$.roles.*</code> kann auf einen Wert innerhalb eines Arrays oder Objects wie z.B.<br>
                    <code>&#123;"roles": ["value", "notMyValue"]&#125;</code> verweisen`,
            },
            lookup: 'Prüfen',
            pathAdminClaim: 'Admin Claim Pfad',
            pathMfaClaim: 'MFA Claim Pfad',
            rootPemCert: 'Root PEM Zertifikat',
            mapMfa: `Sollte der Auth Provider in ID Claim bereit stellen, welches anzeigt, ob eine Art 2FA oder MFA
                beim Login verwandt wurde, so kann Rauthy diesen Werten extrahieren und entsprechend weitergeben.`,
            mapUser: `Es kann beim Login automatisch ein Nutzer mit der Rauthy Admin Rolle verlinkt werden, in
                Abhängigkeit von einem existierenden Upstream ID Claim.`,
            valueAdminClaim: 'Admin Claim Wert',
            valueMfaClaim: 'MFA Claim Wert',
        },
        delete: {
            areYouSure: 'Sicher, dass dieser Provider gelöscht werden soll?',
            forceDelete: 'Löschen Erzwingen',
            isInUse1: 'Dieser Provider wird von aktiven Nutzern verwendet!',
            isInUse2: `Das Löschen kann erzwungen werden. Nutzer ohne lokales Passwort oder
                Passkey werden jedoch nicht mehr in der Lage sein, sich einzuloggen.`,
            linkedUsers: 'Verbundene Nutzer',
        },
    },
    roles: {
        adminNoMod: 'Die <code>rauthy_admin</code> Rolle kann nicht verändert werden.',
        delete1: 'Soll diese Rolle wirklich gelöscht werden?',
        name: 'Rollenname',
    },
    scopes: {
        defaultNoMod: 'Dies ist ein Default OIDC Scope. Diese sind unveränderbar.',
        delete1: 'Soll dieser Scope wirklich gelöscht werden?',
        deleteDefault: 'OIDC default scopes cannot be deleted',
        mapping1: 'Benutzer Attribute können auf eigene Scopes gemapped werden.',
        mapping2: `Jedes existierende Attribut hat einen eigenständigen Wert pro User. Diese Attribute können
            auf einen Scope gemapped werden und werden in diesem Fall im Access bzw. ID Token enthalten sein.`,
        name: 'Scope Name',
    },
    sessions: {
        invalidateAll: 'Alle Sessions Invalidieren',
    },
    search: {
        orderBy: 'Sortieren nach ...',
        orderChangeToAsc: 'Zu aufsteigender Sortierung wechseln',
        orderChangeToDesc: 'Zu absteigende Sortierung wechseln',
    },
    tabs: {
        config: 'Konfiguration',
        delete: 'Löschen',
    },
    tos: {
        accepted: 'Akzeptiert',
        addNewToS: 'Neue AGB',
        addNewToSFromCurrent: 'Neue AGB von Vorlage',
        added: 'Hinzugefügt',
        checkStatus: 'Benutzer Status prüfen',
        immutable: `ACHTUNG: Nach dem Hinzufügen sind AGB unveränderlich und können auch nicht wieder 
			gelöscht werden!`,
        noneExist: 'Es wurden noch keine Allgemeinen Geschäftsbedingungen hinzugefügt.',
        optUntil: {
            desc: `Während der Übergangsphase ist das Akzeptieren neuer AGB optional und wird erst danch 
				Pflicht.`,
            enable: 'Übergangsphase Aktivieren',
            label: 'Ende Übergangsphase',
        },
        tos: 'AGB',
    },
    users: {
        antiLockout: {
            rule: 'Anti-Lockout Regel',
            delete: 'kann nicht gelöscht werden',
            disable: 'kann nicht deaktiviert werden',
            rauthyAdmin: 'rauthy_admin Rolle kann nicht entfernt werden',
        },
        attributes: 'Attribute',
        deleteUser: 'Soll dieser Benutzer wirklich gelöscht werden?',
        descAttr: `Setze individuelle Benutzer Attribute. Alle Key / Value Paare 
            werden als String / JSON Wert gehandhabt.`,
        forceLogout: `Sollen sämtliche, für diesen Benutzer existierenden Sessions invalidiert und 
            Refresh Tokens gelöscht werden?`,
        lastLogin: 'Letzter Login',
        manualInitDesc: `Der Benutzer kann jedoch ebenfalls hier initialisiert werden. In diesem Fall muss das
            Passwort allerdings direkt kommuniziert werden.`,
        manualInit: 'Manuell Initialisieren',
        mfaDelete1: 'Die Passkeys dieses Nutzers können gelöscht werden.',
        mfaDelete2: `Vorsicht! Das Löschen eines Keys <b>kann nicht rückgängig gemacht werden</b>, ohne
            dass der Benutzer die Registrierung erneut durchführt.`,
        noMfaKeys: 'Dieser Benutzer hat keine registrierten Passkeys.',
        pkOnly1: 'Dies ist ein Passkey-Only Account.',
        pkOnly2:
            'Das bedeutet, dass dieser Benutzer den passwortlosen Login nutzt und kein Passwort gesetzt hat.',
        pkOnly3: `Sollte dieser Benutzer alle Passkeys verloren haben, kann der Account vollständig resettet und eine
            Password Reset E-Mail versendet werden. Um dies zu tun, müssen vorher unter dem Reiter 'MFA' sämtliche
            Passkeys gelöscht werden.`,
        pwdNoInit: 'Der Benutzer hat noch nicht den initialien Passwort Reset durchgeführt.',
        pwdSendEmailBtn: 'Reset E-Mail Senden',
        pwdSendEmailDesc:
            'Es kann eine neue Reset E-Mail gesendet werden, sollte der Benutzer keine erhalten haben.',
        savePassword: 'Passwort Speichern',
        selfServiceDesc:
            'Es kann entweder ein neues Passwort gesetzt, order eine Reset E-Mail versendet werden.',
        sendResetEmail: 'Reset E-Mail Senden',
    },
    validation: {
        css: 'Ungültiger CSS Wert',
        origin: 'Ungültige Origin',
        uri: 'Ungültige URI',
    },
};
