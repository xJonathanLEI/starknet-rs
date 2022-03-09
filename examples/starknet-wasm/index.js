const starknet = import("./pkg");

starknet
  .then((m) => {
    const privateKey =
      "0x03c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc";
    console.log("Private Key:", privateKey);

    const publicKey = m.get_public_key(privateKey);

    if (
      publicKey !==
      "0x077a3b314db07c45076d11f62b6f9e748a39790441823307743cf00d6597ea43"
    ) {
      throw new Error("Unexpected public key");
    } else {
      console.log("Public Key:", publicKey);
    }
  })
  .catch(console.error);
