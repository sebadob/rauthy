import { type I18n } from './interface';

export const I18nEs: I18n = {
    lang: 'es',
    common: {
        accept: 'Aceptar',
        activeTheme: 'Esquema de colores activo',
        authenticate: 'Autenticar',
        cancel: 'Cancelar',
        changeTheme: 'Cambiar tema - Activo: {{ CURRENT }}',
        close: 'Cerrar',
        copyToClip: 'Copiar valor al portapapeles',
        delete: 'Eliminar',
        details: 'Detalles',
        edit: 'Editar',
        email: 'Correo electrónico',
        errTooShort: 'Valor demasiado corto',
        errTooLong: 'Valor demasiado largo',
        expandContent: 'Expandir contenido',
        hide: 'Ocultar',
        hours: 'Horas',
        invalidInput: 'Valor no válido',
        legend: 'Leyenda',
        maxFileSize: 'Tamaño máximo de archivo',
        minutes: 'Minutos',
        month: 'Mes',
        months: [
            'Enero',
            'Febrero',
            'Marzo',
            'Abril',
            'Mayo',
            'Junio',
            'Julio',
            'Agosto',
            'Septiembre',
            'Octubre',
            'Noviembre',
            'Diciembre',
        ],
        never: 'Nunca',
        password: 'Contraseña',
        refresh: 'Actualizar',
        required: 'Obligatorio',
        save: 'Guardar',
        search: 'Buscar',
        seconds: 'segundos',
        selectI18n: 'Seleccionar idioma',
        show: 'Mostrar',
        summary: 'Resumen',
        timezone: 'Zona horaria',
        weekDaysShort: ['Dom', 'Lun', 'Mar', 'Mié', 'Jue', 'Vie', 'Sáb'],
        year: 'Año',
    },
    account: {
        account: 'Cuenta de usuario',
        accType: 'Tipo de cuenta',
        accTypePasskeyText1: `Esta cuenta es actualmente una cuenta solo con Passkey.\nEsto significa
            que no tiene contraseña, porque no la necesita.`,
        accTypePasskeyText2: `Puede convertir su cuenta y añadir una contraseña. Pero tenga en cuenta
            que esto implica que deberá verificar también cada dispositivo nuevo con la contraseña.\n
            Ya no podrá iniciar sesión en un dispositivo en el que no haya introducido la contraseña
            al menos una vez.`,
        accTypePasskeyText3: '¿Desea convertir su cuenta y añadir una contraseña?',
        accessExp: 'El acceso caduca',
        accessRenew: 'Acceso renovable hasta',
        accessRenewDelete: 'Eliminar la posibilidad de renovación',
        birthdate: 'Fecha de nacimiento',
        canModifyFor: 'La MFA se puede modificar durante:',
        city: 'Ciudad',
        changePassword: 'Cambiar contraseña',
        convertAccount: 'Convertir cuenta',
        convertAccountP1: `Puede convertir su cuenta en una cuenta solo con Passkey.\nEsta conversión
            elimina su contraseña y, a partir de entonces, solo podrá y deberá iniciar sesión con sus
            Passkeys registradas.\nTenga en cuenta que solo se aceptarán Passkeys con verificación
            adicional del usuario. Si sus Passkeys la admiten, verá un pequeño símbolo junto al nombre
            de la clave en la página 'MFA'.`,
        country: 'País',
        deleteAccount: {
            deleteAccount: 'Eliminar cuenta',
            deleteAccountDesc: `ATENCIÓN: ¡La eliminación de esta cuenta no se puede deshacer! Para
                confirmar, introduzca:`,
        },
        deviceId: 'ID',
        deviceName: 'Nombre',
        devices: 'Dispositivos',
        devicesDesc: 'Dispositivos vinculados a esta cuenta',
        emailUpdateConfirm: `La dirección de correo electrónico aún no se ha actualizado. Se ha
            enviado un mensaje a su nueva dirección.\nDebe hacer clic en el enlace de confirmación que
            contiene. Una vez confirmado, se actualizará su nueva dirección.`,
        emailVerified: 'Correo electrónico verificado',
        familyName: 'Apellidos',
        federatedConvertPassword1: `Tiene una cuenta federada. Esto significa que inicia sesión\n
            mediante un proveedor de autenticación externo. Su proveedor actual es:`,
        federatedConvertPassword2: `Puede solicitar el restablecimiento de la contraseña por correo
            electrónico. Una vez completado,\nse añadirá una contraseña local a su cuenta. Después podrá
            iniciar sesión mediante su\nproveedor externo o con la contraseña local. ¿Desea solicitar
            el restablecimiento?`,
        generateRandom: 'Generar aleatoriamente',
        givenName: 'Nombre',
        groups: 'Grupos',
        key: 'Clave',
        keyUnique: 'La clave debe ser única',
        lastLogin: 'Último inicio de sesión',
        mfaActivated: 'MFA activada',
        navInfo: 'Información',
        navEdit: 'Editar',
        navMfa: 'MFA',
        navLogout: 'Cerrar sesión',
        navBackToAdmin: 'Interfaz de administración',
        other: 'Otros',
        pam: {
            addSshKey: 'Añadir nueva Key',
            comment: 'Comentario',
            generatePassword: 'Nueva contraseña',
            keyAdded: 'Añadida',
            keyType: 'Tipo',
            username: 'Nombre de usuario',
            validFor: 'Contraseña válida durante {{ secs }} segundos',
        },
        passkeys: {
            missingAttestation: 'La Passkey no cumple los estándares de seguridad requeridos.',
            type: 'Tipo de Passkey',
            types: ['Predeterminada', 'Resident Key'],
            rkWarning: `ATENCIÓN: Aunque una Resident Key permite iniciar sesión directamente sin
                indicar la dirección de correo electrónico, se necesita un espacio de almacenamiento
                adicional para cada aplicación. Asegúrese de que queda espacio suficiente.
                Para convertir una Passkey existente en Resident Key, elimínela primero y vuelva a
                añadirla.`,
            // [Default, Resident Key]
            typesDesc: [
                'Debe introducir su correo electrónico antes de poder iniciar sesión con esta Passkey.',
                `Funciona con el botón Passkey de la página de inicio de sesión, sin introducir su
                    correo electrónico.`,
            ],
        },
        passwordConfirm: 'Confirmar contraseña',
        passwordCurr: 'Contraseña actual',
        passwordCurrReq: 'La contraseña actual es obligatoria',
        passwordNew: 'Nueva contraseña',
        passwordNewReq: 'La nueva contraseña es obligatoria',
        passwordNoMatch: 'Es obligatorio confirmar la contraseña',
        passwordExpiry: 'Caducidad de la contraseña',
        passwordPolicyFollow: 'Debe cumplir la política de contraseñas',
        passwordReset: 'Restablecer contraseña',
        phone: 'Teléfono',
        preferredUsername: {
            desc: `Algunas aplicaciones usarán este nombre de usuario. No se puede controlar
				cuáles lo hacen.`,
            forceOverwrite: 'Forzar sobrescritura',
            immutable: 'ATENCIÓN: ¡El nombre de usuario solo se puede elegir una vez!',
            immutableInfo: 'El nombre de usuario no se puede cambiar.',
            notAvailable: 'Este nombre no está disponible.',
            preferredUsername: 'Nombre de usuario preferido',
        },
        providerLink: 'Federar cuenta',
        providerLinkDesc: `Puede vincular esta cuenta a uno de los siguientes proveedores de inicio de
            sesión.\nDespués de activar esta función, se le redirigirá a la página de inicio de sesión
            del proveedor elegido.\nTras iniciar sesión correctamente y si el correo electrónico
            coincide, su cuenta quedará vinculada.`,
        providerUnlink: 'Desvincular federación',
        providerUnlinkDesc: `Solo puede desvincular esta cuenta del proveedor externo si ha
            configurado\nal menos una contraseña o una Passkey para esta cuenta.`,
        regDate: 'Fecha de registro',
        regIp: 'Registro desde la IP',
        roles: 'Roles',
        street: 'Calle',
        user: 'Usuario',
        userCreated: 'Usuario creado',
        userEnabled: 'Usuario activo',
        userExpiry: 'El usuario caduca',
        userVerifiedTooltip: 'Protegida con huella digital o PIN',
        residentKeyTooltip: `Esta es una Resident Key. Puede usarla para iniciar sesión directamente
            sin indicar un correo electrónico.`,
        aaguidTooltip: 'Passkey certificada',
        webIdDesc: `Puede configurar los campos que se mostrarán a través de su WebID.\nEsta es una
            función que usan algunas redes para inicios de sesión descentralizados. Si no sabe qué es,
            \nlo más probable es que no la necesite.`,
        webIdDescData:
            'Puede añadir campos de datos personalizados a su WebID en vocabulario FOAF válido',
        webIdExpertMode: 'Activar modo experto',
        zip: 'Código postal',
    },
    authorize: {
        clientForceMfa: `Este inicio de sesión exige MFA para lograr una mayor seguridad.\nPara
            obtener acceso, debe iniciar sesión en su cuenta y añadir al menos una Passkey`,
        clientGroupPrefixForbidden: 'Falta la asignación de grupo para este inicio de sesión',
        email: 'Correo electrónico',
        emailBadFormat: 'Formato de correo electrónico no válido',
        emailRequired: 'El correo electrónico es obligatorio',
        emailSentMsg: 'Si su correo electrónico existe, se ha enviado una solicitud',
        expectingPasskey: 'Esperando la autenticación con Passkey',
        expectingOtp: 'Esperando la autenticación con OTP',
        http429: 'Demasiados intentos no válidos. Bloqueado hasta:',
        invalidCredentials: 'Credenciales no válidas',
        invalidKeyUsed: 'Clave no válida',
        login: 'Iniciar sesión',
        mfaAck: 'Confirmado',
        mfaOpenApp: 'Abre la aplicación para continuar',
        navigateToAccount: 'Panel de la cuenta',
        needsUserUpdate: `Falta información en esta cuenta. Debe actualizar estos datos antes de
            continuar con el inicio de sesión.`,
        orLoginWith: 'o iniciar sesión con',
        passkeyNotDiscovered: `Las Passkeys que no se añadieron como Resident Key no se pueden
            encontrar de esta forma. Introduzca su correo electrónico en su lugar.`,
        password: 'Contraseña',
        passwordExpired:
            'La contraseña ha caducado. Se ha enviado un correo electrónico con un enlace para restablecerla.',
        passwordForgotten: '¿Ha olvidado su contraseña?',
        passwordRequest: 'Solicitar',
        passwordRequired: 'La contraseña es obligatoria',
        passwordResetDesc: `Indique su correo electrónico para solicitar un enlace de restablecimiento
            de contraseña. Si su dirección existe en nuestra base de datos, recibirá un enlace por correo
            electrónico.`,
        passwordResetSuccess: 'Solicitud recibida. Ya puede cerrar esta ventana.',
        requestExpires: 'La solicitud caduca',
        requestExpired: 'La solicitud ha caducado',
        signUp: 'Registro de usuario',
        validEmail: 'Indique una dirección de correo electrónico válida',
    },
    device: {
        accept: 'Aceptar',
        autoRedirectAccount: 'Se le redirigirá a su cuenta',
        closeWindow: 'Ya puede cerrar esta ventana.',
        decline: 'Rechazar',
        desc: 'Introduzca el código de usuario de {{count}} caracteres que muestra su dispositivo.',
        descScopes: 'El dispositivo solicita acceso a:',
        isAccepted: 'La solicitud se ha aceptado.',
        isDeclined: 'La solicitud se ha rechazado.',
        submit: 'Enviar',
        title: 'Autorización de dispositivo',
        userCode: 'Código de usuario',
        wrongOrExpired: 'Código incorrecto o caducado',
    },
    emailChange: {
        title: 'Cambio de correo electrónico confirmado',
        textChanged: 'Su dirección de correo electrónico se ha cambiado de',
        textLogin: 'Ya puede iniciar sesión con su nueva dirección.',
        to: 'a',
    },
    error: {
        // errorText: "No se han encontrado los datos solicitados",
        details: 'Mostrar detalles',
        // detailsText: undefined,
    },
    index: {
        register: 'Registrarse',
        accountLogin: 'Cuenta',
        adminLogin: 'Admin',
    },
    logout: {
        logout: 'Cerrar sesión',
        confirmMsg: '¿Seguro que desea cerrar sesión y finalizar su sesión?',
        cancel: 'Cancelar',
    },
    mfa: {
        webauthn: {
            title: 'Passkeys',
            p1: `Si piensa usar su clave MFA en varios sistemas, como Windows y Android, debería
                hacer el registro con Android.`,
            p2: `Android es la plataforma que admite menos funciones de la tecnología sin
                contraseña. Las claves que registre con Android también funcionan en otros sistemas.
                Sin embargo, esto no se aplica a la inversa.`,
            p3: 'Para obtener más información, consulte',
            registerNew: 'Registrar nueva Passkey',
            docLinkText: 'la documentación sobre Passkeys',
            unsupportedText: 'Su navegador no admite credenciales WebAuthn y debe actualizarse.',
        },
        otp: {
            title: 'Código de un solo uso',
            titleEmail: 'OTP por correo electrónico',
            activationCode:
                'Introduzca el código de un solo uso que se ha enviado a su dirección de correo electrónico.',
            code: 'Código de un solo uso',
            registerNew: 'Registrar nuevo OTP',
            resendOtp: 'Reenviar código',
            sessionExpiresIn: 'La sesión OTP caduca en:',
            sessionExpired: 'La sesión OTP ha caducado.',
        },
        requestCanceled: 'Se ha cancelado la solicitud',
        errorReg: 'Error al iniciar el proceso de registro',
        lastUsed: 'Último uso',
        noKey: 'No hay ninguna llave de seguridad registrada en esta ranura',
        reAuthenticatePasskey:
            'Antes de poder modificar su MFA, debe autenticarse con una Passkey:',
        reAuthenticatePwd:
            'Antes de poder modificar su MFA, debe volver a autenticarse con su contraseña.',
        reAuthenticateOtp:
            'Antes de poder modificar su MFA, debe volver a autenticarse con un OTP:',
        register: 'Registrar',
        registered: 'Registrada',
        registeredKeys: 'Claves registradas',
        registeredOtps: 'OTP registrados',
        passkeyName: 'Nombre de la Passkey',
        passkeyNameErr: 'De 2 a 32 caracteres, sin caracteres especiales',
        passwordInvalid: 'Contraseña no válida',
        test: 'Probar',
        testError: 'Error al iniciar la prueba',
        testSuccess: 'Prueba correcta',
    },
    pagination: {
        entries: 'Entradas',
        gotoPage: 'Ir a la página',
        gotoPagePrev: 'Ir a la página anterior',
        gotoPageNext: 'Ir a la página siguiente',
        pagination: 'Paginación',
        showCount: 'Número de entradas',
        total: 'Total',
    },
    passwordPolicy: {
        passwordPolicy: 'Política de contraseñas',
        lengthMin: 'Longitud mínima',
        lengthMax: 'Longitud máxima',
        lowercaseMin: 'Mínimo de minúsculas',
        uppercaseMin: 'Mínimo de mayúsculas',
        digitsMin: 'Mínimo de dígitos',
        specialMin: 'Mínimo de caracteres especiales',
        notRecent: 'Distinta de las últimas contraseñas usadas',
    },
    passwordReset: {
        accountLogin: 'Inicio de sesión en la cuenta',
        badFormat: 'Formato no válido',
        fidoLink: 'https://fidoalliance.org/fido2',
        generate: 'Generar',
        newAccDesc1:
            'Puede elegir entre dos tipos de cuenta: sin contraseña o con contraseña tradicional',
        newAccDesc2: `La cuenta sin contraseña es siempre la opción preferida, porque ofrece\nuna
            seguridad más sólida. Necesitará al menos una Passkey (Yubikey, Apple Touch ID, Windows
            Hello,\n...) para crear una cuenta de este tipo. Su dispositivo debe ser compatible con el
            estándar FIDO2. Para obtener más información\nal respecto, puede seguir este enlace: `,
        newAccount: 'Nueva cuenta',
        passwordReset: 'Restablecer contraseña',
        password: 'Contraseña',
        passwordless: 'Passkey',
        passwordConfirm: 'Confirmar contraseña',
        passwordNoMatch: 'Las contraseñas no coinciden',
        required: 'Obligatorio',
        save: 'Guardar',
        success1: 'La contraseña se ha actualizado correctamente.',
        success2: 'Se le redirigirá en unos instantes.',
        success3: 'Si no se le redirige, haga clic aquí:',
        successPasskey1: 'Su nueva Passkey se ha registrado correctamente.',
        successPasskey2: `Inicie sesión en su cuenta y registre una segunda clave de respaldo\nlo antes
            posible. Con una cuenta solo con Passkey, no podrá restablecer la contraseña por\ncorreo
            electrónico si pierde su clave actual.`,
    },
    register: {
        alreadyRegisteredEmail: 'El correo electrónico ya está registrado',
        alreadyRegisteredUsername: 'El nombre de usuario ya está en uso',
        domainAllowed: 'Dominio permitido:',
        domainErr: 'Dominio de correo electrónico no permitido',
        domainRestricted: 'Los dominios de correo electrónico están restringidos',
        email: 'Correo electrónico',
        emailBadFormat: 'Formato de correo electrónico no válido',
        emailCheck: 'Revise la bandeja de entrada de su correo electrónico',
        orRegisterWith: 'o registrarse con',
        regexName: 'El nombre debe tener de 2 a 32 caracteres, sin caracteres especiales',
        register: 'Registrarse',
        success: 'Registro completado correctamente',
        userReg: 'Registro de usuario',
    },
    tos: {
        acceptOptUntil: 'La aceptación es opcional hasta:',
        deny: 'Rechazar',
        tos: 'Términos del servicio',
    },
    userRevoke: {
        title: 'Revocar inicios de sesión',
        desc1: 'Se han revocado, en la medida de lo posible, todos los inicios de sesión y sesiones de este usuario.',
        desc2: '¡Debe renovar inmediatamente todas sus contraseñas!',
    },
};
