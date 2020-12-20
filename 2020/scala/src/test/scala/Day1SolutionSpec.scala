package aoc

import org.scalatest.flatspec.AnyFlatSpec
import scala.io.Source

class Day1SolutionSpec extends AnyFlatSpec {

    "part 1" should "return 514579 for sample input" in {
        val source = sourceFromInts(1721, 979, 366, 299, 675, 1456)
        val underTest = new Day1Solution

        val result = underTest.part1(source)

        assert(Ok("514579") == result)
    }

    "(solve) part 1" should "return _ for real puzzle input" in {
        val source = Source.fromResource("input/day-1.txt")
        val underTest = new Day1Solution

        val result = underTest.part1(source)

        assert(Ok("913824") == result)
    }

    "part 2" should "return 241861950 for sample input" in {
        val source = sourceFromInts(1721, 979, 366, 299, 675, 1456)
        val underTest = new Day1Solution

        val result = underTest.part2(source)

        assert(Ok("241861950") == result)
    }

    "(solve )part 2" should "return 240889536 for sample input" in {
        val source = Source.fromResource("input/day-1.txt")
        val underTest = new Day1Solution

        val result = underTest.part2(source)

        assert(Ok("240889536") == result)
    }

    private def sourceFromInts(ints: Int *): Source =
      Source.fromString(ints.map(_.toString).mkString("\n"))
}