class FileTree:
    def __init__(self, parent, children, name, size):
        self.parent = parent
        self.children = children
        self.name = name
        self.size = size


current_dir = FileTree(None, [], "/", 0)
root_dir = current_dir

def change_dir(dir, current_dir):
    if dir == "/":
        return root_dir
    if dir == "..":
        return current_dir.parent
    for child in current_dir.childen:
        if child.name == dir:
            return child
    new_dir = FileTree(current_dir, [], dir, size)

def process_command(command, current_dir):
    if command[2:4] == "cd":
        return change_dir(command[4:],current_dir)
    if command[2:4] == "ls":
        return True


def process_ls():
    pass


def process_input(inp):
    for line in inp.splitlines():
        ls = False
        if line.startswith("$"):
            ls = process_command()
        elif ls:
            process_ls(line)


def main():
    input = """
    $ cd /
    $ ls
    dir a
    14848514 b.txt
    8504156 c.dat
    dir d
    $ cd a
    $ ls
    dir e
    29116 f
    2557 g
    62596 h.lst
    $ cd e
    $ ls
    584 i
    $ cd ..
    $ cd ..
    $ cd d
    $ ls
    4060174 j
    8033020 d.log
    5626152 d.ext
    7214296 k
    """
    process_input(input)


if __name__ == "__main__":
    main()
