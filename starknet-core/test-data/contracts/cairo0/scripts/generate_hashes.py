import json
import sys

from starkware.starknet.core.os.contract_class.deprecated_class_hash import (
    compute_deprecated_class_hash,
)
from starkware.starknet.services.api.contract_class.contract_class import (
    DeprecatedCompiledClass,
)

print("{")

file_path = sys.argv[1]
file_content = open(file_path).read()

contract_definition = DeprecatedCompiledClass.load(data=json.loads(file_content))

class_hash = compute_deprecated_class_hash(contract_class=contract_definition)
print(f'  "class_hash": "{hex(class_hash)}"')

print("}")
