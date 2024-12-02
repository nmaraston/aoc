from aoc.solution.day1 import Day1Solution
from aoc.solution.day2 import Day2Solution
from aoc.solution.day3 import Day3Solution
from aoc.solution.day4 import Day4Solution
from aoc.solution.day5 import Day5Solution
from aoc.solution.day6 import Day6Solution
from aoc.solution.day7 import Day7Solution


def get_solution(day: int):
    match day:
        case 1:
            return Day1Solution()
        case 2:
            return Day2Solution()
        case 3:
            return Day3Solution()
        case 4:
            return Day4Solution()
        case 5:
            return Day5Solution()
        case 6:
            return Day6Solution()
        case 7:
            return Day7Solution()
        case _:
            raise ValueError(f"No solution implemented for given day {day}")

