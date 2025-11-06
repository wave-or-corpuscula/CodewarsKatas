# We'll create a function that takes in two parameters:

#     a sequence (length and types of items are irrelevant)
#     a function (value, index) that will be called on members of the sequence and their index. The function will return either true or false.

# Your function will iterate through the members of the sequence in order until the provided function returns true; at which point your function will return that item's index.

# If the function given returns false for all members of the sequence, your function should return -1.

# true_if_even = lambda value, index: value % 2 == 0
# find_in_array([1,3,5,6,7], true_if_even) # --> 3


def find_in_array(seq: list, predicate): 
    for i in range(len(seq)):
        if predicate(seq[i], i):
            return i
    return -1


if __name__ == "__main__":
    tests = [
        ([], lambda v, i: v % 2 == 0),
        ([1,3,5,6,7], lambda v, i: v % 2 == 0),
        ([2,4,6,8], lambda v, i: v % 2 == 0)
    ]
    for test in tests:
        print(find_in_array(*test))