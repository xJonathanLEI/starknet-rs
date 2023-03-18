#include "starknet_cxx_bridge/lib.h"
#include <iostream>

int main() {
    auto hash = pedersen_hash(
        "0x3d937c035c878245caf64531a5756109c53068da139362728feb561405371cb",
        "0x208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a"
    );

    auto signature = ecdsa_sign(
        "0x1",
        "0x2"
    );
  
    std::cout << "pedersen_hash():" << "\n"
              << "  " << hash.c_str() << "\n"
              << "ecdsa_sign():" << "\n"
              << "  " << signature.c_str() << "\n";
    return 0;
}
