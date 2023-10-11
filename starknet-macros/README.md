# Procedural macros for `starknet`

## abigen

The `abigen` macro aims at generating rust binding from an `ABI` of a smart contract.
The generated bindings contains all the functions, events, structs and enums that are
present in the `ABI` file.

Some types are directly mapped to rust native types (like integers, `Result`, `Option`, boolean etc..),
other specific types like `ContractAddress`, `ClassHash` or `EthAddress` are managed by `starknet-rs`,
and all other types found in the `ABI` are generated as `struct` or `enum` as necessary.

`abigen` will generate all the serialization/deserialization code that is required to
work with plain rust types.

For instance:

```rust,ignore
// Cairo function like fn view_1(self: @ContractState, v: felt252, s: Span<felt252>)
// is generated in rust like:

fn view_1(v: FieldElement, s: Vec<FieldElement>);
```

To generate the bindings for your contract, you can do the following:

```rust,ignore
use starknet::macros::abigen;

abigen!(MyContract, "/path/to/abi.json");
```

This will generate all the types and two `struct` for the contract:

1. `MyContractReader`, which is use to call `view` functions that are only reading the blockchain state.
   To initialize a reader, you need your contract address and a provider:

   ```rust,ignore
   let rpc_url = Url::parse("http://0.0.0.0:5050").unwrap();
   let provider = JsonRpcClient::new(HttpTransport::new(rpc_url.clone()));
   let contract_address = FieldElement::from_hex_be("0x123...").unwrap();

   let reader = MyContractReader::new(contract_address, &provider);
   let result = reader.my_view_1().await;
   ```

2. `MyContract`, which in turn is used to call `external` functions, where a transaction is actually sent to the blockchain.
   This one requires an account, to sign those transactions:

   ```rust,ignore
   let rpc_url = Url::parse("http://0.0.0.0:5050").unwrap();
   let provider = JsonRpcClient::new(HttpTransport::new(rpc_url.clone()));

   let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be("<PRIVATE_KEY_HEX>").unwrap(),
   ));

   let account_address = FieldElement::from_hex_be("<ACCOUNT_ADDRESS_HEX>").unwrap();
   let account = SingleOwnerAccount::new(
        provider.clone(),
        signer,
        address,
        felt!("0x4b4154414e41"), // KATANA
        ExecutionEncoding::Legacy,
    );

   let contract_address = FieldElement::from_hex_be("0x123...").unwrap();

   let reader = MyContract::new(contract_address, &account);
   let result = reader.my_external_1().await;
   ```

An other feature provided by `abigen` macro is the capabilities of deserialiazing events.
In the `ABI`, there is always an `Event` enum, which contains all the events declared in your contract.

You can then do the following:

```rust,ignore
let even_page = provider.fetch_events(...);
for e in event_page.events {
  let my_event: Event = match e.try_into() {
    Ok(ev) => ev,
    Err(_) => continue; // This is an event from an other contract or you may use an out-dated ABI.
  };

  match my_event {
    Event::MyEventA(a) => // work with a, already typed and deserialized,
    Event::MyEventB(b) => // work with b, already typed and deserialized,
    ...
  };
}
```
