from dataclasses import dataclass

@dataclass
class NumberList:
    numbers: list[int]

    def take_all(self, other_list):
        self.numbers += other_list.numbers
        other_list.numbers = []

def test_take_all():
    list_a = NumberList([1, 2, 3])
    list_b = NumberList([4, 5, 6])
    list_a.take_all(list_b)
    assert list_a.numbers == [1, 2, 3, 4, 5, 6]
    assert list_b.numbers == []

    # OOPS: What does this do?
    list_a.take_all(list_a)
    assert list_a.numbers == [1, 2, 3, 4, 5, 6]
