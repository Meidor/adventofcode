using AOC2020;
using NUnit.Framework;

namespace test
{
    [TestFixture]
    public class Day15Tests
    {
        [TestCase(new[] { 1, 3, 2 }, 1)]
        [TestCase(new[] { 2, 1, 3 }, 10)]
        [TestCase(new[] { 1, 2, 3 }, 27)]
        [TestCase(new[] { 2, 3, 1 }, 78)]
        [TestCase(new[] { 3, 2, 1 }, 438)]
        [TestCase(new[] { 3, 1, 2 }, 1836)]
        public void Puzzle1(int[] numbers, int expected)
        {
            var game = new Day15.MemoryGame(numbers, 2020);
            game.TakeTurns();
            var result = game.LastSpoken;
            Assert.AreEqual(expected, result);
        }

        [TestCase(new[] { 0, 3, 6 }, 175594)]
        [TestCase(new[] { 1, 3, 2 }, 2578)]
        [TestCase(new[] { 2, 1, 3 }, 3544142)]
        [TestCase(new[] { 1, 2, 3 }, 261214)]
        [TestCase(new[] { 2, 3, 1 }, 6895259)]
        [TestCase(new[] { 3, 2, 1 }, 18)]
        [TestCase(new[] { 3, 1, 2 }, 362)]
        public void Puzzle2(int[] numbers, int expected)
        {
            var game = new Day15.MemoryGame(numbers, 30000000);
            game.TakeTurns();
            var result = game.LastSpoken;
            Assert.AreEqual(expected, result);
        }
    }
}
