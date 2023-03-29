#[contract]
mod AbiTypes {
    use core::array::ArrayTrait;
    use core::serde::Serde;
    use core::traits::Into;

    use starknet::ContractAddress;

    #[derive(Drop)]
    enum ExampleEnum {
        variant_a: felt252,
        variant_b: u256,
    }

    #[derive(Drop)]
    struct ExampleStruct {
        field_a: felt252,
        field_b: felt252,
        field_c: ExampleEnum,
        field_d: (),
    }

    impl ExampleEnumSerde of Serde::<ExampleEnum> {
        fn serialize(ref serialized: Array<felt252>, input: ExampleEnum) {
            serialized.append(100);
        }

        fn deserialize(ref serialized: Span<felt252>) -> Option<ExampleEnum> {
            Option::None(())
        }
    }

    impl ExampleStructSerde of Serde::<ExampleStruct> {
        fn serialize(ref serialized: Array<felt252>, input: ExampleStruct) {
            serialized.append(100);
        }

        fn deserialize(ref serialized: Span<felt252>) -> Option<ExampleStruct> {
            Option::None(())
        }
    }

    #[event]
    fn ExampleEvent(value_a: u256, value_b: ExampleStruct) {}


    #[view]
    fn example_view_function() -> ExampleEnum {
        ExampleEnum::variant_a(100)
    }

    #[external]
    fn example_external_function(recipient: ContractAddress, amount: u256) -> ExampleStruct {
        ExampleStruct {
            field_a: 200, field_b: 300, field_c: ExampleEnum::variant_b(400.into()), field_d: ()
        }
    }
}
