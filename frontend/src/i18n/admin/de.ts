import type {I18nAdmin} from "./interface.ts";

export let I18nAdminDe: I18nAdmin = {
    api_key: {
        expires: "Erlischt",
        generate1: "Hier kann ein neues Secret für diesen API Key generiert werden.",
        generate2: `Das Secret wird nur einmalig direkt nach dem Generieren angezeigt.
            Wenn ein Neues generiert wurde, wird das Alte unmittelbar, permanent überschrieben.
            Diese Operation kann nicht rückgängig gemacht werden!`,
        generate3: `Der API Key muss im HTTP <code>Authorization</code> Header im folgenden
            Format mitgegeben werden:`,
        generate4: "Der folgende <code>curl</code> request kann zum Testen des Keys verwendet werden:",
        generate5: "Sollte <code>jq</code> nicht installiert sein, hier eine Version ohne:",
        keyName: "Key Name",
        limitedValidity: "Begrenzte Gültigkeit",
    },
    common: {
        account: "Account",
        addNew: "Neu Hinzufügen",
        back: "Zurück",
        copiedToClip: "Wert wurde in die Zwischenablage kopiert",
        details: "Details",
        enabled: "Aktiviert",
        filter: "Filter",
        from: "Von",
        loading: "Lade",
        note: "Notiz",
        reset: "Zurücksetzen",
        search: "Suchen",
        searchOptions: "Suchoptionen",
        until: "Bis",
    },
    docs: {
        book: "Für generelle Dokumentation für Rauthy existiert das",
        encryption: "Verschlüsselung",
        encKeys: {
            header: "Encryption Keys",
            keyActive: "Aktiver Key",
            keysAvailable: "Verfügbare Keys",
            migrate: "Migrieren",
            migrateToKey: "Migriere alle Werte zu folgendem Encryption Key",
            p1: `Diese Schlüssel werden für die zusätzliche Verschlüsselung in verschiedenen Situationen genutzt, wie 
            z.B. gewisse Werte innerhalb der Datenbank oder Session Cookies. Sie sind statisch konfiguriert, aber können
            als best-practice manuell rotiert werden.`,
            p2: `Der aktive Schlüssel ist ebenfalls statisch im Rauthy config file gesetzt. Alle neu-verschlüsselten
            Werte werden mit dem aktiven Schlüssel verschlüsselt, während alte zur Rückwärts-Kompatibilität parallel
            existieren können.`,
            p3: `Das Migrieren aller verschlüsselten Werte an dieser Stelle kann, je nach System, einige Zeit in 
            Anspruch nehmen.`,
        },
        hashing: {
            calculate: "Berechnen",

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

            time: "Zeit",
            targetTime: "Ziel-Zeit",
            tune: 'Wichtig: Diese Werten müssen auf der finalen Architektur eingestellt werden!',
            pDetials: `Für eine detailiertere Einführung in den Argon2ID Alrogithmus stehen vielen Quellen online zur
            Verfügung. Hier werden nur ganz kurz die Werte erklärt. Die folgenden drei Werte müssen konfiguriert werden:`,
            pTune: `Die Werte können stark variieren in Abhängigkeit vom System und der generellen Systemlast. Je 
            stärker das System, desto sicherere Werte können gewählt werden.`,
            pUtility: `Dieses Werkzeug ist eine Hilfe zum Finden der besten Argon2ID Werte für das jeweilige System.
            Argon2ID is der derzeit sicherste, verfügbare Passwort Hashing Algorithmus. Um das volle Potential 
            ausschöpfen zu können, müssen die Werte allerdings auf das System angepasst werden.`,
            mCost3: "Der minimal erlaubte Wert für <code>m_cost</code> ist <code>32768</code>."
        },
        openapi: "Zur Integration einer externen Applikation via Rauthy's API gibt es das",
        openapiNote: `In Abhängigkeit von der Konfiguration ist das Swagger  UI nicht öffentlich zugänglich übber den
            oben genannten Link. Es ist allerdings (standardmäßig) über den internen metrics server verfügbar zur
            Reduzierung der Angriffsfläche.`,
        source: "Der source code kann hier gefunden werden",
    },
    error: {
        needsAdminRole: 'Um Zugriff zu erhalten ist die Rolle <b>rauthy_admin</b> notwendig.',
        noAdmin: `Für Rauthy Admin Accounts ist <b>MFA Pflicht.</b><br>
            Im <b>Account</b> kann ein Passkey hinterlegt und MFA aktiviert werden.<br>
            Danach muss ein Logout und neuer Login folgen`,
    },
    events: {
        eventLevel: "Event Level",
        eventType: "Event Typ",
    },
    jwks: {
        alg: "Algorithmus",
        p1: "Dies sind die Json Web Keys (JWKs) die für das Signieren der Tokens genutzt werden.",
        p2: `JWKs werden standardmäßig automatisch an jedem 1. des Monats rotiert. Für alle neuen Tokens wird 
        immer die aktuellste Version eines Keys für den jeweiligen Algorithmus verwerndet. Alte Keys werden für 
        eine Weile behalten um bestehende Tokens validieren zu können und nach einer gewissen Zeit automatisch gelöscht.`,
        p3: `Die Keys können manuell rotiert werden. Abhängig von der Hardware auf der diese Rauthy Instanz läuft,
        kann dies einige Sekunden in Anspruch nehmen.`,
        type: "Typ",
        rotateKeys: "Keys Rotieren",
    },
    nav: {
        apiKeys: "API Keys",
        attributes: "Attribute",
        blacklist: "Blacklist",
        clients: "Clients",
        config: "Konfiguration",
        docs: "Dokumentation",
        events: "Events",
        groups: "Gruppen",
        providers: "Provider",
        roles: "Rollen",
        scopes: "Scopes",
        sessions: "Sessions",
        users: "Benutzer",
    },
    passwordPolicy: {
        configDesc: "Regeln für neue Passwörter.",
        resetSet0: "Der Wert 0 deaktiviert die Bedingung.",
        validForDays: "Gültigkeit Tage",
        validityNew: "Gültigkeit für neue Passwörter.",
    },
    providers: {
        config: {
            allowInsecureTls: "Erlaube unsicheres TLS",
            clientName: "Client Name",
            custRootCa: "Eigenes Root CA PEM",
            descAuthMethod: `Die Authentication Method, welche für den <code>/token</code> Endpunkt genutzt werden soll.
                Die meisten Provider sollten mit <code>basic</code> funktionieren, manche jedoch nur mit 
                <code>post</code>. In seltenen Fällen müssen beide Optionen aktiviert werden, auch wenn es gegen das
                RFC verstößt.`,
            descClientId: "Client ID, vom Auth Provider vorgegeben.",
            descClientName: "Client Name der auf der Rauthy Login Form angezeigt werden soll.",
            descClientSecret: `Client Secret, vom Auth Provider vorgegeben.
                Es muss mindestens ein Secret gegeben, oder PKCE aktiviert sein.`,
            descScope: `Der scope der beim Redirect zum Login genutzt werden soll. Werte müssen durch Leerzeichen
                getrennt angegeben werden`,
            errNoAuthMethod: "Ein client secret existiert, jedoch ist keine auth Methode aktiv",
            errConfidential: "Es muss mindestens entweder ein client secret existieren oder PKCE aktiv sein.",
            jsonPath: {
                p1: "Werte aus dem ID Token nach einem erfolgreichen Upstream Login können automatisch gemapped werden.",
                p2: `Der <code>Pfad</code> muss in korrekter Regex Syntax angegeben werden. Er kann auf einzelne JSON
                    Werte verweise, oder komplexe Sturkturen wie Arrays oder Objects`,
                p3: "<code>$.</code> markiert den Beginn eines JSON Objects",
                p4: "<code>*</code> kann als Wildcard innerhalb des Pfads genutzt werden",
                p5: "<code>$.roles</code> hätte als Ziel den Wert <code>&#123;\"roles\": \"value\"&#125;</code>",
                p6: `<code>$.roles.*</code> kann auf einen Wert innerhalb eines Arrays oder Objects wie z.B.<br>
                    <code>&#123;"roles": ["value", "notMyValue"]&#125;</code> verweisen`,
            },
            lookup: "Prüfen",
            pathAdminClaim: "Admin Claim Pfad",
            pathMfaClaim: "MFA Claim Pfad",
            rootPemCert: "Root PEM Zertifikat",
            mapMfa: `Sollte der Auth Provider in ID Claim bereit stellen, welches anzeigt, ob eine Art 2FA oder MFA
                beim Login verwandt wurde, so kann Rauthy diesen Werten extrahieren und entsprechend weitergeben.`,
            mapUser: `Es kann beim Login automatisch ein Nutzer mit der Rauthy Admin Rolle verlinkt werden, in
                Abhängigkeit von einem existierenden Upstream ID Claim.`,
            valueAdminClaim: "Admin Claim Wert",
            valueMfaClaim: "MFA Claim Wert",
        },
        delete: {
            areYouSure: "Sicher, dass dieser Provider gelöscht werden soll?",
            forceDelete: "Löschen Erzwingen",
            isInUse1: "Dieser Provider wird von aktiven Nutzern verwendet!",
            isInUse2: `Das Löschen kann erzwungen werden. Nutzer ohne lokales Passwort oder
                Passkey werden jedoch nicht mehr in der Lage sein, sich einzuloggen.`,
            linkedUsers: "Verbundene Nutzer",
        },
    },
    search: {
        orderBy: "Sortieren nach ...",
        orderChangeToAsc: "Zu aufsteigender Sortierung wechseln",
        orderChangeToDesc: "Zu absteigende Sortierung wechseln",
    },
    tabs: {
        config: "Konfiguration",
        delete: "Löschen",
    },
};