using AOC2020;
using NUnit.Framework;
using System;
using System.IO;
using System.Linq;

namespace test
{
    [TestFixture]
    public class Day10Tests
    {
        private const string input = @"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

        [Test]
        public void Puzzle1()
        {
            var day10 = new Day10();
            var numbers = input.ReadLines().ParseInt().ToList();
            numbers.Sort();
            numbers.Insert(0, 0);
            numbers.Add(numbers[^1] + 3);
            day10.adapters = numbers;

            const long expected = 220;
            Assert.AreEqual(expected, day10.CountDifferences());

            const long expected2 = 2400;
            var day10_2 = new Day10();
            Assert.AreEqual(expected2, day10_2.CountDifferences());
        }

        [Test]
        public void Puzzle2()
        {
            var day10 = new Day10();
            var numbers = input.ReadLines().ParseInt().ToList();
            numbers.Sort();
            day10.adapters = numbers;

            const long expected = 19208;
            Assert.AreEqual(expected, day10.CountCombinations());

            const long expected2 = 338510590509056;
            var day10_2 = new Day10();
            Assert.AreEqual(expected2, day10_2.CountCombinations());
        }
    }
}
