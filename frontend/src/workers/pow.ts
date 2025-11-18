let wasm = import('../wasm/spow/spow');

onmessage = (ev) => {
	wasm.then((wasm) => {
		postMessage(wasm.pow_work_wasm(ev.data));
		self.close();
	});
};

onclose = () => {
	console.log('PoW WebWorker closed');
};
