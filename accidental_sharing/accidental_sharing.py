class Person:
    def __init__(self, foods):
        self.favorite_foods = foods

donuts = ["donuts"]

alice = Person(donuts)
alice.favorite_foods.append("apples")

bob = Person(donuts)
bob.favorite_foods.append("bananas")

print("alice:", alice.favorite_foods);
print("  bob:", bob.favorite_foods);
