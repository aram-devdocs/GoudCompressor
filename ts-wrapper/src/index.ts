import * as wasm from "../wasm/goud_compressor";

export async function compress(input: Uint8Array): Promise<Uint8Array> {
    return wasm.compress(input);
}

export async function decompress(input: Uint8Array): Promise<Uint8Array> {
    return wasm.decompress(input);
}