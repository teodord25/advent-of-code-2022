def part_1(input_str: str) -> int:
    result = 0
    input_list = input_str.strip().split("\n")

    l = 0
    r = 4

    stream = input_list[0]
    while len(set(stream[l:r])) != 4:
        l += 1
        r += 1

    return r


def part_2(input_str: str) -> int:
    result = 0
    input_list = input_str.strip().split("\n")

    l = 0
    r = 14

    stream = input_list[0]
    while len(set(stream[l:r])) != 14:
        l += 1
        r += 1

    return r


def main(input_str):
    if len(input_str.strip()) == 0:
        raise NotImplementedError("puzzle input string missing")

    print(part_1(input_str))
    print(part_2(input_str))


if __name__ == "__main__":
    main(
"""
"""
)


