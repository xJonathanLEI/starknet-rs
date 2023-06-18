%lang starknet

@constructor
func constructor{syscall_ptr: felt*, range_check_ptr}(arg: felt) {
    with_attr error_message("💩") {
        assert 1 = 0;
    }
    return ();
}
