def part_1(input_str: str):
    result = 0
    input_list = input_str.split("\n")


    ...



def part_2(input_str: str):
    result = 0
    input_list = input_str.split("\n")
    

    ...


def main(input_str):
    if len(input_str.strip()) == 0:
        raise NotImplementedError("puzzle input string missing")

    part_1(input_str)
    part_2(input_str)


if __name__ == "__main__":
    with open("input.txt") as f:
        main(f.read)

