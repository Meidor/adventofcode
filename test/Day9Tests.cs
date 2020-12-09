using AOC2020;
using NUnit.Framework;
using System;
using System.IO;
using System.Linq;

namespace test
{
    [TestFixture]
    public class Day9Tests
    {
        private const string input = @"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

        [Test]
        public void Puzzle1()
        {
            var numbers = input.Split(Environment.NewLine).ParseLong().ToArray();
            var result = Day9.FirstPart(numbers, 5);
            const long expected = 127;
            Assert.AreEqual(expected, result);
        }

        [Test]
        public void Puzzle2()
        {
            var numbers = input.Split(Environment.NewLine).ParseLong().ToArray();
            var result = Day9.EncryptionWeakness(numbers, 5);
            const long expected = 62;
            Assert.AreEqual(expected, result);
        }
    }
}
