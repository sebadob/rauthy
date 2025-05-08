pub const HTML_INDEX: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Rauthy Axum Example</title>
</head>
<body>
    <template id="logoutUri">{{ LOGOUT_URI }}</template>
    <p>Logged In: <span id="status"></span></p>
    <div id="btn"></div>
    <p>Answer from Token-Protected resource:</p>
    <p id="content-token"></p>
    <p>Answer from Session-Protected resource:</p>
    <p id="content-session"></p>
</body>
<script>
    let status = document.getElementById('status');
    let contentToken = document.getElementById('content-token');
    let contentSession = document.getElementById('content-session');
    let btn = document.getElementById('btn');

    // this token will be saved by the callback endpoint
    const token = localStorage.getItem('access_token');

    fetch('/auth_check', {
        method: 'GET',
        headers: { 'Authorization': `Bearer ${token}` },
    }).then(res => {
        if (res.status === 202) {
            status.innerText = 'Yes';
            btn.innerHTML = '<p>Logout will only work with Backchannel Logout properly set up:</p><button onclick="logout()">Logout</button>';
        } else {
            status.innerText = 'No';
            let loc = res.headers.get('location');
            btn.innerHTML = `<a href=${loc}>Login</a>`;
        }
    });

    fetch('/protected', {
        method: 'GET',
        headers: {'Authorization': `Bearer ${token}`},
    }).then(res => {
        if (res.ok) {
            res.text().then(text => contentToken.innerHTML = text);
        } else {
            contentToken.innerText = `Status: ${res.status}`;
        }
    });

    fetch('/session').then(res => {
        if (res.ok) {
            res.text().then(text => contentSession.innerHTML = text);
        } else {
            contentSession.innerText = `Status: ${res.status}`;
        }
    });

    // This is not really a good logout, just for demonstration here
    function logout() {
        localStorage.removeItem('access_token');
        
        let uri = document.querySelector('#logoutUri');
        let id_token = localStorage.getItem('id_token');
        window.location.href = `${uri.innerText}?id_token_hint=${id_token}&post_logout_redirect_uri=${window.location.origin}`;
    }
</script>
</html>
"#;

pub const HTML_CALLBACK: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Rauthy Axum Callback</title>
</head>
<body>
</body>
<script>
    // This is an example how we could get an XSRF token from
    // the callback endpoint easily without doing some sketchy
    // stuff like setting it with an additional cookie for instance.
    // As long as the redirect URI has the same origin, we will be
    // able to access the localStorage from the main UI later.
    localStorage.setItem('access_token', '{{ ACCESS_TOKEN }}');
    localStorage.setItem('id_token', '{{ ID_TOKEN }}');

    // When we want to manually extract an additional csrf token or
    // other information instead of returning a 302 status, we need
    // to do the redirect manually as well.
    window.location.replace('{{ URI }}');
</script>
</html>
"#;
