using AOC2020;
using NUnit.Framework;
using System.IO;
using System.Linq;

namespace test
{
    [TestFixture]
    public class Day6Tests
    {
        private const string input = @"abc

a
b
c

ab
ac

a
a
a
a

b";

        [Test]
        public void Puzzle1()
        {
            var puzzleReader = new StreamReader(input.ToStream());
            var result = Day6.ParseAnswers(puzzleReader).Sum(c => c.CountAny);
            Assert.AreEqual(11, result);
        }

        [Test]
        public void Puzzle2()
        {
            var puzzleReader = new StreamReader(input.ToStream());
            var group = Day6.ParseAnswers(puzzleReader).ToList();
            var result = group.Sum(c => c.CountAll);
            Assert.AreEqual(6, result);
        }
    }
}
