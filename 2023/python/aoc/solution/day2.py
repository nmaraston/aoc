from collections import defaultdict
import math


_MAX_CUBE_COUNTS: dict[str, int] = { "red": 12, "green": 13, "blue": 14 }


class Day2Solution:
    def part_1(self, input_file) -> str:
        sum = 0
        for line in input_file:
            game = parse_line(line)
            valid = True
            for subset in game.subsets:
                for (color, count) in subset.items():
                    if _MAX_CUBE_COUNTS[color] < count:
                        valid = False
            if valid:
                sum += game.id
        return str(sum)


    def part_2(self, input_file) -> str:
        sum = 0
        for line in input_file:
            game = parse_line(line)
            max_counts = defaultdict(int)
            for subset in game.subsets:
                for (color, count) in subset.items():
                    max_counts[color] = max(max_counts[color], count)
            sum += math.prod(max_counts.values())

        return str(sum)


class Game:
    def __init__(self, id: int):
        self.id = id
        self.subsets = []

    def add_subset(self, cubes: dict[str, int]):
        self.subsets.append(cubes)


def parse_line(line: str) -> Game:
    game_split = line.rstrip().split(': ')
    id = game_split[0].split(' ')[1]
    subsets = game_split[1].split('; ')

    game = Game(int(id))

    for subset in subsets:
        cubes = {}
        for cube_pair in subset.split(', '):
            cube_pair_split = cube_pair.split(' ')
            cubes[cube_pair_split[1]] = int(cube_pair_split[0])
        game.add_subset(cubes)
    return game

