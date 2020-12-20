import Dependencies._

ThisBuild / scalaVersion     := "2.13.3"
ThisBuild / version          := "0.1.0-SNAPSHOT"

lazy val root = (project in file("."))
  .settings(
    name := "aoc",
    libraryDependencies ++= Seq(
      "info.picocli" % "picocli" % "4.5.1",
      "info.picocli" % "picocli-codegen" % "4.5.1" % "provided",
      scalaTest % Test
    )
  )

// See https://www.scala-sbt.org/1.x/docs/Using-Sonatype.html for instructions on how to publish to Sonatype.
