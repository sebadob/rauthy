<script>
    import {onMount} from "svelte";
    import Loading from "../Loading.svelte";

    export let req = {};
    let err = '';
    let success = false;

    onMount(async () => {
        const evtSource = new EventSource(`${req.url}/${req.req_id}/${req.code}`);

        evtSource.onmessage = (event) => {

            let payload = event.data;
            if (payload.includes('ACK')) {
                if (err) {
                    err = '';
                }

                let json = event.data.split('ACK')[1].trim();
                const res = JSON.parse(json);
                if (req.req_id === res.req_id) {
                    success = true;
                    window.location.href = res.loc;
                } else {
                    err = 'MFA Request ACK was poisoned - this must never happen';
                }
            } else if (payload.includes('REJECTED')) {
                err = 'The MFA request was rejected';
            }
            evtSource.close();
        }

        evtSource.onerror = (event) => {
            console.error(event);
            err = 'Internal Server Error - Please try again';
            setTimeout(() => {
                req = undefined;
            }, 3000);
        }
    })

</script>

<div class="wrapperOuter">
    <div class="wrapperInner">
        <div class="content">

            <div class="contentRow">
                <div class="contentHeader">
                    Awaiting MFA request confirmation
                </div>
            </div>

            <div class="contentRow">
                <div class="contentHeader">
                    Request ID:
                </div>
                <div>
                    {req.req_id}
                </div>
            </div>

            <div class="contentRow">
                <div class="contentHeader">
                    Request expires:
                </div>
                <div>
                    {new Date(req.exp).toString()}
                </div>
            </div>

            <div class="contentRow">
                {#if success}
                    <div class="good">
                        Acknowledged
                    </div>
                {:else if err}
                    <div class="err">
                        {err}
                    </div>
                {:else}
                    <Loading background={false} offset="170"/>
                {/if}
            </div>
        </div>

    </div>
</div>

<style>
    .content {
        width: 350px;
        height: 220px;
        border: 1px solid var(--col-acnt);
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        color: white;
        text-align: center;
        z-index: 999;
    }

    .contentRow {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        margin: .25em;
    }

    .contentHeader {
        margin-bottom: 0.2em;
        font-weight: bold;
    }

    .err {
        color: var(--col-err);
    }

    .good {
        color: var(--col-ok);
    }

    .wrapperOuter {
        position: absolute;
        top: 0;
        left: 0;
    }

    .wrapperInner {
        width: 100vw;
        height: 100vh;
        position: relative;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        background: rgba(0, 0, 0, .95);
        z-index: 199;
    }
</style>
