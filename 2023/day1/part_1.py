
if __name__ == '__main__':
    sum = 0
    with open("input", "r") as input:
        for line in input.readlines():
            integers = list(filter(lambda character: character.isdigit(), line))
            if len(integers) > 0:
                sum += int("".join(integers[0] + integers[-1]))
    print(sum)

