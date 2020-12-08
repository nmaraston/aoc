package aoc

import org.scalatest.flatspec.AnyFlatSpec
import scala.io.Source

class Day1SolutionSpec extends AnyFlatSpec {

    "part 1" should "return 2 for a mass of 12" in {
        val source = sourceFromInts(12)
        val underTest = new Day1Solution

        val result = underTest.part1(source)

        assert(Ok("2") == result)
    }

    "part 1" should "return 2 for a mass of 14" in {
        val source = sourceFromInts(14)
        val underTest = new Day1Solution

        val result = underTest.part1(source)

        assert(Ok("2") == result)
    }

    "part 1" should "return 654 for a mass of 1969" in {
        val source = sourceFromInts(1969)
        val underTest = new Day1Solution

        val result = underTest.part1(source)

        assert(Ok("654") == result)
    }

    "part 1" should "return 33583 for a mass of 100756" in {
        val source = sourceFromInts(100756)
        val underTest = new Day1Solution

        val result = underTest.part1(source)

        assert(Ok("33583") == result)
    }

    "part 2" should "return 2 for a mass of 12" in {
        val source = sourceFromInts(12)
        val underTest = new Day1Solution

        val result = underTest.part2(source)

        assert(Ok("2") == result)
    }

    "part 2" should "return 966 for a mass of 1969" in {
        val source = sourceFromInts(12)
        val underTest = new Day1Solution

        val result = underTest.part2(source)

        assert(Ok("2") == result)
    }

    "part 2" should "return 50346 for a mass of 100756" in {
        val source = sourceFromInts(100756)
        val underTest = new Day1Solution

        val result = underTest.part2(source)

        assert(Ok("50346") == result)
    }

    "(solve) part 1" should "return 3402609 for real puzzle input" in {
        val source = Source.fromResource("input/day-1.txt") 
        val underTest = new Day1Solution

        val result = underTest.part1(source)

        assert(Ok("3402609") == result)
    }

    "(solve) part 2" should "return 5101025 for real puzzle input" in {
        val source = Source.fromResource("input/day-1.txt") 
        val underTest = new Day1Solution

        val result = underTest.part2(source)

        assert(Ok("5101025") == result)
    }

    def sourceFromInts(ints: Int *): Source =
        Source.fromString(ints.map(_.toString).mkString("\n"))
}