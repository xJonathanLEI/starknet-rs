%lang starknet

from starkware.cairo.common.cairo_builtins import HashBuiltin, SignatureBuiltin, BitwiseBuiltin

@constructor
func constructor(arg: felt) {
    return ();
}

@external
func __validate_deploy__{}(class_hash: felt, contract_address_salt: felt, arg: felt) {
    return ();
}
