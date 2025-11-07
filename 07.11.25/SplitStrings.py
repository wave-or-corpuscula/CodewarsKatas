# Complete the solution so that it splits the string into pairs of two characters. If the string contains an odd number of characters then it should replace the missing second character of the final pair with an underscore ('_').

# Examples:

# * 'abc' =>  ['ab', 'c_']
# * 'abcdef' => ['ab', 'cd', 'ef']


import re
import time
from textwrap import wrap


# Slowest one. My(((((
def my_solution(s: str):
    return wrap(s + "_" if len(s) % 2 else s, 2)

# Fast, but not enough
def cw_solution(s: str):
    result = []
    if len(s) % 2: s += "_"
    for i in range(0, len(s), 2):
        result.append(s[i:i+2])
    return result

# The fastest one
def so_solution(s: str):
    if len(s) % 2: s += "_"
    return [s[i:i+n] for i in range(0, len(s), n)]

# SLOWWWWW
def re_solution(s: str):
    if len(s) % 2: s += "_"
    return re.findall('..', s)


def timer(func, n: int, tests: list):
    start = time.perf_counter()

    for _ in range(n):
        for test in tests:
            func(test.copy())
    
    end = time.perf_counter()
    print(f"func: {func}, time: {end - start}")


if __name__ == "__main__":
    tests = [
        "asdfadsf",
        "asdfads",
        "",
        "x"
    ]

    n = 100000

    timer(my_solution, n, tests) # time: 4.218546228999912
    timer(cw_solution, n, tests) # time: 0.31829880800069077
    timer(so_solution, n, tests) # time: 0.2254722319994471 WINNER!!!
    timer(re_solution, n, tests) # time: 0.6565213100002438




