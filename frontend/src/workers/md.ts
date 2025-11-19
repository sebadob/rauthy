export interface MDWorkerReq {
	typ: 'render' | 'sanitize' | 'close';
	data: string;
}

export interface MDWorkerResp {
	typ: 'render' | 'sanitize' | 'closed';
	data: string;
}

const MD = import('../wasm/md/md');

onmessage = (ev) => {
	MD.then((wasm) => {
		let req = ev.data as MDWorkerReq;

		switch (req.typ) {
			case 'render':
				let r: MDWorkerResp = {
					typ: 'render',
					data: wasm.render_markdown(req.data)
				};
				postMessage(r);
				break;
			case 'sanitize':
				let s: MDWorkerResp = {
					typ: 'sanitize',
					data: wasm.sanitize_html(req.data)
				};
				postMessage(s);
				break;
			case 'close':
				let c: MDWorkerResp = {
					typ: 'closed',
					data: ''
				};
				postMessage(c);
				console.log('Closing Markdown Worker');
				self.close();
		}
	});
};

onclose = () => {
	console.log('Closing Markdown Worker');
};

console.log('Markdown Worker started');
