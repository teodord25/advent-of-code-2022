def main(input_str):
    pairs = input_str.split("\n")

    total = 0
    for pair in pairs:
        left, right = [[int(s) for s in i] for i in [j.split("-") for j in pair.split(",")]]

        if left[0] <= right[0] and left[1] >= right[1]:
            total += 1

        elif right[0] <= left[0] and right[1] >= left[1]:
            total += 1

    print(total)

if __name__ == "__main__":
    main(input_str)
