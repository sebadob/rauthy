import type { I18nAdmin } from './interface.ts';

export let I18nAdminZh: I18nAdmin = {
    api_key: {
        delete1: '您确定要删除此API密钥吗？',
        expires: '过期时间',
        generate1: '您可以在此处为此API密钥生成新的密钥。',
        generate2: `生成后您只会看到一次该密钥。
            当生成新密钥时，旧密钥将被永久覆盖。
            此操作无法撤销！`,
        generate3: `API密钥必须在HTTP <code>Authorization</code>
            请求头中按以下格式提供：`,
        generate4: '您可以使用以下<code>curl</code>请求来测试您的新密钥：',
        generate5: '如果您没有安装<code>jq</code>且上述命令失败：',
        keyName: '密钥名称',
        limitedValidity: '有限有效期',
    },
    attrs: {
        delete1: '您确定要删除此属性吗？',
        defaultValue: '默认值',
        desc: '描述',
        makeEditable: '设为可编辑',
        makeEditableP1: '您可以转换此属性并允许用户自己进行编辑。',
        makeEditableP2: `<b>注意：</b>此操作无法撤消！所有来自用户的直接输入都是不受信任的数据，
            绝不能用于任何形式的身份验证或授权！`,
        makeEditableP3: `由于过去可能已接受不受信任的输入，
            属性无法从可编辑状态更改为不可编辑状态，无论这种情况持续多长时间。`,
        name: '属性名称',
        userEditable: '用户可编辑',
    },
    backup: {
        createBackup: '创建备份',
        disabledDesc: '此功能仅在配置Hiqlite作为数据库时存在。',
        lastModified: '最后修改',
        local: '本地',
        name: '名称',
        size: '大小',
    },
    clients: {
        backchannelLogout: '如果此客户端支持{{ OIDC_BCL }}，您可以在此处提供URI。',
        branding: {
            descHsl: `以下值必须以HSL值形式给出。您只需提供基本颜色。
            透明度通道和其他值由主题动态调整。`,
            descFullCss: `以下值必须是完全有效的CSS <code>color</code>值。
                您也可以使用复杂计算或上面定义的CSS变量。`,
            descVariables: `每个标签同时也是CSS变量名。这意味着，
                您可以在自由输入框中引用它们，例如<code>hsla(var(--action) / .7)</code>。`,
        },
        confidential: '机密',
        confidentialNoSecret: '这是一个非机密客户端，因此没有密钥。',
        config: '客户端配置',
        delete1: '您确定要删除此客户端吗？',
        descAuthCode: `可以调整认证码的有效性以提高安全性。认证码
            只能使用一次，默认有效时间为60秒。只要客户端能够足够快地执行登录过程，
            有效性越短越好。`,
        descClientUri: `关于此客户端的URI信息和联系方式将显示在
            登录页面上。`,
        descName: `客户端名称可以更改而不会影响客户端配置。
            它只会在登录页面上显示。`,
        descGroupPrefix: `登录到此客户端可能会受到可选组前缀的限制。
            只有分配给匹配组的用户才被允许登录。`,
        descOrigin: `外部额外允许的来源 - 通常只有当此客户端
            需要直接从浏览器向Rauthy发出请求时才需要，典型的是SPA。`,
        descPKCE: `如果客户端支持，您应该始终激活S256 PKCE以获得额外的安全性。
            如果使用非机密客户端（例如SPA），则至少必须激活其中一个PKCE挑战以确保足够的安全性。`,
        descPKCEEnforce: `如果激活了任何PKCE，Rauthy将在登录期间强制使用，
            并拒绝不包含有效挑战的登录请求。`,
        descUri: `您可以提供任意数量的重定向URI。在每个URI末尾，
            您可以使用<code>*</code>作为通配符。`,
        errConfidentialPKCE: `客户端必须是机密客户端或至少激活一个PKCE
            挑战。`,
        forceMfa: '强制MFA',
        groupLoginPrefix: '登录组前缀',
        name: '客户端名称',
        scim: {
            baseUri: `SCIM基础URI是从中可以正确派生子路由的URI，
                如<code>{base_uri}/Users/{id}</base_uri></code>。`,
            desc: '如果此客户端支持{{ SCIM_LINK }}，您可以在此处激活它。',
            enable: '启用SCIMv2',
            groupSync: '同步组',
            groupSyncPrefix: '组过滤器前缀',
            groupSyncPrefixDesc: `您可以通过可选前缀过滤同步的组。
                例如，如果存在<code>app:admins</code>和<code>app:users</code>组，
                前缀<code>app:</code>将只同步这些组，以及至少链接到这些组之一的用户。`,
            reqDesc: '兼容性需要满足几项要求：',
            reqLi1: '客户端必须正确处理<code>externalId</code>。',
            reqLi2: `必须支持至少<code>/Users</code>端点的<code>filter=externalId eq "*"</code>和
                <code>filter=userName eq "*"</code>。`,
            reqLi3: `如果应同步组，则<code>/Groups</code>还必须支持
                <code>filter=displayName eq "*"</code>。`,
        },
        scopes: {
            allowed: '允许的作用域',
            default: '默认作用域',
            desc: `允许的作用域是客户端在使用<code>authorization_code</code>流程
            重定向到登录时动态请求的作用域。默认作用域将始终添加到令牌中，
            以解决使用<code>password</code>等流程时的一些问题。`,
        },
        secret: {
            doCache: '缓存客户端密钥',
            cacheDuration: '缓存时长（小时）',
            generate: '生成新密钥',
            rotateDesc1: `为了实现平滑更新和密钥轮换，您可以选择在内存中缓存当前密钥一段时间。
                您可以输入1到24小时之间的值。`,
            rotateDesc2: '注意：如果发生泄露，您不应缓存当前密钥！',
        },
        tokenLifetime: {
            p1: `令牌生命周期适用于访问令牌和ID令牌，以秒为单位。`,
            p2: `如果客户端支持EdDSA / ed25519算法，这应该是首选。
                RSA算法仅出于兼容性考虑而存在。`,
            p3: `刷新令牌的算法无法更改，因为这些令牌仅供Rauthy使用。`,
        },
    },
    common: {
        account: '账户',
        addNew: '新增',
        back: '返回',
        caution: '注意',
        contact: '联系人',
        copiedToClip: '值已复制到剪贴板',
        details: '详情',
        edit: '编辑',
        enabled: '已启用',
        filter: '筛选',
        from: '从',
        information: '信息',
        language: '语言',
        loading: '加载中',
        name: '名称',
        nameExistsAlready: '名称已存在',
        note: '备注',
        noEntries: '无条目',
        preview: '预览',
        reset: '重置',
        searchOptions: '搜索选项',
        until: '直到',
    },
    docs: {
        book: '有关Rauthy本身的通用文档，请查看',
        encryption: '加密',
        encKeys: {
            header: '加密密钥',
            keyActive: '活动密钥',
            keysAvailable: '可用密钥',
            migrate: '迁移',
            migrateToKey: '将所有现有加密值迁移到以下密钥',
            p1: `这些密钥用于独立于底层使用的任何数据存储技术的额外静态加密。
            它们是静态配置的，但可以在此页面手动轮换和迁移。`,
            p2: `活动密钥在Rauthy配置文件/环境变量中静态设置。无法在此处动态更改。
            所有新的JWK加密都将始终使用当前活动密钥。`,
            p3: `如果您迁移所有现有密钥，对于大数据集可能需要几秒钟才能完成。`,
            pNotPossible: '要能够迁移，至少需要2个加密密钥。',
        },
        hashing: {
            calculate: '计算',

            currValuesHead: '当前值',
            currValues1: '来自后端的当前值如下：',
            currValuesNote: `注意：后端的登录时间仅在Rauthy启动后至少成功登录5次后才能提供良好指导。
            刚重启后的基准值始终为2000毫秒，并会随着时间推移和每次成功登录进行调整。`,
            currValuesThreadsAccess: 'Rauthy可访问的线程数(p_cost)',

            loginTimeHead: '关于登录时间',
            loginTime1: `一般来说，用户希望一切尽可能快速。但在安全登录时，
            500-1000毫秒之间的时间应该不是问题。登录时间不能太短，因为它显然会降低哈希强度。`,
            loginTime2: `为了默认提供尽可能多的安全性，此工具不允许您将登录时间设置低于500毫秒。`,

            mCost1: `<code>m_cost</code>定义了哈希使用的<b>内存（以kB为单位）</b>。
            值越高越好，当然。但您需要记住服务器资源。<br>
            例如，当同时哈希4个密码时，后端在哈希过程中需要<code>4 x m_cost</code>。
            这些资源必须可用。`,
            mCost2: `调整<code>m_cost</code>相当简单。定义Rauthy应使用的最大内存量，
            除以允许的最大并行登录数(<code>MAX_HASH_THREADS</code>)，然后减去少量静态内存量。
            应考虑多少静态内存取决于所使用的数据库和用户总数，但通常在32-96MB范围内。`,
            mCost3: '允许的最小<code>m_cost</code>为<code>32768</code>。',

            pCost1: `<code>p_cost</code>定义了哈希的<b>并行度</b>。这个值通常最多达到约8，
            这也是Rauthy的默认值。`,
            pCost2: `一般规则是：<br>
            将<code>p_cost</code>设置为可用核心数的两倍。<br>
            例如，如果您有4个核心可用，则将<code>p_cost</code>设置为<code>8</code>。<br>
            然而，此值必须考虑配置的允许并行登录数(<code>MAX_HASH_THREADS</code>)
            并相应减少。`,

            tCost1: `<code>t_cost</code>定义了哈希所需的<b>时间</b>。实际上，
            这是唯一需要调整的值，因为<code>m_cost</code>和<code>p_cost</code>
            基本上是由环境决定的。`,
            tCost2: `调整很简单：相应设置<code>m_cost</code>和<code>p_cost</code>，
            然后增加<code>t_cost</code>直到达到您的哈希时间目标。`,

            utilityHead: '参数计算工具',
            utility1: `您可以使用此工具为您的部署近似最佳值。请记住，
            这应该在Rauthy最终位置执行，并具有所有最终可用资源。您应在负载下执行此工具以避免过度调整。`,
            utility2: `<code>m_cost</code>是可选的，如果为空则会选择安全的最小值<code>32768</code>。
            <code>p_cost</code>也是可选的，如果为空Rauthy将利用其能看到的所有线程。`,

            time: '时间',
            targetTime: '目标时间',
            tune: '重要：这些值需要在最终架构上进行调整！',
            pDetials: `如果您想要详细了解Argon2ID，在线有很多资源。本指南仅简要介绍这些值。
            需要配置三个值：`,
            pTune: `它们根据系统能力而变化。系统越强大，这些值就越安全。`,
            pUtility: `此工具帮助您找到适合平台的最佳Argon2ID设置。
            Argon2ID目前是最安全的密码哈希算法。要充分发挥其潜力，
            必须针对每个部署进行调整。`,
        },
        openapi: '如果您想集成外部应用程序并使用Rauthy的API，请查看',
        openapiNote: `根据后端配置，Swagger UI可能在此时不公开暴露。
            但它默认可通过内部指标HTTP服务器获得，以免暴露任何信息。`,
        source: '源代码可以在这里找到',
    },
    editor: {
        bold: '粗体',
        code: '代码',
        heading1: '标题1',
        heading2: '标题2',
        heading3: '标题3',
        italic: '斜体',
        link: '链接',
        listBullet: '列表',
        listTasks: '任务',
        listNumbered: '编号列表',
        paragraph: '段落',
        quote: '引用',
        removeFmt: '清除格式',
        strikeThrough: '删除线',
        textArea: '编辑文本',
    },
    email: {
        cancelJob: '取消任务',
        filterType: ['无', '在组中', '不在组中', '拥有角色', '没有角色'],
        immediate: '立即',
        jobs: '电子邮件任务',
        scheduled: '预定发送',
        sendAllUsers: '此邮件将发送给所有用户。',
        sendAllUsersFiltered: '此邮件将发送给按以下条件筛选的用户：',
        sendMail: '发送邮件',
        subject: '主题',
        userFilter: '用户筛选',
    },
    error: {
        needsAdminRole: `您未分配到<b>rauthy_admin</b>角色。<br/>
            您无权访问管理面板。`,
        noAdmin: `Rauthy管理员账户必须<b>启用MFA。</b><br>
            请导航到您的<b>账户</b>并激活MFA。<br>
            之后，您需要注销并重新登录。`,
    },
    events: {
        eventLevel: '事件级别',
        eventType: '事件类型',
    },
    groups: {
        delete1: '您确定要删除此组吗？',
        name: '组名',
    },
    jwks: {
        alg: '算法',
        p1: '这些是用于令牌签名的Json Web密钥(JWK)。',
        p2: `JWK默认每月1日轮换。对于所有新创建的令牌，
        只会使用给定算法的最新可用密钥进行签名。旧密钥将保留一段时间以确保
        当前有效的令牌仍能正确验证。一段时间后，它们将自动清理。`,
        p3: `密钥也可以手动轮换。根据运行此Rauthy实例的硬件，
        可能需要几秒钟。`,
        type: '类型',
        rotateKeys: '轮换密钥',
    },
    nav: {
        apiKeys: 'API密钥',
        attributes: '属性',
        blacklist: '黑名单',
        clients: '客户端',
        config: '配置',
        docs: '文档',
        events: '事件',
        groups: '组',
        providers: '提供商',
        roles: '角色',
        scopes: '作用域',
        sessions: '会话',
        users: '用户',
    },
    options: {
        expires: '过期',
        lastSeen: '最后查看',
        state: '状态',
    },
    pam: {
        addGroup: '新建PAM组',
        addHost: '新建PAM主机',
        addUser: '新建PAM用户',
        deleteHost: '您真的要删除此主机吗？',
        groupDescGeneric: `通用组是通常在/etc/group中找到的条目的对应物。
            用户可以分配到这些组，并通过NSS查找返回给系统。`,
        groupDescHost: `主机组用于对主机进行分组。对组内主机的NSS查找会返回组内的所有其他主机作为结果。
            用户可以通过将他们分配到主机组来访问主机。`,
        groupDescLocal: `本地组的行为几乎与通用组相同，不同之处在于它们在Rauthy数据库中有ID，
            但NSS代理在相应主机上会将其转换为/etc/group中的ID。
            这样，Rauthy用户就可以分配到已经存在的本地组。`,
        groupDescUser: `用户组由系统自动管理并与同用户名的用户紧密耦合。`,
        groupDescWheel: `此组是特殊的。它是不可变的，并根据用户的组配置动态分配给用户。`,
        groupName: '组名',
        groups: '组',
        groupType: '组类型',
        hostAliases: '主机别名',
        hostLocalPwdOnly: '本地密码登录',
        hostLocalPwdOnlyInfo: `当设置本地密码登录时，它会覆盖本地登录的强制MFA。
            同时，在登录期间（本地）永远不会请求通行密钥，即使用户受MFA保护。
            此选项应仅在确实必要时设置，例如当受MFA保护的用户需要进行本地登录
            而不使用硬件通行密钥时。`,
        ipAddresses: 'IP地址',
        member: '成员',
        nameExistsAlready: '名称已被占用',
        notes: '备注',
        secretShow: '显示密钥',
        secretRotate: '轮换密钥',
        userEmail: '关联用户邮箱',
        username: '用户名',
        usernameNewDesc: `应谨慎选择用户名。出于安全原因，创建后很难轻易更改。`,
    },
    passwordPolicy: {
        configDesc: '新密码策略。',
        resetSet0: '值0表示停用要求。',
        validForDays: '有效天数',
        validityNew: '新密码的有效性。',
    },
    providers: {
        config: {
            allowInsecureTls: '允许不安全的TLS',
            autoLink: '自动链接用户',
            autoLinkDesc1: `如果激活自动链接用户，通过此提供商登录将自动将可能存在的
                未链接用户与此提供商链接。`,
            autoLinkDesc2: `注意：如果提供商不对用户完全验证邮箱地址，
                从而使用户可能添加外来地址，则此选项非常危险并可能导致帐户接管！
                在这种情况下绝不能使用！`,
            clientName: '客户端名称',
            custRootCa: '自定义根CA PEM',
            descAuthMethod: `在<code>/token</code>端点使用的身份验证方法。<br>
                大多数提供商应能与<code>basic</code>配合工作，有些只能使用<code>post</code>。
                在极少数情况下，您需要两者，但这可能导致与其他提供商出现错误。`,
            descClientId: '提供商提供的客户端ID。',
            descClientName: '应在Rauthy登录页面显示的客户端名称。',
            descClientSecret: `提供商提供的客户端密钥。
                至少需要客户端密钥或PKCE。`,
            descScope: `客户端在重定向到登录时应使用的作用域。
                提供以空格分隔的值。`,
            errNoAuthMethod: '您提供了客户端密钥，但没有激活客户端身份验证方法',
            errConfidential: '必须至少是机密客户端或使用PKCE',
            jsonPath: {
                p1: '成功上游登录后，ID令牌中的值可以自动映射。',
                p2: `<code>路径</code>需要以类似正则表达式的语法给出。它可以解析为
                    单个JSON值或解析为JSON对象或数组中的值。`,
                p3: '<code>$.</code>标记JSON对象的开始',
                p4: '<code>*</code>可在路径中用作通配符',
                p5: '<code>$.roles</code>将定位<code>&#123;"roles": "value"&#125;</code>',
                p6: `<code>$.roles.*</code>可以定位对象或数组中的值，如<br>
                    <code>&#123;"roles": ["value", "notMyValue"]&#125;</code>`,
            },
            lookup: '查找',
            pathAdminClaim: '管理员声明路径',
            pathMfaClaim: 'MFA声明路径',
            rootPemCert: '根PEM证书',
            mapMfa: `如果您的提供商在登录期间发出表明用户至少使用了2FA的声明，
                您可以指定MFA声明路径。`,
            mapUser: `您可以根据上游ID声明将用户映射为Rauthy管理员。`,
            valueAdminClaim: '管理员声明值',
            valueMfaClaim: 'MFA声明值',
        },
        delete: {
            areYouSure: '您确定要删除此提供商吗？',
            forceDelete: '强制删除',
            isInUse1: '此提供商正在被活跃用户使用！',
            isInUse2: `您可以强制删除它，但没有本地密码或通行密钥的用户
                将无法再登录。`,
            linkedUsers: '链接用户',
        },
    },
    roles: {
        adminNoMod: '<code>rauthy_admin</code>角色是不可变的。',
        delete1: '您确定要删除此角色吗？',
        name: '角色名称',
    },
    scopes: {
        defaultNoMod: '这是默认OIDC作用域。这些是不可变的。',
        delete1: '您确定要删除此作用域吗？',
        deleteDefault: '默认OIDC作用域无法删除。',
        mapping1: '您可以将自定义作用域映射到属性。',
        mapping2: `所有已配置的附加属性都可以为每个用户提供自定义值。
            当它们映射到作用域时，可以包含在访问令牌和/或ID令牌中。`,
        name: '作用域名称',
    },
    search: {
        orderBy: '排序方式...',
        orderChangeToAsc: '更改为升序',
        orderChangeToDesc: '更改为降序',
    },
    sessions: {
        invalidateAll: '使所有会话无效',
    },
    tabs: {
        config: '配置',
        delete: '删除',
    },
    tos: {
        accepted: '已接受',
        addNewToS: '添加新服务条款',
        addNewToSFromCurrent: 'New ToS from template',
        added: '已添加',
        checkStatus: '检查用户状态',
        immutable: `注意：添加新服务条款后，它们是不可变的且无法删除！`,
        noneExist: '尚未添加任何服务条款。',
        optUntil: {
            desc: `在过渡期内，接受更新的服务条款是可选的。此后才变为强制性。`,
            enable: '启用过渡时间',
            label: '过渡时间结束',
        },
        tos: '服务条款',
    },
    users: {
        antiLockout: {
            rule: '防锁定规则',
            delete: '无法删除',
            disable: '无法禁用',
            rauthyAdmin: '无法移除rauthy_admin规则',
        },
        attributes: '属性',
        deleteUser: '您确定要删除此用户吗？',
        descAttr: `设置自定义用户属性。所有键/值对将作为字符串/JSON值处理。`,
        forceLogout: `您确定要使此用户的所有现有会话无效并删除所有刷新令牌吗？`,
        lastLogin: '最后登录',
        manualInitDesc: `也可以在此处初始化用户。在这种情况下，您需要直接传达密码。`,
        manualInit: '手动初始化',
        mfaDelete1: '您可以删除此用户的通行密钥。',
        mfaDelete2: `注意！通行密钥的删除<b>无法撤销</b>，除非用户
            进行全新的注册。`,
        noMfaKeys: '此用户没有注册的通行密钥。',
        pkOnly1: '这是一个仅通行密钥账户。',
        pkOnly2: '这意味着该用户使用无密码登录，根本没有设置密码。',
        pkOnly3: `如果此用户丢失了所有通行密钥，可以完全重置账户并发送新的密码重置邮件。
            为此，请导航到'MFA'选项卡并删除所有现有的通行密钥。`,
        pwdNoInit: '用户尚未执行初始密码重置。',
        pwdSendEmailBtn: '发送重置邮件',
        pwdSendEmailDesc: '如果用户未收到邮件，您可以发送新的重置邮件。',
        savePassword: '保存密码',
        selfServiceDesc: '您可以设置新密码或发送重置邮件。',
        sendResetEmail: '发送重置邮件',
    },
    validation: {
        css: '无效的CSS值',
        origin: '无效的来源',
        uri: '无效的URI',
    },
};
