package aoc

import scala.io.Source
import scala.math.max
import scala.util.matching.Regex
import java.io.InputStream

class Day2Solution extends Solution {

    val entryPattern = raw"(\d+)-(\d+) ([a-z]): ([a-z]+)".r

    case class Entry(
        val min: Int,
        val max: Int,
        val letter: Char,
        val password: String
    )

    override def part1(source: Source): Result = {
      val count = source
        .getLines()
        .flatMap(parseEntry)
        .filter(e => {
          val freq = e.password.count(_ == e.letter)
          e.min <= freq && freq <= e.max
        })
        .size
      Ok(count.toString)
    }
        
    override def part2(source: Source): Result = {
      val count = source
        .getLines()
        .flatMap(parseEntry)
        .filter(e => 
          xor(e.password(e.min - 1) == e.letter, e.password(e.max - 1) == e.letter))
        .size
      Ok(count.toString)
    }

    private def parseEntry(line: String): Option[Entry] =
      line match {
          case entryPattern(min, max, letter, password) =>
            Some(Entry(min.toInt, max.toInt, letter.charAt(0), password))
          case _ =>
            None
      }

    private def xor(a: Boolean, b: Boolean): Boolean = (a && !b) || (!a && b)
}