import argparse
import enum
import itertools


class Thing(enum.Enum):
    AIR = enum.auto()
    SAND = enum.auto()
    ROCK = enum.auto()
    SOURCE = enum.auto()


parser = argparse.ArgumentParser()
parser.add_argument("input_file")

args = parser.parse_args()

with open(args.input_file) as f:
    lines = f.read().split("\n")

world = [[]]
vertexes = []
for line in lines:
    if len(line) == 0:
        continue
    row_verts = line.split(" -> ")
    tuples_str = map(lambda x: x.split(","), row_verts)
    tuples = map(lambda x: (int(x[0]), int(x[1])), tuples_str)
    vertexes.append(list(tuples))

max_x = max(map(lambda x: x[0], itertools.chain(*vertexes))) + 1000
min_x = min(map(lambda x: x[0], itertools.chain(*vertexes)))
max_y = max(map(lambda x: x[1], itertools.chain(*vertexes)))

world = []
width = max_x - min_x + 1
for _ in range(0, max_y + 1):
    row = [Thing.AIR] * max_x
    world.append(row)

world[0][500] = Thing.SOURCE
for wall in vertexes:
    for a, b in zip(wall[:-1], wall[1:]):
        if a > b:
            a, b = b, a
        if a[0] == b[0]:
            j = a[0]
            for i in range(a[1], b[1] + 1):
                world[i][j] = Thing.ROCK

        elif a[1] == b[1]:
            i = a[1]
            for j in range(a[0], b[0] + 1):
                world[i][j] = Thing.ROCK

world.append([Thing.AIR] * len(world[0]))
world.append([Thing.ROCK] * len(world[0]))


def draw(world):
    for i, row in enumerate(world):
        print(f"{i:2} ", end="")
        for tile in row[min_x - 10 : max_x + 11]:
            if tile is Thing.ROCK:
                print("#", end="")
            elif tile is Thing.SAND:
                print("o", end="")
            elif tile is Thing.SOURCE:
                print("+", end="")
            else:
                print(".", end="")
        print()


is_sand_on_rest = False
sand_position = (500, 0)
units_of_sand = 1
while True:
    i, j = sand_position[1], sand_position[0]
    if is_sand_on_rest:
        world[i][j] = Thing.SAND
        sand_position = (500, 0)
        is_sand_on_rest = False
        units_of_sand += 1
    else:
        if world[i + 1][j] is Thing.AIR:
            sand_position = (sand_position[0], sand_position[1] + 1)
        else:
            if world[i + 1][j] is not Thing.AIR:
                if world[i + 1][j - 1] is Thing.AIR:
                    sand_position = (j - 1, i + 1)
                elif world[i + 1][j + 1] is Thing.AIR:
                    sand_position = (j + 1, i + 1)
                else:
                    is_sand_on_rest = True
                    if sand_position == (500, 0):
                        break


# draw(world)

print(units_of_sand)
