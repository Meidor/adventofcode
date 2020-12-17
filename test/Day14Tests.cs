using NUnit.Framework;

namespace AOC2020.Tests
{
    [TestFixture]
    public class Day14Tests
    {
        private const string input = @"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

        private const string input2 = @"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

        [Test]
        public void Puzzle1()
        {
            var system = new Day14.BitmaskSystem();
            system.ProcessInput(input.ReadLines());
            var result = system.SumMemory();
            const int expected = 165;
            Assert.AreEqual(expected, result);
        }

        [Test]
        public void Puzzle2()
        {
            var system = new Day14.BitmaskSystem();
            system.ProcessInput(input2.ReadLines(), true);
            var result = system.SumMemory();
            const int expected = 208;
            Assert.AreEqual(expected, result);
        }
    }
}
