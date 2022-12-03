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

    print(total)


def main(input_str):
    part_1(input_str)


if __name__ == "__main__":
    main(
            )
