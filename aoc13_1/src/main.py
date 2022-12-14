import argparse
import enum


class Value(enum.Enum):
    RIGHT = enum.auto()
    NOT_RIGHT = enum.auto()
    CONTINUE = enum.auto()


parser = argparse.ArgumentParser()
parser.add_argument("input_file")

args = parser.parse_args()

with open(args.input_file) as f:
    pairs = map(lambda x: x.split("\n"), f.read().split("\n\n"))


def process(a, b, level):
    print(" " * level * 2 + f"- Compare {a} vs {b}")
    if type(a) is list and type(b) is int:
        print(" " * (level + 1) * 2 + f"- Mixed types; convert right to [{b}] and retry comparison")
        b = [b]
        return process(a, b, level + 1)
    elif type(a) is int and type(b) is list:
        print(" " * (level + 1) * 2 + f"- Mixed types; convert left to [{a}] and retry comparison")
        a = [a]
        return process(a, b, level + 1)
    else:
        # Both are the same type
        if type(a) is int:
            if a < b:
                print(" " * (level + 1) * 2 + f"- Left side is smaller, so inputs are in the right order")
                return Value.RIGHT
            elif b < a:
                print(" " * (level + 1) * 2 + f"- Right side is smaller, so inputs are not in the right order")
                return Value.NOT_RIGHT
            else:
                return Value.CONTINUE

        elif type(a) is list:
            for c, d in zip(a, b):
                res = process(c, d, level + 1)
                if res is Value.RIGHT:
                    return Value.RIGHT
                elif res is Value.NOT_RIGHT:
                    return Value.NOT_RIGHT
            if len(a) < len(b):
                print(" " * (level + 1) * 2 + f"- Left side ran out of items, so inputs are in the right order")
                return Value.RIGHT
            elif len(b) < len(a):
                print(" " * (level + 1) * 2 + f"- Right side ran out of items, so inputs are not in the right order")
                return Value.NOT_RIGHT
            else:
                return Value.CONTINUE
    return Value.NOT_RIGHT


right_orders = []
for ind, pair_str in enumerate(pairs, 1):
    print(f"== Pair {ind} ==")
    if len(pair_str) == 0:
        continue
    pair_str = pair_str[:2]
    added = False
    pair = map(eval, pair_str)
    if process(*pair, 0) is Value.RIGHT:
        right_orders.append(ind)
    print()

print(sum(right_orders))
