using NUnit.Framework;

namespace AOC2020.Tests
{
    [TestFixture]
    public class Day16Tests
    {
        private const string input = @"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

        [Test]
        public void Puzzle1()
        {
            var puzzle = new Day16(input);
            var result = puzzle.GetErrorRate();
            const int expected = 71;
            Assert.AreEqual(expected, result);
        }

        [Test]
        public void Puzzle2()
        {
            var puzzle = new Day16();
            var result = puzzle.SolveTicket();
            const long expected = 2843534243843;
            Assert.AreEqual(expected, result);
        }
    }
}
