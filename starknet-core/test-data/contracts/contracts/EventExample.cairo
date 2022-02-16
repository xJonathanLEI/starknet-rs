%lang starknet
%builtins range_check

#
# Events
#

@event
func initialized(arg : felt):
end

#
# Constructor
#

@constructor
func constructor{syscall_ptr : felt*, range_check_ptr}(arg : felt):
    initialized.emit(arg=arg)
    return ()
end
