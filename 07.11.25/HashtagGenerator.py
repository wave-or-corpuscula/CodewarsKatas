# The marketing team is spending way too much time typing in hashtags.
# Let's help them with our own Hashtag Generator!

# Here's the deal:

#     It must start with a hashtag (#).
#     All words must have their first letter capitalized, and remaining letters lowercased.

#     If the final result is longer than 140 chars it must return false.
#     If the input or the result is an empty string it must return false.

# Examples

# " Hello there thanks for trying my Kata"  =>  "#HelloThereThanksForTryingMyKata"
# "    Hello     World   "                  =>  "#HelloWorld"
# ""                                        =>  false

from SplitStrings import timer

def generate_hashtag(s: str):
    result = "".join(list(map(str.title, s.split(" "))))
    if 1 > len(result) or len(result) >= 140: return False
    return "#" + result

def cw_generate_hashtag(s: str):
    result = s.title().replace(" ", "")
    return False if (1 > len(result) or len(result) >= 140) else "#" + result


if __name__ == "__main__":
    tests = [
        'CodewArs',
        'Codewars      ',
        '      Codewars',
        'Codewars   Is Nice',
        'codewars is nice',
        'CoDeWaRs is niCe',
        'c i n',
        'codewars  is  nice',
        '',
        'ABbCccDdddEeeeeFfffffGggggggHhhhhhhhIiiiiiiiiJjjjjjjjjjKkkkkkkkkkkLlllllllllllMmmmmmmmmmmmmNnnnnnnnnnnnnnOooooooooooooooPpppppppppppppppQqqq'
    ]
    n = 100000

    timer(generate_hashtag, n, tests)    # 1.8157455079999636
    timer(cw_generate_hashtag, n, tests) # 0.9047816269994655 - 2 times faster

    # for test in tests:
    #     print(cw_generate_hashtag(test))
