print(sum(sorted([sum([int(i)for i in c.strip().split('\n')])for c in input_str.split('\n\n')])[-3:]))
