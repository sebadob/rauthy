const CSRF_TOKEN = 'session_xsrf';

export function getCookie(cname) {
    let name = cname + "=";
    let decodedCookie = decodeURIComponent(document.cookie);
    let ca = decodedCookie.split(';');
    for (let i = 0; i < ca.length; i++) {
        let c = ca[i];
        while (c.charAt(0) === ' ') {
            c = c.substring(1);
        }
        if (c.indexOf(name) === 0) {
            return c.substring(name.length, c.length);
        }
    }
    return "";
}

function deleteCookie(name) {
    document.cookie = name + '=; Max-Age=-1;';
}

// Returns a short random key, which can be used in components to identify them uniquely.
export const getKey = (i) => {
    let res = '';

    const target = i || 8;
    for (let i = 0; i < target; i += 1) {
        let nextNumber = 60;
        while ((nextNumber > 57 && nextNumber < 65) || (nextNumber > 90 && nextNumber < 97)) {
            nextNumber = Math.floor(Math.random() * 74) + 48;
        }
        res = res.concat(String.fromCharCode(nextNumber));
    }

    return res;
}

// async sleep in ms
export const sleepAwait = async (ms) => await new Promise(x => setTimeout(x, ms));

export const deleteCsrfToken = () => {
    localStorage.removeItem(CSRF_TOKEN);
}

export const saveCsrfToken = (csrf) => {
    localStorage.setItem(CSRF_TOKEN, csrf);
}

export const getCsrfToken = () => {
    let token = getCookie(CSRF_TOKEN);
    if (token) {
        saveCsrfToken(token);
        deleteCookie(CSRF_TOKEN);
        return token;
    }

    return localStorage.getItem(CSRF_TOKEN) || '';
}

export function requestHeaders() {
    return {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
        // TOOD change to session with lifetime observation
        'X-XSRF': getCsrfToken(),
    }
    // 'Authorization': `Bearer ${getAccessToken()}`
}
