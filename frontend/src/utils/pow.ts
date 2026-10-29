import { fetchPost } from '#api/fetch.ts';

export async function fetchSolvePow(): Promise<string | undefined> {
    // spawn the worker in advance so it can async load the wasm already
    let worker = new Worker(new URL('../workers/pow.ts', import.meta.url), { type: 'module' });

    let res = await fetchPost<string>('/auth/v1/pow');
    if (res.text) {
        let challenge = res.text;
        return await new Promise(resolve => {
            if (!worker) {
                worker = new Worker(new URL('../workers/pow.ts', import.meta.url), {
                    type: 'module',
                });
            }

            worker.onmessage = ev => {
                resolve(ev.data as string);
            };
            worker.postMessage(challenge);
        });
    } else {
        console.error(res.error);
    }
}
