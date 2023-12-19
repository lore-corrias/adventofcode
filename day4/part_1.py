#!/usr/bin/python


if __name__ == '__main__':
    total = 0
    with open("input", "r") as input:
        for i, line in enumerate([x.strip() for x in input]):
            numbers, winning = [x.split(' ') for x in line.split(': ')[1].split(' | ')]
            total += int(2**(len([number for number in numbers if number in winning and number != ''])-1))
    print(total)
