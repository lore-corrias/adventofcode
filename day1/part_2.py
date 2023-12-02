
mapping = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
}

if __name__ == "__main__":
    sum = 0
    with open("input") as input:
        for line in input.readlines():
            new_line = line
            for map_element in mapping.keys():
                if map_element in new_line:
                    new_line = new_line.replace(
                        map_element, 
                        # ci sono degli elementi tipo eightwothree che sono
                        # molto fastidiosi, dato che "eight" -> 8 impedisce
                        # di parsare il 2. la soluzione più veloce è rimpiazzare
                        # "eight" con "e8t", dato che così si evitano i falsi
                        # positivi per via del primo/ultimo carattere
                        map_element[0] + str(mapping[map_element]) + map_element[-1]
                    )
            numbers = list(filter(lambda c: c.isdigit(), new_line))
            sum += int(numbers[0] + numbers[-1])
    print(sum) 
