# find and return the biggest sum of values
def main(input_str: str) -> int:
    elves: list = input.split("\n\n")
    most_calories = 0
    for elf in elves:
        calories: list = [int(i) for i in elf.strip().split("\n")]
        if sum(calories) > most_calories:
            most_calories = sum(calories)

    return most_calories


if __name__ == "__main__":
    main(input_str)

