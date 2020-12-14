using AOC2020;
using NUnit.Framework;

namespace test
{
    [TestFixture]
    public class Day13Tests
    {
        private const string input = @"939
7,13,x,x,59,x,31,19";

        [Test]
        public void Puzzle1()
        {
            var puzzle = new Day13(input.ReadLines());
            var result = puzzle.Puzzle1();
            const string expected = "295";
            Assert.AreEqual(expected, result);
        }

        [Test]
        public void Puzzle2()
        {
            var puzzle = new Day13(input.ReadLines());
            var result = puzzle.Puzzle2();
            const string expected = "1068781";
            Assert.AreEqual(expected, result);
        }
    }
}
