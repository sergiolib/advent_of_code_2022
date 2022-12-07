import argparse

args = argparse.ArgumentParser()
args.add_argument("input_file")
input_file = args.parse_args().input_file

with open(input_file) as f:
    lines = f.read().split("\n")


class File:
    def __init__(self, name, size):
        self.name = name
        self.size = int(size)

    def create_file(directory, size, name):
        new = File(name, size)
        directory.files[name] = new

    def __repr__(self):
        return f"{self.size} {self.name}"


class Dir:
    def __init__(self, path, dirs, files, parent):
        self.path = path
        self.dirs = dirs
        self.files = files
        self.parent = parent

    def create_root():
        new = Dir("", dict(), dict(), None)
        new.parent = new
        return new

    def change_dir(self, subdir):
        if subdir == "..":
            return self.parent

        elif subdir == "/":
            # Assume self is root
            return self
        elif subdir not in self.dirs:
            new = self.dirs[subdir] = Dir(self.path + f"/{subdir}", dict(), dict(), self)
            return new
        else:
            return self.dirs[subdir]

    def __repr__(self):
        return self.path if len(self.path) > 0 else "/"

    def get_size(self):
        size = 0
        for f in self.files.values():
            size += f.size
        for d in self.dirs.values():
            size += d.get_size()
        return size


root = Dir.create_root()
current_dir = root
reading_files = False

for line in lines[:-1]:
    if line.startswith("$ cd"):
        reading_files = False
        subdir = line[5:]
        current_dir = current_dir.change_dir(subdir)
    elif line.startswith("$ ls") and not reading_files:
        reading_files = True
    elif reading_files:
        info = line.split(" ")
        if info[0] != "dir":
            File.create_file(current_dir, info[0], info[1])


def transverse(node, results):
    this_size = node.get_size()
    if this_size <= 100000:
        results.append(this_size)
    for d in node.dirs.values():
        transverse(d, results)


result = []
transverse(root, result)
print(sum(result))
