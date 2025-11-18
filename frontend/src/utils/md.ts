import type { MDWorkerReq, MDWorkerResp } from '../workers/md';

let worker: undefined | Worker = new Worker(new URL('../workers/md.ts', import.meta.url));

export async function renderMarkdown(data: string): Promise<string | undefined> {
	return await new Promise((resolve) => {
		if (!worker) {
			worker = new Worker(new URL('../workers/md.ts', import.meta.url));
		}

		worker.onmessage = (ev) => {
			let res = ev.data as MDWorkerResp;
			console.log('md worker message', res);
			resolve(res.data);
		};

		let payload: MDWorkerReq = {
			typ: 'render',
			data
		};
		worker.postMessage(payload);
	});
}

export async function sanitizeHTML(data: string): Promise<string | undefined> {
	return await new Promise((resolve) => {
		if (!worker) {
			worker = new Worker(new URL('../workers/md.ts', import.meta.url));
		}

		worker.onmessage = (ev) => {
			let res = ev.data as MDWorkerResp;
			console.log('md worker message', res);
			resolve(res.data);
		};

		let payload: MDWorkerReq = {
			typ: 'sanitize',
			data
		};
		worker.postMessage(payload);
	});
}

export function closeMDWorker() {
	let payload: MDWorkerReq = {
		typ: 'close',
		data: ''
	};
	worker?.postMessage(payload);
}
