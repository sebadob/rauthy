import type { I18nAdmin } from './interface.ts';

export let I18nAdminCs: I18nAdmin = {
    api_key: {
        delete1: 'Opravdu chcete tento API klíč smazat?',
        expires: 'Platnost',
        generate1: 'Zde můžete pro tento API klíč vygenerovat nové tajemství.',
        generate2: `Tajemství uvidíte jen jednou, hned po vygenerování.
            Vygenerováním nového se staré tajemství natrvalo přepíše.
            Tuto operaci nelze vrátit zpět!`,
        generate3: `API klíč se zadává v HTTP hlavičce <code>Authorization</code>
            v tomto tvaru:`,
        generate4: 'Nový klíč můžete vyzkoušet tímto požadavkem <code>curl</code>:',
        generate5: 'Pokud nemáte nainstalovaný <code>jq</code> a předchozí příkaz selže:',
        keyName: 'Název klíče',
        limitedValidity: 'Omezená platnost',
    },
    attrs: {
        delete1: 'Opravdu chcete tento atribut smazat?',
        defaultValue: 'Výchozí hodnota',
        desc: 'Popis',
        makeEditable: 'Povolit úpravy',
        makeEditableP1: 'Atribut můžete převést tak, aby ho uživatelé mohli upravovat sami.',
        makeEditableP2: `<b>POZOR:</b> Tuto změnu nelze nikdy vrátit zpět! Údaje zadané přímo uživatelem jsou vždy
            nedůvěryhodné a NESMÍ se nikdy použít k ověření ani k udělování oprávnění!`,
        makeEditableP3: `Upravitelný atribut nelze změnit zpět na neupravitelný, protože v minulosti připouštěl
            nedůvěryhodné údaje, bez ohledu na to, jak dlouho.`,
        name: 'Název atributu',
        userEditable: 'Upravuje uživatel',
    },
    backup: {
        createBackup: 'Vytvořit zálohu',
        disabledDesc: 'Tato funkce existuje, jen pokud je jako databáze nastavený Hiqlite.',
        lastModified: 'Naposledy změněno',
        local: 'Místní',
        name: 'Název',
        size: 'Velikost',
    },
    clients: {
        allowedResources: 'Povolené zdroje',
        defaultAud: 'Výchozí příjemci (audience)',
        descAllowedResources: `Volitelné indikátory zdrojů podle RFC 8707, o které smí tento klient žádat. Při prázdném seznamu se každý požadavek s parametrem 'resource' odmítne chybou 'invalid_target'.`,
        descDefaultAud: `Příjemci (audience), kteří se do tokenů tohoto klienta přidají vždy, bez ohledu na parametr požadavku 'resource'.`,
        backchannelLogout: 'Pokud tento klient podporuje {{ OIDC_BCL }}, zadejte sem jeho URI.',
        branding: {
            descHsl: `Následující hodnoty se zadávají jako HSL. Zadáváte jen základní barvy.
            Průhlednost a další hodnoty si vzhled dopočítá sám.`,
            descFullCss: `Následující hodnoty musí být platné CSS hodnoty <code>color</code>.
                Můžete použít i složitější výpočty nebo výše definované CSS proměnné.`,
            descVariables: `Každý následující popisek je zároveň názvem CSS proměnné. Můžete na ně tedy
                odkazovat ve volných polích, např. <code>hsla(var(--action) / .7)</code>.`,
            faviconPreviewAlt: 'Náhled ikony klienta',
            faviconUpload: 'Nahrát ikonu (favicon)',
        },
        claimsAtRoot: 'Claimy v kořeni tokenu',
        claimsAtRootWarning: `Claimy tohoto klienta se zapisují do kořene tokenu,
            ne do 'custom'. Za kolize odpovídáte vy: pokud se název claimu shoduje
            s vyhrazeným claimem JWT, vydání tokenu selže. Vlastní claimy v kořeni se navíc
            mohou rozbít při budoucích změnách protokolu nebo funkcí. Odkaz:`,
        claims: 'Vlastní claimy (client_credentials)',
        claimsDesc: `Objekt JSON, který se vkládá do tokenů client_credentials pod claim
            custom. Nejvýš 1024 znaků po serializaci.`,
        confidential: 'Důvěrný',
        confidentialNoSecret: 'Tento klient není důvěrný, a proto nemá tajemství.',
        config: 'Nastavení klienta',
        delete1: 'Opravdu chcete tohoto klienta smazat?',
        descAuthCode: `Platnost autorizačních kódů lze kvůli bezpečnosti upravit. Kód lze použít
            jen jednou a ve výchozím nastavení platí 60 sekund. Čím kratší platnost, tím
            lépe, pokud klient stihne přihlášení dokončit.`,
        descClientUri: `Údaje o URI tohoto klienta a kontakty, které se ukážou
            na přihlašovací stránce.`,
        descName: `Název klienta lze změnit bez vlivu na jeho nastavení.
            Slouží jen k zobrazení na přihlašovací stránce.`,
        descGroupPrefix: `Přihlášení k tomuto klientovi lze omezit volitelnou předponou skupiny.
            Přihlásit se pak smějí jen uživatelé přiřazení do odpovídající skupiny.`,
        descOrigin: `Další povolené externí původy (origins) – obvykle potřeba jen tehdy, když klient
            posílá požadavky na Rauthy přímo z prohlížeče, typicky SPA.`,
        descPKCE: `Pokud to klient podporuje, vždy kvůli bezpečnosti zapněte PKCE S256.
            U klienta, který není důvěrný (např. SPA), musíte zapnout aspoň jednu
            z výzev PKCE, jinak není dost bezpečný.`,
        descPKCEEnforce: `Je-li zapnuté některé PKCE, Rauthy ho při přihlášení vyžaduje
            a odmítne požadavek bez platné výzvy.`,
        descUri: `Přesměrovacích URI můžete zadat libovolně mnoho. Na konci každého můžete použít
            <code>*</code> jako zástupný znak.`,
        errConfidentialPKCE: `Klient musí být buď důvěrný, nebo mít zapnutou aspoň jednu výzvu
            PKCE.`,
        forceMfa: 'Vyžadovat MFA',
        groupLoginPrefix: 'Předpona skupiny pro přihlášení',
        name: 'Název klienta',
        passwordFlowMfaWarn: `POZOR: Zároveň je zapnuté 'Vyžadovat MFA' a tok 'password'.
            Rauthy v tomto toku NEMŮŽE MFA vynutit, protože by porušil OIDC RFC. Pokud
            MFA striktně vyžadujete, musíte ho ověřit na straně klienta. Použít k tomu
            můžete claim 'amr'.`,
        scim: {
            baseUri: `Základní URI SCIM je to, od kterého lze správně odvodit podcesty jako
                <code>{base_uri}/Users/{id}</code>.`,
            desc: 'Pokud tento klient podporuje {{ SCIM_LINK }}, můžete ho zde zapnout.',
            enable: 'Zapnout SCIMv2',
            groupSync: 'Synchronizovat skupiny',
            groupSyncPrefix: 'Předpona filtru skupin',
            groupSyncPrefixDesc: `Skupiny pro synchronizaci můžete filtrovat volitelnou předponou.
                Pokud například existují skupiny <code>app:admins</code> a <code>app:users</code>, předpona
                 <code>app:</code> bude synchronizovat jen tyto skupiny a jen uživatele, kteří jsou aspoň
                 v jedné z nich.`,
            reqDesc: 'Kvůli kompatibilitě je potřeba několik věcí:',
            reqLi1: 'Klient musí správně zpracovávat <code>externalId</code>.',
            reqLi2: `Musí být podporované aspoň koncové body <code>/Users</code> s <code>filter=externalId eq "*"</code>
                a <code>filter=userName eq "*"</code>.`,
            reqLi3: `Mají-li se synchronizovat skupiny, musí i <code>/Groups</code> podporovat
                <code>filter=displayName eq "*"</code>.`,
        },
        scopes: {
            allowed: 'Povolené rozsahy',
            default: 'Výchozí rozsahy',
            desc: `Povolené rozsahy jsou ty, o které smí klient dynamicky žádat při přesměrování
            na přihlášení v toku <code>authorization_code</code>. Výchozí rozsahy se do tokenů
            přidají vždy, což řeší některé potíže například při použití toku
            <code>password</code>.`,
        },
        secret: {
            doCache: 'Uchovat tajemství klienta v mezipaměti',
            cacheDuration: 'Doba v mezipaměti (hodiny)',
            generate: 'Vygenerovat nové tajemství',
            rotateDesc1: `Aby šlo tajemství vyměnit bez výpadku, můžete současné tajemství ještě nějakou dobu
                ponechat v mezipaměti. Zadejte hodnotu od 1 do 24 hodin.`,
            rotateDesc2: 'Pozor: Pokud tajemství uniklo, v mezipaměti ho nenechávejte!',
        },
        tokenLifetime: {
            p1: `Doba platnosti se týká přístupových a ID tokenů a zadává se v sekundách.`,
            p2: `Pokud klient podporuje algoritmy EdDSA / Ed25519, vždy jim dejte přednost.
                Algoritmy RSA jsou tu jen kvůli kompatibilitě.`,
            p3: `Algoritmus obnovovacích tokenů změnit nelze, protože je používá jen Rauthy.`,
        },
    },
    common: {
        account: 'Účet',
        addNew: 'Přidat',
        back: 'Zpět',
        caution: 'POZOR',
        contact: 'Kontakt',
        copiedToClip: 'Hodnota byla zkopírována do schránky',
        details: 'Podrobnosti',
        edit: 'Upravit',
        enabled: 'Povoleno',
        filter: 'Filtr',
        from: 'Od',
        information: 'Informace',
        language: 'Jazyk',
        loading: 'Načítám',
        jsonMeta: 'Metadata jako hodnota JSON',
        name: 'Název',
        nameExistsAlready: 'Název už existuje',
        note: 'Poznámka',
        noEntries: 'Žádné položky',
        preview: 'Náhled',
        reset: 'Obnovit výchozí',
        searchOptions: 'Možnosti hledání',
        until: 'Do',
    },
    docs: {
        book: 'Obecnou dokumentaci k Rauthy najdete v',
        encryption: 'Šifrování',
        encKeys: {
            header: 'Šifrovací klíče',
            keyActive: 'Aktivní klíč',
            keysAvailable: 'Dostupné klíče',
            migrate: 'Převést',
            migrateToKey: 'Převést všechny zašifrované hodnoty na tento klíč',
            p1: `Tyto klíče slouží k dalšímu šifrování uložených dat, nezávisle na použitém
            úložišti. Nastavují se staticky, ale na této stránce je lze ručně vyměnit a data převést.`,
            p2: `Aktivní klíč se nastavuje staticky v konfiguraci Rauthy / proměnných prostředí. Zde ho
            změnit nelze. Každé nové šifrování JWK použije vždy aktuálně aktivní klíč.`,
            p3: `Převod všech existujících tajemství může u velkého množství dat trvat několik
            sekund.`,
            pNotPossible: 'K převodu musí být k dispozici aspoň 2 šifrovací klíče.',
        },
        hashing: {
            calculate: 'Spočítat',

            currValuesHead: 'Současné hodnoty',
            currValues1: 'Současné hodnoty ze serveru:',
            currValuesNote: `Poznámka: Doba přihlášení ze serveru je spolehlivým vodítkem až po nejméně 5
            úspěšných přihlášeních od spuštění Rauthy. Po novém startu je výchozí hodnota vždy 2000 ms
            a s každým úspěšným přihlášením se upravuje.`,
            currValuesThreadsAccess: 'Vlákna (p_cost), která má Rauthy k dispozici',

            loginTimeHead: 'K době přihlášení',
            loginTime1: `Uživatelé obecně chtějí všechno co nejrychleji. Při bezpečném přihlášení ale doba
            500–1000 ms nevadí. Příliš krátká být nesmí, protože by oslabila
            hash.`,
            loginTime2: `Aby byla výchozí bezpečnost co nejvyšší, nástroj nedovolí nastavit dobu přihlášení
            pod 500 ms.`,

            mCost1: `<code>m_cost</code> určuje množství <b>paměti (v kB)</b>, které se pro hashování použije.
            Čím víc, tím lépe, ale je třeba myslet na prostředky serveru.<br>
            Když se například hashují 4 hesla najednou, server potřebuje <code>4 x m_cost</code>.
            Tolik paměti musí mít k dispozici.`,
            mCost2: `Nastavit <code>m_cost</code> je snadné. Určete, kolik paměti nejvýš má Rauthy použít,
            vydělte to nejvyšším povoleným počtem souběžných přihlášení (<code>MAX_HASH_THREADS</code>) a odečtěte
            malé pevné množství. Kolik pevné paměti počítat, záleží na databázi a počtu uživatelů,
            obvykle 32–96 MB.`,
            mCost3: 'Nejmenší povolená hodnota <code>m_cost</code> je <code>32768</code>.',

            pCost1: `<code>p_cost</code> určuje míru <b>souběžnosti</b> hashování. Hodnota obvykle končí
            kolem 8, což je výchozí hodnota Rauthy.`,
            pCost2: `Obecné pravidlo:<br>
            Nastavte <code>p_cost</code> na dvojnásobek dostupných jader.<br>
            Máte-li například 4 jádra, nastavte <code>p_cost</code> na <code>8</code>.<br>
            Hodnota ale musí zohlednit povolený počet souběžných přihlášení (<code>MAX_HASH_THREADS</code>)
            a podle toho se snížit.`,

            tCost1: `<code>t_cost</code> určuje <b>čas</b> hashování. Je to vlastně jediná hodnota, kterou
            je potřeba ladit, protože <code>m_cost</code> a <code>p_cost</code> jsou dané hlavně
            prostředím.`,
            tCost2: `Ladění je snadné: nastavte <code>m_cost</code> a <code>p_cost</code> a pak zvyšujte
            <code>t_cost</code>, dokud nedosáhnete požadované doby hashování.`,

            utilityHead: 'Výpočet parametrů',
            utility1: `Tímto nástrojem odhadnete vhodné hodnoty pro své nasazení. Spouštějte ho s Rauthy
            na jeho konečném místě a se všemi konečnými prostředky. Spouštějte ho pod zátěží, abyste
            hodnoty nepřehnali.`,
            utility2: `<code>m_cost</code> je nepovinné; když ho nevyplníte, použije se bezpečné minimum
            <code>32768</code>. Nepovinné je i <code>p_cost</code>; když ho nevyplníte, Rauthy použije všechna
            vlákna, která vidí.`,

            time: 'Čas',
            targetTime: 'Cílový čas',
            tune: 'Důležité: Tyto hodnoty je potřeba vyladit na konečném hardwaru!',
            pDetails: `Podrobné úvody do Argon2ID najdete na internetu. Tento návod jen stručně shrnuje
            hodnoty. Nastavit je potřeba tři:`,
            pTune: `Mění se podle výkonu systému. Čím výkonnější systém, tím bezpečnější
            hodnoty mohou být.`,
            pUtility: `Tento nástroj pomáhá najít nejlepší nastavení Argon2ID pro vaši platformu.
            Argon2ID je v současnosti nejbezpečnější algoritmus pro hashování hesel. Aby se plně využil,
            je potřeba ho vyladit pro každé nasazení.`,
        },
        openapi: 'Chcete-li napojit externí aplikaci a používat API Rauthy, podívejte se do',
        openapiNote: `Ve výchozím nastavení není Swagger UI přes odkaz výše dostupné. Zapnout ho lze
            v konfiguraci 'server.swagger_ui_enable', případně 'server.swagger_ui_public'.`,
        source: 'Zdrojový kód najdete zde',
    },
    editor: {
        bold: 'Tučně',
        code: 'Kód',
        heading1: 'Nadpis 1',
        heading2: 'Nadpis 2',
        heading3: 'Nadpis 3',
        italic: 'Kurzíva',
        link: 'Odkaz',
        listBullet: 'Seznam',
        listTasks: 'Úkoly',
        listNumbered: 'Číslovaný seznam',
        paragraph: 'Odstavec',
        quote: 'Citace',
        removeFmt: 'Odstranit formátování',
        strikeThrough: 'Přeškrtnutí',
        textArea: 'Upravit text',
    },
    email: {
        cancelJob: 'Zrušit rozesílku',
        filterType: ['Žádný', 'Ve skupině', 'Mimo skupinu', 'Má roli', 'Nemá roli'],
        immediate: 'Ihned',
        jobs: 'Rozesílky e-mailů',
        scheduled: 'Naplánované odeslání',
        sendAllUsers: 'Tento e-mail dostanou všichni uživatelé.',
        sendAllUsersFiltered: 'Tento e-mail dostanou všichni uživatelé podle filtru:',
        sendMail: 'Odeslat e-mail',
        subject: 'Předmět',
        userFilter: 'Filtr uživatelů',
    },
    error: {
        needsAdminRole: `Nemáte přiřazenou roli <b>rauthy_admin</b>.<br/>
            K administraci nemáte přístup.`,
        noAdmin: `Účet správce Rauthy musí mít <b>zapnuté MFA.</b><br>
            Přejděte prosím do svého <b>účtu</b> a zapněte MFA.<br>
            Pak se odhlaste a znovu přihlaste.`,
    },
    events: {
        eventLevel: 'Úroveň události',
        eventType: 'Typ události',
    },
    groups: {
        delete1: 'Opravdu chcete tuto skupinu smazat?',
        name: 'Název skupiny',
    },
    jwks: {
        alg: 'Algoritmus',
        p1: 'Toto jsou klíče JSON Web Keys (JWK), kterými se podepisují tokeny.',
        p2: `Klíče JWK se ve výchozím nastavení vyměňují každého 1. dne v měsíci. Nové tokeny se podepisují
        vždy nejnovějším klíčem pro daný algoritmus. Staré klíče se ještě nějakou dobu ponechají, aby šlo
        dál ověřovat platné tokeny. Po čase se automaticky
        odstraní.`,
        p3: `Klíče lze vyměnit i ručně. Podle hardwaru, na kterém tato instance Rauthy běží, to může
        trvat několik sekund.`,
        type: 'Typ',
        rotateKeys: 'Vyměnit klíče',
    },
    kv: {
        accessTestDesc: `Klíč k úložišti se zadává v hlavičce <code>Authorization</code> jako
            token <code>Bearer</code>. K vyzkoušení můžete použít následující příkaz
            <code>curl</code>.`,
        addNewKey: 'Nový klíč k úložišti',
        addNewNs: 'Nový jmenný prostor',
        addNewValue: 'Nová hodnota',
        delConfirm: 'Opravdu chcete tento klíč k úložišti smazat?',
        delNsMsg: 'Opravdu chcete smazat tento jmenný prostor včetně všech dat?',
        encryptedDesc: `Kvůli výkonu používejte další šifrování jen u zvlášť citlivých hodnot,
            jako jsou klíče k úložišti nebo osobní údaje.`,
        deleteConfirmMsg: `Opravdu chcete smazat klíč '{{ key }}'?`,
        help: {
            help: 'Nápověda',
            ops: [
                'Vyzkoušet klíč k úložišti',
                'Načíst všechny klíče',
                'Načíst všechny klíče + hodnoty',
                'Nastavit klíč + hodnotu',
                'Načíst hodnotu klíče',
                'Smazat klíč',
            ],
            p1: `Vnější přístup k úložištím KV je záměrně velmi jednoduchý. Má jen několik
                operací:`,
            p2: `Každá z nich vyžaduje klíč k úložišti jako token <code>Bearer</code>
                (<code>{id}\${secret}</code>) v hlavičce <code>Authorization</code>. Klíč
                k úložišti platí jen v tom jmenném prostoru, ve kterém existuje.`,
            p3: `Zde najdete příklady výše uvedených operací s
                <code>curl</code>.`,
        },
        key: 'Klíč',
        loadAllValues: 'Načíst všechny hodnoty',
        storeEncrypted: 'Uložit hodnotu zašifrovanou',
        tabs: ['Data', 'Přístup', 'Upravit', 'Smazat'],
        testCmd: 'Zkušební příkaz',
        value: 'Hodnota JSON',
    },
    nav: {
        apiKeys: 'API klíče',
        attributes: 'Atributy',
        blacklist: 'Blokované adresy',
        clients: 'Klienti',
        config: 'Nastavení',
        docs: 'Dokumentace',
        events: 'Události',
        groups: 'Skupiny',
        providers: 'Poskytovatelé',
        roles: 'Role',
        scopes: 'Rozsahy',
        sessions: 'Relace',
        users: 'Uživatelé',
    },
    options: {
        expires: 'Vyprší',
        lastSeen: 'Poslední aktivita',
        state: 'Stav',
    },
    pam: {
        addGroup: 'Nová skupina PAM',
        addHost: 'Nový hostitel PAM',
        addUser: 'Nový uživatel PAM',
        deleteHost: 'Opravdu chcete tohoto hostitele smazat?',
        deleteUser: 'Opravdu chcete tohoto uživatele smazat?',
        groupDescGeneric: `Obecné skupiny odpovídají záznamům, které bývají v /etc/group. Lze do nich
            přiřazovat uživatele a systému se vracejí při vyhledávání přes NSS.`,
        groupDescHost: `Skupiny hostitelů seskupují hostitele. Vyhledání hostitele ve skupině přes NSS vrátí
            všechny ostatní hostitele v ní. Uživatelé získají přístup k hostitelům přiřazením ke skupině hostitelů.`,
        groupDescLocal: `Místní skupiny se chovají skoro stejně jako obecné, jen mají ID v databázi Rauthy,
            které proxy NSS na daném hostiteli převede na ID z /etc/group. Uživatele Rauthy tak lze přiřadit
            do skupin, které už místně existují.`,
        groupDescUser: `Uživatelské skupiny se spravují automaticky a jsou pevně spojené s uživatelem
            stejného jména.`,
        groupDescWheel: `Tato skupina je zvláštní. Nelze ji měnit a uživatelům se přiřazuje dynamicky podle
            jejich nastavení skupin.`,
        groupName: 'Název skupiny',
        groups: 'Skupiny',
        groupType: 'Typ skupiny',
        hostAliases: 'Aliasy hostitele',
        hostLocalPwdOnly: 'Místní přihlášení heslem',
        hostLocalPwdOnlyInfo: `Je-li zapnuté místní přihlášení heslem, má u místních přihlášení přednost před volbou
            'Vyžadovat MFA'. Přístupové klíče se pak při místním přihlášení nikdy nevyžadují, ani u uživatelů
            s MFA. Zapínejte jen v nutném případě, například když se mají uživatelé s MFA přihlašovat
            místně bez hardwarových přístupových klíčů.`,
        ipAddresses: 'IP adresy',
        member: 'Člen',
        nameExistsAlready: 'Název je už obsazený',
        notes: 'Poznámky',
        secretShow: 'Zobrazit tajemství',
        secretRotate: 'Vyměnit tajemství',
        userEmail: 'E-mail propojeného uživatele',
        username: 'Uživatelské jméno',
        usernameNewDesc: `Uživatelské jméno volte pečlivě. Po vytvoření ho z bezpečnostních důvodů nejde
            snadno změnit.`,
    },
    passwordPolicy: {
        configDesc: 'Pravidla pro nová hesla.',
        resetSet0: 'Hodnota 0 požadavek vypne.',
        validForDays: 'Platnost ve dnech',
        validityNew: 'Platnost nových hesel.',
    },
    providers: {
        config: {
            allowInsecureTls: 'Povolit nezabezpečené TLS',
            autoLink: 'Automaticky propojit uživatele',
            autoLinkDesc1: `Je-li zapnuté automatické propojení, přihlášení přes tohoto poskytovatele
                automaticky propojí případného existujícího, dosud nepropojeného uživatele.`,
            autoLinkDesc2: `POZOR: Tato volba může být velmi nebezpečná a vést k převzetí účtu, pokud
                poskytovatel plně neověřuje e-mailové adresy uživatelů, a umožňuje tak přidat cizí
                adresu! V takovém případě ji NIKDY nepoužívejte!`,
            clientName: 'Název klienta',
            custRootCa: 'Vlastní kořenová CA (PEM)',
            descAuthMethod: `Metoda ověření na koncovém bodu <code>/token</code>.<br>
                Většina poskytovatelů funguje s <code>basic</code>, někteří jen s <code>post</code>.
                Výjimečně jsou potřeba obě, u jiných to ale může vést k chybám.`,
            descClientId: 'ID klienta od poskytovatele přihlášení.',
            descClientName: 'Název klienta, který se ukáže na přihlašovací stránce Rauthy.',
            descClientSecret: `Tajemství klienta od poskytovatele přihlášení.
                Potřeba je aspoň tajemství klienta, nebo PKCE.`,
            descScope: `Rozsah, který má klient použít při přesměrování na přihlášení.
                Hodnoty oddělte mezerou.`,
            errNoAuthMethod:
                'Zadali jste tajemství klienta, ale není zapnutá žádná metoda ověření klienta',
            errConfidential: 'Klient musí být aspoň důvěrný, nebo používat PKCE',
            jsonPath: {
                p1: 'Hodnoty z ID tokenu po úspěšném přihlášení u poskytovatele lze mapovat automaticky.',
                p2: `<code>path</code> se zadává v syntaxi podobné regulárním výrazům. Může odkazovat na
                    jednotlivé hodnoty JSON nebo na hodnotu v objektu či poli JSON.`,
                p3: '<code>$.</code> označuje začátek objektu JSON',
                p4: '<code>*</code> lze v cestě použít jako zástupný znak',
                p5: '<code>$.roles</code> odkazuje na <code>&#123;"roles": "value"&#125;</code>',
                p6: `<code>$.roles.*</code> může odkazovat na hodnotu v objektu nebo poli, např.<br>
                    <code>&#123;"roles": ["value", "notMyValue"]&#125;</code>`,
            },
            lookup: 'Vyhledat',
            pathAdminClaim: 'Cesta ke claimu správce',
            pathMfaClaim: 'Cesta ke claimu MFA',
            rootPemCert: 'Kořenový certifikát PEM',
            mapMfa: `Pokud poskytovatel vydává claim, že uživatel při přihlášení použil aspoň 2FA,
                můžete zadat cestu k claimu MFA.`,
            mapUser: `Uživatele můžete podle claimu z ID tokenu poskytovatele určit jako správce Rauthy.`,
            valueAdminClaim: 'Hodnota claimu správce',
            valueMfaClaim: 'Hodnota claimu MFA',
        },
        delete: {
            areYouSure: 'Opravdu chcete tohoto poskytovatele smazat?',
            forceDelete: 'Vynutit smazání',
            isInUse1: 'Tohoto poskytovatele používají aktivní uživatelé!',
            isInUse2: `Smazání můžete vynutit, ale uživatelé bez místního hesla nebo přístupového klíče
                se pak už nepřihlásí.`,
            linkedUsers: 'Propojení uživatelé',
        },
    },
    roles: {
        adminNoMod: 'Roli <code>rauthy_admin</code> nelze měnit.',
        delete1: 'Opravdu chcete tuto roli smazat?',
        name: 'Název role',
    },
    scopes: {
        claimsAtRoot: 'Claimy v kořeni tokenu',
        claimsAtRootWarning: `Mapované atributy tohoto rozsahu se zapisují do kořene tokenu,
            ne do 'custom'. Za kolize odpovídáte vy: pokud se název mapovaného atributu shoduje
            s vyhrazeným claimem JWT, vydání tokenu selže. Vlastní claimy v kořeni se navíc
            mohou rozbít při budoucích změnách protokolu nebo funkcí. Odkaz:`,
        defaultNoMod: 'Toto je výchozí rozsah OIDC. Výchozí rozsahy nelze měnit.',
        delete1: 'Opravdu chcete tento rozsah smazat?',
        deleteDefault: 'Výchozí rozsahy OIDC nelze smazat.',
        mapping1: 'Vlastní rozsahy můžete mapovat na atributy.',
        mapping2: `Každý nastavený doplňkový atribut může mít pro každého uživatele vlastní hodnotu.
            Když je namapovaný na rozsah, může se dostat do přístupového a / nebo ID tokenu.`,
        name: 'Název rozsahu',
    },
    search: {
        orderBy: 'Řadit podle…',
        orderChangeToAsc: 'Řadit vzestupně',
        orderChangeToDesc: 'Řadit sestupně',
    },
    sessions: {
        invalidateAll: 'Zrušit všechny relace',
    },
    tabs: {
        config: 'Nastavení',
        delete: 'Smazat',
    },
    tos: {
        accepted: 'Přijato',
        addNewToS: 'Nové podmínky',
        addNewToSFromCurrent: 'Nové podmínky z vybraných',
        added: 'Přidáno',
        checkStatus: 'Zjistit stav uživatele',
        immutable: `POZOR: Přidané podmínky použití už nelze změnit ani
				smazat!`,
        noneExist: 'Zatím nebyly přidány žádné podmínky použití.',
        optUntil: {
            desc: `Během přechodného období je přijetí nových podmínek nepovinné. Povinné je
					až potom.`,
            enable: 'Zapnout přechodné období',
            label: 'Konec přechodného období',
        },
        tos: 'Podmínky',
    },
    users: {
        antiLockout: {
            rule: 'Ochrana proti zablokování',
            delete: 'nelze smazat',
            disable: 'nelze vypnout',
            rauthyAdmin: 'roli rauthy_admin nelze odebrat',
        },
        attributes: 'Atributy',
        deleteUser: 'Opravdu chcete tohoto uživatele smazat?',
        descAttr: `Nastavte vlastní atributy uživatele. Všechny dvojice klíč / hodnota se zpracují jako řetězec / hodnota JSON.`,
        forceLogout: `Opravdu chcete zrušit všechny relace a smazat všechny obnovovací tokeny
            tohoto uživatele?`,
        groupAdmin: {
            notManagedTitle: 'Uživatel mimo vaše skupiny',
            notManagedDesc: `Tento uživatel není v žádné skupině, kterou spravujete, proto jsou podrobnosti skryté.
                Chcete-li ho spravovat, přidejte ho do některé ze svých skupin.
                Členství mimo vaše skupiny zůstanou zachována.`,
            addToGroups: 'Přidat do mých skupin',
        },
        lastLogin: 'Poslední přihlášení',
        manualInitDesc: `Uživatele lze založit i zde. Heslo mu pak ale musíte sdělit
            sami.`,
        manualInit: 'Ruční založení',
        mfa: {
            otp: {
                title: 'Jednorázový kód',
                mfaDelete1: 'Jednorázové kódy tohoto uživatele můžete smazat.',
                mfaDelete2: `Pozor! Smazání jednorázového kódu <b>nelze vrátit zpět</b>; uživatel by si ho musel
                    zaregistrovat znovu.`,
                noMfaOtps: 'Tento uživatel nemá žádné registrované jednorázové kódy.',
            },
            webauthn: {
                title: 'Přístupové klíče',
                mfaDelete1: 'Přístupové klíče tohoto uživatele můžete smazat.',
                mfaDelete2: `Pozor! Smazání přístupového klíče <b>nelze vrátit zpět</b>; uživatel by si ho musel
                    zaregistrovat znovu.`,
                noMfaKeys: 'Tento uživatel nemá žádné registrované přístupové klíče.',
            },
        },
        pkOnly1: 'Tento účet je jen s přístupovým klíčem (Passkey-Only).',
        pkOnly2: 'Uživatel se tedy přihlašuje bez hesla a heslo nemá vůbec nastavené.',
        pkOnly3: `Pokud uživatel ztratil všechny přístupové klíče, lze účet vrátit do výchozího stavu a poslat nový e-mail
            k obnovení hesla. Přejděte na kartu 'MFA' a smažte všechny přístupové klíče.`,
        pwdNoInit: 'Uživatel si ještě nenastavil první heslo.',
        pwdSendEmailBtn: 'Poslat e-mail k obnovení',
        pwdSendEmailDesc: 'Pokud uživatel e-mail k obnovení nedostal, můžete poslat nový.',
        savePassword: 'Uložit heslo',
        selfServiceDesc: 'Můžete nastavit nové heslo, nebo poslat e-mail k obnovení.',
        sendResetEmail: 'Poslat e-mail k obnovení',
    },
    validation: {
        css: 'Neplatná hodnota CSS',
        origin: 'Neplatný původ (origin)',
        uri: 'Neplatné URI',
        redirectUri: {
            comma: 'Přesměrovací URI nesmí obsahovat čárku (,)',
            controlChar: 'Přesměrovací URI nesmí v názvu parametru dotazu obsahovat řídicí znaky',
            fragment: 'Přesměrovací URI nesmí obsahovat fragment (#)',
            reservedKey:
                'Přesměrovací URI nesmí obsahovat parametr dotazu "{{ KEY }}", protože ho nastavuje odpověď autorizace',
            reservedKeyLogout:
                'Přesměrovací URI po odhlášení nesmí obsahovat parametr dotazu "{{ KEY }}", protože se nastavuje při odhlášení',
        },
    },
};
