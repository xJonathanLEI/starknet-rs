#[starknet::contract]
mod AbiTypes {
    use core::array::ArrayTrait;
    use core::serde::Serde;
    use core::traits::Into;

    use starknet::ContractAddress;

    #[storage]
    struct Storage {}

    #[derive(Copy, Drop, PartialEq, Serde)]
    enum ExampleEnum {
        variant_a: felt252,
        variant_b: u256,
    }

    #[derive(Copy, Drop, PartialEq, Serde)]
    struct ExampleStruct {
        field_a: felt252,
        field_b: felt252,
        field_c: ExampleEnum,
        field_d: (),
    }


    #[event]
    #[derive(Copy, Drop, PartialEq, starknet::Event)]
    enum Event {
        ExampleEvent: ExampleEvent,
        #[flat]
        FlatEvent: FlatEvent,
    }

    #[derive(Copy, Drop, PartialEq, starknet::Event)]
    struct ExampleEvent {
        value_a: u256,
        value_b: ExampleStruct
    }

    #[derive(Copy, Drop, PartialEq, starknet::Event)]
    struct StaticEvent {}

    #[derive(Copy, Drop, PartialEq, starknet::Event)]
    enum FlatEvent {
        FlatEvent: StaticEvent,
    }


    fn example_view_function() -> ExampleEnum {
        ExampleEnum::variant_a(100)
    }

    #[external(v0)]
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
