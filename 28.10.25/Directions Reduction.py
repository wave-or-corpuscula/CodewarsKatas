"""
How I crossed a mountainous desert the smart way.

The directions given to the man are, for example, the following (depending on the language):

["NORTH", "SOUTH", "SOUTH", "EAST", "WEST", "NORTH", "WEST"].
or
{ "NORTH", "SOUTH", "SOUTH", "EAST", "WEST", "NORTH", "WEST" };
or
[North, South, South, East, West, North, West]

You can immediately see that going "NORTH" and immediately "SOUTH" is not reasonable, better stay to the same place! So the task is to give to the man a simplified version of the plan. A better plan in this case is simply:

["WEST"]
or
{ "WEST" }
or
[West]

"""

eths = [set('EAST' + 'WEST'), set('NORTH' + 'SOUTH')]

def dir_reduc(arr: list):
    prev_el = 0
    for el in range(1, len(arr)):
        if set(arr[prev_el] + arr[el]) in eths:
            return dir_reduc(arr[:prev_el] + arr[el + 1:])
        prev_el = el
    return arr




if __name__ == "__main__":
    tests = [
        ["NORTH", "SOUTH", "SOUTH", "EAST", "WEST", "NORTH", "WEST"],
        ["NORTH", "WEST", "SOUTH", "EAST"],
        ["NORTH", "SOUTH", "SOUTH", "EAST", "WEST", "NORTH", "WEST"],
        ["NORTH", "SOUTH", "EAST", "WEST", "NORTH", "NORTH", "SOUTH", "NORTH","WEST", "EAST"],
        [],
        ["NORTH","SOUTH","SOUTH","EAST","WEST","NORTH"],
        ["NORTH","SOUTH","SOUTH","EAST","WEST","NORTH","NORTH"],
        ["EAST", "EAST", "WEST", "NORTH", "WEST", "EAST", "EAST", "SOUTH", "NORTH", "WEST"],
    ]
    result = dir_reduc(tests[7])
    print(result)