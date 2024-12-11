#!/usr/bin/python

from string import punctuation
from re import finditer


symbols = punctuation.replace(".", "")

def is_near(number, gear):
    return abs(number[0] - gear[1]) <= 1 or abs(number[1] - gear[1]) <= 1

if __name__ == "__main__":
    sum = 0
    with open("input", "r") as input:
        lines = input.readlines()
        numbers = {str(i): [] for i in range(len(lines))}
        gears = []
        for i, line in enumerate(lines):
            for number in finditer(r'\d+', line):
                numbers[str(i)].append((number.span()[0], number.span()[1]-1))
            for gear in finditer(r'\*', line):
                gears.append((i, gear.span()[0]))
        for gear in gears:
            neighbours = [int(lines[gear[0]][number[0]:number[1]+1]) for number in numbers[str(gear[0])] if is_near(number, gear)]
            if gear[0] != 0:
                neighbours += [int(lines[gear[0] - 1][number[0]:number[1]+1]) for number in numbers[str(gear[0] - 1)] if is_near(number, gear)]
            if gear[0] != len(lines) - 1:
                neighbours += [int(lines[gear[0] + 1][number[0]:number[1]+1]) for number in numbers[str(gear[0] + 1)] if is_near(number, gear)]
            if len(neighbours) == 2:
                sum += neighbours[0]*neighbours[1]
    print(sum)
