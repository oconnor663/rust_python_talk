class Person:
    def __init__(self, numbers):
        self.favorite_foods = numbers

default_numbers = ["donuts"]

alice = Person(default_numbers)
alice.favorite_foods.append("apples")

# OOPS: Alice and Bob are both referencing the same list.
bob = Person(default_numbers)
bob.favorite_foods.append("bananas")

print("alice:", alice.favorite_foods);
print("bob:", bob.favorite_foods);
