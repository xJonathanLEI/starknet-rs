import json
import sys

from starkware.starknet.core.os.contract_class.compiled_class_hash import (
    compute_compiled_class_hash,
)
from starkware.starknet.services.api.contract_class.contract_class import (
    CompiledClass,
)

print("{")

file_path = sys.argv[1]
file_content = open(file_path).read()
parsed_json = json.loads(file_content)

# Tricks cairo-lang into thinking we have "pythonic_hints". It's not used for hash calulcation
# anyways.
parsed_json["pythonic_hints"] = []

contract_definition = CompiledClass.load(data=parsed_json)

class_hash = compute_compiled_class_hash(contract_definition)
print(f'  "compiled_class_hash": "{hex(class_hash)}"')

print("}")
