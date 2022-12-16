import argparse
import enum
import itertools


parser = argparse.ArgumentParser()
parser.add_argument("input_file")

args = parser.parse_args()

with open(args.input_file) as f:
    lines = f.read().split("\n")[:-1]

sensors_lines = list(map(lambda line: line.split(":")[0][10:], lines))
sensors_x = list(map(lambda s: int(s.split(", ")[0][2:]), sensors_lines))
sensors_y = list(map(lambda s: int(s.split(", ")[1][2:]), sensors_lines))

beacons_lines = list(map(lambda line: line.split(":")[1][22:], lines))
beacons_x = list(map(lambda s: int(s.split(", ")[0][2:]), beacons_lines))
beacons_y = list(map(lambda s: int(s.split(", ")[1][2:]), beacons_lines))
# for x, y in zip(beacons_x, beacons_y):
#     print(x, y)
# for line in sensors_lines:
# print(line)
# for y in sensors_y:
# print(y)


def manhattan_distance(x0, y0, x1, y1):
    return abs(y1 - y0) + abs(x1 - x0)


def position_can_have_beacon(x, y, beacons, sensors):
    point_closer_to_sensor_than_beacon = []
    for (bx, by), (sx, sy) in zip(beacons, sensors):
        if (x, y) == (bx, by):
            return True
        elif (x, y) == (sx, sy):
            return False
        elif manhattan_distance(x, y, sx, sy) <= manhattan_distance(bx, by, sx, sy):
            return False
    return True


min_x = min([min(beacons_x), min(sensors_x)])
max_x = max([max(beacons_x), max(sensors_x)])
# print(min_x, max_x)
beacons = list(zip(beacons_x, beacons_y))
sensors = list(zip(sensors_x, sensors_y))
max_manhattan = max([manhattan_distance(bx, by, sx, sy) for (bx, by), (sx, sy) in zip(beacons, sensors)])
# print(min_x - 2, max_x + 1)
# for j in range(9, 12):
#     for i in range(min_x - 2, max_x + 2):
#         beac = True if (i, j) in beacons else False
#         sens = True if (i, j) in sensors else False
#         can_have_beac = position_can_have_beacon(i, j, beacons, sensors)
#         if beac:
#             print("B", end="")
#         elif sens:
#             print("S", end="")
#         elif can_have_beac:
#             print(".", end="")
#         else:
#             print("#", end="")
#     print()

y = 2000000
positions = [
    position_can_have_beacon(i, y, beacons, sensors)
    for i in range(min_x - max_manhattan * 2, max_x + max_manhattan * 2 + 1)
]
print(sum([p == False for p in positions]))
