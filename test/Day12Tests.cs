using AOC2020;
using NUnit.Framework;

namespace test
{
    [TestFixture]
    public class Day12Tests
    {
        private const string input = @"F10
N3
F7
R90
F11";

        [Test]
        public void Puzzle1()
        {
            var ship = new Day12.Ship();
            ship.FollowInstructions(input.ReadLines());
            const int expected = 25;
            Assert.AreEqual(expected, ship.ManhattanDistance);
        }

        [Test]
        public void Puzzle2()
        {
            var ship = new Day12.Ship();
            ship.FollowWaypointInstructions(input.ReadLines());
            const int expected = 286;
            Assert.AreEqual(expected, ship.ManhattanDistance);
        }
    }
}
