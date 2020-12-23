package aoc

import org.scalatest.flatspec.AnyFlatSpec
import scala.io.Source

class Day2SolutionSpec extends AnyFlatSpec {

    "part 1" should "return 1 for sample input" in {
        val source = sourceFromStrings(
            "1-3 a: abcde",
            "1-3 b: cdefg",
            "2-9 c: ccccccccc")
        val underTest = new Day2Solution

        val result = underTest.part1(source)

        assert(Ok("2") == result)
    }

    "(solve) part 1" should "return 600 for real puzzle input" in {
        val source = Source.fromResource("input/day-2.txt")
        val underTest = new Day2Solution

        val result = underTest.part1(source)

        assert(Ok("600") == result)
    }

    "part 2" should "return 1 for sample input" in {
        val source = sourceFromStrings(
            "1-3 a: abcde",
            "1-3 b: cdefg",
            "2-9 c: ccccccccc")
        val underTest = new Day2Solution

        val result = underTest.part2(source)

        assert(Ok("1") == result)
    }

    "(solve) part 2" should "return 245 for real puzzle input" in {
        val source = Source.fromResource("input/day-2.txt")
        val underTest = new Day2Solution

        val result = underTest.part2(source)

        assert(Ok("245") == result)
    }

    private def sourceFromStrings(strings: String *): Source =
        Source.fromString(strings.mkString("\n"))
}