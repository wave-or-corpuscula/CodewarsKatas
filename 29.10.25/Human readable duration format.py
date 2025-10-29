"""
Your task in order to complete this Kata is to write a function which formats a duration, given as a number of seconds, in a human-friendly way.

The function must accept a non-negative integer. If it is zero, it just returns "now". Otherwise, the duration is expressed as a combination of years, days, hours, minutes and seconds.

It is much easier to understand with an example:

* For seconds = 62, your function should return 
    "1 minute and 2 seconds"
* For seconds = 3662, your function should return
    "1 hour, 1 minute and 2 seconds"

"""

def format_duration(seconds):
    if seconds == 0:
        return "now"
    result = []

    duration_seconds = [31536000, 86400, 3600, 60, 1]
    duration_names = ['year', 'day', 'hour', 'minute', 'second']

    for dur_sec, dur_name in zip(duration_seconds, duration_names):
        dur_amount = seconds // dur_sec
        if dur_amount > 0:
            result.append(f"{dur_amount} {dur_name}{'s' if dur_amount > 1 else ''}")    
            seconds -= dur_amount * dur_sec

    if len(result) > 1:
        return ', '.join(result[:-1]) + f" and {result[-1]}"
    return ''.join(result)


if __name__ == "__main__":
    tests = [
        132030240,
        0,
        1,
        62,
        120,
        3600,
        3662,
        365 * 24 * 3600,
        100000000,
    ]

    for test in tests:
        print(format_duration(test))
