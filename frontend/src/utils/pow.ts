import {fetchPost} from "$api/fetch.ts";

export async function fetchSolvePow() {
    let res = await fetchPost<string>('/auth/v1/pow');
    if (res.text) {
        let challenge = res.text;
        // TODO solve this issue - static import results in failing pre-rendering
        let pow_work_wasm = (await import('../spow/spow-wasm')).pow_work_wasm;
        return await pow_work_wasm(challenge) || '';
    } else {
        console.error(res.error);
    }
}