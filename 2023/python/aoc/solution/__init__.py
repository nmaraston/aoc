from aoc.solution.day1 import Day1Solution


def get_solution(day: int):
    match day:
        case 1:
            return Day1Solution()
        case _:
            raise ValueError(f"No solution implemented for given day {day}")

