import type { I18nAdmin } from './interface.ts';

export let I18nAdminEs: I18nAdmin = {
    api_key: {
        delete1: '¿Seguro que desea eliminar esta API Key?',
        expires: 'Caducidad',
        generate1: 'Aquí puede generar un nuevo secret para esta API Key.',
        generate2: `Solo verá este secret una vez, justo después de generarlo.
            Cuando se genere uno nuevo, el secret anterior se sobrescribirá de forma permanente.
            ¡Esta operación no se puede deshacer!`,
        generate3: `La API Key debe indicarse en la cabecera HTTP <code>Authorization</code>
            con el siguiente formato:`,
        generate4: 'Puede usar la siguiente petición <code>curl</code> para probar su nueva Key:',
        generate5: 'Si no tiene <code>jq</code> instalado y lo anterior falla:',
        keyName: 'Nombre de la Key',
        limitedValidity: 'Validez limitada',
    },
    attrs: {
        delete1: '¿Seguro que desea eliminar este atributo?',
        defaultValue: 'Valor predeterminado',
        desc: 'Descripción',
        makeEditable: 'Hacer editable',
        makeEditableP1:
            'Puede convertir este atributo y hacer que los propios usuarios puedan editarlo.',
        makeEditableP2: `<b>ATENCIÓN:</b> ¡Este cambio no se podrá deshacer nunca! Todos los datos introducidos
            directamente por un usuario son siempre datos no fiables y NUNCA deben usarse para ninguna forma de
            autenticación ni de autorización.`,
        makeEditableP3: `Un atributo no puede pasar de editable a no editable, porque en el pasado aceptó datos no
            fiables, independientemente de durante cuánto tiempo fuera así.`,
        name: 'Nombre del atributo',
        userEditable: 'Editable por el usuario',
    },
    backup: {
        createBackup: 'Crear copia de seguridad',
        disabledDesc: 'Esta función solo existe si Hiqlite está configurado como base de datos.',
        lastModified: 'Última modificación',
        local: 'Local',
        name: 'Nombre',
        size: 'Tamaño',
    },
    clients: {
        allowedResources: 'Recursos permitidos',
        defaultAud: 'Audiences predeterminadas',
        descAllowedResources: `Resource Indicators (RFC 8707) opcionales que este Client puede solicitar. Una lista vacía rechaza cualquier parámetro 'resource' con 'invalid_target'.`,
        descDefaultAud: `Audiences que siempre se añaden a los tokens de este Client, independientemente de cualquier parámetro 'resource'.`,
        backchannelLogout: 'Si este Client admite {{ OIDC_BCL }}, puede indicar aquí el URI.',
        branding: {
            descHsl: `Los siguientes valores deben indicarse como valores HSL. Indique solo los colores base.
            El tema ajusta dinámicamente los canales alfa y los demás valores.`,
            descFullCss: `Los siguientes valores deben ser valores CSS <code>color</code> totalmente válidos.
                También puede usar cálculos complejos o las variables CSS definidas arriba.`,
            descVariables: `Cada una de las siguientes etiquetas es también el nombre de la variable CSS. Esto significa
                que puede hacer referencia a ellas en los campos libres, por ejemplo con <code>hsla(var(--action) / .7)</code>.`,
            faviconPreviewAlt: 'Vista previa del favicon del Client',
            faviconUpload: 'Subir favicon',
        },
        claimsAtRoot: 'Emitir claims en la raíz del token',
        claimsAtRootWarning: `Las claims de este Client se escriben en la raíz del token,
            en lugar de agruparse bajo 'custom'. Usted es responsable de evitar colisiones: si el nombre de una
            claim coincide con una claim JWT reservada, la emisión del token falla. Las claims personalizadas en la
            raíz también pueden dejar de funcionar con futuros cambios del protocolo o de funciones. Referencia:`,
        claims: 'Custom claims (client_credentials)',
        claimsDesc: `Un objeto JSON que se emite en los tokens client_credentials, agrupado bajo la
            claim custom. Máximo de 1024 caracteres serializados.`,
        confidential: 'Confidencial',
        confidentialNoSecret: 'Este es un Client no confidencial y, por tanto, no tiene secret.',
        config: 'Configuración del Client',
        delete1: '¿Seguro que desea eliminar este Client?',
        descAuthCode: `La validez de los auth codes se puede ajustar para aumentar la seguridad. Los auth codes
            solo se pueden usar una vez y, de forma predeterminada, son válidos durante 60 segundos. Cuanto más corta
            sea la validez, mejor, siempre que el Client pueda completar el proceso de inicio de sesión a tiempo.`,
        descClientUri: `Información sobre el URI y los contactos de este Client que se mostrará en
            la página de inicio de sesión.`,
        descName: `El nombre del Client se puede cambiar sin que afecte a la configuración del Client.
            Solo sirve para mostrarse en la página de inicio de sesión.`,
        descGroupPrefix: `El inicio de sesión en este Client se puede restringir mediante un prefijo de grupo
            opcional. Solo podrán iniciar sesión los usuarios asignados a un grupo que coincida.`,
        descOrigin: `Origins externas permitidas adicionalmente: normalmente solo son necesarias si este Client
            tiene que hacer peticiones a Rauthy directamente desde el navegador, como suele ocurrir con las SPA.`,
        descPKCE: `Si el Client lo admite, debería activar siempre PKCE S256 para una mayor seguridad.
            Si se usa un Client no confidencial (por ejemplo, una SPA), debe activar al menos uno de los
            PKCE challenges para disponer de suficiente seguridad.`,
        descPKCEEnforce: `Si hay algún PKCE activado, Rauthy exigirá su uso al iniciar sesión y
            rechazará las solicitudes de inicio de sesión que no contengan un challenge válido.`,
        descUri: `Puede indicar tantos Redirect URIs como quiera. Al final de cada uno, puede usar
            <code>*</code> como comodín.`,
        errConfidentialPKCE: `El Client debe ser confidencial o tener al menos un PKCE
            challenge activado.`,
        forceMfa: 'Forzar MFA',
        groupLoginPrefix: 'Prefijo de grupo para el inicio de sesión',
        name: 'Nombre del Client',
        passwordFlowMfaWarn: `ATENCIÓN: 'Forzar MFA' y el flow 'password' están activados al mismo
            tiempo. Rauthy NO puede exigir MFA mediante este flow de autenticación, porque infringiría el
            RFC de OIDC. Esto significa que, si necesita MFA de forma estricta, debe validarla en el lado del
            Client. Para ello, puede usar la claim 'amr'.`,
        scim: {
            baseUri: `El base URI de SCIM es aquel a partir del cual se pueden derivar correctamente las subrutas
                como <code>{base_uri}/Users/{id}</base_uri></code>.`,
            desc: 'Si este Client admite {{ SCIM_LINK }}, puede activarlo aquí.',
            enable: 'Activar SCIMv2',
            groupSync: 'Sincronizar grupos',
            groupSyncPrefix: 'Prefijo del filtro de grupos',
            groupSyncPrefixDesc: `Puede filtrar los grupos que se sincronizan mediante un prefijo opcional.
                Por ejemplo, si existen los grupos <code>app:admins</code> y <code>app:users</code>, el prefijo
                <code>app:</code> solo sincronizaría estos grupos y solo los usuarios vinculados a al menos
                uno de ellos.`,
            reqDesc: 'Para la compatibilidad se necesitan algunos requisitos:',
            reqLi1: 'El Client debe gestionar <code>externalId</code> correctamente.',
            reqLi2: `Deben admitirse al menos los endpoints <code>/Users</code> con
                <code>filter=externalId eq "*"</code> y <code>filter=userName eq "*"</code>.`,
            reqLi3: `Si se van a sincronizar grupos, <code>/Groups</code> también debe admitir
                <code>filter=displayName eq "*"</code>.`,
        },
        scopes: {
            allowed: 'Scopes permitidos',
            default: 'Scopes predeterminados',
            desc: `Los Scopes permitidos son los que el Client puede solicitar dinámicamente durante
            la redirección al inicio de sesión al usar el flow <code>authorization_code</code>. Los scopes
            predeterminados se añaden siempre a los tokens para resolver algunos problemas, por ejemplo al usar
            el flow <code>password</code>.`,
        },
        secret: {
            doCache: 'Guardar el Client Secret en caché',
            cacheDuration: 'Duración de la caché (horas)',
            generate: 'Generar nuevo secret',
            rotateDesc1: `Para permitir actualizaciones y rotaciones de secrets sin interrupciones, puede mantener
                el secret actual en una caché en memoria durante un tiempo. Puede indicar un valor entre 1 y 24 horas.`,
            rotateDesc2: 'Atención: ¡No debe guardar en caché el secret actual si se ha filtrado!',
        },
        tokenLifetime: {
            p1: `La duración del token se aplica a los Access tokens y los ID tokens y se indica en segundos.`,
            p2: `Si el Client admite algoritmos EdDSA / Ed25519, deberían ser siempre la opción
                preferida. Los algoritmos RSA solo existen por compatibilidad.`,
            p3: `El algoritmo de los refresh tokens no se puede cambiar, ya que solo los usa Rauthy.`,
        },
    },
    common: {
        account: 'Cuenta',
        addNew: 'Añadir',
        back: 'Volver',
        caution: 'ATENCIÓN',
        contact: 'Contacto',
        copiedToClip: 'El valor se ha copiado al portapapeles',
        details: 'Detalles',
        edit: 'Editar',
        enabled: 'Activado',
        filter: 'Filtro',
        from: 'Desde',
        information: 'Información',
        language: 'Idioma',
        loading: 'Cargando',
        jsonMeta: 'Metadatos como valor JSON',
        name: 'Nombre',
        nameExistsAlready: 'El nombre ya existe',
        note: 'Nota',
        noEntries: 'Sin entradas',
        preview: 'Vista previa',
        reset: 'Restablecer',
        searchOptions: 'Opciones de búsqueda',
        until: 'Hasta',
    },
    docs: {
        book: 'Para la documentación general sobre Rauthy, consulte el',
        encryption: 'Cifrado',
        encKeys: {
            header: 'Claves de cifrado',
            keyActive: 'Clave activa',
            keysAvailable: 'Claves disponibles',
            migrate: 'Migrar',
            migrateToKey: 'Migrar todos los valores cifrados existentes a la siguiente clave',
            p1: `Estas claves se usan para un cifrado adicional de los datos en reposo, independientemente de la
            tecnología de almacenamiento que se use internamente. Se configuran de forma estática, pero se pueden rotar
            y migrar manualmente en esta página.`,
            p2: `La clave activa se define de forma estática en el archivo de configuración de Rauthy / las variables
            de entorno. No se puede cambiar aquí de forma dinámica. Todos los nuevos cifrados de JWK usarán siempre la
            clave activa actual.`,
            p3: `Si migra todos los secrets existentes, la operación puede tardar unos segundos en terminar si tiene
            un gran volumen de datos.`,
            pNotPossible: 'Para poder migrar, debe haber al menos 2 claves de cifrado disponibles.',
        },
        hashing: {
            calculate: 'Calcular',

            currValuesHead: 'Valores actuales',
            currValues1: 'Los valores actuales del backend son los siguientes:',
            currValuesNote: `Nota: El tiempo de inicio de sesión del backend solo es una buena referencia después de
            al menos 5 inicios de sesión correctos desde que se inició Rauthy. El valor base es siempre de 2000 ms tras
            un reinicio y se va ajustando con cada inicio de sesión correcto.`,
            currValuesThreadsAccess: 'Threads (p_cost) a los que Rauthy tiene acceso',

            loginTimeHead: 'Una nota sobre el tiempo de inicio de sesión',
            loginTime1: `En general, los usuarios quieren que todo sea lo más rápido posible. Sin embargo, en un inicio
            de sesión seguro, un tiempo de entre 500 y 1000 ms no debería ser un problema. El tiempo de inicio de sesión
            no debe ser demasiado corto, ya que, lógicamente, reduciría la robustez del hash.`,
            loginTime2: `Para ofrecer la mayor seguridad posible de forma predeterminada, esta herramienta no permite
            bajar de 500 ms en el tiempo de inicio de sesión.`,

            mCost1: `El <code>m_cost</code> define la cantidad de <b>memoria (en kB)</b> que se usa para el hashing.
            Cuanto mayor sea el valor, mejor, lógicamente. Pero debe tener en cuenta los recursos del servidor.<br>
            Por ejemplo, si calcula el hash de 4 contraseñas a la vez, el backend necesita <code>4 x m_cost</code>
            durante el hashing. Estos recursos deben estar disponibles.`,
            mCost2: `Ajustar <code>m_cost</code> es bastante sencillo. Defina la cantidad máxima de memoria que debe
            usar Rauthy, divídala entre el número máximo de inicios de sesión en paralelo permitidos
            (<code>MAX_HASH_THREADS</code>) y reste una pequeña cantidad fija de memoria. La cantidad de memoria fija
            que hay que tener en cuenta depende de la base de datos utilizada y del número total de usuarios, pero
            suele estar entre 32 y 96 MB.`,
            mCost3: 'El valor mínimo permitido de <code>m_cost</code> es <code>32768</code>.',

            pCost1: `El <code>p_cost</code> define el grado de <b>paralelismo</b> del hashing. En la mayoría de los
            casos, este valor alcanza su máximo en ~8, que es el valor predeterminado de Rauthy.`,
            pCost2: `La regla general es:<br>
            Establezca el <code>p_cost</code> en el doble del número de núcleos disponibles.<br>
            Por ejemplo, si tiene 4 núcleos disponibles, establezca el <code>p_cost</code> en <code>8</code>.<br>
            Sin embargo, este valor debe tener en cuenta el número de inicios de sesión en paralelo permitidos
            (<code>MAX_HASH_THREADS</code>) y reducirse en consecuencia.`,

            tCost1: `El <code>t_cost</code> define la cantidad de <b>tiempo</b> del hashing. En realidad, es el único
            valor que hay que ajustar, ya que <code>m_cost</code> y <code>p_cost</code> vienen dados, básicamente,
            por el entorno.`,
            tCost2: `El ajuste es sencillo: establezca <code>m_cost</code> y <code>p_cost</code> de forma adecuada y,
            después, aumente <code>t_cost</code> hasta alcanzar el tiempo de hashing deseado.`,

            utilityHead: 'Herramienta de cálculo de parámetros',
            utility1: `Puede usar esta herramienta para calcular valores aproximados adecuados para su despliegue. Tenga
            en cuenta que debe ejecutarse con Rauthy en su ubicación definitiva y con todos los recursos finales
            disponibles. Debería ejecutar esta herramienta bajo carga para no sobredimensionar los valores.`,
            utility2: `<code>m_cost</code> es opcional y, si se deja vacío, se elegirá el valor mínimo seguro de
            <code>32768</code>. <code>p_cost</code> también es opcional y, si se deja vacío, Rauthy usará todos los
            threads que pueda detectar.`,

            time: 'Tiempo',
            targetTime: 'Tiempo objetivo',
            tune: 'Importante: ¡Estos valores deben ajustarse en la arquitectura final!',
            pDetails: `Si desea una introducción detallada a Argon2ID, existen muchas fuentes en internet. Esta guía
            solo ofrece una visión general muy breve de los valores. Hay que configurar tres de ellos:`,
            pTune: `Varían según las capacidades del sistema. Cuanto más potente sea el sistema, más seguros
            pueden ser estos valores.`,
            pUtility: `Esta herramienta le ayuda a encontrar la mejor configuración de Argon2ID para su plataforma.
            Argon2ID es actualmente el algoritmo de hashing de contraseñas más seguro disponible. Para aprovecharlo al
            máximo, debe ajustarse en cada despliegue.`,
        },
        openapi: 'Si desea integrar una aplicación externa y usar la API de Rauthy, consulte la',
        openapiNote: `En la configuración predeterminada, la Swagger UI no está activa a través del enlace anterior.
            Se puede activar con la configuración 'server.swagger_ui_enable' y, si procede, 'server.swagger_ui_public'.`,
        source: 'El código fuente está disponible aquí',
    },
    editor: {
        bold: 'Negrita',
        code: 'Código',
        heading1: 'Título 1',
        heading2: 'Título 2',
        heading3: 'Título 3',
        italic: 'Cursiva',
        link: 'Enlace',
        listBullet: 'Lista',
        listTasks: 'Tareas',
        listNumbered: 'Lista numerada',
        paragraph: 'Párrafo',
        quote: 'Cita',
        removeFmt: 'Quitar formato',
        strikeThrough: 'Tachado',
        textArea: 'Editar texto',
    },
    email: {
        cancelJob: 'Cancelar tarea',
        filterType: ['Ninguno', 'En el grupo', 'No en el grupo', 'Con el rol', 'Sin el rol'],
        immediate: 'Inmediato',
        jobs: 'Tareas de correo electrónico',
        scheduled: 'Envío programado',
        sendAllUsers: 'Este correo electrónico se enviará a todos los usuarios.',
        sendAllUsersFiltered:
            'Este correo electrónico se enviará a todos los usuarios filtrados por:',
        sendMail: 'Enviar correo electrónico',
        subject: 'Asunto',
        userFilter: 'Filtro de usuarios',
    },
    error: {
        needsAdminRole: `No tiene asignado el rol <b>rauthy_admin</b>.<br/>
            No tiene acceso al panel de administración.`,
        noAdmin: `Una cuenta de administrador de Rauthy debe tener la <b>MFA activada.</b><br>
            Vaya a su <b>cuenta</b> y active la MFA.<br>
            Después, debe cerrar sesión y volver a iniciarla.`,
    },
    events: {
        eventLevel: 'Nivel del evento',
        eventType: 'Tipo de evento',
    },
    groups: {
        delete1: '¿Seguro que desea eliminar este grupo?',
        name: 'Nombre del grupo',
    },
    jwks: {
        alg: 'Algoritmo',
        p1: 'Estas son las Json Web Keys (JWKs) que se usan para firmar los tokens.',
        p2: `De forma predeterminada, las JWKs se rotan el día 1 de cada mes. Para todos los tokens nuevos solo se
        usa la clave más reciente disponible para el algoritmo correspondiente. Las claves antiguas se conservan durante
        un tiempo para garantizar que los tokens válidos actualmente se puedan seguir validando correctamente. Pasado
        un tiempo, se eliminan automáticamente.`,
        p3: `Las claves también se pueden rotar manualmente. Según el hardware en el que se ejecute esta instancia de
        Rauthy, la operación puede tardar unos segundos.`,
        type: 'Tipo',
        rotateKeys: 'Rotar claves',
    },
    kv: {
        accessTestDesc: `La Access Key debe indicarse en la cabecera <code>Authorization</code> como
            token <code>Bearer</code>. Puede usar el siguiente comando <code>curl</code> para
            hacer pruebas.`,
        addNewKey: 'Nueva Access Key',
        addNewNs: 'Nuevo namespace',
        addNewValue: 'Nuevo valor',
        delConfirm: '¿Seguro que desea eliminar esta Access Key?',
        delNsMsg:
            '¿Seguro que desea eliminar este namespace, incluidos todos los datos existentes?',
        encryptedDesc: `Por motivos de rendimiento, el cifrado adicional solo debe usarse para
            valores especialmente sensibles, como access keys o datos personales.`,
        deleteConfirmMsg: `¿Seguro que desea eliminar la clave '{{ key }}'?`,
        help: {
            help: 'Ayuda',
            ops: [
                'Probar una Access Key',
                'Obtener todas las claves existentes',
                'Obtener todas las claves + valores existentes',
                'Establecer una clave + valor',
                'Obtener el valor de una clave',
                'Eliminar una clave',
            ],
            p1: `El acceso externo a los KV stores es muy sencillo a propósito. Solo hay unas
                pocas operaciones:`,
            p2: `Cada una de estas operaciones requiere una Access Key como token <code>Bearer</code>
                (<code>{id}\${secret}</code>) en la cabecera <code>Authorization</code>. Una Access
                Key solo es válida en el namespace en el que existe.`,
            p3: `Aquí verá ejemplos de las operaciones mencionadas usando
                <code>curl</code>.`,
        },
        key: 'Clave',
        loadAllValues: 'Cargar todos los valores',
        storeEncrypted: 'Guardar el valor cifrado',
        tabs: ['Datos', 'Acceso', 'Editar', 'Eliminar'],
        testCmd: 'Comando de prueba',
        value: 'Valor JSON',
    },
    nav: {
        apiKeys: 'API Keys',
        attributes: 'Atributos',
        blacklist: 'Lista de bloqueo',
        clients: 'Clients',
        config: 'Configuración',
        docs: 'Documentación',
        events: 'Eventos',
        groups: 'Grupos',
        providers: 'Proveedores',
        roles: 'Roles',
        scopes: 'Scopes',
        sessions: 'Sesiones',
        users: 'Usuarios',
    },
    options: {
        expires: 'Caduca',
        lastSeen: 'Visto por última vez',
        state: 'Estado',
    },
    pam: {
        addGroup: 'Nuevo grupo PAM',
        addHost: 'Nuevo host PAM',
        addUser: 'Nuevo usuario PAM',
        deleteHost: '¿Seguro que desea eliminar este host?',
        deleteUser: '¿Seguro que desea eliminar este usuario?',
        groupDescGeneric: `Los grupos Generic son el equivalente a las entradas que normalmente se encuentran en
            /etc/group. Se pueden asignar usuarios a estos grupos, y los NSS lookups los devuelven al sistema.`,
        groupDescHost: `Los grupos Host sirven para agrupar hosts. Los NSS lookups de un host dentro del grupo
            devuelven como resultado todos los demás hosts del grupo. Los usuarios pueden acceder a los hosts si se les
            asigna a un grupo Host.`,
        groupDescLocal: `Los grupos Local se comportan casi igual que los grupos Generic, con la diferencia de que
            tienen un ID en la base de datos de Rauthy, pero el proxy NSS del host correspondiente lo convierte en un
            ID de /etc/group. Así, los usuarios de Rauthy se pueden asignar a grupos que ya existen localmente.`,
        groupDescUser: `Los grupos User se gestionan automáticamente y están estrechamente vinculados al usuario con
            el mismo nombre de usuario.`,
        groupDescWheel: `Este grupo es especial. No se puede modificar y se asigna a los usuarios de forma dinámica,
            según su configuración de grupos.`,
        groupName: 'Nombre del grupo',
        groups: 'Grupos',
        groupType: 'Tipo de grupo',
        hostAliases: 'Alias del host',
        hostLocalPwdOnly: 'Inicio de sesión local con contraseña',
        hostLocalPwdOnlyInfo: `Si se establece el inicio de sesión local con contraseña, este prevalece sobre
            'Forzar MFA' en los inicios de sesión locales. Al mismo tiempo, nunca se solicitarán Passkeys (localmente)
            al iniciar sesión, aunque el usuario esté protegido con MFA. Esta opción solo debe establecerse si es
            realmente necesaria, por ejemplo si los usuarios protegidos con MFA deben poder iniciar sesión localmente
            sin usar Passkeys de hardware.`,
        ipAddresses: 'Direcciones IP',
        member: 'Miembro',
        nameExistsAlready: 'El nombre ya está en uso',
        notes: 'Notas',
        secretShow: 'Mostrar secret',
        secretRotate: 'Rotar secret',
        userEmail: 'Correo electrónico del usuario vinculado',
        username: 'Nombre de usuario',
        usernameNewDesc: `El nombre de usuario debe elegirse con cuidado. Una vez creado, no se puede cambiar
            fácilmente por motivos de seguridad.`,
    },
    passwordPolicy: {
        configDesc: 'Política para las nuevas contraseñas.',
        resetSet0: 'El valor 0 desactiva el requisito.',
        validForDays: 'Válida durante (días)',
        validityNew: 'Validez de las nuevas contraseñas.',
    },
    providers: {
        config: {
            allowInsecureTls: 'Permitir TLS no seguro',
            autoLink: 'Vincular usuario automáticamente',
            autoLinkDesc1: `Si se activa 'Vincular usuario automáticamente', el inicio de sesión mediante este
                proveedor vinculará automáticamente a este proveedor un usuario que ya exista y que aún no esté
                vinculado.`,
            autoLinkDesc2: `ATENCIÓN: Esta opción puede ser muy peligrosa y permitir el robo de cuentas si el
                proveedor no valida por completo las direcciones de correo electrónico de los usuarios y, por tanto,
                permite añadir a un usuario una dirección ajena. ¡NUNCA debe usarse en ese caso!`,
            clientName: 'Nombre del Client',
            custRootCa: 'Root CA PEM personalizada',
            descAuthMethod: `El método de autenticación que se usará en el endpoint <code>/token</code>.<br>
                La mayoría de los proveedores deberían funcionar con <code>basic</code>, algunos solo con
                <code>post</code>. En casos excepcionales se necesitan ambos, aunque esto puede provocar errores
                con otros proveedores.`,
            descClientId: 'Client ID proporcionado por el proveedor de autenticación.',
            descClientName:
                'Nombre del Client que se mostrará en la página de inicio de sesión de Rauthy.',
            descClientSecret: `Client Secret proporcionado por el proveedor de autenticación.
                Se necesita al menos un Client Secret o PKCE.`,
            descScope: `El scope que el Client debe usar al redirigir al inicio de sesión.
                Indique los valores separados por espacios.`,
            errNoAuthMethod:
                'Ha indicado un Client Secret, pero no hay ningún método de autenticación del Client activo',
            errConfidential: 'Debe ser al menos un Client confidencial o usar PKCE',
            jsonPath: {
                p1: 'Los valores del ID token se pueden asignar automáticamente tras un inicio de sesión upstream correcto.',
                p2: `El <code>path</code> debe indicarse con una sintaxis similar a regex. Puede apuntar a
                    valores JSON simples o a un valor dentro de un objeto o array JSON.`,
                p3: '<code>$.</code> marca el inicio del objeto JSON',
                p4: '<code>*</code> se puede usar como comodín en el path',
                p5: '<code>$.roles</code> apuntaría a <code>&#123;"roles": "value"&#125;</code>',
                p6: `<code>$.roles.*</code> puede apuntar a un valor dentro de un objeto o array como<br>
                    <code>&#123;"roles": ["value", "notMyValue"]&#125;</code>`,
            },
            lookup: 'Buscar',
            pathAdminClaim: 'Path de la claim de administrador',
            pathMfaClaim: 'Path de la claim de MFA',
            rootPemCert: 'Certificado PEM raíz',
            mapMfa: `Si su proveedor emite una claim que indica que el usuario ha usado al menos 2FA al
                iniciar sesión, puede indicar el path de la claim de MFA.`,
            mapUser: `Puede convertir a un usuario en administrador de Rauthy en función de una claim upstream del ID token.`,
            valueAdminClaim: 'Valor de la claim de administrador',
            valueMfaClaim: 'Valor de la claim de MFA',
        },
        delete: {
            areYouSure: '¿Seguro que desea eliminar este proveedor?',
            forceDelete: 'Forzar eliminación',
            isInUse1: '¡Este proveedor lo están usando usuarios activos!',
            isInUse2: `Puede forzar la eliminación, pero los usuarios sin contraseña local ni Passkey
                ya no podrán iniciar sesión.`,
            linkedUsers: 'Usuarios vinculados',
        },
    },
    roles: {
        adminNoMod: 'El rol <code>rauthy_admin</code> no se puede modificar.',
        delete1: '¿Seguro que desea eliminar este rol?',
        name: 'Nombre del rol',
    },
    scopes: {
        claimsAtRoot: 'Emitir claims en la raíz del token',
        claimsAtRootWarning: `Los atributos asignados de este scope se escriben en la raíz del token,
            en lugar de agruparse bajo 'custom'. Usted es responsable de evitar colisiones: si el nombre de un
            atributo asignado coincide con una claim JWT reservada, la emisión del token falla. Las claims
            personalizadas en la raíz también pueden dejar de funcionar con futuros cambios del protocolo o de
            funciones. Referencia:`,
        defaultNoMod: 'Este es un Scope OIDC predeterminado. No se pueden modificar.',
        delete1: '¿Seguro que desea eliminar este scope?',
        deleteDefault: 'Los scopes OIDC predeterminados no se pueden eliminar.',
        mapping1: 'Puede asignar scopes personalizados a atributos.',
        mapping2: `Todos los atributos adicionales configurados pueden tener un valor personalizado para cada
            usuario. Cuando se asignan a un scope, pueden incluirse en los Access tokens y / o ID tokens.`,
        name: 'Nombre del scope',
    },
    search: {
        orderBy: 'Ordenar por ...',
        orderChangeToAsc: 'Cambiar a orden ascendente',
        orderChangeToDesc: 'Cambiar a orden descendente',
    },
    sessions: {
        invalidateAll: 'Invalidar todas las sesiones',
    },
    tabs: {
        config: 'Configuración',
        delete: 'Eliminar',
    },
    tos: {
        accepted: 'Aceptados',
        addNewToS: 'Nuevos términos',
        addNewToSFromCurrent: 'Nuevos términos a partir de los seleccionados',
        added: 'Añadidos',
        checkStatus: 'Comprobar el estado del usuario',
        immutable: `ATENCIÓN: ¡Una vez añadidos, los nuevos Términos del servicio no se pueden modificar ni
				eliminar!`,
        noneExist: 'Todavía no se han añadido Términos del servicio.',
        optUntil: {
            desc: `Durante el período de transición, aceptar los términos actualizados es opcional. Solo pasa a
					ser obligatorio después.`,
            enable: 'Activar período de transición',
            label: 'Fin del período de transición',
        },
        tos: 'Términos',
    },
    users: {
        antiLockout: {
            rule: 'Regla antibloqueo',
            delete: 'no se puede eliminar',
            disable: 'no se puede desactivar',
            rauthyAdmin: 'el rol rauthy_admin no se puede quitar',
        },
        attributes: 'Atributos',
        deleteUser: '¿Seguro que desea eliminar este usuario?',
        descAttr: `Establezca atributos de usuario personalizados. Todos los pares clave / valor se tratarán como
            String / valor JSON.`,
        forceLogout: `¿Seguro que desea invalidar todas las sesiones existentes y eliminar todos los refresh tokens
            de este usuario?`,
        groupAdmin: {
            notManagedTitle: 'Usuario fuera de sus grupos',
            notManagedDesc: `Este usuario no pertenece a ninguno de los grupos que usted gestiona, por lo que los
                detalles permanecen ocultos. Añada el usuario a uno o varios de sus grupos para gestionarlo.
                Se conservan las pertenencias a grupos ajenos a los suyos.`,
            addToGroups: 'Añadir a mis grupos',
        },
        lastLogin: 'Último inicio de sesión',
        manualInitDesc: `El usuario también se puede inicializar aquí. En ese caso, sin embargo, debe comunicarle
            la contraseña directamente.`,
        manualInit: 'Inicialización manual',
        mfa: {
            otp: {
                title: 'Código de un solo uso',
                mfaDelete1: 'Puede eliminar los OTP de este usuario.',
                mfaDelete2: `¡Atención! La eliminación de un OTP <b>no se puede deshacer</b> sin que el usuario
                    haga un registro completamente nuevo.`,
                noMfaOtps: 'Este usuario no tiene OTP registrados.',
            },
            webauthn: {
                title: 'Passkeys',
                mfaDelete1: 'Puede eliminar las Passkeys de este usuario.',
                mfaDelete2: `¡Atención! La eliminación de una Passkey <b>no se puede deshacer</b> sin que el
                    usuario haga un registro completamente nuevo.`,
                noMfaKeys: 'Este usuario no tiene Passkeys registradas.',
            },
        },
        pkOnly1: 'Esta es una cuenta solo con Passkey.',
        pkOnly2:
            'Esto significa que este usuario inicia sesión sin contraseña y no tiene ninguna contraseña establecida.',
        pkOnly3: `Si este usuario ha perdido todas sus Passkeys, la cuenta se puede restablecer por completo y se
            puede enviar un nuevo correo electrónico de restablecimiento de contraseña. Para ello, vaya a la pestaña
            'MFA' y elimine todas las Passkeys existentes.`,
        pwdNoInit:
            'El usuario todavía no ha realizado el restablecimiento inicial de la contraseña.',
        pwdSendEmailBtn: 'Enviar correo de restablecimiento',
        pwdSendEmailDesc:
            'Puede enviar un nuevo correo de restablecimiento si el usuario no ha recibido ninguno.',
        savePassword: 'Guardar contraseña',
        selfServiceDesc:
            'Puede establecer una nueva contraseña o enviar un correo de restablecimiento.',
        sendResetEmail: 'Enviar correo de restablecimiento',
    },
    validation: {
        css: 'Valor CSS no válido',
        origin: 'Origin no válida',
        uri: 'URI no válido',
        redirectUri: {
            comma: 'Un Redirect URI no puede contener una coma (,)',
            controlChar:
                'Un Redirect URI no puede contener caracteres de control en el nombre de un parámetro de consulta',
            fragment: 'Un Redirect URI no puede contener un fragmento (#)',
            reservedKey:
                'Un Redirect URI no puede contener el parámetro de consulta "{{ KEY }}", porque lo establece la respuesta de autorización',
            reservedKeyLogout:
                'Un Post Logout Redirect URI no puede contener el parámetro de consulta "{{ KEY }}", porque se establece al cerrar sesión',
        },
    },
};
