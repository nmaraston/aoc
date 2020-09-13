package aoc

import java.io.PrintStream
import java.util.concurrent.Callable

import picocli.CommandLine.{Command, Parameters}

@Command(
  name = "sol",
  mixinStandardHelpOptions = true
)
class AocCli(val printStream: PrintStream) extends Callable[Int] {

  @Parameters(index = "0") private var day: Int = 0
  @Parameters(index = "1") private var part: Int = 0

  private val SolutionIndex: Map[Int, Solution] = Map()

  override def call(): Int = {
    val result = (SolutionIndex.get(day), part) match {
      case (Some(solution), 1) => solution.part1()
      case (Some(solution), 2) => solution.part2()
      case (None, _) => Error(f"No solution found for day=$day and part=$part")
    }

    printStream.println(result.either())
    if (result.isOk()) 0 else 1
  }
}
