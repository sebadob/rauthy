import type { I18nAdmin } from './interface.ts';

export let I18nAdminFr: I18nAdmin = {
    api_key: {
        delete1: 'Êtes-vous sûr de vouloir supprimer cette clé API ?',
        expires: 'Expiration',
        generate1: 'Ici, vous pouvez générer un nouveau secret pour cette clé API.',
        generate2: `Ce secret ne sera visible qu'une seule fois après sa génération.
            Lorsqu'un nouveau secret sera généré, l'ancien sera définitivement écrasé.
            Cette opération est irréversible !`,
        generate3: `Une clé API doit être fournie dans l'en-tête HTTP <code>Authorization</code>
            au format suivant :`,
        generate4:
            'Vous pouvez utiliser la requête <code>curl</code> suivante pour tester votre nouvelle clé :',
        generate5:
            "Si vous n'avez pas installé <code>jq</code> et que la requête ci-dessus échoue :",
        keyName: 'Nom de la clé',
        limitedValidity: 'Validité limitée',
    },
    attrs: {
        delete1: 'Êtes-vous sûr de vouloir supprimer cet attribut ?',
        defaultValue: 'Valeur par défaut',
        desc: 'Description',
        makeEditable: 'Rendre modifiable',
        makeEditableP1:
            'Vous pouvez convertir cet attribut et le rendre modifiable par les utilisateurs eux-mêmes.',
        makeEditableP2: `<b>ATTENTION :</b> Ceci ne peut jamais être modifié !
            Toutes les données saisies directement par un utilisateur sont toujours non fiables
            et ne DOIVENT JAMAIS être utilisées pour une quelconque forme d'authentification ou d'autorisation !`,
        makeEditableP3: `Un attribut ne peut pas être rendu modifiable car il a autorisé des saisies non fiables par le
            passé, quelle que soit la durée de cette autorisation.`,
        name: `Nom de l'attribut`,
        userEditable: `Modifiable par l'utilisateur`,
    },
    backup: {
        createBackup: 'Créer une sauvegarde',
        disabledDesc: `Cette fonctionnalité n'est disponible que si Hiqlite est configuré comme base de données.`,
        lastModified: 'Dernière modification',
        local: 'Local',
        name: 'Nom',
        size: 'Taille',
    },
    clients: {
        backchannelLogout:
            'Si ce client prend en charge {{ OIDC_BCL }}, vous pouvez fournir l’URI ici.',
        branding: {
            descHsl: `Les valeurs suivantes doivent être fournies au format HSL. Seules les couleurs de base sont
                fournies. Les canaux alpha et les autres valeurs sont gérés dynamiquement par le thème.`,
            descFullCss: `Les valeurs suivantes doivent être des valeurs CSS <code>color</code> entièrement valides.
                Vous pouvez également utiliser des calculs complexes ou les variables CSS définies ci-dessus.`,
            descVariables: `Chaque étiquette ci-dessous correspond au nom de la variable CSS.
                Vous pouvez donc y faire référence dans les champs libres, par exemple avec
                <code>hsla(var(--action) / .7)</code>.`,
        },
        confidential: 'Confidentiel',
        confidentialNoSecret: 'Ce client n’est pas confidentiel et ne possède donc aucun secret.',
        config: 'Configuration du client',
        delete1: 'Êtes-vous sûr de vouloir supprimer ce client ?',
        descAuthCode: `La validité des codes d'authentification peut être ajustée pour une sécurité renforcée.
            Les codes d'authentification ne peuvent être utilisés qu'une seule fois et sont valides pendant 60 secondes
            par défaut. Plus la validité est courte, mieux c'est, à condition que le client puisse se connecter
            suffisamment rapidement.`,
        descClientUri: `Informations concernant l'URI et les contacts de ce client, à afficher sur la page de connexion.`,
        descName: `Le nom du client peut être modifié sans incidence sur sa configuration. Il est uniquement affiché sur
            la page de connexion.`,
        descGroupPrefix: `La connexion à ce client peut être restreinte par un préfixe de groupe optionnel. Seuls les
            utilisateurs appartenant à ce groupe seront autorisés à se connecter.`,
        descOrigin: `Origines externes autorisées supplémentaires ; généralement nécessaires uniquement si ce client
            doit effectuer des requêtes à Rauthy directement depuis le navigateur, notamment pour les applications
            monopages (SPA).`,
        descPKCE: `Si le client le prend en charge, vous devez toujours activer S256 PKCE pour une sécurité renforcée.
            Si un client non confidentiel (par exemple, une application monopage) est utilisé, vous devez activer au
            moins un défi PKCE pour garantir une sécurité suffisante.`,
        descPKCEEnforce: `Si un défi PKCE est activé, Rauthy l'appliquera lors des connexions et rejettera les requêtes
            de connexion ne contenant pas de défi valide.`,
        descUri: `Vous pouvez fournir autant d'URI de redirection que vous le souhaitez. À la fin de chacune, vous
            pouvez utiliser <code>*</code> comme caractère générique.`,
        errConfidentialPKCE: `Le client doit être confidentiel ou avoir au moins un défi PKCE activé.`,
        forceMfa: 'Forcer l’authentification multifacteur',
        groupLoginPrefix: 'Préfixe du groupe de connexion',
        name: 'Nom du client',
        scim: {
            baseUri: `L'URI de base SCIM est celle à partir de laquelle les sous-routes comme
                <code>{base_uri}/Users/{id}</base_uri></code> peuvent être correctement dérivées.`,
            desc: `Si ce client prend en charge {{ SCIM_LINK }}, vous pouvez l'activer ici.`,
            enable: 'Activer SCIMv2',
            groupSync: 'Synchroniser les groupes',
            groupSyncPrefix: 'Préfixe de filtrage des groupes',
            groupSyncPrefixDesc: `Vous pouvez filtrer les groupes pour la synchronisation à l'aide d'un préfixe optionnel.
                Par exemple, si les groupes <code>app:admins</code> et <code>app:users</code> existent,
                le préfixe <code>app:</code> ne synchroniserait que ces groupes,
                ainsi que uniquement les utilisateurs liés à au moins l'un de ces groupes.`,
            reqDesc: 'Quelques éléments sont requis pour la compatibilité :',
            reqLi1: 'Le client doit gérer correctement <code>externalId</code>.',
            reqLi2: `Au moins les points de terminaison <code>/Users</code> avec <code>filter=externalId eq "*"</code>
                et <code>filter=userName eq "*"</code> doivent être pris en charge.`,
            reqLi3: `Si les groupes doivent être synchronisés, <code>/Groups</code> doit également prendre en charge
                <code>filter=displayName eq "*"</code>.`,
        },
        scopes: {
            allowed: 'Portées autorisées',
            default: 'Portées par défaut',
            desc: `Les portées autorisées sont celles que le client est autorisé à demander dynamiquement lors d'une
                redirection vers la page de connexion en utilisant le flux <code>authorization_code</code>.
                Les portées par défaut seront toujours ajoutées aux jetons pour résoudre certains problèmes lors de
                l'utilisation du <code>password</code> par exemple.`,
        },
        secret: {
            doCache: 'Mettre en cache le secret client',
            cacheDuration: 'Durée du cache (heures)',
            generate: 'Générer un nouveau secret',
            rotateDesc1: `Pour permettre des mises à jour et des rotations de secret fluides,
                vous pouvez conserver le secret actuel dans un cache en mémoire pendant une durée déterminée.
                Vous pouvez saisir une valeur comprise entre 1 et 24 heures.`,
            rotateDesc2:
                'Attention : vous ne devez pas mettre en cache le secret actuel si vous avez subi une fuite !',
        },
        tokenLifetime: {
            p1: `La durée de vie du jeton s’applique aux jetons d’accès et d’identification
                et est exprimée en secondes.`,
            p2: `Si le client prend en charge les algorithmes EdDSA/ed25519, il est toujours préférable de les utiliser.
                Les algorithmes RSA existent uniquement à des fins de compatibilité.`,
            p3: `L’algorithme des jetons d’actualisation ne peut pas être modifié, car ceux-ci sont utilisés
                exclusivement par Rauthy.`,
        },
    },
    common: {
        account: 'Compte',
        addNew: 'Ajouter',
        back: 'Précédent',
        caution: 'ATTENTION',
        contact: 'Contact',
        copiedToClip: 'La valeur a été copiée dans le presse-papiers',
        details: 'Détails',
        edit: 'Modifier',
        enabled: 'Activé',
        filter: 'Filtrer',
        from: 'De',
        information: 'Information',
        language: 'Langue',
        loading: 'Chargement',
        jsonMeta: 'Métadonnées au format JSON',
        name: 'Nom',
        nameExistsAlready: 'Ce nom existe déjà',
        note: 'Note',
        noEntries: 'Aucune entrée',
        preview: 'Aperçu',
        reset: 'Réinitialiser',
        searchOptions: 'Options de recherche',
        until: `Jusqu'à`,
    },
    docs: {
        book: 'Pour la documentation générale sur Rauthy, veuillez consulter le',
        encryption: 'Chiffrement',
        encKeys: {
            header: 'Clés de chiffrement',
            keyActive: 'Clé active',
            keysAvailable: 'Clés disponibles',
            migrate: 'Migrer',
            migrateToKey: 'Migrer toutes les valeurs chiffrées existantes vers la clé suivante',
            p1: `Ces clés sont utilisées pour un chiffrement supplémentaire au repos, indépendamment de toute
                technologie de stockage de données sous-jacente. Elles sont configurées statiquement,
                mais peuvent être renouvelées et migrées manuellement sur cette page.`,
            p2: `La clé active est définie statiquement dans le fichier de configuration Rauthy / les variables
                d'environnement. Elle ne peut pas être modifiée dynamiquement ici.
                Tout nouveau chiffrement JWK utilisera toujours la clé actuellement active.`,
            p3: `Si vous migrez tous les secrets existants, l’opération peut prendre quelques secondes si votre ensemble
                de données est volumineux.`,
            pNotPossible:
                'Pour pouvoir effectuer la migration, au moins deux clés de chiffrement doivent être disponibles.',
        },
        hashing: {
            calculate: 'Calculer',

            currValuesHead: 'Valeurs actuelles',
            currValues1: 'Les valeurs actuelles du serveur sont les suivantes :',
            currValuesNote: `Remarque : Le temps de connexion du serveur n'est indicatif qu'après au moins 5 connexions
                réussies, une fois Rauthy démarré. La valeur de base est toujours de 2000 ms après un redémarrage et
                s'ajuste au fil du temps à chaque connexion réussie.`,
            currValuesThreadsAccess: 'Threads (p_cost) auxquels Rauthy a accès',

            loginTimeHead: 'À propos du temps de connexion',
            loginTime1: `En général, les utilisateurs souhaitent une connexion aussi rapide que possible.
                Toutefois, pour une connexion sécurisée, un temps de connexion compris entre 500 et 1000 ms ne devrait
                pas poser de problème. Le temps de connexion ne doit pas être trop court, car cela réduirait la
                robustesse du hachage.`,
            loginTime2: `Afin d’assurer une sécurité maximale par défaut, cet utilitaire ne permet pas de descendre en
                dessous de 500 ms pour le temps de connexion.`,

            mCost1: `<code>m_cost</code> définit la quantité de <b>mémoire (en ko)</b> utilisée pour le hachage.
                Plus sa valeur est élevée, mieux c'est. Cependant, il faut tenir compte des ressources du serveur.<br>
                Par exemple, si vous hachez 4 mots de passe simultanément, le serveur a besoin de
                <code>4 x m_cost</code> pendant le hachage. Ces ressources doivent être disponibles.`,
            mCost2: `Configurer <code>m_cost</code> est assez simple. Définissez la quantité maximale de mémoire que
                Rauthy doit utiliser, divisez-la par le nombre maximal de connexions parallèles autorisées
                (<code>MAX_HASH_THREADS</code>) et soustrayez une petite quantité de mémoire fixe.
                La quantité de mémoire statique à prendre en compte dépend de la base de données utilisée et du nombre
                total d'utilisateurs, mais se situe généralement entre 32 et 96 Mo.`,
            mCost3: 'La valeur minimale autorisée de <code>m_cost</code> est de <code>32768</code>.',

            pCost1: `La valeur <code>p_cost</code> définit le niveau de <b>parallelism</b> pour le hachage.
                Cette valeur atteint généralement un maximum d'environ 8,
                ce qui correspond à la valeur par défaut pour Rauthy.`,
            pCost2: `La règle générale est la suivante :<br>Définissez <code>p_cost</code> à deux fois le nombre de
                cœurs disponibles.<br>Par exemple, si vous disposez de 4 cœurs, définissez <code>p_cost</code> à
                <code>8</code>.<br>Toutefois, cette valeur doit tenir compte du nombre de connexions parallèles
                autorisées (<code>MAX_HASH_THREADS</code>) et être réduite en conséquence.`,

            tCost1: `La valeur <code>t_cost</code> définit le <b>temps</b> de hachage. C'est la seule valeur qui
                nécessite un réglage, car <code>m_cost</code> et
                <code>p_cost</code> sont généralement fournis par l'environnement.`,
            tCost2: `Le réglage est simple : définissez <code>m_cost</code> et <code>p_cost</code> en conséquence, puis
                augmentez <code>t_cost</code> tant que vous n'avez pas atteint votre objectif de temps de hachage.`,

            utilityHead: 'Utilitaire de calcul de paramètres',
            utility1: `Cet outil permet d’estimer des valeurs optimales pour votre déploiement.
                Il est important de noter qu’il doit être exécuté avec Rauthy installé à son emplacement final et
                disposant de toutes les ressources finales.
                Il est recommandé d’exécuter cet utilitaire pendant la charge afin d’éviter un surdimensionnement.`,
            utility2: `<code>m_cost</code> est facultatif ; la valeur minimale de sécurité <code>32768</code> sera
                choisie s’il est vide. <code>p_cost</code> est également facultatif ; s’il est vide, Rauthy utilisera
                tous les threads disponibles.`,

            time: 'Heure',
            targetTime: 'Heure cible',
            tune: `Important : Ces valeurs doivent être optimisées sur l'architecture finale !`,
            pDetials: `Pour une introduction détaillée à Argon2ID, de nombreuses ressources sont disponibles en ligne.
                Ce guide ne donne qu'un bref aperçu des valeurs. Trois d'entre elles doivent être configurées :`,
            pTune: `Elles varient en fonction des capacités du système. Plus le système est puissant,
                plus ces valeurs peuvent être sécurisées.`,
            pUtility: `Cet utilitaire vous aide à trouver les meilleurs paramètres Argon2ID pour votre plateforme.
                Argon2ID est actuellement l'algorithme de hachage de mot de passe le plus sûr disponible.
                Pour exploiter pleinement son potentiel, il doit être optimisé pour chaque déploiement.`,
        },
        openapi:
            "Si vous souhaitez intégrer une application externe et utiliser l'API de Rauthy, consultez",
        openapiNote: `Selon la configuration du serveur, l'interface utilisateur Swagger peut ne pas être accessible
            publiquement pour le moment. Elle est toutefois disponible par défaut via le serveur HTTP de métriques
            interne afin de ne divulguer aucune information.`,
        source: 'Le code source est disponible ici',
    },
    editor: {
        bold: 'Gras',
        code: 'Code',
        heading1: 'Titre 1',
        heading2: 'Titre 2',
        heading3: 'Titre 3',
        italic: 'Italique',
        link: 'Lien',
        listBullet: 'Liste',
        listTasks: 'Tâches',
        listNumbered: 'Liste numérotée',
        paragraph: 'Paragraphe',
        quote: 'Citation',
        removeFmt: 'Supprimer la mise en forme',
        strikeThrough: 'Barré',
        textArea: 'Modifier le texte',
    },
    email: {
        cancelJob: 'Annuler la tâche',
        filterType: ['Aucun', 'Dans le groupe', 'Hors du groupe', 'A un rôle', `N'a pas de rôle`],
        immediate: 'Immédiat',
        jobs: `Tâches d'envoi d'e-mails`,
        scheduled: 'Envoi programmé',
        sendAllUsers: 'Cet e-mail sera envoyé à tous les utilisateurs.',
        sendAllUsersFiltered: 'Cet e-mail sera envoyé à tous les utilisateurs filtrés par :',
        sendMail: 'Envoyer un e-mail',
        subject: 'Objet',
        userFilter: 'Filtre utilisateur',
    },
    error: {
        needsAdminRole: `Vous n'êtes pas affecté au rôle <b>rauthy_admin</b>.<br/>
            Vous n'avez pas accès au panneau d'administration.`,
        noAdmin: `Un compte administrateur Rauthy doit avoir <b>l'authentification multifacteur (MFA) activée</b>.<br>
            Veuillez vous connecter à votre <b>compte</b> et activer l'authentification multifacteur.<br>
            Ensuite, vous devrez vous déconnecter puis vous reconnecter.`,
    },
    events: {
        eventLevel: `Niveau d'événement`,
        eventType: `Type d'événement`,
    },
    groups: {
        delete1: 'Êtes-vous sûr de vouloir supprimer ce groupe ?',
        name: 'Nom du groupe',
    },
    jwks: {
        alg: 'Algorithme',
        p1: 'Ce sont les clés Web JSON (JWK) utilisées pour la signature des jetons.',
        p2: `Les JWK seront renouvelées par défaut le 1er de chaque mois.
            Pour tous les jetons nouvellement créés, seule la clé la plus récente disponible pour l'algorithme donné
            sera utilisée pour la signature. Les anciennes clés seront conservées pendant un certain temps afin de
            garantir que les jetons actuellement valides puissent toujours être validés correctement.
            Après un certain temps, elles seront supprimées automatiquement.`,
        p3: `Les clés peuvent également être renouvelées manuellement.
            Selon le matériel sur lequel cette instance Rauthy est exécutée, cela peut prendre quelques secondes.`,
        type: 'Type',
        rotateKeys: 'Rotation des clés',
    },
    kv: {
        accessTestDesc: `La clé d'accès doit être fournie dans l'en-tête <code>Authorization</code> sous forme de jeton
            <code>Bearer</code>. La commande <code>curl</code> suivante peut être utilisée pour les tests.`,
        addNewKey: `Nouvelle clé d'accès`,
        addNewNs: 'Nouvel espace de noms',
        addNewValue: 'Nouvelle valeur',
        delConfirm: `Voulez-vous vraiment supprimer cette clé d'accès ?`,
        delNsMsg:
            'Voulez-vous vraiment supprimer cet espace de noms, y compris toutes les données existantes ?',
        encryptedDesc: `Pour des raisons de performance, le chiffrement supplémentaire ne doit être utilisé que pour les
            valeurs particulièrement sensibles telles que les clés d'accès ou les informations personnelles.`,
        deleteConfirmMsg: `Voulez-vous vraiment supprimer la clé '{{ key }}' ?`,
        help: {
            help: 'Aide',
            ops: [
                `Tester une clé d'accès`,
                'Obtenir toutes les clés existantes',
                'Obtenir toutes les clés et leurs valeurs existantes',
                'Définir une clé et sa valeur',
                `Obtenir la valeur d'une clé`,
                'Supprimer une clé',
            ],
            p1: `L’accès externe aux bases de données KV est volontairement très simple.
                Seules quelques opérations sont possibles :`,
            p2: `Chacune de ces opérations requiert une clé d’accès sous forme de jeton <code>Bearer</code>
                (<code>{id}\${secret}</code>) dans l’en-tête <code>Authorization</code>.
                Une clé d’accès n’est valide que pour l’espace de noms dans lequel elle se trouve.`,
            p3: `Vous trouverez ici des exemples d’utilisation de <code>curl</code> pour les opérations mentionnées
                ci-dessus.`,
        },
        key: 'Clé',
        loadAllValues: 'Charger toutes les valeurs',
        storeEncrypted: 'Stocker la valeur chiffrée',
        tabs: ['Données', 'Accès', 'Modifier', 'Supprimer'],
        testCmd: 'Commande de test',
        value: 'Valeur JSON',
    },
    nav: {
        apiKeys: 'Clés API',
        attributes: 'Attributs',
        blacklist: 'Liste noire',
        clients: 'Clients',
        config: 'Configuration',
        docs: 'Documentation',
        events: 'Événements',
        groups: 'Groupes',
        providers: 'Fournisseurs',
        roles: 'Rôles',
        scopes: 'Portées',
        sessions: 'Sessions',
        users: 'Utilisateurs',
    },
    options: {
        expires: 'Expire',
        lastSeen: 'Dernière connexion',
        state: 'État',
    },
    pam: {
        addGroup: 'Nouveau groupe PAM',
        addHost: 'Nouvel hôte PAM',
        addUser: 'Nouvel utilisateur PAM',
        deleteHost: 'Voulez-vous vraiment supprimer cet hôte ?',
        groupDescGeneric: `Les groupes génériques sont l'équivalent des entrées que l'on trouve généralement dans
            /etc/group. Des utilisateurs peuvent y être affectés et ces groupes sont renvoyés au système par
            les requêtes NSS.`,
        groupDescHost: `Les groupes d'hôtes servent à regrouper les hôtes.
            Les requêtes NSS effectuées sur un hôte au sein du groupe renvoient tous les autres hôtes
            qui en font partie.
            Les utilisateurs peuvent accéder aux hôtes en les affectant à un groupe d'hôtes.`,
        groupDescLocal: `Les groupes locaux se comportent de manière presque identique aux groupes génériques,
            à la différence qu'ils possèdent un identifiant dans la base de données Rauthy,
            mais que le proxy NSS sur l'hôte concerné le convertit en un identifiant provenant de /etc/group.
            De cette façon, les utilisateurs Rauthy peuvent être affectés à des groupes existants localement.`,
        groupDescUser: `Les groupes d'utilisateurs sont gérés automatiquement et étroitement liés à l'utilisateur
            portant le même nom d'utilisateur.`,
        groupDescWheel: `Ce groupe est particulier. Il est immuable et est attribué dynamiquement aux utilisateurs
            en fonction de la configuration de leur groupe.`,
        groupName: 'Nom du groupe',
        groups: 'Groupes',
        groupType: 'Type de groupe',
        hostAliases: `Alias d'hôte`,
        hostLocalPwdOnly: 'Connexion par mot de passe local',
        hostLocalPwdOnlyInfo: `Lorsque l'option Connexion par mot de passe local est activée, elle remplace l'option
            Forcer l'authentification multifacteur pour les connexions locales.
            Par conséquent, aucun mot de passe ne sera jamais demandé (localement) lors des connexions,
            même si l'utilisateur est protégé par l'authentification multifacteur.
            Cette option ne doit être activée qu'en cas de réelle nécessité,
            par exemple si les utilisateurs protégés par l'authentification multifacteur doivent pouvoir
            se connecter localement sans utiliser de clé matérielle.`,
        ipAddresses: 'Adresses IP',
        member: 'Membre',
        nameExistsAlready: 'Ce nom est déjà utilisé',
        notes: 'Notes',
        secretShow: 'Afficher le secret',
        secretRotate: 'Rotation du secret',
        userEmail: `Adresse e-mail de l'utilisateur associé`,
        username: `Nom d'utilisateur`,
        usernameNewDesc: `Le nom d'utilisateur doit être choisi avec soin.
            Une fois créé, il ne peut pas être modifié facilement par la suite pour des raisons de sécurité.`,
    },
    passwordPolicy: {
        configDesc: 'Politique relative aux nouveaux mots de passe.',
        resetSet0: `La valeur 0 désactive l'exigence.`,
        validForDays: 'Durée de validité',
        validityNew: 'Validité pour les nouveaux mots de passe.',
    },
    providers: {
        config: {
            allowInsecureTls: 'Autoriser TLS non sécurisé',
            autoLink: `Lien automatique avec l'utilisateur`,
            autoLinkDesc1: `Si l'option Lien automatique avec l'utilisateur est activée,
                la connexion via ce fournisseur liera automatiquement un utilisateur existant,
                mais non lié, à ce fournisseur.`,
            autoLinkDesc2: `ATTENTION : Cette option peut être très dangereuse et entraîner une prise de contrôle de
                compte si le fournisseur ne valide pas entièrement les adresses e-mail des utilisateurs et permet donc
                d'ajouter une adresse étrangère pour un utilisateur ! Ne doit JAMAIS être utilisée dans un tel cas !`,
            clientName: 'Nom du client',
            custRootCa: 'Autorité de certification racine personnelle PEM',
            descAuthMethod: `Méthode d'authentification à utiliser sur le point de terminaison <code>/token</code>.<br>
                La plupart des fournisseurs fonctionnent avec <code>basic</code>,
                certains uniquement avec <code>post</code>.
                Dans de rares cas, les deux sont nécessaires, tandis que d'autres peuvent entraîner des erreurs.`,
            descClientId: `ID client fourni par le fournisseur d'authentification.`,
            descClientName: 'Nom du client à afficher sur la page de connexion Rauthy.',
            descClientSecret: `Secret client fourni par le fournisseur d'authentification.
                Un secret client ou un PKCE est requis.`,
            descScope: `Étendue des droits que le client doit utiliser lors de la redirection vers la page de connexion.
                Séparez les valeurs par un espace.`,
            errNoAuthMethod: `Vous avez fourni un secret client,mais aucune méthode d'authentification client n'est active`,
            errConfidential: 'Vous devez au moins utiliser un client confidentiel ou PKCE',
            jsonPath: {
                p1: `Les valeurs du jeton d'identification après une connexion réussie en amont peuvent être
                    mappées automatiquement.`,
                p2: `Le <code>path</code> doit être fourni selon une syntaxe de type expression régulière.
                    Il peut être résolu en valeurs JSON uniques ou en une valeur dans un objet ou un tableau JSON.`,
                p3: `<code>$.</code> marque le début de l'objet JSON`,
                p4: '<code>*</code> peut être utilisé comme caractère générique dans votre chemin',
                p5: '<code>$.roles</code> ciblerait <code>&#123;"roles": "value"&#125;</code>',
                p6: `<code>$.roles.*</code> peut cibler une valeur à l'intérieur d'un objet ou d'un tableau comme<br>
                    <code>&#123;"roles": ["value", "notMyValue"]&#125;</code>`,
            },
            lookup: 'Chercher',
            pathAdminClaim: `Chemin de revendication d'administration`,
            pathMfaClaim: 'Chemin de revendication MFA',
            rootPemCert: 'Certificat PEM racine',
            mapMfa: `Si votre fournisseur émet une attestation indiquant que l'utilisateur a utilisé au moins
                l'authentification à deux facteurs (2FA) lors de la connexion,
                vous pouvez spécifier le chemin d'accès à l'attestation multifacteur (MFA).`,
            mapUser: `Vous pouvez associer un utilisateur au rôle d'administrateur Rauthy en fonction d'une attestation
                d'identification en amont.`,
            valueAdminClaim: `Valeur de l'attestation d'administrateur`,
            valueMfaClaim: `Valeur de l'attestation multifacteur`,
        },
        delete: {
            areYouSure: 'Êtes-vous sûr de vouloir supprimer ce fournisseur ?',
            forceDelete: 'Forcer la suppression',
            isInUse1: 'Ce fournisseur est utilisé par des utilisateurs actifs !',
            isInUse2: `Vous pouvez forcer sa suppression, mais les utilisateurs sans mot de passe ou clé d'accès locale
                ne pourront plus se connecter.`,
            linkedUsers: 'Utilisateurs liés',
        },
    },
    roles: {
        adminNoMod: 'Le rôle <code>rauthy_admin</code> est immuable.',
        delete1: 'Êtes-vous sûr de vouloir supprimer ce rôle ?',
        name: 'Nom du rôle',
    },
    scopes: {
        defaultNoMod: `Il s'agit d'une étendue OIDC par défaut. Ces étendues sont immuables.`,
        delete1: 'Êtes-vous sûr de vouloir supprimer cette étendue ?',
        deleteDefault: 'Les étendues OIDC par défaut ne peuvent pas être supprimées.',
        mapping1: 'Vous pouvez associer des étendues personnalisées à des attributs.',
        mapping2: `Tous les attributs supplémentaires configurés peuvent avoir une valeur personnalisée pour chaque
            utilisateur. Lorsqu'ils sont associés à une étendue, ils peuvent être inclus dans les jetons d'accès et/ou
            d'identification.`,
        name: `Nom de l'étendue`,
    },
    search: {
        orderBy: 'Trier par...',
        orderChangeToAsc: 'Trier par ordre croissant',
        orderChangeToDesc: 'Trier par ordre décroissant',
    },
    sessions: {
        invalidateAll: 'Invalider toutes les sessions',
    },
    tabs: {
        config: 'Configuration',
        delete: 'Supprimer',
    },
    tos: {
        accepted: 'Accepté',
        addNewToS: 'Nouvelles CGU',
        addNewToSFromCurrent: 'Nouvelles CGU sélectionnées',
        added: 'Ajouté',
        checkStatus: `Vérifier le statut de l'utilisateur`,
        immutable: `ATTENTION : Une fois ajoutées, les nouvelles Conditions d'utilisation sont immuables et ne peuvent
            pas être supprimées !`,
        noneExist: `Aucune condition d'utilisation n'a encore été ajoutée.`,
        optUntil: {
            desc: `Pendant la période de transition, l'acceptation des CGU mises à jour est facultative.
                Elle ne devient obligatoire qu'ensuite.`,
            enable: 'Activer la période de transition',
            label: 'Fin de la période de transition',
        },
        tos: 'CGU',
    },
    users: {
        antiLockout: {
            rule: 'Règle anti-blocage',
            delete: 'ne peut pas être supprimée',
            disable: 'ne peut pas être désactivée',
            rauthyAdmin: 'la règle rauthy_admin ne peut pas être supprimée',
        },
        attributes: 'Attributs',
        deleteUser: 'Êtes-vous sûr de vouloir supprimer cet utilisateur ?',
        descAttr: `Définissez les attributs personnalisés de l'utilisateur. Toutes les paires clé/valeur seront traitées
            comme des chaînes de caractères ou des valeurs JSON.`,
        forceLogout: `Êtes-vous sûr de vouloir invalider toutes les sessions existantes et supprimer tous les jetons
            d'actualisation de cet utilisateur ?`,
        lastLogin: 'Dernière connexion',
        manualInitDesc: `L’utilisateur peut également être initialisé ici. Dans ce cas, vous devez communiquer le
            mot de passe directement.`,
        manualInit: 'Initialisation manuelle',
        mfaDelete1: 'Vous pouvez supprimer les clés d’accès de cet utilisateur.',
        mfaDelete2: `Attention ! La suppression d’une clé d’accès est irréversible ; l’utilisateur doit procéder à une
            nouvelle inscription complète.`,
        noMfaKeys: 'Cet utilisateur n’a aucune clé d’accès enregistrée.',
        pkOnly1: 'Il s’agit d’un compte utilisant uniquement une clé d’accès.',
        pkOnly2: `Cet utilisateur utilise la connexion sans mot de passe et n'a défini aucun mot de passe.`,
        pkOnly3: `Si cet utilisateur a perdu tous ses identifiants, son compte peut être entièrement réinitialisé et un
            nouvel e-mail de réinitialisation de mot de passe peut lui être envoyé. Pour ce faire, accédez à l'onglet
            'MFA' et supprimez tous les identifiants existants.`,
        pwdNoInit: `L'utilisateur n'a pas encore effectué la réinitialisation initiale de son mot de passe.`,
        pwdSendEmailBtn: 'Envoyer un e-mail de réinitialisation',
        pwdSendEmailDesc: `Vous pouvez envoyer un nouvel e-mail de réinitialisation si l'utilisateur n'en a pas reçu.`,
        savePassword: 'Enregistrer le mot de passe',
        selfServiceDesc:
            'Vous pouvez soit définir un nouveau mot de passe, soit envoyer un e-mail de réinitialisation.',
        sendResetEmail: 'Envoyer un e-mail de réinitialisation',
    },
    validation: {
        css: 'Valeur CSS invalide',
        origin: 'Origine invalide',
        uri: 'URI invalide',
    },
};
