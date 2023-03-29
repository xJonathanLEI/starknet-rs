import json
import sys

from starkware.starknet.core.os.class_hash import compute_class_hash
from starkware.starknet.services.api.contract_class import ContractClass

print("{")

file_path = sys.argv[1]
file_content = open(file_path).read()

contract_definition = ContractClass.load(data=json.loads(file_content))

class_hash = compute_class_hash(contract_class=contract_definition)
print(f'  "class_hash": "{hex(class_hash)}"')

print("}")
