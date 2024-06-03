class NumberList:
    def __init__(self, numbers):
        self.numbers = numbers

    def append_all(self, other_list):
        for num in other_list.numbers:
            self.numbers.append(num)


# This works fine.
list_a = NumberList([1, 2, 3])
list_b = NumberList([4, 5, 6])
print("Append list_b to list_a...")
list_a.append_all(list_b)
print(list_a.numbers)

# OOPS: But what does this do?
print("Append list_a to itself...")
list_a.append_all(list_a)
print(list_a.numbers)
