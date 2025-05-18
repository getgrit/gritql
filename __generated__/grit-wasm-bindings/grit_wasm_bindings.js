import { initialize_tree_sitter } from 'web-tree-sitter-wasm-bindgen';

let wasm;

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let WASM_VECTOR_LEN = 0;

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => {
    wasm.__wbindgen_export_2.get(state.dtor)(state.a, state.b)
});

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);
                CLOSURE_DTORS.unregister(state);
            } else {
                state.a = a;
            }
        }
    };
    real.original = state;
    CLOSURE_DTORS.register(real, state, state);
    return real;
}
function __wbg_adapter_32(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h514cdf5dd6f00f9c(arg0, arg1, addHeapObject(arg2));
}

/**
* @returns {Promise<void>}
*/
export function initializeTreeSitter() {
    const ret = wasm.initializeTreeSitter();
    return takeObject(ret);
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

let cachedUint32Memory0 = null;

function getUint32Memory0() {
    if (cachedUint32Memory0 === null || cachedUint32Memory0.byteLength === 0) {
        cachedUint32Memory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32Memory0;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getUint32Memory0();
    const slice = mem.subarray(ptr / 4, ptr / 4 + len);
    const result = [];
    for (let i = 0; i < slice.length; i++) {
        result.push(takeObject(slice[i]));
    }
    return result;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    const mem = getUint32Memory0();
    for (let i = 0; i < array.length; i++) {
        mem[ptr / 4 + i] = addHeapObject(array[i]);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}
/**
* @param {string} pattern
* @param {(string)[]} paths
* @param {(string)[]} contents
* @param {(string)[]} lib_paths
* @param {(string)[]} lib_contents
* @returns {Promise<any>}
*/
export function parseInputFiles(pattern, paths, contents, lib_paths, lib_contents) {
    const ptr0 = passStringToWasm0(pattern, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayJsValueToWasm0(paths, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passArrayJsValueToWasm0(contents, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    const ptr3 = passArrayJsValueToWasm0(lib_paths, wasm.__wbindgen_malloc);
    const len3 = WASM_VECTOR_LEN;
    const ptr4 = passArrayJsValueToWasm0(lib_contents, wasm.__wbindgen_malloc);
    const len4 = WASM_VECTOR_LEN;
    const ret = wasm.parseInputFiles(ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3, ptr4, len4);
    return takeObject(ret);
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}
/**
* @param {UncompiledPatternBuilder} pattern
* @param {(string)[]} paths
* @param {(string)[]} contents
* @param {(string)[]} lib_paths
* @param {(string)[]} lib_contents
* @param {string} llm_api_base
* @param {string} llm_api_bearer_token
* @returns {Promise<any>}
*/
export function runPattern(pattern, paths, contents, lib_paths, lib_contents, llm_api_base, llm_api_bearer_token) {
    _assertClass(pattern, UncompiledPatternBuilder);
    var ptr0 = pattern.__destroy_into_raw();
    const ptr1 = passArrayJsValueToWasm0(paths, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passArrayJsValueToWasm0(contents, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    const ptr3 = passArrayJsValueToWasm0(lib_paths, wasm.__wbindgen_malloc);
    const len3 = WASM_VECTOR_LEN;
    const ptr4 = passArrayJsValueToWasm0(lib_contents, wasm.__wbindgen_malloc);
    const len4 = WASM_VECTOR_LEN;
    const ptr5 = passStringToWasm0(llm_api_base, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len5 = WASM_VECTOR_LEN;
    const ptr6 = passStringToWasm0(llm_api_bearer_token, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len6 = WASM_VECTOR_LEN;
    const ret = wasm.runPattern(ptr0, ptr1, len1, ptr2, len2, ptr3, len3, ptr4, len4, ptr5, len5, ptr6, len6);
    return takeObject(ret);
}

/**
* @param {string} pattern
* @param {(string)[]} paths
* @param {(string)[]} contents
* @param {(string)[]} lib_paths
* @param {(string)[]} lib_contents
* @param {string} llm_api_base
* @param {string} llm_api_bearer_token
* @returns {Promise<any>}
*/
export function matchPattern(pattern, paths, contents, lib_paths, lib_contents, llm_api_base, llm_api_bearer_token) {
    const ptr0 = passStringToWasm0(pattern, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayJsValueToWasm0(paths, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passArrayJsValueToWasm0(contents, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    const ptr3 = passArrayJsValueToWasm0(lib_paths, wasm.__wbindgen_malloc);
    const len3 = WASM_VECTOR_LEN;
    const ptr4 = passArrayJsValueToWasm0(lib_contents, wasm.__wbindgen_malloc);
    const len4 = WASM_VECTOR_LEN;
    const ptr5 = passStringToWasm0(llm_api_base, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len5 = WASM_VECTOR_LEN;
    const ptr6 = passStringToWasm0(llm_api_bearer_token, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len6 = WASM_VECTOR_LEN;
    const ret = wasm.matchPattern(ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3, ptr4, len4, ptr5, len5, ptr6, len6);
    return takeObject(ret);
}

function __wbg_adapter_197(arg0, arg1, arg2, arg3) {
    wasm.wasm_bindgen__convert__closures__invoke2_mut__h3ee746afe2fcb399(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
}

const UncompiledPatternBuilderFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_uncompiledpatternbuilder_free(ptr >>> 0));
/**
*/
export class UncompiledPatternBuilder {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        UncompiledPatternBuilderFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_uncompiledpatternbuilder_free(ptr);
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_gritApiRequest_8ddad184e69874ac = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        const ret = gritApiRequest(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4), getStringFromWasm0(arg5, arg6));
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    }, arguments) };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        const ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        const ret = false;
        return ret;
    };
    imports.wbg.__wbindgen_error_new = function(arg0, arg1) {
        const ret = new Error(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_gritExternalFunctionCall_13717a72daef7cda = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        var v0 = getArrayJsValueFromWasm0(arg3, arg4).slice();
        wasm.__wbindgen_free(arg3, arg4 * 4, 4);
        var v1 = getArrayJsValueFromWasm0(arg5, arg6).slice();
        wasm.__wbindgen_free(arg5, arg6 * 4, 4);
        const ret = gritExternalFunctionCall(getStringFromWasm0(arg1, arg2), v0, v1);
        const ptr3 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len3 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len3;
        getInt32Memory0()[arg0 / 4 + 0] = ptr3;
    }, arguments) };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = getObject(arg0);
        const ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbg_String_b9412f8799faab3e = function(arg0, arg1) {
        const ret = String(getObject(arg1));
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        const ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_bigint_from_u64 = function(arg0) {
        const ret = BigInt.asUintN(64, arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_f975102236d3c502 = function(arg0, arg1, arg2) {
        getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
    };
    imports.wbg.__wbg_new_abda76e883ba8a5f = function() {
        const ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_658279fe44541cf6 = function(arg0, arg1) {
        const ret = getObject(arg1).stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_error_f851667af71bcfc6 = function(arg0, arg1) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg0;
            deferred0_1 = arg1;
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
        }
    };
    imports.wbg.__wbg_crypto_d05b68a3572bb8ca = function(arg0) {
        const ret = getObject(arg0).crypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_process_b02b3570280d0366 = function(arg0) {
        const ret = getObject(arg0).process;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_versions_c1cb42213cedf0f5 = function(arg0) {
        const ret = getObject(arg0).versions;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_node_43b1089f407e4ec2 = function(arg0) {
        const ret = getObject(arg0).node;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_string = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'string';
        return ret;
    };
    imports.wbg.__wbg_require_9a7e0f667ead4995 = function() { return handleError(function () {
        const ret = module.require;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_is_function = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'function';
        return ret;
    };
    imports.wbg.__wbg_msCrypto_10fc94afee92bd76 = function(arg0) {
        const ret = getObject(arg0).msCrypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_randomFillSync_b70ccbdf4926a99d = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).randomFillSync(takeObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_getRandomValues_7e42b4fb8779dc6d = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).getRandomValues(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_row_3fe912945184d6fe = function(arg0) {
        const ret = getObject(arg0).row;
        return ret;
    };
    imports.wbg.__wbg_column_7368d4be463c55a6 = function(arg0) {
        const ret = getObject(arg0).column;
        return ret;
    };
    imports.wbg.__wbg_startIndex_598ea650efab091a = function(arg0) {
        const ret = getObject(arg0).startIndex;
        return ret;
    };
    imports.wbg.__wbg_endIndex_486117fbf941e159 = function(arg0) {
        const ret = getObject(arg0).endIndex;
        return ret;
    };
    imports.wbg.__wbg_startPosition_bb7f993d9a271315 = function(arg0) {
        const ret = getObject(arg0).startPosition;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_endPosition_21e3d24cb31dfb12 = function(arg0) {
        const ret = getObject(arg0).endPosition;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_equals_1e288a0e57ac9935 = function(arg0, arg1) {
        const ret = getObject(arg0).equals(getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_new_42bf3a2dcbc2783d = function() { return handleError(function () {
        const ret = new Parser();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_initializetreesitter_e698d7bc1eef046f = function() {
        const ret = initialize_tree_sitter();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_load_a77f372e0f4ac919 = function(arg0, arg1) {
        const ret = Language.load(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_fieldNameForId_3486d98fd9500f3d = function(arg0, arg1, arg2) {
        const ret = getObject(arg1).fieldNameForId(arg2);
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_fieldIdForName_9ecd53d4ff462e86 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).fieldIdForName(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0xFFFFFF : ret;
    };
    imports.wbg.__wbg_idForNodeType_dc27082894f8a694 = function(arg0, arg1, arg2, arg3) {
        const ret = getObject(arg0).idForNodeType(getStringFromWasm0(arg1, arg2), arg3 !== 0);
        return ret;
    };
    imports.wbg.__wbg_nodeTypeForId_0ea9588e3671d166 = function(arg0, arg1, arg2) {
        const ret = getObject(arg1).nodeTypeForId(arg2);
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_childCount_6f535e90deac3ed1 = function(arg0) {
        const ret = getObject(arg0).childCount;
        return ret;
    };
    imports.wbg.__wbg_children_3a4a0d3dabb05d57 = function(arg0, arg1) {
        const ret = getObject(arg1).children;
        const ptr1 = passArrayJsValueToWasm0(ret, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_endIndex_213a2fa4214ce4fb = function(arg0) {
        const ret = getObject(arg0).endIndex;
        return ret;
    };
    imports.wbg.__wbg_endPosition_ee96555063dd5c9f = function(arg0) {
        const ret = getObject(arg0).endPosition;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_id_bc37dba8d5640757 = function(arg0) {
        const ret = getObject(arg0).id;
        return ret;
    };
    imports.wbg.__wbg_namedChildCount_aaef86502aa381cc = function(arg0) {
        const ret = getObject(arg0).namedChildCount;
        return ret;
    };
    imports.wbg.__wbg_nextNamedSibling_f54d562e9703b931 = function(arg0) {
        const ret = getObject(arg0).nextNamedSibling;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_nextSibling_9527f0e912bea559 = function(arg0) {
        const ret = getObject(arg0).nextSibling;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_parent_b61a48487a4af738 = function(arg0) {
        const ret = getObject(arg0).parent;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_previousNamedSibling_2682d859bef21286 = function(arg0) {
        const ret = getObject(arg0).previousNamedSibling;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_previousSibling_22bbdc7e0009e5b1 = function(arg0) {
        const ret = getObject(arg0).previousSibling;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_startIndex_c9556ff24921b256 = function(arg0) {
        const ret = getObject(arg0).startIndex;
        return ret;
    };
    imports.wbg.__wbg_startPosition_ea6567f051f4477f = function(arg0) {
        const ret = getObject(arg0).startPosition;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_text_23d778e76f6ccbee = function(arg0) {
        const ret = getObject(arg0).text;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_tree_b0f9fbfd9496f894 = function(arg0) {
        const ret = getObject(arg0).tree;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_type_b18a749db6a83f2b = function(arg0) {
        const ret = getObject(arg0).type;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_typeId_d63a1c3be8ab6330 = function(arg0) {
        const ret = getObject(arg0).typeId;
        return ret;
    };
    imports.wbg.__wbg_childForFieldId_cac1ba4538c68bd0 = function(arg0, arg1) {
        const ret = getObject(arg0).childForFieldId(arg1);
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_childForFieldName_bd3dbcc3db2e3fdc = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).childForFieldName(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_hasError_4527af89cddb9753 = function(arg0) {
        const ret = getObject(arg0).hasError();
        return ret;
    };
    imports.wbg.__wbg_isMissing_4d7da8ec8d46f1b4 = function(arg0) {
        const ret = getObject(arg0).isMissing();
        return ret;
    };
    imports.wbg.__wbg_isNamed_ba373d605e0825ae = function(arg0) {
        const ret = getObject(arg0).isNamed();
        return ret;
    };
    imports.wbg.__wbg_namedChild_cb08ca6dfd39ee0c = function(arg0, arg1) {
        const ret = getObject(arg0).namedChild(arg1 >>> 0);
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_toString_d818c9c26017acf3 = function(arg0) {
        const ret = getObject(arg0).toString();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_walk_769563d9b9cb473d = function(arg0) {
        const ret = getObject(arg0).walk();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_rootNode_fef2e9339855127f = function(arg0) {
        const ret = getObject(arg0).rootNode;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_delete_5d7505384d55c5b9 = function(arg0) {
        getObject(arg0).delete();
    };
    imports.wbg.__wbg_walk_7e6698f86774b2a6 = function(arg0) {
        const ret = getObject(arg0).walk();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getLanguage_c7b2924b8843b96e = function(arg0) {
        const ret = getObject(arg0).getLanguage();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_currentFieldId_f1c442dbac9d4af1 = function(arg0) {
        const ret = getObject(arg0).currentFieldId();
        return isLikeNone(ret) ? 0xFFFFFF : ret;
    };
    imports.wbg.__wbg_currentFieldName_8ffeee0b95c3caca = function(arg0) {
        const ret = getObject(arg0).currentFieldName();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_currentNode_9b1e083056b78b25 = function(arg0) {
        const ret = getObject(arg0).currentNode();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_delete_8f677c4cc3aadb41 = function(arg0) {
        getObject(arg0).delete();
    };
    imports.wbg.__wbg_gotoFirstChild_5532ee403383d580 = function(arg0) {
        const ret = getObject(arg0).gotoFirstChild();
        return ret;
    };
    imports.wbg.__wbg_gotoNextSibling_3543316674636e12 = function(arg0) {
        const ret = getObject(arg0).gotoNextSibling();
        return ret;
    };
    imports.wbg.__wbg_gotoParent_b1c54d538b71712d = function(arg0) {
        const ret = getObject(arg0).gotoParent();
        return ret;
    };
    imports.wbg.__wbg_reset_c9640d4eae7a2a45 = function(arg0, arg1) {
        getObject(arg0).reset(getObject(arg1));
    };
    imports.wbg.__wbg_delete_404d78b113097198 = function(arg0) {
        getObject(arg0).delete();
    };
    imports.wbg.__wbg_parse_a8aac250d698e0ea = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        const ret = getObject(arg0).parse(getObject(arg1), getObject(arg2), getObject(arg3));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_setLanguage_bd62a68779d9a24c = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).setLanguage(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_queueMicrotask_3cbae2ec6b6cd3d6 = function(arg0) {
        const ret = getObject(arg0).queueMicrotask;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_queueMicrotask_481971b0d87f3dd4 = function(arg0) {
        queueMicrotask(getObject(arg0));
    };
    imports.wbg.__wbg_new_16b304a2cfa7ff4a = function() {
        const ret = new Array();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newnoargs_e258087cd0daa0ea = function(arg0, arg1) {
        const ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_call_27c0f87801dedf93 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_72fb9a18b5ae2624 = function() {
        const ret = new Object();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_self_ce0dbfc45cf2f5be = function() { return handleError(function () {
        const ret = self.self;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_window_c6fb939a7f436783 = function() { return handleError(function () {
        const ret = window.window;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_globalThis_d1e6af4856ba331b = function() { return handleError(function () {
        const ret = globalThis.globalThis;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_global_207b558942527489 = function() { return handleError(function () {
        const ret = global.global;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbg_set_d4638f722068f043 = function(arg0, arg1, arg2) {
        getObject(arg0)[arg1 >>> 0] = takeObject(arg2);
    };
    imports.wbg.__wbg_push_a5b05aedc7234f9f = function(arg0, arg1) {
        const ret = getObject(arg0).push(getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_instanceof_Error_e20bb56fd5591a93 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Error;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_message_5bf28016c2b49cfb = function(arg0) {
        const ret = getObject(arg0).message;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_call_b3ca7c6051f9bec1 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_81740750da40724f = function(arg0, arg1) {
        try {
            var state0 = {a: arg0, b: arg1};
            var cb0 = (arg0, arg1) => {
                const a = state0.a;
                state0.a = 0;
                try {
                    return __wbg_adapter_197(a, state0.b, arg0, arg1);
                } finally {
                    state0.a = a;
                }
            };
            const ret = new Promise(cb0);
            return addHeapObject(ret);
        } finally {
            state0.a = state0.b = 0;
        }
    };
    imports.wbg.__wbg_resolve_b0083a7967828ec8 = function(arg0) {
        const ret = Promise.resolve(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_0c86a60e8fcfe9f6 = function(arg0, arg1) {
        const ret = getObject(arg0).then(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_a73caa9a87991566 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_buffer_12d079cc21e14bdb = function(arg0) {
        const ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_aa4a17c33a06e5cb = function(arg0, arg1, arg2) {
        const ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_63b92bc8671ed464 = function(arg0) {
        const ret = new Uint8Array(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_a47bac70306a19a7 = function(arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    };
    imports.wbg.__wbg_newwithlength_e9b4878cebadb3d3 = function(arg0) {
        const ret = new Uint8Array(arg0 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_subarray_a1f73cd4b5b42fe1 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_1f9b04f170055d33 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
        return ret;
    }, arguments) };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(getObject(arg1));
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper3854 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 1060, __wbg_adapter_32);
        return addHeapObject(ret);
    };

    return imports;
}

function __wbg_init_memory(imports, maybe_memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedInt32Memory0 = null;
    cachedUint32Memory0 = null;
    cachedUint8Memory0 = null;


    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(input) {
    if (wasm !== undefined) return wasm;

    if (typeof input === 'undefined') {
        input = new URL('grit_wasm_bindings_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await input, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync }
export default __wbg_init;
