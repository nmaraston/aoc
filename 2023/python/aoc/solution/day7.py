from collections import defaultdict
import functools


DECK = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A']
DECK_VALUES = { card: index for (index, card) in enumerate(DECK) }

class Hand:

    FIVE_OF_A_KIND = 6
    FOUR_OF_A_KIND = 5
    FULL_HOUSE = 4
    THREE_OF_A_KIND = 3
    TWO_PAIR = 2
    ONE_PAIR = 1
    HIGH_CARD = 0

    def __init__(self, cards: str):
        self.cards = cards

    def get_class(self) -> int:
        card_counts = defaultdict(int)
        for card in self.cards:
            card_counts[card] += 1

        match sorted(card_counts.values()):
            case [5]:
                return Hand.FIVE_OF_A_KIND
            case [1, 4]:
                return Hand.FOUR_OF_A_KIND
            case [2, 3]:
                return Hand.FULL_HOUSE
            case [1, 1, 3]:
                return Hand.THREE_OF_A_KIND
            case [1, 2, 2]:
                return Hand.TWO_PAIR
            case [1, 1, 1, 2]:
                return Hand.ONE_PAIR
            case _: # [1, 1, 1, 1, 1]
                return Hand.HIGH_CARD

    def __repr__(self) -> str:
        return f"Hand({self.cards})"


def hand_pair_cmp(h1_pair, h2_pair) -> int:
    h1 = h1_pair[0]
    h2 = h2_pair[0]

    h1_class = h1.get_class()
    h2_class = h2.get_class()

    if h1_class < h2_class:
        return -1
    elif h1_class > h2_class:
        return 1
    else:
        for (h1_card, h2_card) in zip(h1.cards, h2.cards):
            if DECK_VALUES[h1_card] < DECK_VALUES[h2_card]:
                return -1
            elif DECK_VALUES[h1_card] > DECK_VALUES[h2_card]:
                return 1
            else:
                continue

    return 0


class Day7Solution:
    def part_1(self, input_file) -> str:
        hand_bid_pairs = []
        for line in input_file:
            split = line.rstrip().split()
            hand_bid_pairs.append((Hand(split[0]), int(split[1])))

        hand_bid_pairs.sort(key=functools.cmp_to_key(hand_pair_cmp))

        result = 0
        for (index, pair) in enumerate(hand_bid_pairs):
            result += ((index + 1) * pair[1])

        return str(result)


    def part_2(self, input_file) -> str:
        pass
