import click

from aoc.solution import get_solution


@click.command()
# Puzzle day number (1 - 24)
@click.argument('day', type=click.INT)
# Puzzle part (1 or 2)
@click.argument('part', type=click.INT)
# Path to puzzle input file
@click.argument('input_file_path', type=click.Path(exists=True))
def run(day: int, part: int, input_file_path: str) -> None:
    solution = get_solution(day)

    with open(input_file_path, 'r') as input_file:
        match part:
            case 1:
                print(solution.part_1(input_file))
            case 2:
                print(solution.part_2(input_file))
            case _:
                raise ValueError(f"Illegal part number specified '{part}'. Must be '1' or '2'")


