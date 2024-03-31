from typing import Any

def hello_eco_system(sub_eco_system: Any | None=None) -> str:
    return ("Hello Sub Eco System " + str(sub_eco_system)) + " from Parent Eco system"


