import re


import numpy as np


def part_1(input_str: str) -> int:
    result = 0
    input_list = input_str.strip().split("\n")

    stackstr = (
"""
[C]         [S] [H]                
[F] [B]     [C] [S]     [W]        
[B] [W]     [W] [M] [S] [B]        
[L] [H] [G] [L] [P] [F] [Q]        
[D] [P] [J] [F] [T] [G] [M] [T]    
[P] [G] [B] [N] [L] [W] [P] [W] [R]
[Z] [V] [W] [J] [J] [C] [T] [S] [C]
[S] [N] [F] [G] [W] [B] [H] [F] [N]
""".strip())

    omg = {}

    # unreal
    size = len(stackstr.split("\n")[0]) + 1
    for i, char in enumerate(stackstr):
        if char.isalpha():
            omg[i % size] = omg[i % size] + char if ( i % size ) in omg else char

             # offset
    stacks = [None] + [list(reversed(s[1])) for s in sorted(omg.items())]
    
    # actual logic
    for line in input_list:
        nums = [int(s) for s in re.split("[a-z]+\s?", line)[1:]]

        moves = nums[0]
        start = stacks[nums[1]]
        destination = stacks[nums[2]]
    
        for _ in range(moves):
            destination.append(start.pop())
        

    print("".join([s.pop() for s in stacks[1:]]))
    return result


def part_2(input_str: str) -> int:
    result = 0
    input_list = input_str.strip().split("\n")

    stackstr = (
"""
[C]         [S] [H]                
[F] [B]     [C] [S]     [W]        
[B] [W]     [W] [M] [S] [B]        
[L] [H] [G] [L] [P] [F] [Q]        
[D] [P] [J] [F] [T] [G] [M] [T]    
[P] [G] [B] [N] [L] [W] [P] [W] [R]
[Z] [V] [W] [J] [J] [C] [T] [S] [C]
[S] [N] [F] [G] [W] [B] [H] [F] [N]
""".strip())

    omg = {}

    # unreal
    size = len(stackstr.split("\n")[0]) + 1
    for i, char in enumerate(stackstr):
        if char.isalpha():
            omg[i % size] = omg[i % size] + char if ( i % size ) in omg else char

             # offset
    stacks = [None] + [list(reversed(s[1])) for s in sorted(omg.items())]

    # actual logic
    for line in input_list:
        nums = [int(s) for s in re.split("[a-z]+\s?", line)[1:]]

        moves = nums[0]
        start = stacks[nums[1]]
        destination = stacks[nums[2]]
    
        stacks[nums[1]] = start[:-moves]
        destination += start[-moves:]
    
    
    print("".join([s.pop() for s in stacks[1:]]))
    return result


def main(input_str):
    if len(input_str.strip()) == 0:
        raise NotImplementedError("puzzle input string missing")

    print(part_1(input_str))
    print(part_2(input_str))


if __name__ == "__main__":
    main(
)

