import {type I18n} from "./interface.ts";

export const I18nKo: I18n = {
    lang: "ko",
    common: {
        cancel: "취소",
        changeTheme: "Change Theme",
        close: "Close",
        copyToClip: "Copy value to clipboard",
        delete: "삭제",
        details: "Details",
        email: "이메일",
        errTooShort: "입력이 너무 짧습니다.",
        errTooLong: "입력이 너무 깁니다.",
        expandContent: "Expand content",
        hide: "Hide",
        hours: "Hours",
        invalidInput: "유효하지 않은 입력입니다.",
        legend: "Legend",
        maxFileSize: "Max File Size",
        minutes: "Minutes",
        month: "Month",
        months: [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December"
        ],
        never: "무기한",
        password: "비밀번호",
        required: "필수 항목입니다.",
        save: "저장",
        selectI18n: "Select Language",
        show: "Show",
        summary: "Summary",
        weekDaysShort: [
            "Sun",
            "Mon",
            "Tue",
            "Wed",
            "Thu",
            "Fri",
            "Sat",
        ],
        year: "Year",
    },
    account: {
        account: "사용자 계정",
        accType: "계정 종류",
        accTypePasskeyText1: "이 계정은 현재 패스키 전용 계정입니다.\n즉, 비밀번호가 필요하지 않으므로, 비밀번호가 전혀 없다는 뜻입니다.",
        accTypePasskeyText2: "계정을 전환하고 비밀번호를 추가할 수 있습니다.\n다만, 이렇게 하면 새 기기를 인증할 때마다 추가적으로 비밀번호 인증을 해야 한다는 것을 명심해 주세요.\n한 번도 비밀번호를 입력한 적이 없는 기기에서는 바로 로그인할 수 없습니다.",
        accTypePasskeyText3: "계정을 전환하고 비밀번호를 추가하겠습니까?",
        accessExp: "접근 만료",
        accessRenew: "접근 갱신 기한",
        accessRenewDelete: "갱신할 가능성을 제거",
        birthdate: "생년월일",
        city: "도시",
        changePassword: "비밀번호 변경",
        convertAccount: "계정 전환",
        convertAccountP1: "계정을 패스키 전용 계정으로 전환할 수 있습니다.\n이 전환은 비밀번호를 삭제하며, 등록된 패스키를 사용해서만 로그인할 수 있습니다.\n추가적인 사용자 인증이 가능한 패스키만 허용된다는 점을 명심해 주세요.\n패스키가 지원한다면, 'MFA' 페이지에서 키 이름 뒤에 작은 표시가 있는 것을 찾을 수 있을 겁니다.",
        country: "국가",
        deviceId: "아이디",
        deviceName: "이름",
        devices: "기기",
        devicesDesc: "이 계정에 연결된 기기",
        emailUpdateConfirm: "아직 이메일 주소가 변경되지 않았습니다. 새 주소로 메시지가 전송되었습니다.\n내부에 있는 승인 링크를 클릭해야 합니다. 승인되면 이메일 주소가 새 주소로 변경될 겁니다.",
        emailVerified: "이메일 인증 여부",
        familyName: "성",
        federatedConvertPassword1: "페더레이션 계정을 가지고 있습니다. 즉, 외부 인증 제공자를 사용하여\n로그인한다는 것을 의미합니다. 현재 제공자는 다음과 같습니다:",
        federatedConvertPassword2: "이메일을 통해 비밀번호 초기화를 요청할 수 있습니다. 완료하면 내부적인\n비밀번호가 계정에 추가될 겁니다. 그러면 외부 제공자 또는 내부적인 비밀번호를 통하여 로그인할 수 있습니다. 초기화를 요청하겠습니까?",
        generateRandom: "무작위로 생성",
        givenName: "이름",
        groups: "그룹",
        key: "키",
        keyUnique: "키는 고유해야 합니다.",
        lastLogin: "마지막 로그인 일",
        mfaActivated: "MFA 활성화 여부",
        navInfo: "정보",
        navEdit: "수정",
        navMfa: "MFA",
        navLogout: "로그아웃",
        passwordConfirm: "비밀번호 확인",
        passwordCurr: "현재 비밀번호",
        passwordCurrReq: "현재 비밀번호가 필요합니다.",
        passwordNew: "새 비밀번호",
        passwordNewReq: "새 비밀번호가 필요합니다.",
        passwordNoMatch: "비밀번호 확인이 필요합니다.",
        passwordExpiry: "비밀번호 만료일",
        passwordPolicyFollow: "비밀번호 정책을 준수해야 합니다.",
        passwordReset: "비밀번호 초기화",
        phone: "전화번호",
        providerLink: "페더레이션 계정",
        providerLinkDesc: "이 계정은 다음 중 하나의 로그인 제공자에 연결할 수 있습니다.\n이 기능을 활성화하면, 선택한 것의 로그인 페이지로 리다이렉트될 겁니다. 성공적으로 로그인하고 이메일이 일치하면, 계정이 연결될 겁니다.",
        providerUnlink: "페더레이션 해제",
        providerUnlinkDesc: "이 계정에 최소 하나의 비밀번호 또는 패스키를 설정하면, 상위 제공자로부터 연결을 해제할 수 있습니다.",
        regDate: "가입일",
        regIp: "IP에서 가입",
        roles: "역할",
        street: "주소",
        user: "사용자",
        userCreated: "사용자 생성일",
        userEnabled: "사용자 활성화 여부",
        userExpiry: "사용자 만료",
        userVerifiedTooltip: "지문 또는 PIN을 통해 보호",
        webIdDesc: "WebID와 함께 노출할 항목을 설정할 수 있습니다.\n이 기능은 몇몇 탈중앙화된 로그인 네트워크에서 사용됩니다. 이것이 무엇인지 모르면, 대체로 필요하지 않을 것입니다.",
        webIdDescData: "WebID에서 유효한 FOAF 어휘를 사용자 지정 데이터 항목에 추가할 수 있습니다",
        webIdExpertMode: "전문가 모드 활성화",
        zip: "우편번호"
    },
    authorize: {
        clientForceMfa: "이 로그인은 더 높은 수준의 보안을 위해서 MFA를 강제합니다.\n접근하려면, 계정에 로그인하고 최소 하나 이상의 패스키를 추가해야 합니다.",
        email: "이메일",
        emailBadFormat: "잘못된 이메일 형식입니다.",
        emailRequired: "이메일이 필요합니다.",
        emailSentMsg: "이메일이 존재하면, 요청이 전송되었을 것입니다.",
        expectingPasskey: "MFA 기기를 통해 로그인해 주세요.",
        http429: "유효하지 않은 입력이 너무 많습니다. 다음 시간 전까지 비활성화합니다:",
        invalidCredentials: "유효하지 않은 인증 정보입니다.",
        invalidKeyUsed: "유효하지 않은 키입니다.",
        login: "로그인",
        mfaAck: "확인되었습니다.",
        orLoginWith: "or login with",
        password: "비밀번호",
        passwordForgotten: "비밀번호를 잊으셨나요?",
        passwordRequest: "요청",
        passwordRequired: "비밀번호는 필요합니다.",
        requestExpires: "요청 만료일",
        requestExpired: "Request has expired",
        signUp: "사용자 가입",
        validEmail: "Valid E-Mail Address",
    },
    device: {
        accept: "수락",
        autoRedirectAccount: "지금 계정으로 리다이렉트될 예정입니다.",
        closeWindow: "이제 창을 닫아도 됩니다.",
        decline: "거절",
        desc: "기기에서 {{count}}자의 사용자 코드를 입력해 주세요.",
        descScopes: "다음 기기가 접근을 요청합니다:",
        isAccepted: "요청이 수락되었습니다.",
        isDeclined: "요청이 거절되었습니다.",
        submit: "제출",
        title: "기기 인증",
        userCode: "사용자 코드",
        wrongOrExpired: "잘못되거나 만료된 코드입니다."
    },
    emailChange: {
        title: "이메일 변경이 승인되었습니다.",
        textChanged: "이메일 주소가 다음으로부터 변경되었습니다",
        textLogin: "이제 새로운 주소로 로그인할 수 있습니다.",
        to: "에서",
    },
    error: {
        // errorText: "요청 정보를 찾을 수 없습니다.",
        details: "자세한 정보 표시",
        // detailsText: undefined,
    },
    index: {
        register: "가입",
        accountLogin: "계정",
        adminLogin: "관리",
    },
    logout: {
        logout: "로그아웃",
        confirmMsg: "정말로 로그아웃하고 세션을 종료하겠습니까?",
        cancel: "취소",
    },
    mfa: {
        p1: "윈도우와 안드로이드와 같은 여러 시스템에서 MFA 키를 사용하려면, 안드로이드에서 등록해야 합니다.",
        p2: "안드로이드는 비밀번호 없이 인증하는 기술을 가장 적게 지원하는 플랫폼 중 하나입니다. 안드로이드에서 등록한 키는 다른 곳에서도 작동합니다. 그러나, 그 반대로는 적용되지 않습니다.",
        errorReg: "가입 절차 시작 중 오류가 발생했습니다.",
        lastUsed: "마지막 사용",
        noKey: "이 슬롯에 등록된 보안 키가 없습니다.",
        register: "등록",
        registerNew: "새 키 등록",
        registerd: "등록되었습니다.",
        registerdKeys: "등록된 키",
        passkeyName: "패스키 이름",
        passkeyNameErr: "특수문자를 제외한 2자에서 32자이어야 합니다.",
        test: "테스트",
        testError: "테스트 시작 중에 오류가 발생했습니다.",
        testSuccess: "테스트 성공"
    },
    pagination: {
        entries: "Entries",
        gotoPage: "Go to page",
        gotoPagePrev: "Go to previous page",
        gotoPageNext: "Go to next page",
        pagination: "Pagination",
        showCount: "Show count",
        total: "Total",
    },
    passwordPolicy: {
        passwordPolicy: "비밀번호 정책",
        lengthMin: "최소 자수",
        lengthMax: "최대 자수",
        lowercaseMin: "최소 소문자 자수",
        uppercaseMin: "최소 대문자 자수",
        digitsMin: "최소 숫자 자수",
        specialMin: "최소 특수문자 자수",
        notRecent: "최근 사용한 비밀번호 중 하나가 아닌 것"
    },
    passwordReset: {
        accountLogin: "계정 로그인",
        badFormat: "잘못된 형식",
        fidoLink: "https://fidoalliance.org/fido2/?lang=ko",
        generate: "생성",
        newAccDesc1: "계정 종류는 비밀번호가 없는 계정 또는 기존의 비밀번호가 있는 계정 중 하나를 선택할 수 있습니다.",
        newAccDesc2: "비밀번호가 없는 계정은 더 강력한 보안 방법을 제공하기 때문에 항상 선호됩니다.\n이러한 계정을 생성하려면 최소 하나의 패스키(Yubikey, Apple Touch ID, Windows Hello, ...)가 필요합니다. 기기가 FIDO2 표준을 지원해야 합니다.\n더 자세한 사항은 다음 링크를 참고해 주세요: ",
        newAccount: "새 계정",
        passwordReset: "비밀번호 초기화",
        password: "비밀번호",
        passwordless: "패스키",
        passwordConfirm: "비밀번호 확인",
        passwordNoMatch: "비밀번호가 일치하지 않습니다.",
        required: "필수",
        save: "저장",
        success1: "비밀번호가 성공적으로 변경되었습니다.",
        success2: "곧 리다이렉트됩니다.",
        success3: "만약 리다이렉트가 되지 않으면, 여기를 클릭해 주세요:",
        successPasskey1: "새로운 패스키가 성공적으로 등록되었습니다.",
        successPasskey2: "계정에 로그인하여 가능한 한 빨리 두 번째 백업 키를 등록해 주세요.\n패스키 전용 계정은 현재 패스키를 잃어버리면, 이메일을 통하여 비밀번호 초기화할 수 없습니다."
    },
    register: {
        alreadyRegistered: "E-Mail is already registered",
        domainAllowed: "허용된 도메인:",
        domainErr: "허용되지 않은 이메일의 도메인입니다.",
        domainRestricted: "이메일의 도메인이 제한되어 있습니다.",
        email: "이메일",
        emailBadFormat: "잘못된 이메일 형식입니다.",
        emailCheck: "이메일 보관함을 확인해 주세요.",
        regexName: "이름은 특수문자를 제외한 2자에서 32자이어야 합니다.",
        register: "가입",
        success: "성공적으로 가입되었습니다.",
        userReg: "사용자 가입"
    }
};