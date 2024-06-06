class Person:
    def __init__(self, numbers):
        self.favorite_numbers = numbers

default_numbers = [42]
william = Person(default_numbers)
william.favorite_numbers.append(1066)
# OOPS: William and George are both referencing the same list.
george = Person(default_numbers)
george.favorite_numbers.append(1776)
print("william:", william.favorite_numbers);
print("george:", george.favorite_numbers);
