# The pepe market is on the verge of collapse. In a last-ditch attempt to make some quick cash, you decide to sift through the thousands of pepes on the internet to identify the rarest ones, which are worth more. Since doing this by hand would be tedious, you decide to use your programming skills to streamline the process.

# Your task in this kata is to implement a function that, given a non-empty list of pepes (pepes), returns the rarest pepe in the list.

#     If two or more pepes are equally rare, return a sorted list of these pepes.
#     If the rarest pepe (or pepes) has a frequency of 5 or more, then there are no truly rare pepes, so your function should return "No rare pepes!".


from collections import Counter


def find_rarest_pepe(pepes: list):
    pepes_freq = Counter(pepes)
    uniq_pepes = list(set(pepes))
    rarest_freq = 5
    result = []

    for pepe in uniq_pepes:
        if pepes_freq[pepe] < 5:
            if pepes_freq[pepe] < rarest_freq:
                result.clear()
                result.append(pepe)
                rarest_freq = pepes_freq[pepe]
            elif pepes_freq[pepe] == rarest_freq:
                result.append(pepe)

    if rarest_freq == 5:
        return 'No rare pepes!'
    return result[0] if len(result) == 1 else sorted(result)


if __name__ == "__main__":
    tests = [['Donald Trump Pepe',
            'Melania Trump Pepe',
            'Clown Pepe',
            'Clown Pepe',
            'Donald Trump Pepe'],
            ['Deep Learning Pepe',
                'Go Pepe',
                'Machine Learning Pepe',
                'Machine Learning Pepe',
                'Machine Learning Pepe'],
                ['Codewars Pepe',
                       'Codewars Pepe',
                       'Codewars Pepe',
                       'Codewars Pepe',
                       'Codewars Pepe']
            ]
    for test in tests:
        print(find_rarest_pepe(test))