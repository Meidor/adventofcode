using AOC2020;
using AOC2020.Models;
using NUnit.Framework;
using System;

namespace test
{
    [TestFixture]
    public class Day3Tests
    {
        const string input = @"..##.........##.........##.........##.........##.........##.......
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#";

        [Test]
        public void Puzzle1()
        {
            var rows = input.Split(Environment.NewLine);
            var result = Day3.ParseMap(rows);
            var expected = new bool[] { false, false, true, true, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false };
            CollectionAssert.AreEqual(expected, result.GetRow(0));


            var result2 = Day3.CountTrees(result, new Point(0, 0), new Point(3, 1));
            var expected2 = 7;
            Assert.AreEqual(expected2, result2);
        }

        [Test]
        public void Puzzle2()
        {
            var rows = input.Split(Environment.NewLine);
            var map = Day3.ParseMap(rows);
            const int expected = 336;
            var slopes = new Point[]
            {
                new Point(1, 1),
                new Point(3, 1),
                new Point(5, 1),
                new Point(7, 1),
                new Point(1, 2),
            };

            var start = new Point(0, 0);
            var result = 1L;
            foreach(var slope in slopes)
            {
                var trees = Day3.CountTrees(map, start, slope);
                result *= trees;
            }

            Assert.AreEqual(expected, result);
;        }
    }
}
