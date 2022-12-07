# Copyright (c) 2022 David Chan
#
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

class Directory:
    def __init__(self, name, parent=None):
        self.name = name
        self.children = []
        self.files = []
        self.parent = parent

    def size(self):
        return sum([file[1] for file in self.files]) + sum([child.size() for child in self.children])


def get_small_dirs(root):
    small_dirs = []
    for child in root.children:
        if child.size() < 100000:
            small_dirs.append(child)
        small_dirs += get_small_dirs(child)
    return small_dirs


def main():

    # Load the input file
    with open("input.txt", "r") as f:
        lines = f.readlines()

    # Create the root directory
    root = Directory("/")
    current_dir = root

    for line in lines:
        if line.startswith("$"):
            # This is a command line
            tok = line.split()
            if tok[1] == "cd":
                # Change directory
                if tok[2] == "..":
                    # Go up one directory
                    current_dir = current_dir.parent or current_dir
                elif tok[2] == '/':
                    # Go to the root directory
                    current_dir = root
                else:
                    for child in current_dir.children:
                        if child.name == tok[2]:
                            current_dir = child
                            break
        else: # We're in LS mode --
            # This is a file or directory
            size, name = line.split()
            if size == 'dir':
                if name not in [child.name for child in current_dir.children]:
                    # This is a directory
                    new_dir = Directory(name, current_dir)
                    current_dir.children.append(new_dir)
            else:
                if name not in [file[0] for file in current_dir.files]:
                    # This is a file
                    current_dir.files.append((name, int(size)))

    print('Total Size', root.size())

    # Get every directory with size < 100000
    small_dirs = get_small_dirs(root)

    # Print the names of the small directories
    for small_dir in small_dirs:
        print(small_dir.name)

    # Get the sum of the sizes of the small dirs
    print('Sum of small dirs', sum([small_dir.size() for small_dir in small_dirs]))


if __name__ == "__main__":
    main()
