import adder


# the class MUST be named `Adder`, same as the abstract class
class Adder(adder.Adder):
    def add(self, a: int, b: int) -> int:
        return a + b
