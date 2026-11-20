import { onDestroy } from 'svelte';
import type { MDWorkerReq, MDWorkerResp } from '../../workers/md';

export function useMarkdownWorker() {
    let worker: Worker | undefined;
    let renderedMarkdown = $state('');
    let sanitizedHTML = $state('');

    function post(payload: MDWorkerReq) {
        if (!worker) {
            worker = new Worker(new URL('../../workers/md.ts', import.meta.url), {
                type: 'module',
            });
            worker.onmessage = ({ data }: MessageEvent<MDWorkerResp>) => {
                if (data.typ === 'render') {
                    renderedMarkdown = data.data;
                } else if (data.typ === 'sanitize') {
                    sanitizedHTML = data.data;
                }
            };
        }
        worker.postMessage(payload);
    }

    onDestroy(() => worker?.terminate());

    return {
        renderMarkdown(markdown: string) {
            post({ typ: 'render', data: markdown });
        },
        sanitizeHTML(html: string) {
            post({ typ: 'sanitize', data: html });
        },
        renderedMarkdown: () => renderedMarkdown,
        sanitizedHTML: () => sanitizedHTML,
    };
}
