using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.RegularExpressions;

namespace AOC2020
{
    public sealed class Day16 : Puzzle
    {
        public Regex rangeRegex = new Regex("([a-z ]*): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)");

        public sealed record Ticket(int[] Numbers)
        {
            public int[] InvalidFields(Range[] ranges) => Numbers.Where(n => !ranges.Any(r => r.Contains(n))).ToArray();
            public bool IsValid(Range[] ranges) => InvalidFields(ranges).Length == 0;

            public Range[][] PossibleRanges(Range[] ranges)
            {
                Range[][] result = new Range[Numbers.Length][];
                for (int i = 0; i < Numbers.Length; i++)
                {
                    result[i] = ranges.Where(r => r.Contains(Numbers[i])).ToArray();
                }
                return result;
            }
        }

        public sealed record Range(string Name, int StartA, int EndA, int StartB, int EndB)
        {
            public bool Contains(int n) => (n >= StartA && n <= EndA) || (n >= StartB && n <= EndB);
        }

        private readonly Range[] ranges;
        private readonly Ticket ticket;
        private readonly Ticket[] otherTickets;

        public Day16()
        {
            (ticket, otherTickets, ranges) = ParseInput(RawText);
        }

        public Day16(string input)
        {
            (ticket, otherTickets, ranges) = ParseInput(input);
        }

        public (Ticket myTicket, Ticket[] otherTickets, Range[] ranges) ParseInput(string input)
        {
            var parts = input.Split(Environment.NewLine + Environment.NewLine);
            var ranges = GetRanges(parts[0].ReadLines().ToArray()).ToArray();
            var myTicket = new Ticket(parts[1].ReadLines().Skip(1).First().Split(",").Select(n => int.Parse(n)).ToArray());
            var otherTickets = parts[2].ReadLines().Skip(1).Select(t => new Ticket(t.Split(",").Select(n => int.Parse(n)).ToArray())).ToArray();
            return (myTicket, otherTickets, ranges);
        }

        private IEnumerable<Range> GetRanges(string[] rangeLines)
        {
            foreach (var line in rangeLines)
            {
                var match = rangeRegex.Match(line);
                yield return new Range(match.Groups[1].Value, int.Parse(match.Groups[2].Value), int.Parse(match.Groups[3].Value), int.Parse(match.Groups[4].Value), int.Parse(match.Groups[5].Value));
            }
        }

        public long SolveTicket()
        {
            var validTickets = otherTickets.Where(t => t.IsValid(ranges));
            var p1 = validTickets.Select(t => t.PossibleRanges(ranges)).ToList();
            var numberOfFields = p1[0].Length;
            var considerRanges = new List<Range>[numberOfFields];

            for (int i = 0; i < numberOfFields; i++)
            {
                var x = p1.ConvertAll(r => r[i]);
                considerRanges[i] = ranges.Where(r => x.All(y => y.Contains(r))).ToList();
            }

            var solvedTicket = new Range[numberOfFields];
            while (considerRanges.Any(r => r.Count > 0))
            {
                for(int f = 0; f < numberOfFields; f++)
                {
                    if(considerRanges[f].Count == 1)
                    {
                        solvedTicket[f] = considerRanges[f][0];
                        foreach(var field in considerRanges)
                        {
                            field.Remove(solvedTicket[f]);
                        }
                    }
                }
            }

            int[] indexes = new int[6];
            var j = 0;
            for(int i = 0; i < solvedTicket.Length; i++)
            {
                if (solvedTicket[i].Name.StartsWith("departure"))
                {
                    indexes[j] = i;
                    j++;
                }
            }

            long result = 1;
            foreach(var index in indexes)
            {
                result *= ticket.Numbers[index];
            }
            return result;
        }

        public int GetErrorRate() => otherTickets.SelectMany(t => t.InvalidFields(ranges)).Sum();

        public override string Puzzle1() => GetErrorRate().ToString();

        public override string Puzzle2() => SolveTicket().ToString();
    }
}
