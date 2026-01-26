import type { I18nAdmin } from './interface.ts';

export let I18nAdminNb: I18nAdmin = {
    api_key: {
        delete1: 'Skal denne API nøkkelen slettes?',
        expires: 'Utløper',
        generate1: 'Her kan man opprette et nytt Secret for denne API nøkkelen.',
        generate2: `Secreten vises kun en gang etter opprettelse.
            Hvis en ny genereres, vil den gamle umiddelbart og permanent overskrives.
            Denne operasjonen kan ikke angres!`,
        generate3: `API nøkkelen må sendes i HTTP <code>Authorization</code> headeren i følgende
            format:`,
        generate4: 'Følgende <code>curl</code> spørringen kan brukes til testing av nøkkelen:',
        generate5: 'Hivs <code>jq</code> ikke er installiert, bruk denne versjonen uten:',
        keyName: 'Nøkkel navn',
        limitedValidity: 'Begrenset gyldighet',
    },
    attrs: {
        delete1: 'Skal dette attributtet slettes?',
        defaultValue: 'Standardverdi',
        desc: 'Beskrivelse',
        makeEditable: 'Gjør redigerbart',
        makeEditableP1: 'Dette attributtet kan gjøres redigerbart av brukere.',
        makeEditableP2: `<b>ADVARSEL:</b> Denne endringen kan aldri angres! Alle opplysninger gitt direkte av en
            bruker er alltid uvalidert og må ALDRI brukes for noen form for autentisering
            eller autorisasjon!`,
        makeEditableP3: `Et attributt kan derfor aldri endres fra redigerbart til ikke-redigerbart, fordi
        for en viss tid, uavhengig av varighet, ble uvalidert input tillatt.`,
        name: 'Attribut navn',
        userEditable: 'Redigerbart av brukere',
    },
    backup: {
        createBackup: 'Lag Backup',
        disabledDesc:
            'Disse funksjonene er kun tilgjengelige hvis Hiqlite er konfigurert som database.',
        lastModified: 'Sist endret',
        local: 'Lokal',
        name: 'Navn',
        size: 'Størrelse',
    },
    clients: {
        backchannelLogout: 'Hvis denne klienten støtter {{ OIDC_BCL }}, kan URIen angis her.',
        branding: {
            descHsl: `Fargene må angis som HSL. Her defineres kun basisfargen.
                Alpha kanaler og andre verdier justeres dynamisk av temaet.`,
            descFullCss: `Fargene må angis som fullstendige, gyldige CSS <code>color</code> verdier.
                Kompleks kalkulasjon og definerte CSS variablene over kan også brukes.`,
            descVariables: `Hver påfølgende etikett er samtidig navnet på CSS variabelen. Det betyr at
                for eksempel fritekst kan referere til variablene, like som med
                <code>hsla(var(--action) / .7)</code>.`,
        },
        confidential: 'Følsomt',
        confidentialNoSecret: 'Dette er ikke en følsom klient, og har derfor ingen hemmelighet.',
        config: 'Klient konfigurasjon',
        delete1: 'Skal denne klienten slettes?',
        descAuthCode: `Gyldigheten til Auth kodene kan justeres for å oppnå ekstra sikkerhet.
            Auth kodene kan kun brukes en gang og er normalt gyldige i 60 sekunder. Jo kortere, jo bedre
            så lenge klienten kan bruke koden raskt nok.`,
        descClientUri: `Informasjon om URI og kontakter for denne klienten, som vises
            på innloggingssiden.`,
        descName: `Klientnavnet kan endres uten å påvirke konfigurasjonen.
            Det brukes kun for visning på innloggingssiden.`,
        descGroupPrefix: `Pålogging for denne klienten kan begrenses ved et valgfritt gruppeprefiks.
            Kun brukere som tilhører den tilsvarende gruppen får lov til å logge inn på denne klienten.`,
        descOrigin: `Eksterne, ytterligere tillatte Origins - vanligvis bare nødvendig hvis denne
            klienten må gjøre forespørsler til Rauthy direkte fra nettleseren, typisk SPAs.`,
        descPKCE: `Hvis klienten støtter PKCE, bør S256 PKCE alltid aktiveres for ekstra sikkerhet.
            Hvis en ikke-følsom klient (f.eks. en SPA) brukes, må minst én PKCE Challenge aktiveres for å
            tilby tilstrekkelig sikkerhet.`,
        descPKCEEnforce: `Hvis PKCE er aktivert, håndhever Rauthy bruken og nekter pålogginger
            som ikke gir en gyldig Challenge.`,
        descUri: `Det kan angis et vilkårlig antall Redirect URIs. På slutten av hver kan
            valgfritt <code>*</code> aksepteres som en jokertegn.`,
        errConfidentialPKCE: `Klienten må enten være følsom eller ha minst én PKCE
            Challenge aktivert.`,
        forceMfa: 'Tving MFA',
        groupLoginPrefix: 'Gruppepåloggingsprefiks',
        name: 'Klientnavn',
        scim: {
            baseUri: `SCIM Base URI'en må være den som underordnede ruter som
                <code>{base_uri}/Users/{id}</code> kan avledes korrekt fra.`,
            desc: 'Hvis denne klienten støtter {{ SCIM_LINK }}, kan den aktiveres her.',
            enable: 'Aktiver SCIMv2',
            groupSync: 'Synkroniser grupper',
            groupSyncPrefix: 'Gruppe filterprefiks',
            groupSyncPrefixDesc: `Grupper som bør synkroniseres kan filtreres med et valgfritt prefiks.
                Hvis det for eksempel finnes grupper som <code>app:admins</code> og <code>app:users</code>,
                vil prefikset <code>app:</code> sørge for at bare disse gruppene synkroniseres, og bare
                brukere som tilhører minst én av disse gruppene.`,
            reqDesc: 'Følgende betingelser må være oppfylt for kompatibilitet:',
            reqLi1: 'Klienten må håndtere <code>externalId</code> korrekt.',
            reqLi2: `Minst <code>/Users</code> endepunkter med <code>filter=externalId eq "*"</code> og
                <code>filter=userName eq "*"</code> må støttes.`,
            reqLi3: `Hvis grupper skal synkroniseres, må i tillegg under <code>/Groups</code>
                <code>filter=displayName eq "*"</code> støttes.`,
        },
        scopes: {
            allowed: 'Tillatte Scopes',
            default: 'Standard Scopes',
            desc: `Tillatte Scopes er de som klienten dynamisk kan be om ved
            omdirigering til innlogging i <code>authorization_code</code> flyten.
            Standard Scopes legges derimot alltid til og kan løse problemer hvis
            for eksempel <code>password</code> flyten brukes.`,
        },
        secret: {
            doCache: 'Cacher klient hemmelighet',
            cacheDuration: 'Cache lengde (timer)',
            generate: 'Generer nytt hemmelighet',
            rotateDesc1: `For å muliggjøre oppdateringer uten nedetid, kan den eksisterende hemmeligheten
                beholdes i in-memory cachen i en viss tid. En verdi mellom 1 og 24 timer kan angis.`,
            rotateDesc2:
                'ADVARSEL: Den nåværende hemmeligheten bør ikke beholdes i cachen hvis det har vært et lekkasje!',
        },
        tokenLifetime: {
            p1: 'Tokenets levetid brukes for Access og ID Tokens og angis i sekunder.',
            p2: `Hvis klienten støtter EdDSA / ed25519 algoritmer, bør dette være det foretrukne valget.
                RSA algoritmer eksisterer kun av kompatibilitetsgrunner.`,
            p3: `Algoritmen for Refresh Tokens kan ikke endres, da disse kun skal brukes av Rauthy.`,
        },
    },
    common: {
        account: 'Konto',
        addNew: 'Legg til ny',
        back: 'Tilbake',
        caution: 'ADVARSEL',
        contact: 'Kontakt',
        copiedToClip: 'Kopiert til utklippstavle',
        details: 'Detaljer',
        edit: 'Rediger',
        enabled: 'Aktivert',
        filter: 'Filter',
        from: 'Fra',
        information: 'Informasjon',
        language: 'Språk',
        loading: 'Laster',
        name: 'Navn',
        nameExistsAlready: 'Navnet finnes allerede',
        note: 'Notat',
        noEntries: 'Ingen oppføringer',
        preview: 'Preview',
        reset: 'Tilbakestill',
        searchOptions: 'Søkealternativer',
        until: 'Til',
    },
    docs: {
        book: 'For generell dokumentasjon for Rauthy, se',
        encryption: 'Kryptering',
        encKeys: {
            header: 'Krypteringsnøkler',
            keyActive: 'Aktivert nøkkel',
            keysAvailable: 'Tilgjengelige nøkler',
            migrate: 'Migrere',
            migrateToKey: 'Migrere alle verdiene til følgende krypteringsnøkkel',
            p1: `Disse nøklene brukes for ekstra kryptering i forskjellige situasjoner, som for eksempel
            visse verdier i databasen eller sesjonscookies. De er statisk konfigurert, men kan roteres
            manuelt som en beste praksis.`,
            p2: `Den aktive nøkkelen er også statisk satt i Rauthy konfigurasjonsfilen. Alle ny-krypterte
            verdier vil bli kryptert med den aktive nøkkelen, mens gamle kan eksistere parallelt for
            bakoverkompatibilitet.`,
            p3: `Migrering av alle krypterte verdier på dette tidspunktet kan, avhengig av systemet,
            ta litt tid.`,
            pNotPossible: 'For å migrere må det finnes minst 2 kypteringsnøkler.',
        },
        hashing: {
            calculate: 'Beregn',

            currValuesHead: 'Nåværende verdier',
            currValues1: 'Nåværende konfigurerte veridene i backenden er følgende:',
            currValuesNote: `Notat: Logintiden fra backenden vil kun være en god retningslinje etter at minst
            5 gylidge Logins har blitt gjort siden siste omstart. Startverdien er alltid 2000 ms og vil
            tilpasses med hver gylidg Login.`,
            currValuesThreadsAccess: 'Tråder (p_cost) tilgjengelig for Rauthy',

            loginTimeHead: 'Noe ord om login tid',
            loginTime1: `Generelt ønsker brukere at alt skal gå så raskt som mulig. For en sikker
            login prosedyre bør imidlertid en tid på minst 500 - 1000 ms siktes mot og bør ikke være
            et problem. Tiden for passord hashing bør ikke være for kort, da dette vil redusere styrken
            til hashen.`,
            loginTime2: `For å sikre tilstrekkelig sikkerhet som standard, tillater dette verktøyet ikke
            småere verider enn 500ms for login tid.`,

            mCost1: `<code>m_cost</code> definerer mengden <b>minne (i kB)</b> som brukes til hashing. Jo høyere
            denne verdien er,jo bedre (sikrere), men de nødvendige ressursene må selvfølgelig være tilgjengelige.<br>
            Hvis f.eks. 4 passord skal hashes samtidig, vil det naturligvis kreves <code>4 x m_cost</code> minne, som
            alltid må være tilgjengelig.`,
            mCost2: `Å finne den "riktige" verdien for <code>m_cost</code> er heldigvis veldig enkelt.
            Definer maksimum minne Rauthy skal bruke, del mengden på antall parallelle innlogginger som skal
            være mulig (<code>MAX_HASH_THREADS</code>) og trekk fra en viss statisk mengde.
            Mengden statisk minne som trengs avhenger av valgt database og antall brukere, men vil i de fleste
            tilfeller ligge mellom 32 - 96 MB.`,

            pCost1: `<code>p_cost</code> definerer parallellismen for hashing.<br>
            I de fleste tilfeller vil verdier over 8 ikke øke den nødvendige tiden, fordi algoritmen vil være mettet. Dette er også standardverdien for Rauthy.`,
            pCost2: `Den generelle regelen er:<br>
            Sett <code>p_cost</code> til det dobbelte av antall tilgjengelige CPU-kjerner.<br>
            Hvis f.eks. 4 kjerner er tilgjengelig, vil en <code>p_cost</code> på 8 være en god verdi.<br>
            Verdien må imidlertid ta hensyn til maksimalt antall tillatte parallelle innlogginger (<code>MAX_HASH_THREADS</code>) og eventuelt justeres deretter.`,

            tCost1: `<code>t_cost</code> er en multiplikator for <b>tiden</b> til hashing. Dette er den eneste verdien som må finnes ved testing på målarkitekturen, fordi <code>m_cost</code> og <code>p_cost</code> i stor grad er forhåndsdefinert.`,
            tCost2: `Å finne verdien er enkelt: Sett <code>m_cost</code> og <code>p_cost</code> som forklart ovenfor og øk <code>t_cost</code> til ønsket innloggingstid er oppnådd.`,

            utilityHead: 'Parameter Beregningsverktøy',
            utility1: `Verktøyet nedenfor kan brukes til å finne passende verdier for denne Rauthy-installasjonen. Siden verdiene avhenger av mange faktorer, bør de settes på den endelige arkitekturen, helst på tidspunkter med forventet høyest belastning, for å unngå å sette for høye verdier.`,
            utility2: `<code>m_cost</code> er valgfritt og den minimale sikre verdien på 32768 vil automatisk
            vælges. Skulle <code>p_cost</code> heller ikke være angitt, vil Rauthy bruke den maksimale
            tilgjengelige mengden kjerner.`,
            time: 'Tid',
            targetTime: 'Mål-Tid',
            tune: 'Viktig: Disse verdiene må settes på den endelige arkitekturen!',
            pDetials: `For en mer detaljert innføring i Argon2ID-algoritmen finnes det mange kilder på nettet.
            Her forklares bare verdiene veldig kort. De følgende tre verdiene må konfigureres:`,
            pTune: `Verdiene kan variere sterkt avhengig av systemet og den generelle systembelastningen. Jo
            kraftigere systemet er, desto sikrere verdier kan velges.`,
            pUtility: `Dette verktøyet er en hjelp til å finne de beste Argon2ID-verdiene for det aktuelle systemet.
            Argon2ID er den for tiden sikreste tilgjengelige passordhash-algoritmen. For å kunne utnytte det fulle potensialet
            må imidlertid verdiene tilpasses systemet.`,
            mCost3: 'Den minimale tillatte verdien for <code>m_cost</code> er <code>32768</code>.',
        },
        openapi: 'For integrering av en ekstern applikasjon via Rauthys API, se',
        openapiNote: `Avhengig av konfigurasjonen er Swagger UI kanskje ikke offentlig tilgjengelig via lenken over.
            Den er imidlertid (som standard) tilgjengelig via den interne metrics-serveren for å redusere angrepsflaten.`,
        source: 'Kildekoden finnes her',
    },
    editor: {
        bold: 'Bold',
        code: 'Code',
        heading1: 'Heading 1',
        heading2: 'Heading 2',
        heading3: 'Heading 3',
        italic: 'Italic',
        link: 'Link',
        listBullet: 'List',
        listTasks: 'Tasks',
        listNumbered: 'Numbered List',
        paragraph: 'Paragraph',
        quote: 'Quote',
        removeFmt: 'Remove Formatting',
        strikeThrough: 'Strikethrough',
        textArea: 'Edit Text',
    },
    email: {
        cancelJob: 'Cancel Job',
        filterType: ['None', 'In Group', 'Not in Group', 'Has Role', 'Has not Role'],
        immediate: 'Immediate',
        jobs: 'E-Mail Jobs',
        scheduled: 'Scheduled Dispatch',
        sendAllUsers: 'This E-Mail will be sent to all users.',
        sendAllUsersFiltered: 'This E-Mail will be sent to all users filtered by:',
        sendMail: 'Send E-Mail',
        subject: 'Subject',
        userFilter: 'User Filter',
    },
    error: {
        needsAdminRole: 'For å få tilgang må du ha rollen <b>rauthy_admin</b>.',
        noAdmin: `For Rauthy admin-kontoer er <b>MFA påkrevd.</b><br>
            Gå til <b>konto</b> og aktiver MFA.<br>
            Deretter må du logge ut og inn igjen`,
    },
    events: {
        eventLevel: 'Hendelsesnivå',
        eventType: 'Hendelsestype',
    },
    groups: {
        delete1: 'Skal denne gruppen slettes?',
        name: 'Gruppenavn',
    },
    jwks: {
        alg: 'Algoritme',
        p1: 'Dette er Json Web Keys (JWKs) som brukes til signering av tokens.',
        p2: `JWKs roteres automatisk den 1. hver måned. For alle nye tokens brukes alltid den nyeste versjonen av en nøkkel for den aktuelle algoritmen. Gamle nøkler beholdes en stund for å validere eksisterende tokens og slettes automatisk etter en tid.`,
        p3: `Nøkler kan også roteres manuelt. Avhengig av maskinvaren denne Rauthy-instansen kjører på, kan dette ta noen sekunder.`,
        type: 'Type',
        rotateKeys: 'Roter nøkler',
    },
    nav: {
        apiKeys: 'API-nøkler',
        attributes: 'Attributter',
        blacklist: 'Svarteliste',
        clients: 'Klienter',
        config: 'Konfigurasjon',
        docs: 'Dokumentasjon',
        events: 'Hendelser',
        groups: 'Grupper',
        providers: 'Leverandører',
        roles: 'Roller',
        scopes: 'Scopes',
        sessions: 'Økter',
        users: 'Brukere',
    },
    options: {
        expires: 'Utløper',
        lastSeen: 'Sist sett',
        state: 'Status',
    },
    pam: {
        addGroup: 'Ny PAM-gruppe',
        addHost: 'Ny PAM-vert',
        addUser: 'Ny PAM-bruker',
        deleteHost: 'Skal denne verten slettes?',
        groupDescGeneric: `Generiske grupper tilsvarer oppføringer man vanligvis finner i /etc/group. Brukere kan tilordnes disse og de returneres til systemet via NSS-oppslag.`,
        groupDescHost: `Vertgrupper brukes til å gruppere verter. NSS-oppslag av en vert i gruppen returnerer alle andre verter i gruppen. Brukere får tilgang til verter ved å tilordnes en vertgruppe.`,
        groupDescLocal: `Lokale grupper oppfører seg nesten identisk med generiske grupper, med den forskjellen at de har en ID i Rauthy-databasen, men NSS-proxyen på den aktuelle verten kobler dem til en ID fra /etc/group. Slik kan Rauthy-brukere tilordnes grupper som allerede finnes lokalt.`,
        groupDescUser: `Brukergrupper administreres automatisk og er tett koblet til brukeren med samme brukernavn.`,
        groupDescWheel: `Denne gruppen er spesiell. Den er uforanderlig og tildeles brukere dynamisk avhengig av gruppeoppsettet.`,
        groupName: 'Gruppenavn',
        groups: 'Grupper',
        groupType: 'Gruppetype',
        hostAliases: 'Vert-aliaser',
        hostLocalPwdOnly: 'Lokal passordpålogging',
        hostLocalPwdOnlyInfo: `Hvis lokal passordpålogging er aktivert, overstyrer dette tvungen MFA for lokale pålogginger. Det vil heller aldri bli bedt om passnøkkel (lokalt), selv om brukeren har MFA aktivert. Denne innstillingen bør kun brukes hvis det er absolutt nødvendig, for eksempel hvis MFA-sikrede brukere skal kunne logge inn lokalt uten å bruke maskinvare-passnøkler.`,
        ipAddresses: 'IP-adresser',
        member: 'Medlem',
        nameExistsAlready: 'Navnet er allerede tatt',
        notes: 'Notater',
        secretShow: 'Vis hemmelighet',
        secretRotate: 'Roter hemmelighet',
        userEmail: 'Koblet bruker-e-post',
        username: 'Brukernavn',
        usernameNewDesc: `Brukernavnet bør velges nøye. Når det er opprettet, kan det ikke enkelt endres senere av sikkerhetsgrunner.`,
    },
    passwordPolicy: {
        configDesc: 'Regler for nye passord.',
        resetSet0: 'Verdien 0 deaktiverer kravet.',
        validForDays: 'Gyldig i dager',
        validityNew: 'Gyldighet for nye passord.',
    },
    providers: {
        config: {
            allowInsecureTls: 'Tillat usikker TLS',
            autoLink: 'Auto-link bruker',
            autoLinkDesc1: `Hvis auto-link bruker er aktivert, vil en eventuell eksisterende, ikke-koblet bruker automatisk kobles til denne leverandøren ved innlogging.`,
            autoLinkDesc2: `ADVARSEL: Dette kan være svært farlig og føre til kontoovertakelse hvis leverandøren ikke utfører fullstendig e-postverifisering og lar en fremmed adresse bli registrert for en bruker! MÅ ALDRI brukes i slike tilfeller!`,
            clientName: 'Klientnavn',
            custRootCa: 'Egen Root CA PEM',
            descAuthMethod: `Autentiseringsmetoden som skal brukes på <code>/token</code>-endepunktet.<br>De fleste leverandører bør fungere med <code>basic</code>, noen kun med <code>post</code>. I sjeldne tilfeller må begge aktiveres, selv om det kan føre til feil med andre leverandører.`,
            descClientId: 'Klient-ID gitt av leverandøren.',
            descClientName: 'Klientnavn som skal vises på Rauthy-innloggingssiden.',
            descClientSecret: `Klienthemmelighet gitt av leverandøren. Minst én hemmelighet eller PKCE må være aktivert.`,
            descScope: `Omfanget klienten skal bruke ved omdirigering til innlogging. Verdier skilles med mellomrom.`,
            errNoAuthMethod:
                'Du har oppgitt en klienthemmelighet, men ingen autentiseringsmetode er aktiv',
            errConfidential: 'Må være enten en følsom klient eller bruke PKCE',
            jsonPath: {
                p1: 'Verdier fra ID-token etter vellykket innlogging kan mappes automatisk.',
                p2: `Stien må oppgis i regex-lignende syntaks. Den kan referere til enkeltverdier eller verdier i et JSON-objekt eller array.`,
                p3: '<code>$.</code> markerer starten på JSON-objektet',
                p4: '<code>*</code> kan brukes som jokertegn i stien',
                p5: '<code>$.roles</code> vil treffe <code>&#123;"roles": "verdi"&#125;</code>',
                p6: `<code>$.roles.*</code> kan treffe en verdi i et objekt eller array som<br><code>&#123;\"roles\": [\"verdi\", \"ikkeMinVerdi\"]&#125;</code>`,
            },
            lookup: 'Søk',
            pathAdminClaim: 'Sti til admin-claim',
            pathMfaClaim: 'Sti til MFA-claim',
            rootPemCert: 'Root PEM-sertifikat',
            mapMfa: `Hvis leverandøren gir en claim som indikerer at brukeren har brukt minst 2FA ved innlogging, kan du oppgi stien til MFA-claimen her.`,
            mapUser: `Du kan mappe en bruker til å være Rauthy-admin basert på en upstream ID-claim.`,
            valueAdminClaim: 'Verdi for admin-claim',
            valueMfaClaim: 'Verdi for MFA-claim',
        },
        delete: {
            areYouSure: 'Er du sikker på at denne leverandøren skal slettes?',
            forceDelete: 'Tving sletting',
            isInUse1: 'Denne leverandøren brukes av aktive brukere!',
            isInUse2: `Du kan tvinge sletting, men brukere uten lokalt passord eller passnøkkel vil ikke kunne logge inn lenger.`,
            linkedUsers: 'Koblede brukere',
        },
    },
    roles: {
        adminNoMod: 'Rollen <code>rauthy_admin</code> kan ikke endres.',
        delete1: 'Skal denne rollen slettes?',
        name: 'Rollenavn',
    },
    scopes: {
        defaultNoMod: 'Dette er en standard OIDC Scope. Disse kan ikke endres.',
        delete1: 'Skal denne scope slettes?',
        deleteDefault: 'OIDC standard scopes kan ikke slettes',
        mapping1: 'Bruker-attributter kan mappes til egne scopes.',
        mapping2: `Hvert eksisterende attributt har en egen verdi per bruker. Disse attributtene kan mappes til en scope og vil da inkluderes i Access- eller ID-tokenet.`,
        name: 'Scope-navn',
    },
    sessions: {
        invalidateAll: 'Invalidér alle økter',
    },
    search: {
        orderBy: 'Sorter etter ...',
        orderChangeToAsc: 'Bytt til stigende sortering',
        orderChangeToDesc: 'Bytt til synkende sortering',
    },
    tabs: {
        config: 'Konfigurasjon',
        delete: 'Slett',
    },
    tos: {
        accepted: 'Accepted',
        addNewToS: 'New ToS',
        addNewToSFromCurrent: 'New ToS from template',
        added: 'Added',
        checkStatus: 'Check user status',
        immutable: `CAUTION: After adding new Terms of Service, they are immutable and cannot be 
			deleted!`,
        noneExist: 'No Terms of Service have been added yet.',
        optUntil: {
            desc: `During the transition time, accepting updated ToS is optional. It only becomes 
				mandatory afterward.`,
            enable: 'Enable Transition Time',
            label: 'End of Transition Time',
        },
        tos: 'ToS',
    },
    users: {
        antiLockout: {
            rule: 'Anti-lockout-regel',
            delete: 'kan ikke slettes',
            disable: 'kan ikke deaktiveres',
            rauthyAdmin: 'rauthy_admin-rollen kan ikke fjernes',
        },
        attributes: 'Attributter',
        deleteUser: 'Skal denne brukeren slettes?',
        descAttr: `Sett individuelle bruker-attributter. Alle nøkkel/verdi-par håndteres som String/JSON-verdi.`,
        forceLogout: `Skal alle økter for denne brukeren invalidiseres og alle refresh tokens slettes?`,
        lastLogin: 'Siste innlogging',
        manualInitDesc: `Brukeren kan også initialiseres her. I så fall må passordet kommuniseres direkte.`,
        manualInit: 'Manuell initialisering',
        mfaDelete1: 'Passnøkler for denne brukeren kan slettes.',
        mfaDelete2: `Advarsel! Sletting av en passnøkkel <b>kan ikke angres</b> uten at brukeren registrerer seg på nytt.`,
        noMfaKeys: 'Denne brukeren har ingen registrerte passnøkler.',
        pkOnly1: 'Dette er en passkey-only-konto.',
        pkOnly2:
            'Det betyr at denne brukeren bruker passordløs innlogging og ikke har noe passord satt.',
        pkOnly3: `Hvis denne brukeren har mistet alle passnøkler, kan kontoen tilbakestilles og en ny e-post for tilbakestilling av passord sendes. For å gjøre dette, gå til 'MFA'-fanen og slett alle eksisterende passnøkler først.`,
        pwdNoInit: 'Brukeren har ikke gjennomført den første tilbakestillingen av passord ennå.',
        pwdSendEmailBtn: 'Send e-post for tilbakestilling',
        pwdSendEmailDesc:
            'Du kan sende en ny e-post for tilbakestilling hvis brukeren ikke har mottatt en.',
        savePassword: 'Lagre passord',
        selfServiceDesc:
            'Du kan enten sette et nytt passord eller sende en e-post for tilbakestilling.',
        sendResetEmail: 'Send e-post for tilbakestilling',
    },
    validation: {
        css: 'Gyldig CSS-verdi',
        origin: 'Gyldig origin',
        uri: 'Gyldig URI',
    },
};
