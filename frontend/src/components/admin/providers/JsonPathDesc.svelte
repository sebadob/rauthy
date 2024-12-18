<script></script>

<div class="desc">
    <h4>ID token claim mappings</h4>
    <p>
        Values from the ID token after a successful upstream login can be mapped automatically.
    </p>
    <p>
        The <code>path</code> needs to be given in a regex like syntax. It can resolve to
        single JSON values or resolve to a value in a JSON object or array.
    </p>
    <p>
        <code>$.</code> marks the start of the JSON object
    </p>
    <p>
        <code>*</code> can be used as a wildcard in your path
    </p>
    <p>
        <code>$.roles</code> would target <code>&#123;"roles": "value"&#125;</code>
    </p>
    <p>
        <code>$.roles.*</code> can target a value inside an object or array like<br>
        <code>&#123;"roles": ["value", "notMyValue"]&#125;</code>
    </p>
</div>

<style>
    h4 {
        margin-bottom: .5rem;
    }

    .desc {
        display: flex;
        flex-direction: column;
        margin: .5rem;
    }

    .desc > p {
        margin: .2rem 0;
    }

    .desc > p > code {
        background: var(--col-gmid);
    }
</style>
