from collections import defaultdict

class Day4Solution:
    def part_1(self, input_file) -> str:
        result = 0
        for line in input_file:
            split = line.rstrip().split(': ')[1].split(' | ')
            nums_winning = { int(n) for n in split[0].split() }
            nums_have = { int(n) for n in split[1].split() }
            match_count = len(nums_winning.intersection(nums_have))
            result += int((2 ** (match_count - 1)))
        return str(result)


    def part_2(self, input_file) -> str:
        instances = defaultdict(int)

        for line in input_file:
            line_split = line.rstrip().split(': ')

            card_num = int(line_split[0].split()[1])
            num_split = line_split[1].split(' | ')

            nums_winning = { int(n) for n in num_split[0].split() }
            nums_have = { int(n) for n in num_split[1].split() }
            match_count = len(nums_winning.intersection(nums_have))

            instances[card_num] += 1
            for card in range(card_num + 1, card_num + match_count + 1):
                instances[card] += instances[card_num]

        return str(sum(instances.values()))








