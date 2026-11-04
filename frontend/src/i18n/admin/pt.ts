import type { I18nAdmin } from './interface.ts';

export let I18nAdminPt: I18nAdmin = {
    api_key: {
        delete1: 'Tem a certeza de que pretende eliminar esta API Key?',
        expires: 'Expiração',
        generate1: 'Aqui pode gerar um novo secret para esta API Key.',
        generate2: `Só verá este secret uma vez, logo após ser gerado.
            Quando for gerado um novo, o secret antigo será substituído de forma permanente.
            Esta operação não pode ser revertida!`,
        generate3: `A API Key tem de ser indicada no cabeçalho HTTP <code>Authorization</code>
            com o seguinte formato:`,
        generate4: 'Pode usar o seguinte pedido <code>curl</code> para testar a sua nova Key:',
        generate5: 'Se não tiver o <code>jq</code> instalado e o comando acima falhar:',
        keyName: 'Nome da Key',
        limitedValidity: 'Validade limitada',
    },
    attrs: {
        delete1: 'Tem a certeza de que pretende eliminar este atributo?',
        defaultValue: 'Valor predefinido',
        desc: 'Descrição',
        makeEditable: 'Tornar editável',
        makeEditableP1:
            'Pode converter este atributo e torná-lo editável pelos próprios utilizadores.',
        makeEditableP2: `<b>ATENÇÃO:</b> Esta alteração nunca poderá ser revertida! Todos os dados introduzidos
            diretamente por um utilizador são sempre dados não fidedignos e NUNCA podem ser usados para qualquer
            forma de autenticação ou autorização!`,
        makeEditableP3: `Um atributo não pode passar de editável a não editável, porque já aceitou dados não
            fidedignos no passado, independentemente do tempo durante o qual isso aconteceu.`,
        name: 'Nome do atributo',
        userEditable: 'Editável pelo utilizador',
    },
    backup: {
        createBackup: 'Criar cópia de segurança',
        disabledDesc:
            'Esta funcionalidade só existe se o Hiqlite estiver configurado como base de dados.',
        lastModified: 'Última modificação',
        local: 'Local',
        name: 'Nome',
        size: 'Tamanho',
    },
    clients: {
        allowedResources: 'Recursos permitidos',
        defaultAud: 'Audiences predefinidas',
        descAllowedResources: `Resource Indicators (RFC 8707) opcionais que este Client pode pedir. Uma lista vazia rejeita qualquer parâmetro 'resource' com 'invalid_target'.`,
        descDefaultAud: `Audiences que são sempre adicionadas aos tokens deste Client, independentemente de qualquer parâmetro 'resource'.`,
        backchannelLogout: 'Se este Client suportar {{ OIDC_BCL }}, pode indicar aqui o URI.',
        branding: {
            descHsl: `Os valores seguintes têm de ser indicados como valores HSL. Indique apenas as cores base.
            Os canais alfa e outros valores são ajustados dinamicamente pelo tema.`,
            descFullCss: `Os valores seguintes têm de ser valores CSS <code>color</code> totalmente válidos.
                Também pode usar cálculos complexos ou as variáveis CSS definidas acima.`,
            descVariables: `Cada uma das etiquetas seguintes é também o nome da variável CSS. Isto significa
                que as pode referenciar nos campos livres, por exemplo com <code>hsla(var(--action) / .7)</code>.`,
            faviconPreviewAlt: 'Pré-visualização do favicon do Client',
            faviconUpload: 'Carregar favicon',
        },
        claimsAtRoot: 'Emitir claims na raiz do token',
        claimsAtRootWarning: `As claims deste Client são escritas na raiz do token,
            em vez de ficarem agrupadas em 'custom'. A responsabilidade de evitar colisões é sua: se o nome
            de uma claim colidir com uma claim JWT reservada, a emissão do token falha. As claims personalizadas
            na raiz também podem deixar de funcionar com futuras alterações do protocolo ou de funcionalidades.
            Referência:`,
        claims: 'Custom claims (client_credentials)',
        claimsDesc: `Um objeto JSON emitido nos tokens client_credentials, agrupado na
            claim custom. Máximo de 1024 carateres serializados.`,
        confidential: 'Confidencial',
        confidentialNoSecret: 'Este é um Client não confidencial e, por isso, não tem secret.',
        config: 'Configuração do Client',
        delete1: 'Tem a certeza de que pretende eliminar este Client?',
        descAuthCode: `A validade dos auth codes pode ser ajustada para aumentar a segurança. Os auth codes
            só podem ser usados uma vez e, por predefinição, são válidos durante 60 segundos. Quanto mais curta a
            validade, melhor, desde que o Client consiga concluir o processo de início de sessão a tempo.`,
        descClientUri: `Informações sobre o URI e os contactos deste Client, a mostrar na
            página de início de sessão.`,
        descName: `O nome do Client pode ser alterado sem qualquer impacto na configuração do Client.
            Serve apenas para ser mostrado na página de início de sessão.`,
        descGroupPrefix: `O início de sessão neste Client pode ser restringido por um prefixo de grupo opcional.
            Só os utilizadores atribuídos a um grupo correspondente poderão iniciar sessão.`,
        descOrigin: `Origins externas adicionais permitidas - normalmente só são necessárias se este Client
            tiver de fazer pedidos ao Rauthy diretamente a partir do navegador, como acontece tipicamente nas SPAs.`,
        descPKCE: `Se o Client o suportar, deve ativar sempre o PKCE S256 para uma segurança adicional.
            Se for usado um Client não confidencial (por exemplo, uma SPA), tem de ativar pelo menos
            um dos PKCE challenges para garantir segurança suficiente.`,
        descPKCEEnforce: `Se algum PKCE estiver ativo, o Rauthy impõe a sua utilização no início de sessão e
            rejeita os pedidos de início de sessão que não contenham um challenge válido.`,
        descUri: `Pode indicar tantos Redirect URIs quantos quiser. No final de cada um, pode usar
            <code>*</code> como wildcard.`,
        errConfidentialPKCE: `O Client tem de ser confidencial ou ter pelo menos um PKCE
            challenge ativo.`,
        forceMfa: 'Forçar MFA',
        groupLoginPrefix: 'Prefixo de grupo para início de sessão',
        name: 'Nome do Client',
        passwordFlowMfaWarn: `ATENÇÃO: 'Forçar MFA' e o flow 'password' estão ativos ao mesmo
            tempo. O Rauthy NÃO consegue impor a MFA através deste flow de autenticação, porque isso
            violaria o RFC do OIDC. Assim, se precisar estritamente de MFA, tem de a validar do lado do
            Client. Para isso, pode usar a claim 'amr'.`,
        scim: {
            baseUri: `O base URI do SCIM é aquele a partir do qual as sub-rotas, como
                <code>{base_uri}/Users/{id}</base_uri></code>, podem ser derivadas corretamente.`,
            desc: 'Se este Client suportar {{ SCIM_LINK }}, pode ativá-lo aqui.',
            enable: 'Ativar SCIMv2',
            groupSync: 'Sincronizar grupos',
            groupSyncPrefix: 'Prefixo do filtro de grupos',
            groupSyncPrefixDesc: `Pode filtrar os grupos a sincronizar através de um prefixo opcional.
                Por exemplo, se existirem os grupos <code>app:admins</code> e <code>app:users</code>, o prefixo
                <code>app:</code> sincronizaria apenas estes grupos, bem como apenas os utilizadores associados a
                pelo menos um destes grupos.`,
            reqDesc: 'Para garantir a compatibilidade, são necessários alguns requisitos:',
            reqLi1: 'O Client tem de tratar <code>externalId</code> corretamente.',
            reqLi2: `Têm de ser suportados pelo menos os endpoints <code>/Users</code> com
                <code>filter=externalId eq "*"</code> e <code>filter=userName eq "*"</code>.`,
            reqLi3: `Se os grupos forem sincronizados, <code>/Groups</code> também tem de suportar
                <code>filter=displayName eq "*"</code>.`,
        },
        scopes: {
            allowed: 'Scopes permitidos',
            default: 'Scopes predefinidos',
            desc: `Os Scopes permitidos são aqueles que o Client pode pedir dinamicamente durante
            o redirecionamento para o início de sessão, ao usar o flow <code>authorization_code</code>. Os
            scopes predefinidos são sempre adicionados aos tokens, para resolver alguns problemas, por exemplo
            ao usar o flow <code>password</code>.`,
        },
        secret: {
            doCache: 'Guardar o Client Secret em cache',
            cacheDuration: 'Duração da cache (horas)',
            generate: 'Gerar novo secret',
            rotateDesc1: `Para permitir atualizações e rotações de secrets sem interrupções, pode manter o
                secret atual numa cache em memória durante algum tempo. Pode indicar um valor entre 1 e 24 horas.`,
            rotateDesc2:
                'Atenção: Não deve guardar o secret atual em cache se este tiver sido divulgado!',
        },
        tokenLifetime: {
            p1: `A duração do token aplica-se aos Access tokens e ID tokens e é indicada em segundos.`,
            p2: `Se o Client suportar algoritmos EdDSA / Ed25519, estes devem ser sempre a opção
                preferida. Os algoritmos RSA existem apenas por questões de compatibilidade.`,
            p3: `O algoritmo dos refresh tokens não pode ser alterado, porque estes são usados apenas pelo Rauthy.`,
        },
    },
    common: {
        account: 'Conta',
        addNew: 'Adicionar',
        back: 'Voltar',
        caution: 'ATENÇÃO',
        contact: 'Contacto',
        copiedToClip: 'O valor foi copiado para a área de transferência',
        details: 'Detalhes',
        edit: 'Editar',
        enabled: 'Ativo',
        filter: 'Filtro',
        from: 'De',
        information: 'Informação',
        language: 'Idioma',
        loading: 'A carregar',
        jsonMeta: 'Metadados como valor JSON',
        name: 'Nome',
        nameExistsAlready: 'O nome já existe',
        note: 'Nota',
        noEntries: 'Sem entradas',
        preview: 'Pré-visualização',
        reset: 'Repor',
        searchOptions: 'Opções de pesquisa',
        until: 'Até',
    },
    docs: {
        book: 'Para a documentação geral sobre o próprio Rauthy, consulte o',
        encryption: 'Encriptação',
        encKeys: {
            header: 'Chaves de encriptação',
            keyActive: 'Chave ativa',
            keysAvailable: 'Chaves disponíveis',
            migrate: 'Migrar',
            migrateToKey: 'Migrar todos os valores encriptados existentes para a seguinte chave',
            p1: `Estas chaves são usadas para uma encriptação adicional dos dados em repouso, independentemente da
            tecnologia de armazenamento usada internamente. São configuradas de forma estática, mas podem ser rodadas
            e migradas manualmente nesta página.`,
            p2: `A chave ativa é definida de forma estática no ficheiro de configuração do Rauthy / nas variáveis de
            ambiente. Não pode ser alterada aqui de forma dinâmica. Todas as novas encriptações de JWK usarão sempre a
            chave ativa atual.`,
            p3: `Se migrar todos os secrets existentes, a operação pode demorar alguns segundos a terminar, caso tenha
            um grande volume de dados.`,
            pNotPossible:
                'Para poder migrar, têm de estar disponíveis pelo menos 2 chaves de encriptação.',
        },
        hashing: {
            calculate: 'Calcular',

            currValuesHead: 'Valores atuais',
            currValues1: 'Os valores atuais do backend são os seguintes:',
            currValuesNote: `Nota: O tempo de início de sessão indicado pelo backend só é uma boa referência depois
            de pelo menos 5 inícios de sessão bem-sucedidos desde o arranque do Rauthy. O valor base é sempre 2000 ms
            após um reinício e vai sendo ajustado com cada início de sessão bem-sucedido.`,
            currValuesThreadsAccess: 'Threads (p_cost) a que o Rauthy tem acesso',

            loginTimeHead: 'Uma nota sobre o tempo de início de sessão',
            loginTime1: `Em geral, os utilizadores querem que tudo seja o mais rápido possível. No entanto, num início
            de sessão seguro, um tempo entre 500 e 1000 ms não deve ser um problema. O tempo de início de sessão não pode
            ser demasiado curto, porque isso reduziria, naturalmente, a robustez do hash.`,
            loginTime2: `Para garantir a maior segurança possível por predefinição, esta ferramenta não permite
            valores inferiores a 500 ms para o tempo de início de sessão.`,

            mCost1: `O <code>m_cost</code> define a quantidade de <b>memória (em kB)</b> usada no hashing.
            Quanto maior o valor, melhor, naturalmente. Mas tem de ter em conta os recursos do servidor.<br>
            Por exemplo, ao calcular o hash de 4 palavras-passe em simultâneo, o backend precisa de
            <code>4 x m_cost</code> durante o hashing. Estes recursos têm de estar disponíveis.`,
            mCost2: `Ajustar o <code>m_cost</code> é bastante simples. Defina a quantidade máxima de memória que o
            Rauthy deve usar, divida-a pelo número máximo de inícios de sessão em paralelo permitidos
            (<code>MAX_HASH_THREADS</code>) e subtraia uma pequena quantidade fixa de memória. A quantidade de memória
            fixa a considerar depende da base de dados usada e do número total de utilizadores, mas situa-se
            normalmente entre 32 e 96 MB.`,
            mCost3: 'O valor mínimo permitido para <code>m_cost</code> é <code>32768</code>.',

            pCost1: `O <code>p_cost</code> define o grau de <b>paralelismo</b> do hashing. Na maioria dos casos, este
            valor atinge o máximo em ~8, que é o valor predefinido do Rauthy.`,
            pCost2: `A regra geral é:<br>
            Defina o <code>p_cost</code> para o dobro do número de núcleos disponíveis.<br>
            Por exemplo, se tiver 4 núcleos disponíveis, defina o <code>p_cost</code> para <code>8</code>.<br>
            No entanto, este valor tem de ter em conta o número de inícios de sessão em paralelo permitidos
            (<code>MAX_HASH_THREADS</code>) e ser reduzido em conformidade.`,

            tCost1: `O <code>t_cost</code> define a quantidade de <b>tempo</b> do hashing. Na prática, este é o
            único valor que precisa de ser ajustado, porque o <code>m_cost</code> e o <code>p_cost</code> são, no
            essencial, determinados pelo ambiente.`,
            tCost2: `O ajuste é simples: defina o <code>m_cost</code> e o <code>p_cost</code> de forma adequada e,
            depois, aumente o <code>t_cost</code> até atingir o tempo de hashing pretendido.`,

            utilityHead: 'Ferramenta de cálculo de parâmetros',
            utility1: `Pode usar esta ferramenta para estimar bons valores para a sua instalação. Tenha em conta que
            deve ser executada com o Rauthy no seu local definitivo e com todos os recursos finais disponíveis. Deve
            executar esta ferramenta sob carga, para não sobredimensionar os valores.`,
            utility2: `O <code>m_cost</code> é opcional e, se ficar vazio, será escolhido o valor mínimo seguro de
            <code>32768</code>. O <code>p_cost</code> também é opcional e, se ficar vazio, o Rauthy usará todas as
            threads que conseguir detetar.`,

            time: 'Tempo',
            targetTime: 'Tempo pretendido',
            tune: 'Importante: Estes valores têm de ser ajustados na arquitetura final!',
            pDetails: `Se pretender uma introdução detalhada ao Argon2ID, existem muitas fontes online. Este guia
            apresenta apenas uma visão geral muito breve dos valores. Três deles têm de ser configurados:`,
            pTune: `Variam consoante as capacidades do sistema. Quanto mais potente for o sistema, mais seguros
            podem ser estes valores.`,
            pUtility: `Esta ferramenta ajuda a encontrar as melhores definições de Argon2ID para a sua plataforma.
            O Argon2ID é atualmente o algoritmo de hashing de palavras-passe mais seguro disponível. Para tirar o
            máximo partido dele, tem de ser ajustado em cada instalação.`,
        },
        openapi: 'Se pretende integrar uma aplicação externa e usar a API do Rauthy, consulte a',
        openapiNote: `Na configuração predefinida, a Swagger UI não está ativa através da ligação acima. Pode ser
            ativada através da configuração 'server.swagger_ui_enable' e, eventualmente, 'server.swagger_ui_public'.`,
        source: 'O código-fonte pode ser consultado aqui',
    },
    editor: {
        bold: 'Negrito',
        code: 'Código',
        heading1: 'Título 1',
        heading2: 'Título 2',
        heading3: 'Título 3',
        italic: 'Itálico',
        link: 'Ligação',
        listBullet: 'Lista',
        listTasks: 'Tarefas',
        listNumbered: 'Lista numerada',
        paragraph: 'Parágrafo',
        quote: 'Citação',
        removeFmt: 'Remover formatação',
        strikeThrough: 'Rasurado',
        textArea: 'Editar texto',
    },
    email: {
        cancelJob: 'Cancelar tarefa',
        filterType: ['Nenhum', 'No grupo', 'Fora do grupo', 'Com a função', 'Sem a função'],
        immediate: 'Imediato',
        jobs: 'Tarefas de e-mail',
        scheduled: 'Envio agendado',
        sendAllUsers: 'Este e-mail será enviado a todos os utilizadores.',
        sendAllUsersFiltered: 'Este e-mail será enviado a todos os utilizadores filtrados por:',
        sendMail: 'Enviar e-mail',
        subject: 'Assunto',
        userFilter: 'Filtro de utilizadores',
    },
    error: {
        needsAdminRole: `Não tem a função <b>rauthy_admin</b> atribuída.<br/>
            Não tem acesso ao painel de administração.`,
        noAdmin: `Uma conta de administrador do Rauthy tem de ter a <b>MFA ativa.</b><br>
            Aceda à sua <b>conta</b> e ative a MFA.<br>
            Depois, tem de terminar sessão e voltar a iniciar sessão.`,
    },
    events: {
        eventLevel: 'Nível do evento',
        eventType: 'Tipo de evento',
    },
    groups: {
        delete1: 'Tem a certeza de que pretende eliminar este grupo?',
        name: 'Nome do grupo',
    },
    jwks: {
        alg: 'Algoritmo',
        p1: 'Estas são as Json Web Keys (JWKs) usadas para assinar os tokens.',
        p2: `Por predefinição, as JWKs são rodadas no dia 1 de cada mês. Para todos os tokens novos, é usada
        apenas a chave mais recente disponível para o algoritmo em causa. As chaves antigas são mantidas durante algum
        tempo, para garantir que os tokens atualmente válidos continuam a poder ser validados corretamente. Passado
        algum tempo, são removidas automaticamente.`,
        p3: `As chaves também podem ser rodadas manualmente. Consoante o hardware em que esta instância do Rauthy
        está a ser executada, a operação pode demorar alguns segundos.`,
        type: 'Tipo',
        rotateKeys: 'Rodar chaves',
    },
    kv: {
        accessTestDesc: `A Access Key tem de ser indicada no cabeçalho <code>Authorization</code> como
            token <code>Bearer</code>. Pode usar o seguinte comando <code>curl</code> para
            testar.`,
        addNewKey: 'Nova Access Key',
        addNewNs: 'Novo namespace',
        addNewValue: 'Novo valor',
        delConfirm: 'Pretende mesmo eliminar esta Access Key?',
        delNsMsg: 'Pretende mesmo eliminar este namespace, incluindo todos os dados existentes?',
        encryptedDesc: `Por motivos de desempenho, a encriptação adicional só deve ser usada para
            valores particularmente sensíveis, como access keys ou dados pessoais.`,
        deleteConfirmMsg: `Pretende mesmo eliminar a chave '{{ key }}'?`,
        help: {
            help: 'Ajuda',
            ops: [
                'Testar uma Access Key',
                'Obter todas as chaves existentes',
                'Obter todas as chaves + valores existentes',
                'Definir uma chave + valor',
                'Obter o valor de uma chave',
                'Eliminar uma chave',
            ],
            p1: `O acesso externo aos KV stores é propositadamente muito simples. Existem apenas
                algumas operações:`,
            p2: `Cada uma destas operações exige uma Access Key como token <code>Bearer</code>
                (<code>{id}\${secret}</code>) no cabeçalho <code>Authorization</code>. Uma Access
                Key só é válida no namespace em que existe.`,
            p3: `Aqui encontra exemplos das operações acima referidas usando
                <code>curl</code>.`,
        },
        key: 'Chave',
        loadAllValues: 'Carregar todos os valores',
        storeEncrypted: 'Guardar o valor encriptado',
        tabs: ['Dados', 'Acesso', 'Editar', 'Eliminar'],
        testCmd: 'Comando de teste',
        value: 'Valor JSON',
    },
    nav: {
        apiKeys: 'API Keys',
        attributes: 'Atributos',
        blacklist: 'Lista de bloqueio',
        clients: 'Clients',
        config: 'Configuração',
        docs: 'Documentação',
        events: 'Eventos',
        groups: 'Grupos',
        providers: 'Fornecedores',
        roles: 'Funções',
        scopes: 'Scopes',
        sessions: 'Sessões',
        users: 'Utilizadores',
    },
    options: {
        expires: 'Expira',
        lastSeen: 'Visto pela última vez',
        state: 'Estado',
    },
    pam: {
        addGroup: 'Novo grupo PAM',
        addHost: 'Novo host PAM',
        addUser: 'Novo utilizador PAM',
        deleteHost: 'Pretende mesmo eliminar este host?',
        deleteUser: 'Pretende mesmo eliminar este utilizador?',
        groupDescGeneric: `Os grupos Generic são o equivalente às entradas que normalmente se encontram em
            /etc/group. Os utilizadores podem ser atribuídos a estes grupos, que são devolvidos ao sistema pelos
            NSS lookups.`,
        groupDescHost: `Os grupos Host servem para agrupar hosts. Os NSS lookups de um host dentro do grupo devolvem
            como resultado todos os outros hosts desse grupo. Os utilizadores podem aceder a hosts ao serem atribuídos a
            um grupo Host.`,
        groupDescLocal: `Os grupos Local comportam-se de forma quase idêntica aos grupos Generic, com a diferença de
            que têm um ID na base de dados do Rauthy, mas o proxy NSS no respetivo host converte-o num ID de
            /etc/group. Desta forma, os utilizadores do Rauthy podem ser atribuídos a grupos que já existem localmente.`,
        groupDescUser: `Os grupos User são geridos automaticamente e estão fortemente associados ao utilizador com o
            mesmo nome de utilizador.`,
        groupDescWheel: `Este grupo é especial. Não pode ser alterado e é atribuído dinamicamente aos utilizadores,
            consoante a respetiva configuração de grupos.`,
        groupName: 'Nome do grupo',
        groups: 'Grupos',
        groupType: 'Tipo de grupo',
        hostAliases: 'Aliases do host',
        hostLocalPwdOnly: 'Início de sessão local com palavra-passe',
        hostLocalPwdOnlyInfo: `Quando o início de sessão local com palavra-passe está definido, sobrepõe-se a
            'Forçar MFA' nos inícios de sessão locais. Ao mesmo tempo, as Passkeys nunca serão pedidas (localmente)
            durante o início de sessão, mesmo que o utilizador esteja protegido por MFA. Esta opção só deve ser
            definida se for mesmo necessária, por exemplo se utilizadores protegidos por MFA tiverem de poder
            iniciar sessão localmente sem usar Passkeys de hardware.`,
        ipAddresses: 'Endereços IP',
        member: 'Membro',
        nameExistsAlready: 'O nome já está a ser usado',
        notes: 'Notas',
        secretShow: 'Mostrar secret',
        secretRotate: 'Rodar secret',
        userEmail: 'E-mail do utilizador associado',
        username: 'Nome de utilizador',
        usernameNewDesc: `O nome de utilizador deve ser escolhido com cuidado. Depois de criado, não pode ser
            facilmente alterado por motivos de segurança.`,
    },
    passwordPolicy: {
        configDesc: 'Política para novas palavras-passe.',
        resetSet0: 'O valor 0 desativa o requisito.',
        validForDays: 'Válida durante (dias)',
        validityNew: 'Validade das novas palavras-passe.',
    },
    providers: {
        config: {
            allowInsecureTls: 'Permitir TLS inseguro',
            autoLink: 'Associar utilizador automaticamente',
            autoLinkDesc1: `Se a opção 'Associar utilizador automaticamente' estiver ativa, o início de sessão
                através deste fornecedor associa automaticamente a este fornecedor um utilizador que já exista e
                ainda não esteja associado.`,
            autoLinkDesc2: `ATENÇÃO: Esta opção pode ser muito perigosa e levar ao roubo de contas se o fornecedor
                não validar completamente os endereços de e-mail dos utilizadores, permitindo assim associar a um
                utilizador um endereço que não lhe pertence! NUNCA pode ser usada nesse caso!`,
            clientName: 'Nome do Client',
            custRootCa: 'Root CA PEM personalizada',
            descAuthMethod: `O método de autenticação a usar no endpoint <code>/token</code>.<br>
                A maioria dos fornecedores deve funcionar com <code>basic</code>, alguns apenas com <code>post</code>.
                Em situações raras, são necessários os dois, embora isso possa causar erros com outros fornecedores.`,
            descClientId: 'Client ID atribuído pelo fornecedor de autenticação.',
            descClientName: 'Nome do Client a mostrar na página de início de sessão do Rauthy.',
            descClientSecret: `Client Secret atribuído pelo fornecedor de autenticação.
                É necessário pelo menos um Client Secret ou PKCE.`,
            descScope: `O scope que o Client deve usar ao redirecionar para o início de sessão.
                Indique os valores separados por espaços.`,
            errNoAuthMethod:
                'Indicou um Client Secret, mas nenhum método de autenticação do Client está ativo',
            errConfidential: 'Tem de ser pelo menos um Client confidencial ou usar PKCE',
            jsonPath: {
                p1: 'Os valores do ID token podem ser mapeados automaticamente após um início de sessão upstream bem-sucedido.',
                p2: `O <code>path</code> tem de ser indicado numa sintaxe semelhante a regex. Pode corresponder a
                    valores JSON simples ou a um valor dentro de um objeto ou array JSON.`,
                p3: '<code>$.</code> marca o início do objeto JSON',
                p4: '<code>*</code> pode ser usado como wildcard no seu path',
                p5: '<code>$.roles</code> corresponderia a <code>&#123;"roles": "value"&#125;</code>',
                p6: `<code>$.roles.*</code> pode corresponder a um valor dentro de um objeto ou array como<br>
                    <code>&#123;"roles": ["value", "notMyValue"]&#125;</code>`,
            },
            lookup: 'Procurar',
            pathAdminClaim: 'Path da claim de administrador',
            pathMfaClaim: 'Path da claim de MFA',
            rootPemCert: 'Certificado PEM raiz',
            mapMfa: `Se o seu fornecedor emitir uma claim que indique que o utilizador usou pelo menos 2FA no
                início de sessão, pode indicar o path da claim de MFA.`,
            mapUser: `Pode definir um utilizador como administrador do Rauthy com base numa claim upstream do ID token.`,
            valueAdminClaim: 'Valor da claim de administrador',
            valueMfaClaim: 'Valor da claim de MFA',
        },
        delete: {
            areYouSure: 'Tem a certeza de que pretende eliminar este fornecedor?',
            forceDelete: 'Forçar eliminação',
            isInUse1: 'Este fornecedor está a ser usado por utilizadores ativos!',
            isInUse2: `Pode forçar a eliminação, mas os utilizadores sem palavra-passe local nem Passkey
                deixarão de conseguir iniciar sessão.`,
            linkedUsers: 'Utilizadores associados',
        },
    },
    roles: {
        adminNoMod: 'A função <code>rauthy_admin</code> não pode ser alterada.',
        delete1: 'Tem a certeza de que pretende eliminar esta função?',
        name: 'Nome da função',
    },
    scopes: {
        claimsAtRoot: 'Emitir claims na raiz do token',
        claimsAtRootWarning: `Os atributos mapeados deste scope são escritos na raiz do token,
            em vez de ficarem agrupados em 'custom'. A responsabilidade de evitar colisões é sua: se o nome de
            um atributo mapeado colidir com uma claim JWT reservada, a emissão do token falha. As claims
            personalizadas na raiz também podem deixar de funcionar com futuras alterações do protocolo ou de
            funcionalidades. Referência:`,
        defaultNoMod: 'Este é um Scope OIDC predefinido. Estes não podem ser alterados.',
        delete1: 'Tem a certeza de que pretende eliminar este scope?',
        deleteDefault: 'Os scopes OIDC predefinidos não podem ser eliminados.',
        mapping1: 'Pode mapear scopes personalizados para atributos.',
        mapping2: `Todos os atributos adicionais configurados podem ter um valor personalizado para cada
            utilizador. Quando são mapeados para um scope, podem ser incluídos nos Access tokens e / ou ID tokens.`,
        name: 'Nome do scope',
    },
    search: {
        orderBy: 'Ordenar por ...',
        orderChangeToAsc: 'Mudar para ordem crescente',
        orderChangeToDesc: 'Mudar para ordem decrescente',
    },
    sessions: {
        invalidateAll: 'Invalidar todas as sessões',
    },
    tabs: {
        config: 'Configuração',
        delete: 'Eliminar',
    },
    tos: {
        accepted: 'Aceite',
        addNewToS: 'Novos termos',
        addNewToSFromCurrent: 'Novos termos a partir dos selecionados',
        added: 'Adicionados',
        checkStatus: 'Verificar estado do utilizador',
        immutable: `ATENÇÃO: Depois de adicionados, os novos Termos de Serviço não podem ser alterados nem
				eliminados!`,
        noneExist: 'Ainda não foram adicionados Termos de Serviço.',
        optUntil: {
            desc: `Durante o período de transição, a aceitação dos termos atualizados é opcional. Só passa a
					ser obrigatória depois disso.`,
            enable: 'Ativar período de transição',
            label: 'Fim do período de transição',
        },
        tos: 'Termos',
    },
    users: {
        antiLockout: {
            rule: 'Regra anti-bloqueio',
            delete: 'não pode ser eliminado',
            disable: 'não pode ser desativado',
            rauthyAdmin: 'a função rauthy_admin não pode ser removida',
        },
        attributes: 'Atributos',
        deleteUser: 'Tem a certeza de que pretende eliminar este utilizador?',
        descAttr: `Defina atributos personalizados do utilizador. Todos os pares chave / valor serão tratados como
            String / valor JSON.`,
        forceLogout: `Tem a certeza de que pretende invalidar todas as sessões existentes e eliminar todos os
            refresh tokens deste utilizador?`,
        groupAdmin: {
            notManagedTitle: 'Utilizador fora dos seus grupos',
            notManagedDesc: `Este utilizador não pertence a nenhum grupo gerido por si, pelo que os detalhes
                permanecem ocultos. Adicione o utilizador a um ou mais dos seus grupos para o gerir.
                As pertenças a grupos fora dos seus grupos são mantidas.`,
            addToGroups: 'Adicionar aos meus grupos',
        },
        lastLogin: 'Último início de sessão',
        manualInitDesc: `O utilizador também pode ser inicializado aqui. Neste caso, porém, tem de lhe comunicar
            a palavra-passe diretamente.`,
        manualInit: 'Inicialização manual',
        mfa: {
            otp: {
                title: 'Código de utilização única',
                mfaDelete1: 'Pode eliminar os OTPs deste utilizador.',
                mfaDelete2: `Atenção! A eliminação de um OTP <b>não pode ser revertida</b> sem que o utilizador
                    faça um registo completamente novo.`,
                noMfaOtps: 'Este utilizador não tem OTPs registados.',
            },
            webauthn: {
                title: 'Passkeys',
                mfaDelete1: 'Pode eliminar as Passkeys deste utilizador.',
                mfaDelete2: `Atenção! A eliminação de uma Passkey <b>não pode ser revertida</b> sem que o
                    utilizador faça um registo completamente novo.`,
                noMfaKeys: 'Este utilizador não tem Passkeys registadas.',
            },
        },
        pkOnly1: 'Esta é uma conta só com Passkey.',
        pkOnly2:
            'Isto significa que este utilizador inicia sessão sem palavra-passe e não tem qualquer palavra-passe definida.',
        pkOnly3: `Se este utilizador tiver perdido todas as Passkeys, a conta pode ser totalmente reposta e pode ser
            enviado um novo e-mail de redefinição da palavra-passe. Para isso, aceda ao separador 'MFA' e elimine
            todas as Passkeys existentes.`,
        pwdNoInit: 'O utilizador ainda não fez a redefinição inicial da palavra-passe.',
        pwdSendEmailBtn: 'Enviar e-mail de redefinição',
        pwdSendEmailDesc:
            'Pode enviar um novo e-mail de redefinição, caso o utilizador não tenha recebido nenhum.',
        savePassword: 'Guardar palavra-passe',
        selfServiceDesc: 'Pode definir uma nova palavra-passe ou enviar um e-mail de redefinição.',
        sendResetEmail: 'Enviar e-mail de redefinição',
    },
    validation: {
        css: 'Valor CSS inválido',
        origin: 'Origin inválida',
        uri: 'URI inválido',
        redirectUri: {
            comma: 'Um Redirect URI não pode conter uma vírgula (,)',
            controlChar:
                'Um Redirect URI não pode conter carateres de controlo no nome de um parâmetro de consulta',
            fragment: 'Um Redirect URI não pode conter um fragmento (#)',
            reservedKey:
                'Um Redirect URI não pode conter o parâmetro de consulta "{{ KEY }}", porque este é definido pela resposta de autorização',
            reservedKeyLogout:
                'Um Post Logout Redirect URI não pode conter o parâmetro de consulta "{{ KEY }}", porque este é definido ao terminar sessão',
        },
    },
};
