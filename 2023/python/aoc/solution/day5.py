from sre_constants import IN_IGNORE


class Day5Solution:
    def part_1(self, input_file) -> str:
        seeds = parse_seeds(input_file)
        category_mappings = parse_category_mappings(input_file)

        locations = []
        for seed in seeds:
            location = seed
            for category_mapping in category_mappings:
                location = category_mapping.map(location)
            locations.append(location)
            
        return str(min(locations))

    def part_2(self, input_file) -> str:
        seeds = parse_seeds(input_file)
        category_mappings = parse_category_mappings(input_file)

    
        min_location = 10 ** 12 # Some big number. Python does not have MAX_INT
        for seed_range in zip(seeds[0:len(seeds):2], seeds[1:len(seeds):2]):
            for seed in range(seed_range[0], seed_range[0] + seed_range[1]):
                location = map_seed(seed, category_mappings)
                min_location = min(min_location, location)
            
        return str(min_location)


def map_seed(seed, category_mappings):
    result = seed
    for category_mapping in category_mappings:
        result = category_mapping.map(result)
    return result


class CategoryMapping:
    def __init__(self, mappings: list[tuple[int, int, int]]):
        self.mappings = mappings

    def map(self, num: int) -> int:
        result = num
        for mapping in self.mappings:
            if mapping[1] <= num and num < mapping[1] + mapping[2]:
                result = mapping[0] + (num - mapping[1])
                break
        return result


def parse_seeds(input_file):
    line = input_file.readline().rstrip()
    input_file.readline() # skip blank line below seeds line
    return [int(n) for n in line.split(': ')[1].split()]


def parse_category_mappings(input_file) -> list[CategoryMapping]:
    category_mappings = []
    category_mappings.append(parse_category_mapping(input_file))
    category_mappings.append(parse_category_mapping(input_file))
    category_mappings.append(parse_category_mapping(input_file))
    category_mappings.append(parse_category_mapping(input_file))
    category_mappings.append(parse_category_mapping(input_file))
    category_mappings.append(parse_category_mapping(input_file))
    category_mappings.append(parse_category_mapping(input_file))
    return category_mappings


def parse_category_mapping(input_file):
    input_file.readline() # skip header line

    mappings = []
    line = input_file.readline()
    while line != '\n' and line: 
        numbers = [int(n) for n in line.rstrip().split()]
        mappings.append(tuple(numbers))
        line = input_file.readline()

    return CategoryMapping(mappings)

