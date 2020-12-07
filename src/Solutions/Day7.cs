using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace AOC2020
{
    public sealed class Day7 : Puzzle
    {
        public record Bag(string Color, List<Bag.Content> Contents)
        {
            public record Content(string Color, int Amount);

            public bool CanContain(string color, IList<Bag> bags)
                => Contents.Any(c => c.Color == color || bags.Single(b => b.Color == c.Color).CanContain(color, bags));

            public int CountContents(IList<Bag> bags)
                => Contents.Sum(c => c.Amount + (c.Amount * bags.Single(b => b.Color == c.Color).CountContents(bags)));
        }

        private static IEnumerable<Bag.Content> ParseContents(string contents)
        {
            if (contents != "no other bags.")
            {
                foreach (var bag in contents.Split(", "))
                {
                    var res = bag.Split(" ", 2);
                    yield return new Bag.Content(res[1].TrimEnd(" bags.".ToCharArray()), int.Parse(res[0]));
                }
            }
        }

        public static IEnumerable<Bag> ParseInput(StreamReader puzzleReader)
        {
            while (!puzzleReader.EndOfStream)
            {
                var split = puzzleReader.ReadLine()!.Split(" contain ");
                yield return new Bag(split[0].TrimEnd(" bags".ToCharArray()), ParseContents(split[1]).ToList());
            }
            puzzleReader.Rewind();
        }

        public static int CountCanContain(string color, IList<Bag> bags) => bags.Count(b => b.CanContain(color, bags));
        public static int CountAmount(string color, IList<Bag> bags) => bags.Single(b => b.Color == color).CountContents(bags);

        public override string Puzzle1() => CountCanContain("shiny gold", ParseInput(PuzzleReader).ToList()).ToString();
        public override string Puzzle2() => CountAmount("shiny gold", ParseInput(PuzzleReader).ToList()).ToString();
    }
}
