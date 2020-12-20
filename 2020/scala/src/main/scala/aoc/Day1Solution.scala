package aoc

import scala.io.Source
import scala.math.max
import java.io.InputStream

class Day1Solution extends Solution {

    override def part1(source: Source): Result = {
        val ints = source.getLines().map(_.toInt).toSeq
        cross(ints)
          .find { case (x, y) => x + y == 2020 }
          .map { case (x, y) => Ok((x * y).toString) }
          .getOrElse(Error("No solution found"))
    }
        
    override def part2(source: Source): Result = {
        val ints = source.getLines().map(_.toInt).toSeq
        triples(ints)
          .find { case (x, y, z) => x + y + z == 2020 }
          .map { case (x, y, z) => Ok((x * y * z).toString) }
          .getOrElse(Error("No solution found"))
    }

    private def cross[T](xs: Iterable[T]): Iterable[(T, T)] =
        for (x <- xs; y <- xs) yield (x, y)

    private def triples[T](xs: Iterable[T]): Iterable[(T, T, T)] =
        for (x <- xs; y <- xs; z <- xs) yield (x, y, z)
}