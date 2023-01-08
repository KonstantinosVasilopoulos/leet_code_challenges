import scala.collection.immutable.Map

object Main extends App {
  println(Solution.romanToInt("III"))
  println(Solution.romanToInt("LVIII"))
  println(Solution.romanToInt("MCMXCIV"))
}

object Solution {
  def romanToInt(s: String): Int = {
    val romanToIntAssociation: Map[Char, Int] = Map(
      'I' -> 1,
      'V' -> 5,
      'X' -> 10,
      'L' -> 50,
      'C' -> 100,
      'D' -> 500,
      'M' -> 1000
    )

    // Return the answer for one numeral numbers
    if (s.length == 1) {
      return romanToIntAssociation.get(s.charAt(0)).get
    }

    // Iterate over the numeral and check for combinations
    val numerals = s.toCharArray()
    var sum = 0
    var i = 0
    var numeral: Char = numerals(i)
    do {
      numeral = numerals(i)
      sum += (numeral match {
        case 'I' => numerals(i + 1) match {
          case 'V' => {
            i += 2
            4
          }
          case 'X' => {
            i += 2
            9
          }
          case _ => {
            i += 1
            1
          }
        }
        case 'X' => numerals(i + 1) match {
          case 'L' => {
            i += 2
            40
          }
          case 'C' => {
            i += 2
            90
          }
          case _ => {
            i += 1
            10
          }
        }
        case 'C' => numerals(i + 1) match {
          case 'D' => {
            i += 2
            400
          }
          case 'M' => {
            i += 2
            900
          }
          case _ => {
            i += 1
            100
          }
        }
        case _ => {
          i += 1
          romanToIntAssociation.get(numeral).get
        }
      })
    } while (i < numerals.length - 1)

    // Include the last element if it's not a part of a combination
    val combinations: Array[String] = Array("IV", "IX", "XL", "XC", "CD", "CM")
    val lastPair = numerals.slice(numerals.length - 2, numerals.length).foldLeft("")((x, y) => x.toString + y.toString)
    if (!combinations.contains(lastPair)) {
      return sum + romanToIntAssociation.get(numerals.last).get
    }

    sum
  }
}
