export const PKCE_VERIFIER = 'pkce_verifier';
export const CSRF_TOKEN = 'csrf_token';
export const ACCESS_TOKEN = 'access_token';
export const ID_TOKEN = 'id_token';

export const AUTH_ENDPOINT = '/auth/v1/oidc/authorize';
export const CLIENT_ID = 'rauthy';
export const LOGOUT_URL = '/auth/v1/oidc/logout';
export const REDIRECT_URI = '/auth/v1/oidc/callback';
export const REDIRECT_URI_SUCCESS = '/auth/v1/admin';
export const REDIRECT_URI_SUCCESS_ACC = '/auth/v1/account';
export const POST_LOGOUT_REDIRECT_URI = '/auth/v1/';

export const REGEX_LOWERCASE = /^[a-z0-9-_/]{2,128}$/gm;
export const REGEX_NAME = /^[\w\sÀ-ÿ\-]{0,32}$/gm;
export const REGEX_ATTR_DESC = /^[a-zA-Z0-9\-_/\s]{0,128}$/gm;
export const REGEX_ATTR_KEY = /^[a-zA-Z0-9\-_/]{2,32}$/gm;
export const REGEX_CLIENT_ID = /^[a-zA-Z0-9\-_/]{2,128}$/gm;
export const REGEX_CLIENT_NAME = /^[a-zA-Z0-9À-ÿ\-\s]{0,128}$/gm;
export const REGEX_ROLES = /^[a-z0-9\-_/]{2,128}$/gm;
export const REGEX_URI = /^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]+$/gm;
export const REGEX_IP_V4 = /^(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)(?:\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)){3}$/gm;

// https://gist.github.com/olmokramer/82ccce673f86db7cda5e
export const REGEX_CSS_COLOR = /(#(?:[0-9a-f]{2}){2,4}$|(#[0-9a-f]{3}$)|(rgb|hsl)a?\((-?\d+%?[,\s]+){2,3}\s*[\d\.]+%?\)$|black$|silver$|gray$|whitesmoke$|maroon$|red$|purple$|fuchsia$|green$|lime$|olivedrab$|yellow$|navy$|blue$|teal$|aquamarine$|orange$|aliceblue$|antiquewhite$|aqua$|azure$|beige$|bisque$|blanchedalmond$|blueviolet$|brown$|burlywood$|cadetblue$|chartreuse$|chocolate$|coral$|cornflowerblue$|cornsilk$|crimson$|currentcolor$|darkblue$|darkcyan$|darkgoldenrod$|darkgray$|darkgreen$|darkgrey$|darkkhaki$|darkmagenta$|darkolivegreen$|darkorange$|darkorchid$|darkred$|darksalmon$|darkseagreen$|darkslateblue$|darkslategray$|darkslategrey$|darkturquoise$|darkviolet$|deeppink$|deepskyblue$|dimgray$|dimgrey$|dodgerblue$|firebrick$|floralwhite$|forestgreen$|gainsboro$|ghostwhite$|goldenrod$|gold$|greenyellow$|grey$|honeydew$|hotpink$|indianred$|indigo$|ivory$|khaki$|lavenderblush$|lavender$|lawngreen$|lemonchiffon$|lightblue$|lightcoral$|lightcyan$|lightgoldenrodyellow$|lightgray$|lightgreen$|lightgrey$|lightpink$|lightsalmon$|lightseagreen$|lightskyblue$|lightslategray$|lightslategrey$|lightsteelblue$|lightyellow$|limegreen$|linen$|mediumaquamarine$|mediumblue$|mediumorchid$|mediumpurple$|mediumseagreen$|mediumslateblue$|mediumspringgreen$|mediumturquoise$|mediumvioletred$|midnightblue$|mintcream$|mistyrose$|moccasin$|navajowhite$|oldlace$|olive$|orangered$|orchid$|palegoldenrod$|palegreen$|paleturquoise$|palevioletred$|papayawhip$|peachpuff$|peru$|pink$|plum$|powderblue$|rosybrown$|royalblue$|saddlebrown$|salmon$|sandybrown$|seagreen$|seashell$|sienna$|skyblue$|slateblue$|slategray$|slategrey$|snow$|springgreen$|steelblue$|tan$|thistle$|tomato$|transparent$|turquoise$|violet$|wheat$|white$|yellowgreen$|rebeccapurple$)/i;


export const EVENT_LEVELS = [
	'Info',
	'Notice',
	'Warning',
	'Critical'
]
export const LANGUAGES = ['DE', 'EN'];
export const TOKEN_ALGS = [
	'RS256',
	'RS384',
	'RS512',
	'EdDSA'
]

export const FLOWS = [{
		label: 'authorization_code',
		value: false,
	},
	{
		label: 'client_credentials',
		value: false,
	},
	{
		label: 'password',
		value: false,
	},
	{
		label: 'refresh_token',
		value: false,
	},
];

export const PKCE_CHALLENGES = [{
		label: 'S256',
		value: false,
	},
	{
		label: 'plain',
		value: false,
	},
];
