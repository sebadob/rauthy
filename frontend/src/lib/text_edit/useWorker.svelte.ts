import type { MDWorkerReq, MDWorkerResp } from '../../workers/md';

let worker: undefined | Worker = $state();
let renderedMarkdown = $state('');
let sanitizedHTML = $state('');

export function useMarkdownWorker() {
    if (!worker) {
        worker = new Worker(new URL('../../workers/md.ts', import.meta.url));
    }

    worker.onmessage = ev => {
        let res = ev.data as MDWorkerResp;

        if (res.typ === 'render') {
            renderedMarkdown = res.data;
        } else if (res.typ === 'sanitize') {
            sanitizedHTML = res.data;
        } else if (res.typ === 'closed') {
            worker = undefined;
        }
    };

    return {
        renderMarkdown(markdown: string) {
            let payload: MDWorkerReq = {
                typ: 'render',
                data: markdown,
            };
            worker?.postMessage(payload);
        },
        sanitizeHTML(html: string) {
            let payload: MDWorkerReq = {
                typ: 'sanitize',
                data: html,
            };
            worker?.postMessage(payload);
        },
        closeWorker() {
            let payload: MDWorkerReq = {
                typ: 'close',
                data: '',
            };
            worker?.postMessage(payload);
        },
        renderedMarkdown(): string {
            return renderedMarkdown;
        },
        sanitizedHTML(): string {
            return sanitizedHTML;
        },
    };
}
