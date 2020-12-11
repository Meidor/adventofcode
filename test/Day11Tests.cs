using AOC2020;
using NUnit.Framework;

namespace test
{
    [TestFixture]
    public class Day11Tests
    {
        private const string input = @"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        [Test]
        public void Puzzle1()
        {
            var day11 = new Day11(input.ReadLines());
            var result = day11.Puzzle1();
            const string expected = "37";
            Assert.AreEqual(expected, result);
        }

        [Test]
        public void Puzzle2()
        {
            var day11 = new Day11(input.ReadLines());
            var result = day11.Puzzle2();
            const string expected = "26";
            const string expected2 = @"#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#";
            Assert.AreEqual(expected, result);
            Assert.AreEqual(expected2, day11.GoC.ToString());
        }
    }
}
