# Copyright (c) 2022 David Chan
#
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT


def pair_in_order(p0, p1):
    if isinstance(p0, int) and isinstance(p1, int):
        if p0 < p1:
            return True
        if p0 > p1:
            return False
        return None
    if isinstance(p0, int) and isinstance(p1, list):
        p0 = [p0]
        return pair_in_order(p0, p1)
    if isinstance(p0, list) and isinstance(p1, int):
        p1 = [p1]
        return pair_in_order(p0, p1)
    if isinstance(p0, list) and isinstance(p1, list):
        for i in range(len(p0)):
            if i >= len(p1):
                # Right side is shorter, so it's not in order
                return False
            value = pair_in_order(p0[i], p1[i])
            if value is not None:
                return value
        if len(p0) < len(p1):
            # Left side is shorter, so it's in order
            return True

        return None

    # No decision could be made
    return None


# Read in the input
with open("input.txt", "r") as f:
    lines = f.read().split("\n\n")

# Parse the input into pairs
pairs = []
for line in lines:
    ls = line.split('\n')
    if len(ls) == 2:
        pairs.append([eval(l) for l in ls])
    else:
        print(line)

print(pairs)
# Iterate through the pairs and check to see if they're in order
output = 0
for idx, (p0, p1) in enumerate(pairs):
    print(p0, p1)
    if pair_in_order(p0, p1):
        output += (idx + 1)

print(output)
