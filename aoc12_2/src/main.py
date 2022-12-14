import argparse

arg_parse = argparse.ArgumentParser()
arg_parse.add_argument("filename")
args = arg_parse.parse_args()

filename = args.filename

with open(filename) as f:
    content = f.read()

content = content.split("\n")[:-1]


def get_scores_matrix_and_relevant_points(content):
    start_point = None
    end_point = None
    ret = []
    a_positions = []
    for i, line in enumerate(content):
        row = []
        for j, char in enumerate(line):
            point_value = None
            if char == "S":
                start_point = i, j
                point_value = 1
            elif char == "E":
                end_point = i, j
                point_value = ord("z") - ord("a") + 1
            if char == "a":
                a_positions.append((i, j))
                point_value = ord(char) - ord("a") + 1
            else:
                point_value = ord(char) - ord("a") + 1
            row.append(point_value)
        ret.append(row)

    return ret, start_point, end_point, a_positions


class Node:
    def __init__(self):
        self.neighbors = []
        self.i = -1
        self.j = -1

    def __repr__(self):
        return f"({self.i}, {self.j})"


def is_up_possible(mat, i, j):
    if i > 0 and mat[i - 1][j] - mat[i][j] <= 1:
        return True
    return False


def is_down_possible(mat, i, j):
    height = len(mat)
    if i < height - 1 and mat[i + 1][j] - mat[i][j] <= 1:
        return True
    return False


def is_left_possible(mat, i, j):
    if j > 0 and mat[i][j - 1] - mat[i][j] <= 1:
        return True
    return False


def is_right_possible(mat, i, j):
    width = len(mat[0])
    if j < width - 1 and mat[i][j + 1] - mat[i][j] <= 1:
        return True
    return False


def get_nodes_array(scores_matrix):
    ret = []
    for i, score_row in enumerate(scores_matrix):
        row = []
        for j, elem in enumerate(score_row):
            node = Node()
            node.i = i
            node.j = j
            row.append(node)
        ret.append(row)

    for i in range(len(scores_matrix)):
        for j in range(len(scores_matrix[0])):
            node = ret[i][j]
            if is_up_possible(scores_matrix, i, j):
                up_node = ret[i - 1][j]
                node.neighbors.append(up_node)
            if is_down_possible(scores_matrix, i, j):
                down_node = ret[i + 1][j]
                node.neighbors.append(down_node)
            if is_left_possible(scores_matrix, i, j):
                left_node = ret[i][j - 1]
                node.neighbors.append(left_node)
            if is_right_possible(scores_matrix, i, j):
                right_node = ret[i][j + 1]
                node.neighbors.append(right_node)
    return ret


def get_shortest_path(nodes_matrix, start_point, end_point):
    # Initialize
    q = list()
    dist = {}

    def is_unvisited(v):
        return (v.i, v.j) in map(lambda x: x[1], q)

    for i in range(len(nodes_matrix)):
        for j in range(len(nodes_matrix[0])):
            if (i, j) == start_point:
                dist[i, j] = 0
            else:
                dist[i, j] = 100000

            q.append((dist[i, j], (i, j)))

    q = sorted(q, key=lambda x: x[0])

    while len(q) > 0:
        u = q.pop(0)
        u = u[1]
        u = nodes_matrix[u[0]][u[1]]
        for v in u.neighbors:
            if is_unvisited(v):
                alt = dist[u.i, u.j] + 1
                q.remove((dist[v.i, v.j], (v.i, v.j)))
                if alt < dist[v.i, v.j]:
                    dist[v.i, v.j] = alt
                    q.append((alt, (v.i, v.j)))
                    q = sorted(q, key=lambda x: x[0])
                else:
                    q.append((dist[v.i, v.j], (v.i, v.j)))
                    q = sorted(q, key=lambda x: x[0])
        if (u.i, u.j) == end_point:
            break
    return dist[end_point]


scores_matrix, start_point, end_point, a_positions = get_scores_matrix_and_relevant_points(content)
nodes_matrix = get_nodes_array(scores_matrix)
distances = []
for a_pos in a_positions:
    distances.append((a_pos, get_shortest_path(nodes_matrix, a_pos, end_point)))
print(min(distances, key=lambda x: x[1]))
