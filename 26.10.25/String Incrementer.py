"""

Your job is to write a function which increments a string, to create a new string.

    If the string already ends with a number, the number should be incremented by 1.
    If the string does not end with a number. the number 1 should be appended to the new string.

Examples:

foo -> foo1

foobar23 -> foobar24

foo0042 -> foo0043

foo9 -> foo10

foo099 -> foo100

Attention: If the number has leading zeros the amount of digits should be considered.


"""


def string_incrementer(string: str):
    if not string.endswith(('0', '1', '2', '3', '4', '5', '6', '7', '8', '9')):
        return string + '1'
    
    num = ""
    for char in reversed(string):
        if char.isdigit():
            num += char
            continue
        break
    st = string[:len(string) - len(num)]
    inc_num = int(''.join(reversed(num))) + 1
    return st + "0" * (len(num) - len(str(inc_num))) + str(inc_num)
    


if __name__ == "__main__":
    tests = ["foo", "foobar001", "foobar1", "foobar00", "foobar99", "foobar099", "fo99obar99", ""]

    for test in tests:
        result = string_incrementer(test)
        print(result)

            