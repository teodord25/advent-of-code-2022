# find and return the biggest sum of values
def part_1(input_str: str) -> int:
    elves: list = input_str.split("\n\n")
    most_calories = 0
    for elf in elves:
        calories: list = [int(i) for i in elf.strip().split("\n")]
        if sum(calories) > most_calories:
            most_calories = sum(calories)

    return most_calories


# find and return the sum of the top three sums
def part_2(input_str: str) -> list:
    elves: list = input_str.split("\n\n")
    calorie_sums = []
    for elf in elves:
        calories: list = sum([int(i) for i in elf.strip().split("\n")])
        calorie_sums.append(calories)
    
    return sum(sorted(calorie_sums)[-3:])


if __name__ == "__main__":
    print(part_1(input_str))
    print(part_2(input_str))

