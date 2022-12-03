import re


def part_1(input_str):
    total = 0
    rucksacks = input_str.split("\n")

    for rucksack in rucksacks:
        size = len(rucksack) // 2

        char = ''

        left, right = rucksack[size:], rucksack[:size]
        for ch in left:
            if ch in right:
                char = ch

        total += ord(char) - 96 if char.islower() else ord(char) - 64 + 26

    return total


def part_2(input_str):
    total = 0

    # split at every third occurrence of \n
    rucksacks = re.findall("\n".join(["[^\n]+"] * 3), input_str)

    char = ""
    for triple in rucksacks:
        ruck_1, ruck_2, ruck_3 = triple.split("\n")
        for ch in ruck_1:
            if ch in ruck_2:
                if ch in ruck_3:
                    char = ch


        total += ord(char) - 96 if char.islower() else ord(char) - 64 + 26
    return total
    


def main(input_str):
    print(part_1(input_str))
    print(part_2(input_str))


if __name__ == "__main__":
    main(
            )
