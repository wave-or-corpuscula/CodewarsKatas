import time

"""
Task

In this simple Kata your task is to create a function that turns a string into a Mexican Wave. 
You will be passed a string and you must return an array of strings where an uppercase letter is a person standing up.
Rules

1.  The input string will always consist of lowercase letters and spaces, but may be empty, in which case you must return an empty array. 
2.  If the character in the string is whitespace then pass over it as if it was an empty seat
"""


def wave(people: str):
    result_arr = []
    for i in range(len(people)):
        if people[i] == ' ':
            continue
        res_p = list(people)
        res_p[i] = people[i].capitalize()
        result_arr.append(''.join(res_p))
    return result_arr


if __name__ == "__main__":
    tasks = ["hello", "codewars", "", "two words", " gap ", "a       b    ", "this is a few words"]

    result = wave(tasks[1])
    while True:
        for r in result:
            print(r)
            time.sleep(0.1)