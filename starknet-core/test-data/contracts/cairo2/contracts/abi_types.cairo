#[starknet::contract]
mod AbiTypes {
    use core::array::ArrayTrait;
    use core::serde::Serde;
    use core::traits::Into;

    use starknet::ContractAddress;

    #[storage]
    struct Storage {}

    #[derive(Drop, Serde)]
    enum ExampleEnum {
        variant_a: felt252,
        variant_b: u256,
    }

    #[derive(Drop, Serde)]
    struct ExampleStruct {
        field_a: felt252,
        field_b: felt252,
        field_c: ExampleEnum,
        field_d: (),
    }


    #[event]
    fn ExampleEvent(value_a: u256, value_b: ExampleStruct) {}


    #[view]
    fn example_view_function() -> ExampleEnum {
        ExampleEnum::variant_a(100)
    }

    #[external]
    fn example_external_function(
        ref self: ContractState, recipient: ContractAddress, amount: u256
    ) -> ExampleStruct {
        ExampleStruct {
            field_a: 200, field_b: 300, field_c: ExampleEnum::variant_b(400.into()), field_d: ()
        }
    }

    #[l1_handler]
    fn example_l1_handler(ref self: ContractState, from_address: felt252, arg1: felt252) {}
}
