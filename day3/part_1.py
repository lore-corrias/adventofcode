#!/usr/bin/python

import re
from string import punctuation


symbols = punctuation.replace(".", "")
def is_special(start, end, line, line_up=None, line_down=None):
    return \
        (start != 0 and line[start-1] in symbols) or \
        (end != len(line)-1 and line[end] in symbols) or \
        (line_up is not None and any([line_up[x] in symbols for x in range(start if start==0 else start-1, end if end>=len(line_up)-1 else end+1)])) or \
        (line_down is not None and any([line_down[x] in symbols for x in range(start if start==0 else start-1, end if end>=len(line_down)-1 else end+1)]))

if __name__ == "__main__":
    sum = 0
    with open("input", "r") as input:
        lines = input.readlines()
        for i, line in enumerate(lines):
            line_numbers = re.finditer(r"\d+", line)
            for number in line_numbers:
                if is_special(
                    *number.span(),
                    line, 
                    None if i == 0 else lines[i-1],
                    None if i == len(lines)-1 else lines[i+1]
                ):
                    sum += int(number.group())
    print(sum)
