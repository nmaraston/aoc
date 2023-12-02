_NUM_WORD_MAP: dict[str, int] = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
}

class Day1Solution:
    def part_1(self, input_file) -> str:
        sum = 0
        for line in input_file:
            nums = [c for c in line if c.isdigit()]
            sum += ((nums[0] * 10) + nums[-1])

        return str(sum)

    def part_2(self, input_file) -> str:
        sum = 0
        for line in input_file:
            first = find_first_num(line)
            last = find_last_num(line)
            sum += ((first * 10) + last)

        return str(sum)


def find_first_num(line: str) -> int:
    for i in range(len(line)):
        if line[i].isdigit():
            return int(line[i])
        for (k, v) in _NUM_WORD_MAP.items():
            if line[i:].startswith(k):
                return v

    raise ValueError(f"No number found in line: {line}")

def find_last_num(line: str) -> int:
    i = len(line) - 1
    while i >= 0:
        if line[i].isdigit():
            return int(line[i])
        for (k, v) in _NUM_WORD_MAP.items():
            if line[i:].startswith(k):
                return v
        i -= 1

    raise ValueError(f"No number found in line: {line}")

