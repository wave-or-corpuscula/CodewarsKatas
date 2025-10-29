"""
┌───┬───┬───┐
│ 1 │ 2 │ 3 │
├───┼───┼───┤
│ 4 │ 5 │ 6 │
├───┼───┼───┤
│ 7 │ 8 │ 9 │
└───┼───┼───┘
    │ 0 │
    └───┘

He noted the PIN 1357, but he also said, it is possible that each of the digits he saw could 
actually be another adjacent digit (horizontally or vertically, but not diagonally). 
E.g. instead of the 1 it could also be the 2 or 4. And instead of the 5 it could also be the 2, 4, 6 or 8.

He also mentioned, he knows this kind of locks. You can enter an unlimited amount of wrong PINs, they never finally 
lock the system or sound the alarm. That's why we can try out all possible (*) variations.
"""

import itertools


close_numbers = {
    '1': ['1', '2', '4'],
    '2': ['2', '1', '3', '5'],
    '3': ['3', '2', '6'],
    '4': ['4', '1', '5', '7'],
    '5': ['5', '2', '4', '6', '8'],
    '6': ['6', '3', '5', '9'],
    '7': ['7', '4', '8'],
    '8': ['8', '0', '5', '7', '9'],
    '9': ['9', '6', '8'],
    '0': ['0', '8']
}


def get_pins(observed):
    possible_digits = [close_numbers[dig] for dig in observed]
    a = list(itertools.product(*possible_digits))
    return [''.join(lst) for lst in a]


if __name__ == "__main__":
    tests = ['8', '11', '369']
    for test in tests:
        print(get_pins(test))
