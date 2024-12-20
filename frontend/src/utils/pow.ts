import {getPow} from "./dataFetching";

export async function fetchSolvePow() {
    let res = await getPow();

    if (res.ok) {
        let challenge = await res.text();
        // TODO solve this issue - static import results in failing pre-rendering
        let pow_work_wasm = (await import('../spow/spow-wasm')).pow_work_wasm;
        return await pow_work_wasm(challenge) || '';
    } else {
        let err = await res.json();
        console.error(err);
    }
}