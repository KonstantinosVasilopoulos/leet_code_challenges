object Main extends App {
  Solution.twoSum(Array(2, 7, 11, 15), 9).foreach(println)
  Solution.twoSum(Array(3, 2, 4), 6).foreach(println)
  Solution.twoSum(Array(3, 3), 6).foreach(println)
}

object Solution {
    def twoSum(nums: Array[Int], target: Int): Array[Int] = {
      for (i <- 0 until nums.length - 1) {
        for (j <- i + 1 until nums.length) {
          if (nums(i) + nums(j) == target) {
            return Array(i, j)
          }
        }
      }

      Array[Int](-1, -1)
    }
}
