"""
Complete the function scramble(str1, str2) that returns true if a portion of str1 characters can be rearranged to match str2, otherwise returns false.

Notes:

    Only lower case letters will be used (a-z). No punctuation or digits will be included.
    Performance needs to be considered.

Examples

scramble('rkqodlw', 'world') ==> True
scramble('cedewaraaossoqqyt', 'codewars') ==> True
scramble('katas', 'steak') ==> False
"""

from collections import Counter

def scramble(s1, s2):
    return Counter(s1) >= Counter(s2)


def scramble_2(s1: str, s2: str):
    for el in set(s2):
        if s1.count(el) < s2.count(el):
            return False
    return True


if __name__ == "__main__":
    tests = [
        ('rkqodlw', 'world'),
        ('cedewaraaossoqqyt', 'codewars'),
        ('katas', 'steak')
    ]

    for test in tests:
        print(scramble(test[0], test[1]))