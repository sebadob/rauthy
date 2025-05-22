export interface PatchOp {
    put: PatchValue[],
    del: string[],
}

export interface PatchValue {
    key: string,
    value: any,
}
