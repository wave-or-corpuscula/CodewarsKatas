"""
A format for expressing an ordered list of integers is to use a comma separated list of either

    individual integers
    or a range of integers denoted by the starting integer separated from the end integer in the range by a dash, '-'. The range includes all integers in the interval including both endpoints. It is not considered a range unless it spans at least 3 numbers. For example "12,13,15-17"

Complete the solution so that it takes a list of integers in increasing order and returns a correctly formatted string in the range format.

Example:

solution([-10, -9, -8, -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20])
# returns "-10--8,-6,-3-1,3-5,7-11,14,15,17-20"

"""

def solution(args: list):
    result = []
    order_count = 0
    for i in range(len(args) - 1):
        if abs(args[i + 1] - args[i]) == 1:
            order_count += 1
        elif order_count > 1:
            result.append(f"{args[i - order_count]}-{args[i]}")
            order_count = 0
        elif order_count > 0:
            result.append(f"{args[i - 1]},{args[i]}")
            order_count = 0
        else:
            result.append(f"{args[i]}")
    if order_count > 1:
        result.append(f"{args[i - order_count + 1]}-{args[i + 1]}")
    elif order_count > 0:
        result.append(f"{args[i]},{args[i + 1]}")
    else:
        result.append(f"{args[i + 1]}")
    return ','.join(result)


if __name__ == "__main__":
    tests = [
        [-10, -9, -8, -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20, 22],
        [-3,-2,-1,2,10,15,16,18,19,20]
    ]

    for test in tests:
        print(solution(test))