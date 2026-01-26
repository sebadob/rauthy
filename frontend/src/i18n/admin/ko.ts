import type { I18nAdmin } from './interface.ts';

export let I18nAdminKo: I18nAdmin = {
    api_key: {
        delete1: '이 API 키를 삭제하시겠습니까?',
        expires: '만료일',
        generate1: '이 API 키에 대한 새 Secret을 생성할 수 있습니다.',
        generate2: `이 Secret은 생성 이후 단 한 번만 볼 수 있습니다. 새 Secret이 생성되면 이전 Secret은
            사용할 수 없으며, 이 작업은 되돌릴 수 없습니다!`,
        generate3: `API 키는 HTTP <code>Authorization</code> 헤더에 다음과 같은 형식으로 제공해야 합니다.`,
        generate4: '아래의 <code>curl</code> 요청으로 테스트할 수 있습니다.',
        generate5:
            '<code>jq</code> 가 설치되어 있지 않거나 위의 코드가 실패한 경우는 아래 코드를 사용하세요.',
        keyName: 'API 키 이름',
        limitedValidity: '만료일',
    },
    attrs: {
        delete1: '이 속성을 삭제하시겠습니까?',
        defaultValue: 'Default Value',
        desc: '설명',
        makeEditable: 'Make Editable',
        makeEditableP1: 'You can convert this attribute and make it editable by users themselves.',
        makeEditableP2: `<b>CAUTION:</b> This can never be changed back! All inputs from a user directly are always
            untrusted data and MUST NEVER be used for any form of authentication or authorization!`,
        makeEditableP3: `An attribute cannot be changed from editable to non-editable, because it allowed untrusted
            inputs in the past, no matter for how long this was the case.`,
        name: '속성 이름',
        userEditable: 'User Editable',
    },
    backup: {
        createBackup: 'Create Backup',
        disabledDesc: 'This functionality only exists, if Hiqlite is configured as the database.',
        lastModified: 'Last Modified',
        local: 'Local',
        name: 'Name',
        size: 'Size',
    },
    clients: {
        backchannelLogout: 'If this client supports {{ OIDC_BCL }}, you can provide the URI here.',
        branding: {
            descHsl: `HSL 값으로 입력해야 합니다. 기본 색상만 제공하면 알파 채널 및 기타 값은
                테마에 의해 동적으로 설정됩니다.`,
            descFullCss: `다음 값은 완전히 유효한 <code>CSS 색상</code> 값이어야 합니다. 복잡한 계산이나 위에
                정의된 <code>CSS 변수</code> 를 사용할 수도 있습니다.`,
            descVariables: `색상의 각 레이블들은 동시에 CSS 변수의 이름입니다. 즉, 입력 칸에서 이를 참조할 수
                있습니다. (예: <code>hsla(var(--action) / .7)</code>)`,
        },
        confidential: '기밀',
        confidentialNoSecret: '이 클라이언트는 기밀이 아닌 클라이언트이므로 Secret이 없습니다.',
        config: '클라이언트 설정',
        delete1: '이 클라이언트를 삭제하시겠습니까?',
        descAuthCode: `보안을 강화하기 위해 인증 코드의 유효 기간을 조정할 수 있습니다. 인증 코드는
            한 번만 사용할 수 있으며 기본적으로 60초 동안 유효합니다. 클라이언트가 로그인 절차를
            충분히 빠르게 수행할 수 있다면 유효 기간이 짧을수록 좋습니다.`,
        descClientUri: `로그인 페이지에 표시할 클라이언트 URI 및 연락처에 대한 정보입니다.`,
        descName: `클라이언트 이름은 클라이언트 구성에 영향을 주지 않고 변경할 수 있으며,
            로그인 페이지에서만 표시됩니다.`,
        descGroupPrefix: `The login to this client may be restricted by an optional group prefix.
            Only users, that are assigned to a matching group, will be allowed to log in.`,
        descOrigin: `추가로 허용되는 외부 오리진을 설정합니다. 일반적으로 클라이언트가 브라우저에서 직접
            Rauthy에 요청해야 하는 경우(일반적으로 SPA)에만 필요합니다.`,
        descPKCE: `클라이언트가 이를 지원하는 경우, 추가 보안을 위해 항상 S256 PKCE를 활성화해야 합니다.
            기밀이 아닌 클라이언트(예: SPA)를 사용하는 경우에는 충분한 보안을 위해 최소한 PKCE 챌린지 중
            하나를 활성화해야 합니다.`,
        descPKCEEnforce: `PKCE가 활성화된 경우, 반드시 사용되어야 합니다. 유효한 챌린지가 포함되지
            않은 로그인 요청은 거부됩니다.`,
        descUri: `원하는 만큼 리디렉션 URI를 제공할 수 있습니다. 각각의 끝에 <code>*</code> 를
            와일드카드로 사용할 수 있습니다.`,
        errConfidentialPKCE: `클라이언트는 기밀 또는 PKCE 챌린지 중 하나 이상 활성화되어야 합니다.`,
        forceMfa: '강제 MFA',
        groupLoginPrefix: 'Login Group Prefix',
        name: '클라이언트 이름',
        scim: {
            baseUri: `The SCIM base URI is the one from which the sub routes like 
                <code>{base_uri}/Users/{id}</base_uri></code> can be derived correctly.`,
            desc: 'If this client supports {{ SCIM_LINK }}, you can activate it here.',
            enable: 'Enable SCIMv2',
            groupSync: 'Synchronize Groups',
            groupSyncPrefix: 'Groups Filter Prefix',
            groupSyncPrefixDesc: `You can filter the groups for the synchronization by an optional prefix.
                For instance, if the groups <code>app:admins</code> and <code>app:users</code> exist, the prefix
                 <code>app:</code> would only sync these groups, as well as only those users that are linked to at least
                 one of these groups.`,
            reqDesc: 'A few things are required for compatibility:',
            reqLi1: 'The client must handle <code>externalId</code> correctly.',
            reqLi2: `At least <code>/Users</code> endpoints with <code>filter=externalId eq "*"</code> and
                <code>filter=userName eq "*"</code> must be supported.`,
            reqLi3: `If groups should be synchronized, <code>/Groups</code> must also support 
                <code>filter=displayName eq "*"</code>.`,
        },
        scopes: {
            allowed: '허용된 범위',
            default: '기본 범위',
            desc: `허용된 범위는 <code>authorization_code</code> 플로우를 사용할 때 로그인 리디렉션 중에
                클라이언트가 동적으로 요청할 수 있는 범위입니다. 예를 들어 <code>password</code> 를
                사용할 때, 일부 문제를 해결하기 위해 기본 범위가 항상 토큰에 추가됩니다.`,
        },
        secret: {
            doCache: 'Cache Client Secret',
            cacheDuration: 'Cache Duration (hours)',
            generate: '새 Secret 생성',
            rotateDesc1: `To make graceful updates and secret rotations possible, you have the ability to keep the
                current secret in an in-memory cache for some time. You can enter a value between 1 and 24 hours.`,
            rotateDesc2: 'Caution: You should not cache the current secret if you had a leak!',
        },
        tokenLifetime: {
            p1: `토큰 수명은 액세스 토큰과 ID 토큰에 적용되며, 초 단위입니다.`,
            p2: `클라이언트가 EdDSA 또는 ed25519 알고리즘을 지원하면, 항상 선호되는 옵션이어야 합니다.
                RSA 알고리즘은 호환성을 위해서만 존재합니다.`,
            p3: `Refresh 토큰의 알고리즘은 Rauthy에서만 사용되므로, 변경할 수 없습니다.`,
        },
    },
    common: {
        account: '계정',
        addNew: '생성',
        back: '뒤로',
        caution: 'CAUTION',
        contact: '연락처',
        copiedToClip: '클립보드로 복사되었습니다.',
        details: '자세히',
        edit: '편집',
        enabled: '활성화',
        filter: '필터',
        from: '부터',
        information: '정보',
        language: '언어',
        loading: '로딩중...',
        name: '이름',
        nameExistsAlready: '이미 존재하는 이름입니다.',
        note: '참고',
        noEntries: '비어 있음',
        preview: 'Preview',
        reset: '초기화',
        searchOptions: '검색 옵션',
        until: '까지',
    },
    docs: {
        book: 'Rauthy에 대한 문서:',
        encryption: '암호화',
        encKeys: {
            header: '암호화 키',
            keyActive: '활성화된 키',
            keysAvailable: '가능한 키',
            migrate: '마이그레이션',
            migrateToKey: '기존의 모든 암호화된 값을 다음 키로 마이그레이션',
            p1: `이 키는 내부에서 사용되는 데이터베이스와는 별개로, 세션, 쿠키 등 추가 암호화에 사용됩니다.
                이 키는 정적으로 구성되지만 이 페이지에서 수동으로 변경 및 마이그레이션할 수 있습니다.`,
            p2: `활성 키는 Rauthy 설정 파일 또는 환경 변수에 정적으로 설정됩니다. 여기에서 동적으로 생성할 수
                없습니다. 모든 새 JWK 암호화는 항상 현재 활성 키를 사용합니다.`,
            p3: `기존 Secret을 모두 마이그레이션하는 경우, 데이터 세트가 큰 경우에는 완료하는 데 수 초가 걸릴 수
                있습니다.`,
            pNotPossible: '마이그레이션하려면 최소 2개의 암호화 키를 사용할 수 있어야 합니다.',
        },
        hashing: {
            calculate: '계산',

            currValuesHead: '현재 값',
            currValues1: '백엔드의 현재 값은 다음과 같습니다.',
            currValuesNote: `참고: 백엔드의 로그인 시간은 Rauthy가 시작된 후 최소 5회 이상 로그인에 성공한 후에만
                측정된 가이드라인을 제공합니다. 기본값은 항상 새로 다시 시작한 후 2000 ms이며, 로그인에 성공할
                때마다 재계산됩니다.`,
            currValuesThreadsAccess: '사용 가능한 스레드 개수 (p_cost)',

            loginTimeHead: '로그인 시간에 대한 견해',
            loginTime1: `일반적으로 사용자는 모든 것이 가능한 한 빨리 처리되기를 원합니다. 하지만 안전한 로그인을
                위해 500~1000 ms 정도는 문제가 되지 않습니다. 로그인 시간이 너무 짧으면 해시의 강도가 낮아질 수
                있으므로 너무 짧아서는 안 됩니다.`,
            loginTime2: `이 유틸리티는 최대한의 보안을 제공하기 위해 로그인 시간을 500ms 미만으로 설정할 수
                없습니다.`,
            mCost1: `<code>m_cost</code> 는 해싱에 사용되는 <b>메모리 (in kB)</b> 양입니다.
                물론 값이 높을수록 좋지만, 시스템의 메모리를 고려하여야 합니다.<br>
                예를 들어 4개의 비밀번호를 동시에 해시하는 경우, 해시하는 동안 <code>4 x m_cost</code> 가
                필요합니다. 이러한 리소스를 사용할 수 있어야 합니다.`,
            mCost2: `<code>m_cost</code> 를 조정하는 것은 매우 쉽습니다. Rauthy가 사용해야 하는 최대 메모리 양을
                정의하고, 이를 최대 허용 병렬 로그인 수(<code>MAX_HASH_THREADS</code>)로 나눈 다음 소량의 정적
                메모리를 빼면 됩니다. 얼마나 많은 정적 메모리를 고려해야 하는지는 사용되는 데이터베이스와 총
                사용자 수에 따라 다르지만 일반적으로 <code>32~96MB</code> 범위가 적당합니다.`,
            mCost3: '허용되는 <code>m_cost</code> 의 최소값은 <code>32768</code> 입니다.',
            pCost1: `<code>p_cost</code> 는 해싱을 위한 <b>병렬 처리 수</b>를 정의합니다. 이 값은 대부분 Rauthy의
                기본값인 8 이내로 사용됩니다.`,
            pCost2: `일반적인으로 <code>p_cost</code> 는 사용가능한 코어의 2배로 설정됩니다.
                예를 들어, 4코어 시스템을 사용중이라면, <code>p_cost</code> 가 <code>8</code> 로 설정됩니다.
                단, 이 값은 구성된 허용된 병렬 로그인 (<code>MAX_HASH_THREADS</code>) 을 고려하여 적절히 줄여야
                합니다.`,
            tCost1: `<code>t_cost</code> 는 해싱을 위한 <b>시간</b> 을 정의합니다. <code>m_cost</code> 와
                <code>p_cost</code> 는 기본적으로 환경에 의해 주어지기 때문에, 이 값은 실제로 튜닝이 필요한
                유일한 값입니다.`,
            tCost2: `튜닝은 간단합니다. <code>m_cost</code> 와 <code>p_cost</code> 를 적절히 설정한 다음
                <code>t_cost</code> 를 로그인 목표 시간에 도달할 때까지 증가시키면 됩니다.`,
            utilityHead: '파라메터 계산 유틸리티',
            utility1: `이 도구를 사용하여 시스템에 적합한 값을 대략적으로 구할 수 있습니다. 이 도구는 시스템의
                실제 성능을 테스트하므로 모든 다른 서비스를 사용하는 상태에서 Rauthy를 실행해야 한다는 점에
                유의하세요. 해당 시스템의 일반적인 사용 환경에서 이 유틸리티를 실행해야 오버 튜닝을 방지할 수
                있습니다.`,
            utility2: `<code>m_cost</code> 은 옵션이며, 비어 있으면 안전한 최소값인 <code>32768</code> 이 사용됩니다.
                <code>p_cost</code> 은 옵션이며, 비어 있으면 시스템의 <code>모든 스레드</code> 를 사용합니다.`,

            time: '시간',
            targetTime: '목표 시간',
            tune: '중요: 이러한 값은 최종 아키텍처에서 설정되어야 합니다!',
            pDetials: `Argon2ID에 대한 자세한 소개를 원하신다면 온라인에 많은 자료가 있습니다. 이 가이드에서는 값에
                대한 아주 간략한 개요만 제공합니다. 다음 세 가지를 구성해야 합니다.`,
            pTune: `시스템의 성능에 따라 다릅니다. 성능이 좋을수록 더 안전한 값을 설정할 수 있습니다.`,
            pUtility: `이 유틸리티는 플랫폼에 가장 적합한 Argon2ID 설정을 찾을 수 있도록 도와줍니다. Argon2ID는 현재
                사용 가능한 가장 안전한 비밀번호 해싱 알고리즘입니다. 이 알고리즘을 최대한 활용하려면 각 시스템에
                맞게 조정해야 합니다.`,
        },
        openapi: '외부 애플리케이션을 통합하기 위한 Rauthy의 API 사용 방법:',
        openapiNote: `백엔드 설정에 따라 Swagger UI가 외부에 노출되지 않을 수도 있습니다.
            하지만 기본적으로 내부에서는 Swagger UI를 통해 정보를 확인할 수 있습니다.`,
        source: 'Rauthy의 소스코드',
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
        needsAdminRole: `<b>rauthy_admin</b> 역할이 부여되지 않았습니다.<br/>
            관리자 패널에 접근할 수 없습니다.`,
        noAdmin: `Rauthy 관리자 계정에는 <b>MFA가 활성화</b>되어 있어야 합니다.<br>
            <b>계정</b> 으로 이동 후 MFA를 활성화하세요.<br>
            활성화 후에는 로그아웃한 뒤 다시 로그인해야 합니다.`,
    },
    events: {
        eventLevel: '이벤트 레벨',
        eventType: '이벤트 타입',
    },
    groups: {
        delete1: '이 그룹을 삭제하시겠습니까?',
        name: '그룹 이름',
    },
    jwks: {
        alg: '알고리즘',
        p1: '토큰 서명에 사용되는 Json Web Keys(JWKs)입니다.',
        p2: `JWKs는 기본적으로 매월 1일마다 회전됩니다. 새로 생성된 모든 토큰의 경우, 주어진 알고리즘에 대해
            사용 가능한 최신 키만 서명에 사용됩니다. 현재 유효한 토큰에 대해 유효성을 검사하기 위해 이전 키는
            잠시 보관되며, 이후 자동으로 정리됩니다.`,
        p3: `키를 수동으로 회전할 수도 있습니다. Rauthy 인스턴스가 실행 중인 하드웨어에 따라 수 초가 걸릴 수
            있습니다.`,
        type: '타입',
        rotateKeys: '키 회전',
    },
    nav: {
        apiKeys: 'API 키',
        attributes: '속성',
        blacklist: '블랙리스트',
        clients: '클라이언트',
        config: '설정',
        docs: '문서',
        events: '이벤트',
        groups: '그룹',
        providers: '공급자',
        roles: '역할',
        scopes: '범위',
        sessions: '세션',
        users: '사용자',
    },
    options: {
        expires: '만료일',
        lastSeen: '마지막 사용시간',
        state: '상태',
    },
    pam: {
        addGroup: 'New PAM Group',
        addHost: 'New PAM Host',
        addUser: 'New PAM User',
        deleteHost: 'Do you really want to delete this host?',
        groupDescGeneric: `Generic groups are the counterpart to entries that are usually found in /etc/group. Users 
            can be assigned to these and they are returned to the system by NSS Lookups.`,
        groupDescHost: `Host groups are used to group hosts. NSS lookups of a host within the group return all other 
            hosts within it as a result. Users can access hosts by assigning them to a host group.`,
        groupDescLocal: `Local groups behave almost identically to Generic groups, with the difference that they have 
            an ID in the Rauthy database, but the NSS proxy on the respective host will convert it to an ID from 
            /etc/group. In this way, Rauthy users can be assigned to groups that already exist locally.`,
        groupDescUser: `User groups are managed automatically and tightly coupled with the user with the same
            username.`,
        groupDescWheel: `This group is special. It is immutable and is assigned to users dynamically depending on their
            group configuration.`,
        groupName: 'Groupname',
        groups: 'Groups',
        groupType: 'Group Type',
        hostAliases: 'Host Aliases',
        hostLocalPwdOnly: 'Local Password Login',
        hostLocalPwdOnlyInfo: `When Local Password Login is set, it overwrites Force MFA for local logins. At the same
            time, passkeys will never be requested (locally) during logins, even if a user is MFA-secured. This option
            should only be set if really necessary, for instance if MFA-secured users should be able to do local logins
            while not using hardware passkeys.`,
        ipAddresses: 'IP Addresses',
        member: 'Member',
        nameExistsAlready: 'Name is already taken',
        notes: 'Notes',
        secretShow: 'Show Secret',
        secretRotate: 'Rotate Secret',
        userEmail: 'Linked User E-Mail',
        username: 'Username',
        usernameNewDesc: `The Username should be chosen carefully. Once created, it cannot be changed easily afterwards
            for security reasons.`,
    },
    passwordPolicy: {
        configDesc: '새 비밀번호에 대한 정책입니다.',
        resetSet0: '0으로 설정하면 비활성화됩니다.',
        validForDays: '기간 제한',
        validityNew: '이전 비밀번호 사용 제한 정책입니다.',
    },
    providers: {
        config: {
            allowInsecureTls: '안전하지 않은 TLS 허용',
            autoLink: 'Auto-Link User',
            autoLinkDesc1: `If Auto-Link User is activated, the login via this provider will automatically link a
                possibly existing, non-linked user to this provider.`,
            autoLinkDesc2: `CAUTION: This option can be very dangerous and lead to account takeover if the provider
                does not fully validate E-Mail addresses for users and therefore makes it possible to add a foreign
                address for a user! MUST NEVER be used in such a case!`,
            clientName: '클라이언트 이름',
            custRootCa: '사용자 지정 Root CA PEM 사용',
            descAuthMethod: `<code>/token</code> 엔드포인트에서 사용할 인증 방법입니다.<br>
                대부분의 인증 공급자는 <code>basic</code> 으로 작업해야 하며, 일부 인증 공급자는
                <code>post</code> 만 사용해야 합니다. 드문 경우지만 두 가지를 모두 사용해야 하는 경우도
                있습니다. 인증 공급자의 가이드대로 설정하시면 됩니다.`,
            descClientId: '인증 공급자가 제공한 클라이언트 ID',
            descClientName: '로그인 페이지에 표시될 클라이언트 이름',
            descClientSecret: `인증 공급자가 제공한 클라이언트 Secret입니다. 최소한 클라이언트
                Secret 또는 PKCE가 필요합니다.`,
            descScope: `로그인 리디렉션에 사용할 범위입니다. 공백으로 구분하여 입력합니다.`,
            errNoAuthMethod: `클라이언트 Secret이 입력되어 있지만, 클라이언트 인증 방법이 활성화되어
                있지 않습니다.`,
            errConfidential: '최소한 기밀 클라이언트이거나 PKCE를 사용해야 합니다.',
            jsonPath: {
                p1: '업스트림 로그인 성공 후, ID 토큰의 값을 자동으로 매핑할 수 있습니다.',
                p2: `<code>경로</code>는 정규식과 같은 구문으로 지정해야 합니다. 단일 JSON 값이나
                    JSON 객체 또는 배열의 값으로 처리할 수 있습니다.`,
                p3: '<code>$.</code> 는 JSON 객체의 시작을 표시합니다.',
                p4: '<code>*</code> 는 경로에서 와일드카드로 사용됩니다.',
                p5: `<code>$.roles</code> 는 <code>&#123;\"roles\": \"value\"&#125;</code> 을 목표로
                    할 수 있습니다.`,
                p6: `<code>$.roles.*</code> 는 <code>&#123;"roles": ["value", "notMyValue"]&#125;</code>
                    과 같이 객체 또는 배열 내부의 값을 목표로 할 수 있습니다.`,
            },
            lookup: '조회',
            pathAdminClaim: '관리자 Claim 경로',
            pathMfaClaim: 'MFA Claim 경로',
            rootPemCert: 'Root CA의 PEM 인증서',
            mapMfa: `공급자에서 사용자가 로그인하는 동안 2FA 이상을 사용했음을 나타내는 Claim을 발행하는 경우,
                MFA Claim 경로를 지정할 수 있습니다.`,
            mapUser: `업스트림 ID Claim 에 따라 사용자를 Rauthy 관리자로 매핑할 수 있습니다.`,
            valueAdminClaim: '관리자 Claim 값',
            valueMfaClaim: 'MFA Claim 값',
        },
        delete: {
            areYouSure: '이 공급자를 삭제하시겠습니까?',
            forceDelete: '강제 삭제',
            isInUse1: '이 공급자는 활성 사용자가 사용 중입니다!',
            isInUse2: `강제로 삭제할 수는 있지만 로컬 비밀번호나 패스키가 없는 사용자는 더 이상 로그인할 수
                없습니다.`,
            linkedUsers: '연결된 사용자',
        },
    },
    roles: {
        adminNoMod: '<code>rauthy_admin</code> 역할은 변경할 수 없습니다.',
        delete1: '이 역할을 삭제하시겠습니까?',
        name: '역할 이름',
    },
    scopes: {
        defaultNoMod: '이것은 OIDC 기본 범위 중 하나입니다. 변경할 수 없습니다.',
        delete1: '이 범위를 삭제하시겠습니까?',
        deleteDefault: 'OIDC 기본 범위는 삭제할 수 없습니다.',
        mapping1: '사용자 지정 범위를 속성에 매핑할 수 있습니다.',
        mapping2: `구성된 모든 추가 속성은 각 사용자에 대해 사용자 지정 값을 가질 수 있습니다.
            이러한 속성이 범위에 매핑되면 액세스 및 ID 토큰에 포함될 수 있습니다.`,
        name: '범위 이름',
    },
    search: {
        orderBy: '정렬 기준',
        orderChangeToAsc: '오름차순으로 정렬',
        orderChangeToDesc: '내림차순으로 정렬',
    },
    sessions: {
        invalidateAll: '모든 세션 삭제',
    },
    tabs: {
        config: '설정',
        delete: '삭제',
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
            rule: 'Anti-Lockout Rule',
            delete: 'cannot be deleted',
            disable: 'cannot be disabled',
            rauthyAdmin: 'rauthy_admin rule cannot be removed',
        },
        attributes: '속성',
        deleteUser: '이 사용자를 삭제하시겠습니까?',
        descAttr: `사용자 지정 속성을 설정합니다. 모든 키/값 쌍은 문자열/JSON 값으로 처리됩니다.`,
        forceLogout: `기존 세션을 모두 삭제하고, 이 사용자의 모든 Refresh 토큰을 삭제하시겠습니까?`,
        lastLogin: '마지막 로그인',
        manualInitDesc: `The user can also be initialized here, In this case though, you need to communicate the 
            password directly.`,
        manualInit: 'Manual Initialization',
        mfaDelete1: '이 사용자의 패스키를 삭제할 수 있습니다.',
        mfaDelete2: `이 작업은 되돌릴 수 없습니다!`,
        noMfaKeys: '등록된 패스키 없음',
        pkOnly1: '이 사용자는 패스키 전용 계정입니다.',
        pkOnly2: '이 사용자는 비밀번호 없는 로그인을 사용하며, 설정된 비밀번호가 없습니다.',
        pkOnly3: `이 사용자가 모든 비밀번호를 분실한 경우, 계정을 완전히 재설정하고 새로운 비밀번호
            재설정 이메일을 보낼 수 있습니다. 이렇게 하려면 'MFA' 탭으로 이동하여 기존의 모든 패스키를
            삭제하세요.`,
        pwdNoInit: '사용자가 아직 초기 비밀번호 재설정을 수행하지 않았습니다.',
        pwdSendEmailBtn: '재설정 이메일 보내기',
        pwdSendEmailDesc: '새 비밀번호를 설정하거나, 재설정 이메일을 보낼 수 있습니다.',
        savePassword: '비밀번호 저장',
        selfServiceDesc: '새 비밀번호를 설정하거나, 재설정 이메일을 보낼 수 있습니다.',
        sendResetEmail: '재설정 이메일 보내기',
    },
    validation: {
        css: '비정상적인 CSS',
        origin: '비정상적인 오리진',
        uri: '비정상적인 URI',
    },
};
