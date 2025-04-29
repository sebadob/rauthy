import * as wasm from "./spow-wasm_bg.wasm";
export * from "./spow-wasm_bg.js";
import { __wbg_set_wasm } from "./spow-wasm_bg.js";
__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
