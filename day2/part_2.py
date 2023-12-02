#!/usr/bin/python

from re import findall
from math import prod

if __name__ == "__main__":
    sum = 0
    with open("input", "r") as input:
        for game_id, game in enumerate(input):
            maximums = {
                "red": 0,
                "green": 0,
                "blue": 0
            }
            for k in maximums:
                matches = map(int, findall(rf'(\d+) {k}', game))
                maximums[k] = max(list(matches))
            sum += prod(maximums.values())
    print(sum)
