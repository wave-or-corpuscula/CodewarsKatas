"""
If we were to set up a Tic-Tac-Toe game, we would want to know whether the board's current state is solved, wouldn't we? Our goal is to create a function that will check that for us!

Assume that the board comes in the form of a 3x3 array, where the value is 0 if a spot is empty, 1 if it is an "X", or 2 if it is an "O", like so:

[[0, 0, 1],
 [0, 1, 2],
 [2, 1, 0]]

We want our function to return:

    -1 if the board is not yet finished AND no one has won yet (there are empty spots),
    1 if "X" won,
    2 if "O" won,
    0 if it's a cat's game (i.e. a draw).

You may assume that the board passed in is valid in the context of a game of Tic-Tac-Toe.
"""

players = (1, 2)

def is_solved(board):
    # Check rows
    for row in board:
        if all(row[0] == el for el in row) and row[0] in players:
            return row[0]

    # Check columns:
    for col in zip(*board): # Still don't really understand, what is going on here
        if all(col[0] == el for el in col) and col[0] in players:
            return col[0]
    if board[0][0] == board[1][1] == board[2][2] and board[0][0] in players or \
        board[2][0] == board[1][1] == board[0][2] and board[2][0] in players:
        return board[1][1]

    if any(el == 0 for el in [element for row in board for element in row]):
        return -1

    return 0


# Nice try, man, nice try
def is_solved_nice(board):
    grid = [
        [a1, a2, a3],
        [b1, b2, b3],
        [c1, c2, c3]
    ]= board
    if a1 == a2 == a3 == 1: return 1
    if b1 == b2 == b3 == 1: return 1
    if c1 == c2 == c3 == 1: return 1
    if a1 == b2 == c3 == 1: return 1
    if a3 == b2 == c1 == 1: return 1
    if a1 == b1 == c1 == 1: return 1
    if a2 == b2 == c2 == 1: return 1
    if a3 == b3 == c3 == 1: return 1
    
    if a1 == a2 == a3 == 2: return 2
    if b1 == b2 == b3 == 2: return 2
    if c1 == c2 == c3 == 2: return 2
    if a1 == b2 == c3 == 2: return 2
    if a3 == b2 == c1 == 2: return 2
    if a1 == b1 == c1 == 2: return 2
    if a2 == b2 == c2 == 2: return 2
    if a3 == b3 == c3 == 2: return 2

    if 0 in [a1, a2, a3, b1, b2, b3, c1, c2, c3]:
        return -1
    else: return 0



if __name__ == "__main__":
    tests = [
        [[1, 2, 2],
         [2, 1, 1],
         [2, 1, 2]],
        [[1, 1, 1],
         [0, 2, 2],
         [0, 0, 0]],
        [[2, 1, 2],
         [2, 1, 1],
         [1, 1, 2]],
        [[2, 1, 2],
         [2, 1, 1],
         [1, 2, 1]]
    ]
    # for test in tests:
    print(is_solved(tests[0]))
