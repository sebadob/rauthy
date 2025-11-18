import type { MDWorkerReq, MDWorkerResp } from '../../workers/md';

let worker: undefined | Worker = $state();
let renderedMarkdown = $state('');
let sanitizedHTML = $state('');

export function useMarkdownWorker() {
	if (!worker) {
		// console.log('initializing MD Worker');
		worker = new Worker(new URL('../../workers/md.ts', import.meta.url));
	}

	worker.onmessage = (ev) => {
		let res = ev.data as MDWorkerResp;
		// console.log('md worker message', res);

		if (res.typ === 'render') {
			renderedMarkdown = res.data;
		} else if (res.typ === 'sanitize') {
			sanitizedHTML = res.data;
		}
	};

	return {
		renderMarkdown(markdown: string) {
			let payload: MDWorkerReq = {
				typ: 'render',
				data: markdown
			};
			worker?.postMessage(payload);
		},
		sanitizeHTML(html: string) {
			let payload: MDWorkerReq = {
				typ: 'sanitize',
				data: html
			};
			worker?.postMessage(payload);
		},
		closeWorker() {
			let payload: MDWorkerReq = {
				typ: 'close',
				data: ''
			};
			worker?.postMessage(payload);
		},
		renderedMarkdown(): string {
			return renderedMarkdown;
		},
		sanitizedHTML(): string {
			return sanitizedHTML;
		}
	};
}
