#[starknet::contract]
mod Trivial {
    #[storage]
    struct Storage {}

    #[abi(embed_v0)]
    fn something(ref self: ContractState) -> felt252 {
        1
    }
}
