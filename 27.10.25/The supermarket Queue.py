"""
There is a queue for the self-checkout tills at the supermarket. 
Your task is write a function to calculate the total time required for all the customers to check out!
input

    customers: an array of positive integers representing the queue. 
    Each integer represents a customer, and its value is the amount of time they require to check out.
    n: a positive integer, the number of checkout tills.

output

The function should return an integer, the total time required."""


def queue_time(customers, n):
    if len(customers) == 0:
        return 0
    if n == 1:
        return sum(customers)
    
    line_arr = []
    busy_lines = n if n < len(customers) else len(customers)

    for i in range(busy_lines):
        line_arr.append(customers[i])
    for i in range(busy_lines, len(customers)):
        min_line_ind = line_arr.index(min(line_arr))
        line_arr[min_line_ind] += customers[i]
    return max(line_arr)


def queue_time_cool(customers, n):
    line = [0] * n
    for c in customers:
        line[line.index(min(line))] += c
    return max(line)
    


if __name__ == "__main__":
    tests = [
        ([], 1),
        ([5], 1),
        ([2], 5),
        ([1,2,3,4,5], 1),
        ([1,2,3,4,5], 100),
        ([2,2,3,3,4,4], 2)
    ]

    for test in tests:
        result = queue_time_cool(test[0], test[1])
        print(result)
