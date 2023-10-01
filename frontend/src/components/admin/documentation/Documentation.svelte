<script>
</script>

<div class="content">
    <div class="container">
        <h2>Documentation</h2>

        <p>
            For general documentation about Rauthy itself, you should take a look at the
            <a href="https://sebadob.github.io/rauthy/" target="_blank">Rauthy Book</a>
        </p>

        <p>
            If you want to integrate an external application and use Rauthy's REST API, take a look at the
            <a href="/docs/v1/swagger-ui/" target="_blank">Swagger UI</a><br/>

            <b>Note:</b>
            Depending on the backend configuration, the Swagger UI may not be exposed publicly at this point.
            It is however by default available via the internal <code>/metrics</code> HTTP server to not expose any information.
        </p>

        <p>
            The source code can be found in
            <a href="https://github.com/sebadob/rauthy" target="_blank">Github</a>
        </p>
    </div>
</div>

<style>
    a {
        text-decoration: none;
        color: var(--col-act2);
        font-weight: bold;
    }

    a:hover {
        color: var(--col-act2a);
    }

    p {
        margin: .5rem 0;
    }

    .container {
        display: flex;
        flex-direction: column;
        width: 100%;
        max-width: 1280px;
        max-height: 90vh;
        overflow-y: auto;
    }
</style>
