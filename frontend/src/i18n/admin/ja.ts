import type { I18nAdmin } from './interface.ts';

export let I18nAdminJa: I18nAdmin = {
    api_key: {
        delete1: 'この API キーを削除してもよろしいですか？',
        expires: '有効期限',
        generate1: 'ここで、この API キーの新しいシークレットを生成できます。',
        generate2:
            'シークレットは生成した直後に一度だけ表示されます。新しく生成すると、古いシークレットは完全に上書きされます。この操作は元に戻せません！',
        generate3:
            'API キーは、HTTP の <code>Authorization</code> ヘッダーで次の形式で渡す必要があります:',
        generate4: '次の <code>curl</code> コマンドで、新しいキーを試せます:',
        generate5: '<code>jq</code> が入っておらず上のコマンドが失敗する場合:',
        keyName: 'キーの名前',
        limitedValidity: '有効期限を設ける',
    },
    attrs: {
        delete1: 'この属性を削除してもよろしいですか？',
        defaultValue: '既定値',
        desc: '説明',
        makeEditable: '編集可能にする',
        makeEditableP1: 'この属性を変換して、ユーザー自身が編集できるようにできます。',
        makeEditableP2:
            '<b>注意:</b> この変更は二度と元に戻せません！ユーザーが直接入力した値は常に信頼できないデータであり、認証や認可には絶対に使ってはいけません！',
        makeEditableP3:
            '一度でも信頼できない入力を受け付けた属性は、その期間の長さにかかわらず、編集可能から編集不可に戻すことはできません。',
        name: '属性の名前',
        userEditable: 'ユーザーが編集可能',
    },
    backup: {
        createBackup: 'バックアップを作成',
        disabledDesc: 'この機能は、データベースに Hiqlite を使っている場合だけ使えます。',
        lastModified: '最終更新',
        local: 'ローカル',
        name: '名前',
        size: 'サイズ',
    },
    clients: {
        allowedResources: '許可するリソース',
        defaultAud: '既定のオーディエンス',
        descAllowedResources:
            'このクライアントが要求できる RFC 8707 のリソースインジケーター（任意）。空にすると、「resource」リクエストパラメーターはすべて「invalid_target」で拒否されます。',
        descDefaultAud:
            '「resource」リクエストパラメーターに関係なく、このクライアントのトークンに常に追加されるオーディエンス。',
        backchannelLogout:
            'このクライアントが {{ OIDC_BCL }} に対応している場合は、ここに URI を入力できます。',
        branding: {
            descHsl:
                '次の値は HSL 値で指定してください。指定するのは基本色だけです。アルファチャンネルなどの値は、テーマが動的に調整します。',
            descFullCss:
                '次の値は、CSS の <code>color</code> として完全に正しい値で指定してください。複雑な計算や、上で定義した CSS 変数も使えます。',
            descVariables:
                '次の各ラベルは、そのまま CSS 変数の名前でもあります。つまり、自由入力欄で参照できます。例: <code>hsla(var(--action) / .7)</code>',
            faviconPreviewAlt: 'クライアントのファビコンのプレビュー',
            faviconUpload: 'ファビコンのアップロード',
        },
        claimsAtRoot: 'クレームをトークンの最上位に出力する',
        claimsAtRootWarning:
            'このクライアントのクレームは、「custom」の下ではなくトークンの最上位に書き込まれます。名前の衝突を避ける責任はあなたにあります: クレーム名が予約済みの JWT クレームと衝突すると、トークンを発行できません。最上位の独自クレームは、今後のプロトコルや機能の変更で動かなくなる可能性もあります。参考:',
        claims: '独自クレーム（client_credentials）',
        claimsDesc:
            'client_credentials のトークンに、custom クレームの下に入れて出力する JSON オブジェクト。シリアライズ後で最大 1024 文字。',
        confidential: 'コンフィデンシャル',
        confidentialNoSecret:
            'これはコンフィデンシャルではないクライアントのため、シークレットはありません。',
        config: 'クライアントの設定',
        delete1: 'このクライアントを削除してもよろしいですか？',
        descAuthCode:
            '安全性を高めるため、認可コードの有効期間を調整できます。認可コードは一度しか使えず、既定では 60 秒間有効です。クライアントがログインの手続きを十分速く行える範囲で、有効期間は短いほど良いです。',
        descClientUri: 'ログイン画面に表示する、このクライアントの URI と連絡先の情報。',
        descName:
            'クライアントの名前は、クライアントの設定に影響を与えずに変更できます。ログイン画面に表示するためだけのものです。',
        descGroupPrefix:
            'このクライアントへのログインは、グループの接頭辞（任意）で制限できます。一致するグループに割り当てられたユーザーだけがログインできます。',
        descOrigin:
            '追加で許可する外部のオリジン。通常は、このクライアントがブラウザから直接 Rauthy にリクエストを送る必要がある場合（主に SPA）だけ必要です。',
        descPKCE:
            'クライアントが対応していれば、安全性を高めるため常に S256 の PKCE を有効にしてください。コンフィデンシャルではないクライアント（SPA など）を使う場合は、十分な安全性のため少なくとも1つの PKCE チャレンジを有効にする必要があります。',
        descPKCEEnforce:
            'PKCE を有効にすると、Rauthy はログイン時にその使用を強制し、正しいチャレンジを含まないログインリクエストを拒否します。',
        descUri:
            'リダイレクト URI はいくつでも入力できます。それぞれの末尾に、ワイルドカードとして <code>*</code> を使えます。',
        errConfidentialPKCE:
            'クライアントはコンフィデンシャルにするか、少なくとも1つの PKCE チャレンジを有効にする必要があります。',
        forceMfa: '多要素認証を必須にする',
        groupLoginPrefix: 'ログインを許可するグループの接頭辞',
        name: 'クライアントの名前',
        passwordFlowMfaWarn:
            '注意: 「多要素認証を必須にする」と「password」フローが同時に有効になっています。OIDC の RFC に反するため、Rauthy はこの認証フローで多要素認証を強制できません。つまり、多要素認証を厳密に求めるなら、クライアント側で確かめる必要があります。そのためには「amr」クレームを使えます。',
        scim: {
            baseUri:
                'SCIM のベース URI は、<code>{base_uri}/Users/{id}</code> のような下位のルートを正しく導き出せる URI です。',
            desc: 'このクライアントが {{ SCIM_LINK }} に対応している場合は、ここで有効にできます。',
            enable: 'SCIMv2 を有効にする',
            groupSync: 'グループを同期する',
            groupSyncPrefix: 'グループを絞り込む接頭辞',
            groupSyncPrefixDesc:
                '同期するグループを、接頭辞（任意）で絞り込めます。たとえば <code>app:admins</code> と <code>app:users</code> というグループがある場合、接頭辞 <code>app:</code> を指定すると、これらのグループと、これらのグループの少なくとも1つに属するユーザーだけを同期します。',
            reqDesc: '互換性のため、いくつかの条件があります:',
            reqLi1: 'クライアントが <code>externalId</code> を正しく扱えること。',
            reqLi2: '少なくとも <code>/Users</code> のエンドポイントが <code>filter=externalId eq "*"</code> と <code>filter=userName eq "*"</code> に対応していること。',
            reqLi3: 'グループを同期する場合は、<code>/Groups</code> も <code>filter=displayName eq "*"</code> に対応していること。',
        },
        scopes: {
            allowed: '許可するスコープ',
            default: '既定のスコープ',
            desc: '許可するスコープは、<code>authorization_code</code> フローでログインへリダイレクトするときに、クライアントが動的に要求できるスコープです。既定のスコープは、<code>password</code> フローなどで起きる問題を避けるため、常にトークンに追加されます。',
        },
        secret: {
            doCache: 'クライアントシークレットをキャッシュする',
            cacheDuration: 'キャッシュする時間（時間）',
            generate: '新しいシークレットを生成',
            rotateDesc1:
                '停止を伴わない更新やシークレットのローテーションができるよう、現在のシークレットをしばらくメモリー上にキャッシュしておけます。1〜24 時間の値を入力できます。',
            rotateDesc2:
                '注意: シークレットが漏れた場合は、現在のシークレットをキャッシュしないでください！',
        },
        tokenLifetime: {
            p1: 'トークンの有効期間はアクセストークンと ID トークンに適用され、秒で指定します。',
            p2: 'クライアントが EdDSA / Ed25519 のアルゴリズムに対応していれば、常にそれを優先してください。RSA のアルゴリズムは互換性のためだけにあります。',
            p3: 'リフレッシュトークンは Rauthy だけが使うため、そのアルゴリズムは変更できません。',
        },
    },
    common: {
        account: 'アカウント',
        addNew: '新規追加',
        back: '戻る',
        caution: '注意',
        contact: '連絡先',
        copiedToClip: '値をクリップボードにコピーしました',
        details: '詳細',
        edit: '編集',
        enabled: '有効',
        filter: '絞り込み',
        from: '開始',
        information: '情報',
        language: '言語',
        loading: '読み込み中',
        jsonMeta: 'JSON 形式のメタデータ',
        name: '名前',
        nameExistsAlready: 'この名前はすでに存在します',
        note: 'メモ',
        noEntries: '項目がありません',
        preview: 'プレビュー',
        reset: 'リセット',
        searchOptions: '検索オプション',
        until: '終了',
    },
    docs: {
        book: 'Rauthy そのものについての一般的な説明は、次をご覧ください:',
        encryption: '暗号化',
        encKeys: {
            header: '暗号鍵',
            keyActive: '使用中の鍵',
            keysAvailable: '利用できる鍵',
            migrate: '移行',
            migrateToKey: '暗号化済みのすべての値を、次の鍵へ移行する',
            p1: 'これらの鍵は、裏で使われるデータストアの技術とは別に、保存データを追加で暗号化するために使われます。鍵は設定で固定的に指定しますが、この画面で手動でローテーション・移行できます。',
            p2: '使用中の鍵は、Rauthy の設定ファイルか環境変数で固定的に指定します。ここで動的に変更することはできません。新しく行う JWK の暗号化には、常に現在使用中の鍵が使われます。',
            p3: '既存のシークレットをすべて移行する場合、データが多いと完了までに数秒かかることがあります。',
            pNotPossible: '移行するには、少なくとも2つの暗号鍵が必要です。',
        },
        hashing: {
            calculate: '計算',

            currValuesHead: '現在の値',
            currValues1: 'バックエンドの現在の値は次のとおりです:',
            currValuesNote:
                'メモ: バックエンドが示すログイン時間は、Rauthy の起動後に少なくとも 5 回ログインに成功してから、ようやく目安として使えるようになります。起動直後の基準値は常に 2000 ms で、ログインに成功するたびに少しずつ調整されます。',
            currValuesThreadsAccess: 'Rauthy が使えるスレッド数（p_cost）',

            loginTimeHead: 'ログイン時間について',
            loginTime1:
                '一般に、ユーザーは何でもできるだけ速いことを望みます。ただし安全なログインであれば、500〜1000 ms 程度の時間は問題になりません。もちろん、ログイン時間が短すぎるとハッシュの強度が下がるため、短くしすぎてはいけません。',
            loginTime2:
                '既定でできるだけ安全にするため、このツールではログイン時間を 500 ms 未満にはできません。',

            mCost1: '<code>m_cost</code> は、ハッシュ計算に使う<b>メモリー量（kB）</b>を決めます。もちろん値は大きいほど良いですが、サーバーの資源を考える必要があります。<br>たとえば 4 つのパスワードを同時にハッシュすると、バックエンドはその間 <code>4 x m_cost</code> を必要とします。この資源が使えなければなりません。',
            mCost2: '<code>m_cost</code> の調整は簡単です。Rauthy に使わせる最大メモリー量を決め、同時に許可する最大ログイン数（<code>MAX_HASH_THREADS</code>）で割り、少しの固定メモリー量を引きます。どれだけの固定メモリーを見込むかは使うデータベースとユーザー数によりますが、通常は 32〜96 MB の範囲です。',
            mCost3: '<code>m_cost</code> に指定できる最小値は <code>32768</code> です。',

            pCost1: '<code>p_cost</code> は、ハッシュ計算の<b>並列度</b>を決めます。この値はたいてい 8 前後で頭打ちになり、これが Rauthy の既定値です。',
            pCost2: '基本の目安は次のとおりです:<br><code>p_cost</code> を、使えるコア数の 2 倍にします。<br>たとえば 4 コア使えるなら、<code>p_cost</code> を <code>8</code> にします。<br>ただし、設定した同時ログイン数（<code>MAX_HASH_THREADS</code>）を考慮して、その分だけ減らす必要があります。',

            tCost1: '<code>t_cost</code> は、ハッシュ計算にかける<b>時間</b>を決めます。<code>m_cost</code> と <code>p_cost</code> は基本的に環境で決まるため、実際に調整が必要なのはこの値だけです。',
            tCost2: '調整は簡単です: <code>m_cost</code> と <code>p_cost</code> を適切に設定し、目標のハッシュ時間に届くまで <code>t_cost</code> を増やします。',

            utilityHead: 'パラメーター計算ツール',
            utility1:
                'このツールで、お使いの環境に合う値の目安を出せます。Rauthy を最終的な場所に置き、最終的な資源がすべて使える状態で実行してください。調整しすぎを防ぐため、負荷がかかっている間に実行してください。',
            utility2:
                '<code>m_cost</code> は任意で、空欄なら安全な最小値の <code>32768</code> が選ばれます。<code>p_cost</code> も任意で、空欄なら Rauthy は見えるすべてのスレッドを使います。',

            time: '時間',
            targetTime: '目標時間',
            tune: '重要: これらの値は、最終的な環境で調整する必要があります！',
            pDetails:
                'Argon2ID について詳しく知りたい場合は、オンラインに多くの資料があります。このガイドでは値についてごく簡単に説明します。設定が必要な値は次の 3 つです:',
            pTune: 'これらの値はシステムの性能によって変わります。システムが強力なほど、より安全な値にできます。',
            pUtility:
                'このツールは、お使いのプラットフォームに最適な Argon2ID の設定を見つける手助けをします。Argon2ID は現在利用できる最も安全なパスワードのハッシュアルゴリズムです。その力を最大限に引き出すには、環境ごとに調整する必要があります。',
        },
        openapi:
            '外部のアプリケーションを連携させて Rauthy の API を使いたい場合は、次をご覧ください:',
        openapiNote:
            '既定の設定では、上のリンクから Swagger UI は使えません。設定の「server.swagger_ui_enable」と、必要に応じて「server.swagger_ui_public」で有効にできます。',
        source: 'ソースコードはこちら',
    },
    editor: {
        bold: '太字',
        code: 'コード',
        heading1: '見出し 1',
        heading2: '見出し 2',
        heading3: '見出し 3',
        italic: '斜体',
        link: 'リンク',
        listBullet: '箇条書き',
        listTasks: 'タスク',
        listNumbered: '番号付きリスト',
        paragraph: '段落',
        quote: '引用',
        removeFmt: '書式を解除',
        strikeThrough: '取り消し線',
        textArea: 'テキストを編集',
    },
    email: {
        cancelJob: 'ジョブを取り消す',
        filterType: ['なし', 'グループに所属', 'グループに非所属', 'ロールあり', 'ロールなし'],
        immediate: 'すぐに送信',
        jobs: 'メール送信ジョブ',
        scheduled: '予約送信',
        sendAllUsers: 'このメールはすべてのユーザーに送信されます。',
        sendAllUsersFiltered: 'このメールは、次の条件で絞り込んだすべてのユーザーに送信されます:',
        sendMail: 'メールを送信',
        subject: '件名',
        userFilter: 'ユーザーの絞り込み',
    },
    error: {
        needsAdminRole:
            '<b>rauthy_admin</b> ロールが割り当てられていません。<br/>管理画面にはアクセスできません。',
        noAdmin:
            'Rauthy の管理者アカウントでは、<b>多要素認証を有効にする</b>必要があります。<br>お使いの<b>アカウント</b>の画面で多要素認証を有効にしてください。<br>その後、一度ログアウトしてから、もう一度ログインしてください。',
    },
    events: {
        eventLevel: 'イベントのレベル',
        eventType: 'イベントの種類',
    },
    groups: {
        delete1: 'このグループを削除してもよろしいですか？',
        name: 'グループの名前',
    },
    jwks: {
        alg: 'アルゴリズム',
        p1: 'これらは、トークンの署名に使う JSON Web Key（JWK）です。',
        p2: 'JWK は既定で毎月 1 日にローテーションされます。新しく作られるトークンの署名には、そのアルゴリズムで利用できる最新の鍵だけが使われます。現在有効なトークンを引き続き正しく検証できるよう、古い鍵はしばらく残され、一定期間が過ぎると自動で削除されます。',
        p3: '鍵は手動でもローテーションできます。この Rauthy が動いているハードウェアによっては、数秒かかることがあります。',
        type: '種類',
        rotateKeys: '鍵をローテーション',
    },
    kv: {
        accessTestDesc:
            'アクセスキーは、<code>Authorization</code> ヘッダーで <code>Bearer</code> トークンとして渡す必要があります。試すには、次の <code>curl</code> コマンドを使えます。',
        addNewKey: '新しいアクセスキー',
        addNewNs: '新しい名前空間',
        addNewValue: '新しい値',
        delConfirm: 'このアクセスキーを本当に削除しますか？',
        delNsMsg: 'この名前空間を、中のデータもすべて含めて本当に削除しますか？',
        encryptedDesc:
            '性能上の理由から、追加の暗号化は、アクセスキーや個人情報など特に機密性の高い値だけに使ってください。',
        deleteConfirmMsg: 'キー「{{ key }}」を本当に削除しますか？',
        help: {
            help: 'ヘルプ',
            ops: [
                'アクセスキーを試す',
                '既存のキーをすべて取得',
                '既存のキーと値をすべて取得',
                'キーと値を設定',
                'キーの値を取得',
                'キーを削除',
            ],
            p1: 'KV ストアへの外部からのアクセスは、あえてとても単純にしてあります。操作は次のいくつかだけです:',
            p2: 'どの操作にも、<code>Authorization</code> ヘッダーに <code>Bearer</code> トークン（<code>{id}\${secret}</code>）としてアクセスキーが必要です。アクセスキーは、それが属する名前空間でだけ有効です。',
            p3: '上で挙げた操作の、<code>curl</code> を使った例を示します。',
        },
        key: 'キー',
        loadAllValues: 'すべての値を読み込む',
        storeEncrypted: '値を暗号化して保存する',
        tabs: ['データ', 'アクセス', '編集', '削除'],
        testCmd: 'テスト用コマンド',
        value: 'JSON の値',
    },
    nav: {
        apiKeys: 'API キー',
        attributes: '属性',
        blacklist: 'ブロックリスト',
        clients: 'クライアント',
        config: '設定',
        docs: 'ドキュメント',
        events: 'イベント',
        groups: 'グループ',
        providers: 'プロバイダー',
        roles: 'ロール',
        scopes: 'スコープ',
        sessions: 'セッション',
        users: 'ユーザー',
    },
    options: {
        expires: '有効期限',
        lastSeen: '最終確認',
        state: '状態',
    },
    pam: {
        addGroup: '新しい PAM グループ',
        addHost: '新しい PAM ホスト',
        addUser: '新しい PAM ユーザー',
        deleteHost: 'このホストを本当に削除しますか？',
        deleteUser: 'このユーザーを本当に削除しますか？',
        groupDescGeneric:
            '汎用グループは、通常 /etc/group にある項目に相当します。ユーザーを割り当てることができ、NSS の照会でシステムに返されます。',
        groupDescHost:
            'ホストグループは、ホストをまとめるために使います。グループ内のホストを NSS で照会すると、そのグループ内のほかのすべてのホストが返されます。ユーザーは、ホストグループに割り当てることでホストにアクセスできます。',
        groupDescLocal:
            'ローカルグループは汎用グループとほぼ同じように動きますが、Rauthy のデータベース上の ID を持つ一方で、各ホストの NSS プロキシーがそれを /etc/group の ID に変換する点が異なります。これにより、Rauthy のユーザーを、ローカルにすでにあるグループに割り当てられます。',
        groupDescUser:
            'ユーザーグループは自動で管理され、同じユーザー名のユーザーと密接に結び付いています。',
        groupDescWheel:
            'このグループは特別です。変更できず、ユーザーのグループ設定に応じて動的に割り当てられます。',
        groupName: 'グループ名',
        groups: 'グループ',
        groupType: 'グループの種類',
        hostAliases: 'ホストの別名',
        hostLocalPwdOnly: 'ローカルのパスワードでのログイン',
        hostLocalPwdOnlyInfo:
            '「ローカルのパスワードでのログイン」を設定すると、ローカルでのログインについて「多要素認証を必須にする」を上書きします。同時に、ユーザーが多要素認証で保護されていても、（ローカルの）ログインでパスキーは一切求められなくなります。このオプションは、多要素認証で保護されたユーザーがハードウェアのパスキーを使わずにローカルでログインできる必要がある場合など、本当に必要なときだけ設定してください。',
        ipAddresses: 'IP アドレス',
        member: 'メンバー',
        nameExistsAlready: 'この名前はすでに使われています',
        notes: 'メモ',
        secretShow: 'シークレットを表示',
        secretRotate: 'シークレットをローテーション',
        userEmail: '紐づいたユーザーのメールアドレス',
        username: 'ユーザー名',
        usernameNewDesc:
            'ユーザー名は慎重に決めてください。安全上の理由から、作成後に簡単には変更できません。',
    },
    passwordPolicy: {
        configDesc: '新しいパスワードのポリシー。',
        resetSet0: '0 にすると、その条件は無効になります。',
        validForDays: '有効日数',
        validityNew: '新しいパスワードの有効期間。',
    },
    providers: {
        config: {
            allowInsecureTls: '安全でない TLS を許可する',
            autoLink: 'ユーザーを自動で連携',
            autoLinkDesc1:
                '「ユーザーを自動で連携」を有効にすると、このプロバイダーでログインしたときに、まだ連携していない既存のユーザーがいれば自動でこのプロバイダーと連携します。',
            autoLinkDesc2:
                '注意: プロバイダーがユーザーのメールアドレスを十分に確認せず、他人のアドレスを追加できてしまう場合、このオプションは非常に危険で、アカウントの乗っ取りにつながります！そのような場合は絶対に使わないでください！',
            clientName: 'クライアントの名前',
            custRootCa: '独自のルート CA（PEM）',
            descAuthMethod:
                '<code>/token</code> エンドポイントで使う認証方式。<br>ほとんどのプロバイダーは <code>basic</code> で動きますが、<code>post</code> でしか動かないものもあります。まれに両方が必要な場合がありますが、ほかのプロバイダーではエラーになることがあります。',
            descClientId: '認証プロバイダーから発行されたクライアント ID。',
            descClientName: 'Rauthy のログイン画面に表示するクライアントの名前。',
            descClientSecret:
                '認証プロバイダーから発行されたクライアントシークレット。クライアントシークレットか PKCE の少なくとも一方が必要です。',
            descScope:
                'ログインへリダイレクトするときにクライアントが使うスコープ。値はスペースで区切って入力してください。',
            errNoAuthMethod:
                'クライアントシークレットが入力されていますが、クライアントの認証方式が1つも有効になっていません',
            errConfidential: 'コンフィデンシャルなクライアントにするか、PKCE を使う必要があります',
            jsonPath: {
                p1: '外部のプロバイダーでのログインに成功した後、ID トークンの値を自動で対応付けられます。',
                p2: '<code>path</code> は正規表現に似た書き方で指定します。単一の JSON の値にも、JSON のオブジェクトや配列の中の値にも解決できます。',
                p3: '<code>$.</code> は JSON オブジェクトの始まりを表します',
                p4: 'パスの中で <code>*</code> をワイルドカードとして使えます',
                p5: '<code>$.roles</code> は <code>&#123;"roles": "value"&#125;</code> を対象にします',
                p6: '<code>$.roles.*</code> は、次のようなオブジェクトや配列の中の値を対象にできます<br><code>&#123;"roles": ["value", "notMyValue"]&#125;</code>',
            },
            lookup: '照会',
            pathAdminClaim: '管理者クレームのパス',
            pathMfaClaim: '多要素認証クレームのパス',
            rootPemCert: 'ルート証明書（PEM）',
            mapMfa: 'ユーザーがログイン時に少なくとも 2 要素認証を使ったことを示すクレームをプロバイダーが発行する場合は、多要素認証クレームのパスを指定できます。',
            mapUser:
                '外部のプロバイダーの ID クレームに応じて、ユーザーを Rauthy の管理者に対応付けられます。',
            valueAdminClaim: '管理者クレームの値',
            valueMfaClaim: '多要素認証クレームの値',
        },
        delete: {
            areYouSure: 'このプロバイダーを削除してもよろしいですか？',
            forceDelete: '強制的に削除',
            isInUse1: 'このプロバイダーは、有効なユーザーが使っています！',
            isInUse2:
                '強制的に削除することもできますが、ローカルのパスワードもパスキーも持たないユーザーはログインできなくなります。',
            linkedUsers: '連携しているユーザー',
        },
    },
    roles: {
        adminNoMod: '<code>rauthy_admin</code> ロールは変更できません。',
        delete1: 'このロールを削除してもよろしいですか？',
        name: 'ロールの名前',
    },
    scopes: {
        claimsAtRoot: 'クレームをトークンの最上位に出力する',
        claimsAtRootWarning:
            'このスコープに対応付けた属性は、「custom」の下ではなくトークンの最上位に書き込まれます。名前の衝突を避ける責任はあなたにあります: 対応付けた属性の名前が予約済みの JWT クレームと衝突すると、トークンを発行できません。最上位の独自クレームは、今後のプロトコルや機能の変更で動かなくなる可能性もあります。参考:',
        defaultNoMod: 'これは OIDC の既定のスコープです。変更できません。',
        delete1: 'このスコープを削除してもよろしいですか？',
        deleteDefault: 'OIDC の既定のスコープは削除できません。',
        mapping1: '独自のスコープを属性に対応付けられます。',
        mapping2:
            '設定した追加の属性は、ユーザーごとに独自の値を持てます。スコープに対応付けると、アクセストークンや ID トークンに含められます。',
        name: 'スコープの名前',
    },
    search: {
        orderBy: '並べ替え ...',
        orderChangeToAsc: '昇順に並べ替える',
        orderChangeToDesc: '降順に並べ替える',
    },
    sessions: {
        invalidateAll: 'すべてのセッションを無効にする',
    },
    tabs: {
        config: '設定',
        delete: '削除',
    },
    tos: {
        accepted: '同意日時',
        addNewToS: '新しい利用規約',
        addNewToSFromCurrent: '選択中の利用規約から新規作成',
        added: '追加日時',
        checkStatus: 'ユーザーの状況を確認',
        immutable: '注意: 新しい利用規約は、追加した後は変更も削除もできません！',
        noneExist: '利用規約はまだ追加されていません。',
        optUntil: {
            desc: '移行期間中は、更新された利用規約への同意は任意です。移行期間が終わると必須になります。',
            enable: '移行期間を設ける',
            label: '移行期間の終了日時',
        },
        tos: '利用規約',
    },
    users: {
        antiLockout: {
            rule: '締め出し防止ルール',
            delete: '削除できません',
            disable: '無効にできません',
            rauthyAdmin: 'rauthy_admin ロールは外せません',
        },
        attributes: '属性',
        deleteUser: 'このユーザーを削除してもよろしいですか？',
        descAttr:
            'ユーザーの独自属性を設定します。すべてのキーと値の組は、文字列 / JSON の値として扱われます。',
        forceLogout:
            'このユーザーの既存のセッションをすべて無効にし、リフレッシュトークンをすべて削除してもよろしいですか？',
        groupAdmin: {
            notManagedTitle: 'あなたが管理するグループ外のユーザー',
            notManagedDesc:
                'このユーザーは、あなたが管理するどのグループにも属していないため、詳細は表示されません。管理するには、このユーザーをあなたのグループのいずれかに追加してください。あなたのグループ以外の所属はそのまま残ります。',
            addToGroups: '自分のグループに追加',
        },
        lastLogin: '最終ログイン',
        manualInitDesc:
            'ここでユーザーを初期設定することもできます。ただしその場合は、パスワードを本人に直接伝える必要があります。',
        manualInit: '手動で初期設定',
        mfa: {
            otp: {
                title: 'ワンタイムパスワード',
                mfaDelete1: 'このユーザーのワンタイムパスワードを削除できます。',
                mfaDelete2:
                    '注意！ワンタイムパスワードを削除すると、ユーザーが新しく登録し直さない限り<b>元に戻せません</b>。',
                noMfaOtps: 'このユーザーには、登録済みのワンタイムパスワードがありません。',
            },
            webauthn: {
                title: 'パスキー',
                mfaDelete1: 'このユーザーのパスキーを削除できます。',
                mfaDelete2:
                    '注意！パスキーを削除すると、ユーザーが新しく登録し直さない限り<b>元に戻せません</b>。',
                noMfaKeys: 'このユーザーには、登録済みのパスキーがありません。',
            },
        },
        pkOnly1: 'これはパスキーだけでログインするアカウントです。',
        pkOnly2:
            'つまり、このユーザーはパスワードを使わないログインを使っており、パスワードはまったく設定されていません。',
        pkOnly3:
            'このユーザーがすべてのパスキーをなくした場合は、アカウントを完全にリセットし、パスワードリセットのメールを新たに送れます。そのためには、「多要素認証」タブで既存のパスキーをすべて削除してください。',
        pwdNoInit: 'このユーザーは、最初のパスワード設定をまだ済ませていません。',
        pwdSendEmailBtn: 'リセットメールを送信',
        pwdSendEmailDesc: 'ユーザーが受け取っていない場合は、リセットメールを新たに送れます。',
        savePassword: 'パスワードを保存',
        selfServiceDesc: '新しいパスワードを設定するか、リセットメールを送れます。',
        sendResetEmail: 'リセットメールを送信',
    },
    validation: {
        css: 'CSS の値が正しくありません',
        origin: 'オリジンが正しくありません',
        uri: 'URI が正しくありません',
        redirectUri: {
            comma: 'リダイレクト URI にカンマ（,）は使えません',
            controlChar: 'リダイレクト URI のクエリーパラメーター名に制御文字は使えません',
            fragment: 'リダイレクト URI にフラグメント（#）は使えません',
            reservedKey:
                'リダイレクト URI にクエリーパラメーター「{{ KEY }}」は使えません。認可の応答で設定されるためです',
            reservedKeyLogout:
                'ログアウト後のリダイレクト URI にクエリーパラメーター「{{ KEY }}」は使えません。ログアウト時に設定されるためです',
        },
    },
};
