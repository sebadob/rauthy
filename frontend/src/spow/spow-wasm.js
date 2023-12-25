import * as wasm from "./spow-wasm_bg.wasm";
import { __wbg_set_wasm } from "./spow-wasm_bg.js";
__wbg_set_wasm(wasm);
export * from "./spow-wasm_bg.js";
