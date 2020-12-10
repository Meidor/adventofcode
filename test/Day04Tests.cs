using AOC2020;
using AOC2020.Models;
using NUnit.Framework;
using System;
using System.IO;
using System.Linq;

namespace test
{
    [TestFixture]
    public class Day04Tests
    {
        private const string input = @"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in

ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";

        [Test]
        public void Puzzle1()
        {
            var puzzleReader = new StreamReader(input.ToStream());
            var result = Day04.ParsePassports(puzzleReader).ToList();

            Assert.AreEqual(3, result.Count(p => p.IsComplete));
        }

        [Test]
        public void Puzzle2()
        {
        }
    }
}
