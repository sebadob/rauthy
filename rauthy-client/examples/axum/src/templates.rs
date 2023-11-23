pub const HTML_INDEX: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Rauthy Axum Example</title>
</head>
<body>
    <p>Logged In: <span id="status"></span></p>
    <div id="btn"></div>
    <p>Answer from protected resource:</p>
    <p id="content"></p>
</body>
<script>
    let status = document.getElementById('status');
    let content = document.getElementById('content');
    let btn = document.getElementById('btn');

    // this token will be saved by the callback endpoint
    const token = localStorage.getItem('access_token');

    fetch('/auth_check', {
        method: 'GET',
        headers: { 'Authorization': `Bearer ${token}` },
    }).then(res => {
        if (res.status === 202) {
            status.innerText = 'Yes';
            btn.innerHTML = '<button onclick="logout()">Logout</button>';
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
            res.text().then(text => content.innerHTML = text);
        } else {
            content.innerText = `Status: ${res.status}`;
        }
    });

    // This is not really a good logout, just for demonstration here
    function logout() {
        localStorage.removeItem('access_token');
        window.location.reload();
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
    <template id="token">{{ TOKEN }}</template>
    <template id="uri">{{ URI }}</template>
</body>
<script>
    // This is an example how we could get an XSRF token from
    // the callback endpoint easily without doing some sketchy
    // stuff like setting it with an additional cookie for instance.
    // As long as the redirect URI has the same origin, we will be
    // able to access the localStorage from the main UI later.
    let token = document.querySelector('#token');
    localStorage.setItem('access_token', token.content.textContent);

    // When we want to manually extract an additional csrf token or
    // other information instead of returning a 302 status, we need
    // to do the redirect manually as well.
    let redirectUri = document.querySelector('#uri');
    window.location.replace(redirectUri.content.textContent);
</script>
</html>
"#;
