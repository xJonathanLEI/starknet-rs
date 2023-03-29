%lang starknet

//
// Events
//

@event
func initialized(arg: felt) {
}

//
// Constructor
//

@constructor
func constructor{syscall_ptr: felt*, range_check_ptr}(arg: felt) {
    initialized.emit(arg=arg);
    return ();
}
