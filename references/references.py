from dataclasses import dataclass

@dataclass
class Person:
    favorite_foods: set[str]

    def add_food(self, food):
        self.favorite_foods.add(food)

def test_foods():
    default_foods = {"donut"}
    alice = Person(default_foods)
    alice.add_food("apple")
    # OOPS: Alice and Bob are both referencing the same set.
    bob = Person(default_foods)
    bob.add_food("banana")
    assert alice.favorite_foods == {"donut", "apple"}
    assert bob.favorite_foods == {"donut", "banana"}
