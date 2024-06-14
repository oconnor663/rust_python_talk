class Person:
    def __init__(self, foods):
        self.favorite_foods = foods

    def add_favorites(self, other_person):
        for food in other_person.favorite_foods:
            self.favorite_foods.append(food)

alice = Person(["apples"])
alice.add_favorites(alice)
