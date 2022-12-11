# Copyright (c) 2022 David Chan
#
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

from typing import Tuple, Optional
from dataclasses import dataclass
import tqdm

@dataclass
class Monkey:
    items: list
    operation: Tuple[str, Optional[int], Optional[int]]
    test: int
    target: Tuple[int, int]
    inspections: int = 0


monkeys = [
    Monkey([79, 98], ('*', None, 19), 23, (2, 3)),
    Monkey([54, 65, 75, 74], ('+', None, 6), 19, (2, 0)),
    Monkey([79, 60, 97], ('*', None, None), 13, (1, 3)),
    Monkey([74], ('+', None, 3), 17, (0, 1)),
]

for _ in tqdm.tqdm(range(1000)):
    for m in monkeys:
        for i in m.items:
            new_item_value = (m.operation[1] or i) * (m.operation[2] or i) if m.operation[0] == '*' else (m.operation[1] or i) + (m.operation[2] or i)
            m.inspections += 1
            if new_item_value > (23 * 19 * 17 * 17):
                # If the new item value is greater than the product of all the monkeys' tests, then we need to reduce it
                new_item_value = new_item_value % (23 * 19 * 13 * 17)
            if new_item_value % m.test == 0:
                monkeys[m.target[0]].items.append(new_item_value)
            else:
                monkeys[m.target[1]].items.append(new_item_value)
        m.items = []

sorted_by_inspections = sorted(monkeys, key=lambda m: m.inspections)
# Print all of the monkeys and their inspections
for (i, m) in enumerate(monkeys):
    print(i, m.inspections, m.items)
