class Person:
    def __init__(self, foods):
        self.favorite_foods = foods

    def add_food(self, food):
        self.favorite_foods.add(food)


default_foods = {"donut"}
alice = Person(default_foods)
alice.add_food("apple")
# OOPS: Alice and Bob are both referencing the same set.
bob = Person(default_foods)
bob.add_food("banana")
assert alice.favorite_foods == {"donut", "apple"}
assert bob.favorite_foods == {"donut", "banana"}
