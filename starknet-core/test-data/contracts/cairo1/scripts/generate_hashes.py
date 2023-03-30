import json
import sys

from starkware.starknet.core.os.contract_class.class_hash import (
    compute_class_hash,
)
from starkware.starknet.core.os.contract_class.compiled_class_hash import (
    compute_compiled_class_hash,
)
from starkware.starknet.services.api.contract_class.contract_class import (
    CompiledClass,
    ContractClass,
)
from starkware.starknet.services.api.contract_class.contract_class_utils import (
    load_sierra_from_dict,
)

base_file_path = sys.argv[1]

sierra_class_json = json.loads(open(f"{base_file_path}_sierra.txt").read())
compiled_class_json = json.loads(open(f"{base_file_path}_compiled.txt").read())

# Tricks cairo-lang into thinking we have "pythonic_hints". It's not used for hash calulcation
# anyways.
compiled_class_json["pythonic_hints"] = []

sierra_class_hash = compute_class_hash(load_sierra_from_dict(sierra=sierra_class_json))
compiled_class_hash = compute_compiled_class_hash(
    CompiledClass.load(data=compiled_class_json)
)

print("{")

print(f'  "sierra_class_hash": "{hex(sierra_class_hash)}",')
print(f'  "compiled_class_hash": "{hex(compiled_class_hash)}"')

print("}")
