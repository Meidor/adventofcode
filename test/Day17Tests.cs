using NUnit.Framework;

namespace AOC2020.Tests
{
    [TestFixture]
    public class Day17Tests
    {
        private const string input = @".#.
..#
###";

        [Test]
        public void Puzzle1()
        {
            var puzzle = new Day17.CubesOfLife(input.ReadLines());
            puzzle.Run(6);
            var result = puzzle.ActiveCubes.Count;
            const int expected = 112;
            Assert.AreEqual(expected, result);
        }

        [Test]
        public void Puzzle2()
        {
            var puzzle = new Day17.CubesOfLife(input.ReadLines());
            puzzle.Run(6, true);
            var result = puzzle.ActiveCubes.Count;
            const int expected = 848;
            Assert.AreEqual(expected, result);
        }
    }
}
