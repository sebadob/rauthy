import type { I18nAdmin } from './interface.ts';

export let I18nAdminNl: I18nAdmin = {
    api_key: {
        delete1: 'Weet u zeker dat u deze API-sleutel wilt verwijderen?',
        expires: 'Vervaldatum',
        generate1: 'Hier kunt u een nieuw geheim genereren voor deze API-sleutel.',
        generate2: `Het geheim wordt slechts één keer weergegeven direct na het genereren.
            Wanneer een nieuw geheim wordt gegenereerd, wordt het oude permanent overschreven.
            Deze bewerking kan niet ongedaan worden gemaakt!`,
        generate3: `De API-sleutel moet worden opgegeven in de HTTP <code>Authorization</code>
            header in het volgende formaat:`,
        generate4:
            'U kunt het volgende <code>curl</code>-verzoek gebruiken om uw nieuwe sleutel te testen:',
        generate5: 'Als u <code>jq</code> niet heeft geïnstalleerd en het bovenstaande mislukt:',
        keyName: 'Sleutelnaam',
        limitedValidity: 'Beperkte geldigheid',
    },
    attrs: {
        delete1: 'Weet u zeker dat u dit attribuut wilt verwijderen?',
        defaultValue: 'Standaardwaarde',
        desc: 'Beschrijving',
        makeEditable: 'Bewerkbaar maken',
        makeEditableP1:
            'U kunt dit attribuut omzetten en het bewerkbaar maken door gebruikers zelf.',
        makeEditableP2: `<b>LET OP:</b> Dit kan nooit worden teruggedraaid! Alle invoer van een gebruiker direct is altijd
            onvertrouwde gegevens en MAG NOOIT worden gebruikt voor enige vorm van authenticatie of autorisatie!`,
        makeEditableP3: `Een attribuut kan niet worden gewijzigd van bewerkbaar naar niet-bewerkbaar, omdat het
            onvertrouwde invoer in het verleden heeft toegestaan, ongeacht hoe lang dat het geval was.`,
        name: 'Attribuutnaam',
        userEditable: 'Door gebruiker bewerkbaar',
    },
    backup: {
        createBackup: 'Back-up maken',
        disabledDesc:
            'Deze functionaliteit bestaat alleen als Hiqlite is geconfigureerd als de database.',
        lastModified: 'Laatst gewijzigd',
        local: 'Lokaal',
        name: 'Naam',
        size: 'Grootte',
    },
    clients: {
        allowedResources: 'Toegestane resources',
        defaultAud: 'Standaard audiences',
        descAllowedResources: `Optionele RFC 8707 resource-indicatoren die deze client mag opvragen. Een lege lijst weigert elke 'resource'-parameter met 'invalid_target'.`,
        descDefaultAud: `Audiences die altijd aan de tokens van deze client worden toegevoegd, onafhankelijk van een 'resource'-parameter.`,
        backchannelLogout:
            'Als deze client {{ OIDC_BCL }} ondersteunt, kunt u de URI hier opgeven.',
        branding: {
            descHsl: `De volgende waarden moeten worden opgegeven als HSL-waarden. U geeft alleen de basiskleuren op.
            Alfakanalen en andere waarden worden dynamisch gemanipuleerd door het thema.`,
            descFullCss: `De volgende waarden moeten volledig geldige CSS <code>color</code>-waarden zijn.
                U kunt ook complexe berekeningen of de hierboven gedefinieerde CSS-variabelen gebruiken.`,
            descVariables: `Elk volgend label is tegelijkertijd de naam van de CSS-variabele. Dit betekent,
                dat u er in de vrije invoervelden naar kunt verwijzen, bijv. met <code>hsla(var(--action) / .7)</code>.`,
        },
        claims: 'Custom claims (client_credentials)',
        claimsAtRoot: 'Emit claims at the token root',
        claimsAtRootWarning: `This client's claims are written at the token root
            instead of being nested under 'custom'. You own collision-correctness: if a claim
            name collides with a reserved JWT claim, token issuance fails. Root-level custom
            claims may also break with future protocol or feature changes. Reference:`,
        claimsDesc: `A JSON object emitted into client_credentials tokens, nested under the
            custom claim. Max 1024 serialized characters.`,
        confidential: 'Vertrouwelijk',
        confidentialNoSecret: 'Dit is een niet-vertrouwelijke client en heeft daarom geen geheim.',
        config: 'Clientconfiguratie',
        delete1: 'Weet u zeker dat u deze client wilt verwijderen?',
        descAuthCode: `De geldigheid van auth-codes kan worden aangepast voor extra beveiliging. Auth-codes
            kunnen slechts eenmaal worden gebruikt en zijn standaard 60 seconden geldig. Hoe korter de geldigheid, hoe
            beter, zolang de client de loginprocedure snel genoeg kan uitvoeren.`,
        descClientUri: `Informatie over de URI en contacten van deze client voor weergave op
            de loginpagina.`,
        descName: `De clientnaam kan worden gewijzigd zonder invloed op de clientconfiguratie.
            Het bestaat alleen om te worden weergegeven op de loginpagina.`,
        descGroupPrefix: `De login voor deze client kan worden beperkt door een optioneel groepsprefix.
            Alleen gebruikers die zijn toegewezen aan een overeenkomende groep, mogen inloggen.`,
        descOrigin: `Externe, aanvullend toegestane origins - meestal alleen nodig als deze client
            rechtstreeks vanuit de browser verzoeken moet doen aan Rauthy, typisch SPAs.`,
        descPKCE: `Als de client het ondersteunt, moet u altijd S256 PKCE activeren voor extra
            beveiliging. Als een niet-vertrouwelijke client (bijv. een SPA) wordt gebruikt, moet u minimaal
            één van de PKCE-uitdagingen activeren voor voldoende beveiliging.`,
        descPKCEEnforce: `Als PKCE is geactiveerd, dwingt Rauthy het gebruik af tijdens logins en
            weigert loginverzoeken die geen geldige uitdaging bevatten.`,
        descUri: `U kunt zoveel omleidings-URI's opgeven als u wilt. Aan het einde van elk kunt u
            <code>*</code> gebruiken als jokerteken.`,
        errConfidentialPKCE: `De client moet vertrouwelijk zijn of minimaal één PKCE-uitdaging
            geactiveerd hebben.`,
        forceMfa: 'MFA verplichten',
        groupLoginPrefix: 'Login-groepsprefix',
        name: 'Clientnaam',
        scim: {
            baseUri: `De SCIM-basis-URI is die waarvan de subroutes zoals
                <code>{base_uri}/Users/{id}</base_uri></code> correct kunnen worden afgeleid.`,
            desc: 'Als deze client {{ SCIM_LINK }} ondersteunt, kunt u het hier activeren.',
            enable: 'SCIMv2 inschakelen',
            groupSync: 'Groepen synchroniseren',
            groupSyncPrefix: 'Groepfilterprefix',
            groupSyncPrefixDesc: `U kunt de groepen voor synchronisatie filteren op een optioneel prefix.
                Als er bijvoorbeeld groepen <code>app:admins</code> en <code>app:users</code> bestaan, zou het prefix
                 <code>app:</code> alleen deze groepen synchroniseren, evenals alleen die gebruikers die zijn gekoppeld
                 aan minimaal één van deze groepen.`,
            reqDesc: 'Een paar dingen zijn vereist voor compatibiliteit:',
            reqLi1: 'De client moet <code>externalId</code> correct verwerken.',
            reqLi2: `Minimaal <code>/Users</code>-eindpunten met <code>filter=externalId eq "*"</code> en
                <code>filter=userName eq "*"</code> moeten worden ondersteund.`,
            reqLi3: `Als groepen moeten worden gesynchroniseerd, moet <code>/Groups</code> ook
                <code>filter=displayName eq "*"</code> ondersteunen.`,
        },
        scopes: {
            allowed: 'Toegestane scopes',
            default: 'Standaardscopes',
            desc: `Toegestane scopes zijn degenen die de client dynamisch mag aanvragen tijdens
            een omleiding naar de login bij gebruik van de <code>authorization_code</code>-flow. De standaard
            scopes worden altijd toegevoegd aan de tokens om sommige problemen op te lossen bij gebruik van de
            <code>password</code>-flow.`,
        },
        secret: {
            doCache: 'Clientgeheim cachen',
            cacheDuration: 'Cacheduur (uren)',
            generate: 'Nieuw geheim genereren',
            rotateDesc1: `Om naadloze updates en geheimrotaties mogelijk te maken, heeft u de mogelijkheid om het
                huidige geheim voor enige tijd in een in-memory cache te bewaren. U kunt een waarde invoeren tussen
                1 en 24 uur.`,
            rotateDesc2: 'Let op: U moet het huidige geheim niet cachen als er een lek was!',
        },
        tokenLifetime: {
            p1: `De token-levensduur is van toepassing op Access- en ID-tokens en wordt opgegeven in seconden.`,
            p2: `Als de client EdDSA / ed25519-algoritmen ondersteunt, moet dit altijd de voorkeurskeuze
                zijn. RSA-algoritmen bestaan alleen voor compatibiliteit.`,
            p3: `Het algoritme voor vernieuwingstokens kan niet worden gewijzigd, omdat deze alleen door Rauthy
                worden gebruikt.`,
        },
    },
    common: {
        account: 'Account',
        addNew: 'Nieuw toevoegen',
        back: 'Terug',
        caution: 'LET OP',
        contact: 'Contact',
        copiedToClip: 'Waarde is gekopieerd naar klembord',
        details: 'Details',
        edit: 'Bewerken',
        enabled: 'Ingeschakeld',
        filter: 'Filter',
        from: 'Van',
        information: 'Informatie',
        language: 'Taal',
        loading: 'Laden',
        jsonMeta: 'Metagegevens als JSON-waarde',
        name: 'Naam',
        nameExistsAlready: 'Naam bestaat al',
        note: 'Notitie',
        noEntries: 'Geen vermeldingen',
        preview: 'Voorvertoning',
        reset: 'Resetten',
        searchOptions: 'Zoekopties',
        until: 'Tot',
    },
    docs: {
        book: 'Voor algemene documentatie over Rauthy zelf, kunt u het beste het',
        encryption: 'Versleuteling',
        encKeys: {
            header: 'Versleutelingssleutels',
            keyActive: 'Actieve sleutel',
            keysAvailable: 'Beschikbare sleutels',
            migrate: 'Migreren',
            migrateToKey: 'Migreer alle bestaande versleutelde waarden naar de volgende sleutel',
            p1: `Deze sleutels worden gebruikt voor aanvullende versleuteling in rust, onafhankelijk van de gebruikte
            databasetechnologie. Ze zijn statisch geconfigureerd, maar kunnen handmatig worden geroteerd en
            gemigreerd op deze pagina.`,
            p2: `De actieve sleutel is statisch ingesteld in het Rauthy-configuratiebestand / omgevingsvariabelen.
            Het kan hier niet dynamisch worden gewijzigd. Alle nieuwe JWK-versleutelingen zullen altijd de
            huidige actieve sleutel gebruiken.`,
            p3: `Als u alle bestaande geheimen migreert, kan het een paar seconden duren als u een grote dataset heeft.`,
            pNotPossible:
                'Om te kunnen migreren, moeten er minimaal 2 versleutelingssleutels beschikbaar zijn.',
        },
        hashing: {
            calculate: 'Berekenen',

            currValuesHead: 'Huidige waarden',
            currValues1: 'De huidige waarden van de backend zijn als volgt:',
            currValuesNote: `Opmerking: De logintijd van de backend geeft pas een goede richtlijn nadat minimaal 5
            succesvolle logins hebben plaatsgevonden nadat Rauthy is gestart. De basiswaarde is altijd 2000 ms na
            een herstart en past zich aan met elke succesvolle login.`,
            currValuesThreadsAccess: 'Threads (p_cost) waar Rauthy toegang toe heeft',

            loginTimeHead: 'Een woord over logintijd',
            loginTime1: `Over het algemeen willen gebruikers alles zo snel mogelijk. Bij een veilige login is echter
            een tijd van 500 - 1000 ms geen probleem. De logintijd mag niet te kort zijn, omdat dit de sterkte
            van de hash zou verminderen.`,
            loginTime2: `Om standaard zoveel mogelijk veiligheid te bieden, staat dit hulpprogramma niet toe dat u
            onder de 500 ms gaat voor de logintijd.`,

            mCost1: `De <code>m_cost</code> definieert de hoeveelheid <b>geheugen (in kB)</b> die wordt gebruikt
            voor het hashen. Hoe hoger de waarde, hoe beter. Maar u moet de serverresources in gedachten houden.<br>
            Als u 4 wachtwoorden tegelijkertijd hasht, heeft de backend <code>4 x m_cost</code>
            nodig tijdens het hashen. Deze resources moeten beschikbaar zijn.`,
            mCost2: `Het afstemmen van <code>m_cost</code> is vrij eenvoudig. Definieer de maximale hoeveelheid
            geheugen die Rauthy mag gebruiken, deel het door het aantal maximaal toegestane parallelle logins
            (<code>MAX_HASH_THREADS</code>) en trek een kleine statische hoeveelheid geheugen af. Hoeveel statisch
            geheugen in aanmerking moet worden genomen, hangt af van de gebruikte database en het totale aantal
            gebruikers, maar ligt doorgaans in het bereik van 32 - 96 MB.`,
            mCost3: 'De minimaal toegestane <code>m_cost</code> is <code>32768</code>.',

            pCost1: `De <code>p_cost</code> definieert de hoeveelheid <b>parallelisme</b> voor het hashen.
            Deze waarde topt doorgaans op ~8, wat de standaard is voor Rauthy.`,
            pCost2: `De algemene regel is:<br>
            Stel de <code>p_cost</code> in op het dubbele aantal beschikbare cores.<br>
            Als u bijvoorbeeld 4 cores beschikbaar heeft, stel de <code>p_cost</code> in op <code>8</code>.<br>
            Deze waarde moet echter rekening houden met de geconfigureerde toegestane parallelle logins
            (<code>MAX_HASH_THREADS</code>) en dienovereenkomstig worden verminderd.`,

            tCost1: `De <code>t_cost</code> definieert de hoeveelheid <b>tijd</b> voor het hashen. Dit is de
            enige waarde die afgesteld moet worden, omdat <code>m_cost</code> en <code>p_cost</code> in principe
            worden bepaald door de omgeving.`,
            tCost2: `Afstemmen is eenvoudig: stel <code>m_cost</code> en <code>p_cost</code> in en verhoog dan
            <code>t_cost</code> totdat u uw hashtime-doel heeft bereikt.`,

            utilityHead: 'Hulpprogramma voor parameterberekening',
            utility1: `U kunt dit hulpprogramma gebruiken om goede waarden te berekenen voor uw implementatie.
            Houd er rekening mee dat dit moet worden uitgevoerd met Rauthy op zijn uiteindelijke locatie met alle
            uiteindelijke resources beschikbaar. U moet dit hulpprogramma uitvoeren onder belasting om niet te
            hoog af te stellen.`,
            utility2: `<code>m_cost</code> is optioneel en de veilige minimumwaarde van <code>32768</code> zou
            worden gekozen als leeg. <code>p_cost</code> is ook optioneel en Rauthy zal alle threads gebruiken
            die het kan zien als leeg.`,

            time: 'Tijd',
            targetTime: 'Doeltijd',
            tune: 'Belangrijk: Deze waarden moeten worden afgestemd op de uiteindelijke architectuur!',
            pDetials: `Voor een gedetailleerde inleiding tot Argon2ID zijn er veel bronnen online beschikbaar.
            Deze handleiding geeft slechts een zeer kort overzicht van de waarden. Drie ervan moeten worden
            geconfigureerd:`,
            pTune: `Ze veranderen afhankelijk van de mogelijkheden van het systeem. Hoe krachtiger het systeem,
            hoe veiliger de waarden kunnen zijn.`,
            pUtility: `Dit hulpprogramma helpt u de beste Argon2ID-instellingen voor uw platform te vinden.
            Argon2ID is momenteel het veiligste beschikbare wachtwoord-hashing-algoritme. Om het volledige
            potentieel te benutten, moet het voor elke implementatie worden afgestemd.`,
        },
        openapi:
            "Als u een externe applicatie wilt integreren en Rauthy's API wilt gebruiken, kijk dan naar het",
        openapiNote: `Afhankelijk van de backendconfiguratie is de Swagger UI mogelijk niet publiekelijk
            beschikbaar op dit punt. Het is echter standaard beschikbaar via de interne metrics HTTP-server
            om geen informatie bloot te stellen.`,
        source: 'De broncode is hier te vinden',
    },
    editor: {
        bold: 'Vet',
        code: 'Code',
        heading1: 'Kop 1',
        heading2: 'Kop 2',
        heading3: 'Kop 3',
        italic: 'Cursief',
        link: 'Link',
        listBullet: 'Lijst',
        listTasks: 'Taken',
        listNumbered: 'Genummerde lijst',
        paragraph: 'Alinea',
        quote: 'Citaat',
        removeFmt: 'Opmaak verwijderen',
        strikeThrough: 'Doorhalen',
        textArea: 'Tekst bewerken',
    },
    email: {
        cancelJob: 'Taak annuleren',
        filterType: ['Geen', 'In groep', 'Niet in groep', 'Heeft rol', 'Heeft rol niet'],
        immediate: 'Direct',
        jobs: 'E-Mail taken',
        scheduled: 'Geplande verzending',
        sendAllUsers: 'Deze e-mail wordt naar alle gebruikers gestuurd.',
        sendAllUsersFiltered: 'Deze e-mail wordt naar alle gebruikers gestuurd gefilterd op:',
        sendMail: 'E-Mail versturen',
        subject: 'Onderwerp',
        userFilter: 'Gebruikersfilter',
    },
    error: {
        needsAdminRole: `U bent niet toegewezen aan de <b>rauthy_admin</b>-rol.<br/>
            U heeft geen toegang tot het beheerpaneel.`,
        noAdmin: `Een Rauthy-beheerdersaccount moet <b>MFA ingeschakeld hebben.</b><br>
            Navigeer naar uw <b>account</b> en activeer MFA.<br>
            Daarna moet u uitloggen en opnieuw inloggen.`,
    },
    events: {
        eventLevel: 'Gebeurtenisniveau',
        eventType: 'Gebeurtenistype',
    },
    groups: {
        delete1: 'Weet u zeker dat u deze groep wilt verwijderen?',
        name: 'Groepsnaam',
    },
    jwks: {
        alg: 'Algoritme',
        p1: 'Dit zijn de Json Web Keys (JWKs) die worden gebruikt voor het ondertekenen van tokens.',
        p2: `De JWKs worden standaard elke 1e van de maand geroteerd. Voor alle nieuw aangemaakte tokens wordt
        alleen de laatste beschikbare sleutel voor het gegeven algoritme gebruikt voor ondertekening. Oude sleutels
        worden een tijdje bewaard om ervoor te zorgen dat momenteel geldige tokens nog steeds correct kunnen worden
        gevalideerd. Na een tijdje worden ze automatisch opgeschoond.`,
        p3: `Sleutels kunnen ook handmatig worden geroteerd. Afhankelijk van de hardware waarop deze
        Rauthy-instantie draait, kan het een paar seconden duren.`,
        type: 'Type',
        rotateKeys: 'Sleutels roteren',
    },
    kv: {
        accessTestDesc: `De toegangssleutel moet worden opgegeven in de <code>Authorization</code>-header als
            <code>Bearer</code>-token. De volgende <code>curl</code>-opdracht kan worden gebruikt voor testen.`,
        addNewKey: 'Nieuwe toegangssleutel',
        addNewNs: 'Nieuwe naamruimte',
        addNewValue: 'Nieuwe waarde',
        delConfirm: 'Weet u zeker dat u deze toegangssleutel wilt verwijderen?',
        delNsMsg:
            'Weet u zeker dat u deze naamruimte inclusief alle bestaande gegevens wilt verwijderen?',
        encryptedDesc: `Om prestatieredenen mag de aanvullende versleuteling alleen worden gebruikt voor
            bijzonder gevoelige waarden zoals toegangssleutels of persoonlijke informatie.`,
        deleteConfirmMsg: `Weet u zeker dat u de sleutel '{{ key }}' wilt verwijderen?`,
        help: {
            help: 'Help',
            ops: [
                'Toegangssleutel testen',
                'Alle bestaande sleutels ophalen',
                'Alle bestaande sleutels + waarden ophalen',
                'Een sleutel + waarde instellen',
                'De waarde voor een sleutel ophalen',
                'Een sleutel verwijderen',
            ],
            p1: `De externe toegang tot KV-opslag is bewust heel eenvoudig gehouden. Er zijn slechts
                een paar bewerkingen:`,
            p2: `Elk van deze bewerkingen vereist een toegangssleutel als <code>Bearer</code>-token
                (<code>{id}\${secret}</code>) in de <code>Authorization</code>-header. Een toegangssleutel
                is alleen geldig voor de naamruimte waarbinnen ze bestaat.`,
            p3: `Hier ziet u voorbeelden voor de bovengenoemde bewerkingen met <code>curl</code>.`,
        },
        key: 'Sleutel',
        loadAllValues: 'Alle waarden laden',
        storeEncrypted: 'Waarde versleuteld opslaan',
        tabs: ['Gegevens', 'Toegang', 'Bewerken', 'Verwijderen'],
        testCmd: 'Testopdracht',
        value: 'JSON-waarde',
    },
    nav: {
        apiKeys: 'API-sleutels',
        attributes: 'Attributen',
        blacklist: 'Zwarte lijst',
        clients: 'Clients',
        config: 'Configuratie',
        docs: 'Documentatie',
        events: 'Gebeurtenissen',
        groups: 'Groepen',
        providers: 'Providers',
        roles: 'Rollen',
        scopes: 'Scopes',
        sessions: 'Sessies',
        users: 'Gebruikers',
    },
    options: {
        expires: 'Vervalt',
        lastSeen: 'Laatste activiteit',
        state: 'Status',
    },
    pam: {
        addGroup: 'Nieuwe PAM-groep',
        addHost: 'Nieuwe PAM-host',
        addUser: 'Nieuwe PAM-gebruiker',
        deleteHost: 'Weet u zeker dat u deze host wilt verwijderen?',
        deleteUser: 'Weet u zeker dat u deze gebruiker wilt verwijderen?',
        groupDescGeneric: `Generieke groepen zijn het equivalent van vermeldingen die gewoonlijk in /etc/group
            staan. Gebruikers kunnen hieraan worden toegewezen en ze worden door NSS-opzoekingen aan het systeem
            teruggegeven.`,
        groupDescHost: `Hostgroepen worden gebruikt om hosts te groeperen. NSS-opzoekingen van een host binnen de
            groep geven alle andere hosts daarin als resultaat terug. Gebruikers kunnen hosts benaderen door ze aan
            een hostgroep toe te wijzen.`,
        groupDescLocal: `Lokale groepen gedragen zich bijna identiek aan generieke groepen, met het verschil dat ze
            een ID in de Rauthy-database hebben, maar de NSS-proxy op de betreffende host deze omzet naar een ID uit
            /etc/group. Op deze manier kunnen Rauthy-gebruikers worden toegewezen aan groepen die al lokaal bestaan.`,
        groupDescUser: `Gebruikersgroepen worden automatisch beheerd en zijn sterk gekoppeld aan de gebruiker met
            dezelfde gebruikersnaam.`,
        groupDescWheel: `Deze groep is speciaal. Ze is onveranderlijk en wordt dynamisch aan gebruikers toegewezen
            afhankelijk van hun groepsconfiguratie.`,
        groupName: 'Groepsnaam',
        groups: 'Groepen',
        groupType: 'Groepstype',
        hostAliases: 'Hostaliasen',
        hostLocalPwdOnly: 'Lokaal wachtwoord inloggen',
        hostLocalPwdOnlyInfo: `Wanneer Lokaal wachtwoord inloggen is ingesteld, overschrijft dit MFA verplichten voor
            lokale logins. Tegelijkertijd zullen passkeys nooit worden gevraagd (lokaal) tijdens logins, zelfs als een
            gebruiker MFA-beveiligd is. Deze optie mag alleen worden ingesteld als het echt noodzakelijk is,
            bijvoorbeeld als MFA-beveiligde gebruikers lokale logins moeten kunnen doen zonder hardware passkeys
            te gebruiken.`,
        ipAddresses: 'IP-adressen',
        member: 'Lid',
        nameExistsAlready: 'Naam is al bezet',
        notes: 'Notities',
        secretShow: 'Geheim tonen',
        secretRotate: 'Geheim roteren',
        userEmail: 'Gekoppeld gebruikersmailadres',
        username: 'Gebruikersnaam',
        usernameNewDesc: `De gebruikersnaam moet zorgvuldig worden gekozen. Eenmaal aangemaakt kan deze niet
            gemakkelijk achteraf worden gewijzigd om veiligheidsredenen.`,
    },
    passwordPolicy: {
        configDesc: 'Beleid voor nieuwe wachtwoorden.',
        resetSet0: 'De waarde 0 deactiveert de vereiste.',
        validForDays: 'Geldig voor dagen',
        validityNew: 'Geldigheid voor nieuwe wachtwoorden.',
    },
    providers: {
        config: {
            allowInsecureTls: 'Onveilige TLS toestaan',
            autoLink: 'Gebruiker automatisch koppelen',
            autoLinkDesc1: `Als Gebruiker automatisch koppelen is geactiveerd, wordt bij inloggen via deze provider
                automatisch een mogelijk bestaande, niet-gekoppelde gebruiker aan deze provider gekoppeld.`,
            autoLinkDesc2: `LET OP: Deze optie kan zeer gevaarlijk zijn en leiden tot accountovername als de provider
                e-mailadressen niet volledig valideert voor gebruikers en het daardoor mogelijk maakt een vreemd
                adres voor een gebruiker toe te voegen! MAG NOOIT in zo'n geval worden gebruikt!`,
            clientName: 'Clientnaam',
            custRootCa: 'Aangepaste root CA PEM',
            descAuthMethod: `De authenticatiemethode voor het <code>/token</code>-eindpunt.<br>
                De meeste providers werken met <code>basic</code>, sommige alleen met <code>post</code>.
                In zeldzame situaties heeft u beide nodig, terwijl het bij anderen tot fouten kan leiden.`,
            descClientId: 'Client-ID opgegeven door de auth-provider.',
            descClientName: 'Clientnaam die op de Rauthy-inlogpagina moet worden getoond.',
            descClientSecret: `Clientgeheim opgegeven door de auth-provider.
                Minimaal een clientgeheim of PKCE is vereist.`,
            descScope: `Het bereik dat de client moet gebruiken bij het omleiden naar de inlogpagina.
                Geef de waarden gescheiden door spaties op.`,
            errNoAuthMethod:
                'U heeft een clientgeheim opgegeven, maar er is geen clientauthenticatiemethode actief',
            errConfidential: 'Moet minimaal een vertrouwelijke client zijn of PKCE gebruiken',
            jsonPath: {
                p1: 'Waarden uit het ID-token na een succesvolle upstream-login kunnen automatisch worden toegewezen.',
                p2: `Het <code>pad</code> moet worden opgegeven in een regex-achtige syntaxis. Het kan worden omgezet
                    naar enkelvoudige JSON-waarden of naar een waarde in een JSON-object of -array.`,
                p3: '<code>$.</code> markeert het begin van het JSON-object',
                p4: '<code>*</code> kan worden gebruikt als jokerteken in uw pad',
                p5: '<code>$.roles</code> zou <code>&#123;"roles": "value"&#125;</code> targeten',
                p6: `<code>$.roles.*</code> kan een waarde targeten in een object of array zoals<br>
                    <code>&#123;"roles": ["value", "notMyValue"]&#125;</code>`,
            },
            lookup: 'Opzoeken',
            pathAdminClaim: 'Beheerdersclaimpad',
            pathMfaClaim: 'MFA-claimpad',
            rootPemCert: 'Root PEM-certificaat',
            mapMfa: `Als uw provider een claim uitgeeft die aangeeft dat de gebruiker minimaal 2FA heeft gebruikt
                tijdens inloggen, kunt u het MFA-claimpad opgeven.`,
            mapUser: `U kunt een gebruiker toewijzen als Rauthy-beheerder op basis van een upstream ID-claim.`,
            valueAdminClaim: 'Beheerdersclaimwaarde',
            valueMfaClaim: 'MFA-claimwaarde',
        },
        delete: {
            areYouSure: 'Weet u zeker dat u deze provider wilt verwijderen?',
            forceDelete: 'Verwijderen forceren',
            isInUse1: 'Deze provider wordt gebruikt door actieve gebruikers!',
            isInUse2: `U kunt het geforceerd verwijderen, maar gebruikers zonder een lokaal wachtwoord of passkey
                kunnen niet meer inloggen.`,
            linkedUsers: 'Gekoppelde gebruikers',
        },
    },
    roles: {
        adminNoMod: 'De <code>rauthy_admin</code>-rol is onveranderlijk.',
        delete1: 'Weet u zeker dat u deze rol wilt verwijderen?',
        name: 'Rolnaam',
    },
    scopes: {
        claimsAtRoot: 'Claims op tokenroot plaatsen',
        claimsAtRootWarning: `Wanneer ingeschakeld, worden de toegewezen attributen van dit bereik op de tokenroot
            geschreven in plaats van te worden genest onder 'custom'. U bent verantwoordelijk voor correcte
            botsingen: als een toegewezen attribuutnaam botst met een gereserveerde JWT-claim, mislukt de
            tokenuitgifte. Root-niveau aangepaste claims kunnen ook breken bij toekomstige protocol- of
            functiewijzigingen. Referentie:`,
        defaultNoMod: 'Dit is een standaard OIDC-scope. Deze zijn onveranderlijk.',
        delete1: 'Weet u zeker dat u dit bereik wilt verwijderen?',
        deleteDefault: 'Standaard OIDC-scopes kunnen niet worden verwijderd.',
        mapping1: 'U kunt aangepaste scopes toewijzen aan attributen.',
        mapping2: `Alle aanvullende attributen die zijn geconfigureerd kunnen een aangepaste waarde hebben voor elke
            gebruiker. Wanneer ze zijn toegewezen aan een scope, kunnen ze worden opgenomen in de Access- en/of
            ID-tokens.`,
        name: 'Scopenaam',
    },
    search: {
        orderBy: 'Sorteren op ...',
        orderChangeToAsc: 'Sortering wijzigen naar oplopend',
        orderChangeToDesc: 'Sortering wijzigen naar aflopend',
    },
    sessions: {
        invalidateAll: 'Alle sessies ongeldig maken',
    },
    tabs: {
        config: 'Configuratie',
        delete: 'Verwijderen',
    },
    tos: {
        accepted: 'Geaccepteerd',
        addNewToS: 'Nieuwe gebruiksvoorwaarden',
        addNewToSFromCurrent: 'Nieuwe gebruiksvoorwaarden van geselecteerde',
        added: 'Toegevoegd',
        checkStatus: 'Gebruikersstatus controleren',
        immutable: `LET OP: Na het toevoegen van nieuwe gebruiksvoorwaarden zijn deze onveranderlijk en kunnen ze
				niet worden verwijderd!`,
        noneExist: 'Er zijn nog geen gebruiksvoorwaarden toegevoegd.',
        optUntil: {
            desc: `Tijdens de overgangstijd is het accepteren van bijgewerkte gebruiksvoorwaarden optioneel.
					Het wordt pas daarna verplicht.`,
            enable: 'Overgangstijd inschakelen',
            label: 'Einde overgangstijd',
        },
        tos: 'Gebruiksvoorwaarden',
    },
    users: {
        antiLockout: {
            rule: 'Anti-vergrendelregel',
            delete: 'kan niet worden verwijderd',
            disable: 'kan niet worden uitgeschakeld',
            rauthyAdmin: 'rauthy_admin-rol kan niet worden verwijderd',
        },
        attributes: 'Attributen',
        deleteUser: 'Weet u zeker dat u deze gebruiker wilt verwijderen?',
        descAttr: `Stel aangepaste gebruikersattributen in. Alle sleutel/waarde-paren worden verwerkt als tekenreeks/JSON-waarde.`,
        forceLogout: `Weet u zeker dat u alle bestaande sessies wilt ongeldig maken en alle vernieuwingstokens
            voor deze gebruiker wilt verwijderen?`,
        groupAdmin: {
            notManagedTitle: 'Gebruiker buiten uw groepen',
            notManagedDesc: `Deze gebruiker is geen lid van een groep die u beheert, dus de details blijven verborgen.
                Voeg de gebruiker toe aan een of meer van uw groepen om deze te beheren.
                Lidmaatschappen buiten uw groepen blijven behouden.`,
            addToGroups: 'Aan mijn groepen toevoegen',
        },
        lastLogin: 'Laatste login',
        manualInitDesc: `De gebruiker kan ook hier worden geïnitialiseerd. In dit geval moet u het wachtwoord
            echter direct communiceren.`,
        manualInit: 'Handmatig initialiseren',
        mfaDelete1: 'U kunt Passkeys voor deze gebruikers verwijderen.',
        mfaDelete2: `Let op! Het verwijderen van een Passkey <b>kan niet ongedaan worden gemaakt</b> zonder dat
            de gebruiker een volledig nieuwe registratie uitvoert.`,
        noMfaKeys: 'Deze gebruiker heeft geen geregistreerde Passkeys.',
        pkOnly1: 'Dit is een passkey-only account.',
        pkOnly2:
            'Dit betekent dat deze gebruiker de wachtwoordloze login gebruikt en helemaal geen wachtwoord heeft ingesteld.',
        pkOnly3: `Als deze gebruiker alle Passkeys heeft verloren, kan het account volledig worden gereset en kan er
            een nieuw wachtwoord-reset e-mail worden verstuurd. Om dit te bereiken, navigeert u naar het tabblad
            'MFA' en verwijdert u alle bestaande passkeys.`,
        pwdNoInit: 'De gebruiker heeft de initiële wachtwoordreset nog niet uitgevoerd.',
        pwdSendEmailBtn: 'Reset e-mail versturen',
        pwdSendEmailDesc:
            'U kunt een nieuwe reset-e-mail sturen als de gebruiker er geen heeft ontvangen.',
        savePassword: 'Wachtwoord opslaan',
        selfServiceDesc: 'U kunt een nieuw wachtwoord instellen of een reset-e-mail versturen.',
        sendResetEmail: 'Reset e-mail versturen',
    },
    validation: {
        css: 'Ongeldige CSS-waarde',
        origin: 'Ongeldige origin',
        uri: 'Ongeldige URI',
    },
};
