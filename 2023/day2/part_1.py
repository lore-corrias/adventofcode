#!/usr/bin/python

import re

allowed_combinations = {
    "red": 12,
    "green": 13,
    "blue": 14
}

if __name__ == "__main__":
    sum = 0
    with open("input", "r") as input:
        for game_id, game in enumerate(input):
            valid = True
            for k, v in allowed_combinations.items():
                matches = map(int, re.findall(rf'(\d+) {k}', game))
                if any([x > v for x in list(matches)]):
                    valid = False
                    break
            if valid:
                sum += int(game_id) + 1
    print(sum)
