from aoc.solution.day1 import Day1Solution
from aoc.solution.day2 import Day2Solution
from aoc.solution.day3 import Day3Solution


def get_solution(day: int):
    match day:
        case 1:
            return Day1Solution()
        case 2:
            return Day2Solution()
        case 3:
            return Day3Solution()
        case _:
            raise ValueError(f"No solution implemented for given day {day}")

