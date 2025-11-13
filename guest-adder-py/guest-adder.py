# wit_world is generated in ./adder
from wit_world import WitWorld


# the class MUST be named `WitWorld`, same as the abstract class
class WitWorld(WitWorld):
    def add(self, a: int, b: int) -> int:
        return a + b
