using AOC2020;
using NUnit.Framework;
using System.IO;
using System.Linq;

namespace test
{
    [TestFixture]
    public class Day7Tests
    {
        private const string input = @"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        private const string input2 = @"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        [Test]
        public void Puzzle1()
        {
            var puzzleReader = new StreamReader(input.ToStream());
            const int expected = 4;
            var result = Day7.CountCanContain("shiny gold", Day7.ParseInput(puzzleReader).ToList());
            Assert.AreEqual(expected, result);
        }

        [Test]
        public void Puzzle2()
        {
            var puzzleReader = new StreamReader(input.ToStream());
            var puzzleReader2 = new StreamReader(input2.ToStream());
            const int expected = 32;
            const int expected2 = 126;
            var result = Day7.CountAmount("shiny gold", Day7.ParseInput(puzzleReader).ToList());
            var result2 = Day7.CountAmount("shiny gold", Day7.ParseInput(puzzleReader2).ToList());
            Assert.AreEqual(expected, result);
            Assert.AreEqual(expected2, result2);
        }
    }
}
