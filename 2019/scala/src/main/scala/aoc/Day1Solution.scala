package aoc

import scala.io.Source
import scala.math.max
import java.io.InputStream

class Day1Solution extends Solution {

  override def part1(source: Source): Result = {
    val result = source 
      .getLines
      .map(_.toInt)
      .map(fuelReq)
      .sum
    Ok(result.toString)
  }

override def part2(source: Source): Result = {

    def betterFuelReq(mass: Int): Int = fuelReq(mass) match {
      case 0 => 0
      case value => value + betterFuelReq(value)
    }

    val result = source 
      .getLines
      .map(_.toInt)
      .map(betterFuelReq)
      .sum
    Ok(result.toString)
  }

  private def fuelReq(mass: Int): Int = max((mass / 3) - 2, 0)
}
