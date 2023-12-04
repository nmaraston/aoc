from collections import defaultdict

class Day3Solution:
    def part_1(self, input_file) -> str:
        grid, numbers = parse(input_file)
        
        sum = 0
        for number in numbers:
            for neighbour in neighbours(number[0], number[1], grid):
                if is_symbol(neighbour[0]):
                    sum += int(number[0])
                    break

        return str(sum)

    def part_2(self, input_file) -> str:
        grid, numbers = parse(input_file)

        gear_to_numbers = defaultdict(list)

        for number in numbers:
            for neighbour in neighbours(number[0], number[1], grid):
                if neighbour[0] == '*':
                    gear_to_numbers[neighbour[1]].append(number)

        sum = 0
        for numbers in gear_to_numbers.values():
            if len(numbers) == 2:
                sum += (int(numbers[0][0]) * int(numbers[1][0]))

        return str(sum)


def parse(input_file) -> tuple[list[str], list[tuple[str, tuple[int, int]]]]:
    grid = []
    numbers = []

    row = 0
    in_num = False
    num = ''
    coord = None
    
    for line in input_file:
        line = line.rstrip()

        grid.append(line)

        for (col, char) in enumerate(line):
            if char.isdigit():
                if in_num:
                    num += char
                else:
                    num = char
                    coord = (row, col)
                    in_num = True
            elif in_num:
                numbers.append((num, coord))
                in_num = False

        row += 1

    return (grid, numbers)


def neighbours(num: str, coord: tuple[int, int], grid: list[str]) -> list[tuple[str, tuple[int, int]]]:
    grid_width = len(grid[0])
    grid_length = len(grid)
    num_len = len(num)

    neighbours = []

    def top_or_bottom_neighbours(row: int):
        slice_start = coord[1]
        slice_len = num_len
        if coord[1] > 0:
            slice_start -= 1
            slice_len += 1
        if coord[1] + num_len < grid_width - 1:
            slice_len += 1

        neighbours = []
        for col in range(slice_start, slice_start + slice_len):
            neighbours.append((grid[row][col], (row, col)))

        return neighbours

    if coord[0] > 0:
        neighbours += top_or_bottom_neighbours(coord[0] - 1)
    if coord[0] < grid_length - 1:
        neighbours += top_or_bottom_neighbours(coord[0] + 1)
    if coord[1] > 0:
        neighbours.append((grid[coord[0]][coord[1] - 1], (coord[0], coord[1] - 1)))
    if coord[1] + num_len < grid_width - 1:
        neighbours.append((grid[coord[0]][coord[1] + num_len], (coord[0], coord[1] + num_len)))

    return neighbours


def is_symbol(char: str) -> bool:
    return not (char.isdigit() or char == '.')






    


















