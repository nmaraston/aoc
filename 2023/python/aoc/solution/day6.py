from dataclasses import dataclass
import math


@dataclass
class RaceSpec:
    time: int
    best_distance: int

    def ways_to_win(self) -> int:
        quad_sqrt = math.sqrt(self.time*self.time - 4*self.best_distance)
        min_ht = math.ceil(((-self.time + quad_sqrt) / -2) + 0.01)
        return (self.time - min_ht) -  min_ht + 1


class Day6Solution:
    def part_1(self, input_file) -> str:
        times = [int(n) for n in parse_numbers(input_file)]
        distances = [int(n) for n in parse_numbers(input_file)]
        race_specs = [RaceSpec(t, d) for (t, d) in zip(times, distances)]
        result = math.prod([rs.ways_to_win() for rs in race_specs])
        return str(result)

    def part_2(self, input_file) -> str:
        time = int(''.join(parse_numbers(input_file)))
        distance = int(''.join(parse_numbers(input_file)))
        return str(RaceSpec(time, distance).ways_to_win())




def parse_numbers(input_file) -> list[str]:
    return input_file.readline().rstrip().split(':')[1].lstrip().split()


