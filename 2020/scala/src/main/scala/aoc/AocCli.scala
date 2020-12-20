package aoc

import java.io.PrintStream
import java.util.concurrent.Callable
import scala.io.Source

import picocli.CommandLine.{Command, Parameters}

@Command(
    name = "sol",
    mixinStandardHelpOptions = true
)
class AocCli(val printStream: PrintStream) extends Callable[Int] {

    @Parameters(index = "0") private var day: Int = 0
    @Parameters(index = "1") private var part: Int = 0
    @Parameters(index = "2") private var inputFilename: String = _

    private val SolutionIndex: Map[Int, Solution] = Map(
        1 -> new Day1Solution()
    )

    override def call(): Int = {
        val result = (SolutionIndex.get(day), part) match {
            case (Some(solution), 1) => solution.part1(fileSource(inputFilename))
            case (Some(solution), 2) => solution.part2(fileSource(inputFilename))
            case _ => Error(f"No solution found for day '$day'")
        }

        printStream.println(result.either())
        if (result.isOk()) 0 else 1
    }

    private def fileSource(filename: String): Source =
        Source.fromResource(filename)
}
