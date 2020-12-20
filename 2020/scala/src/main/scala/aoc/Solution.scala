package aoc

import scala.io.Source

trait Result {

    def either(): String = this match {
        case Ok(answer) => answer
        case Error(msg) => msg
    }

    def isOk(): Boolean = this match {
        case Ok(_) => true
        case _ => false
    }
}
case class Ok(answer: String) extends Result
case class Error(msg: String) extends Result

trait Solution {
    def part1(source: Source): Result
    def part2(source: Source): Result
}