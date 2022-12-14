def part_1(input_str: str) -> int:
    result = 0
    input_list = input_str.split("\n")


    ...


    return result


def part_2(input_str: str) -> int:
    result = 0
    input_list = input_str.split("\n")
    

    ...


    return result


def main(input_str):
    if len(input_str.strip()) == 0:
        raise NotImplementedError("puzzle input string missing")

    print(part_1(input_str))
    print(part_2(input_str))


if __name__ == "__main__":
    with open("input.txt") as f:
        main(f.read)

