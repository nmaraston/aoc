package aoc

import picocli.CommandLine

object Main {
  def main(args: Array[String]): Unit = {
    val cli = new CommandLine(new AocCli(System.out))
    cli.execute(args : _*)
  }
}
