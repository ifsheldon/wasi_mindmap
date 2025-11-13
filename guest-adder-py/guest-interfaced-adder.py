# wit_world is generated in ./interfaced_adder
from wit_world import exports


# the class MUST be named `Add`, same as the abstract class
class Add(exports.Add):
    def add(self, a: int, b: int) -> int:
        return a + b
