using AOC2020.Helpers;
using System.Collections.Generic;
using System.Linq;

namespace AOC2020
{
    public sealed class Day13 : Puzzle
    {
        public long EarliestTime { get; init; }
        public Dictionary<long, long> Schedule { get; init; }

        public Day13()
        {
            EarliestTime = long.Parse(InputLines[0]);
            Schedule = ParseSchedule(InputLines[1]);
        }

        public Day13(string[] input)
        {
            EarliestTime = long.Parse(input[0]);
            Schedule = ParseSchedule(input[1]);
        }

        public (long id, long time) NextDeparture(long busId)
        {
            var time = EarliestTime;
            while (time % busId != 0)
            {
                time++;
            }
            return (busId, time);
        }

        private static Dictionary<long, long> ParseSchedule(string schedule)
        {
            Dictionary<long, long> result = new();
            var ids = schedule.Split(',');
            for (var i = 0; i < ids.Length; i++)
            {
                if (ids[i] != "x")
                {
                    result[i] = long.Parse(ids[i]);
                }
            }
            return result;
        }

        public override string Puzzle1()
        {
            var nextDepartures = Schedule.Values.Select(NextDeparture);
            var minTime = nextDepartures.Min(x => x.time);
            var (id, time) = nextDepartures.Single(x => x.time == minTime);
            var diff = time - EarliestTime;
            return (diff * id).ToString();
        }

        public override string Puzzle2()
        {
            long timestamp = Schedule[0];
            long waitTime = Schedule[0];
            foreach(var (position, minutes) in Schedule)
            {
                if (position == 0)
                {
                    continue;
                }

                while (true)
                {
                    if((timestamp + position) % minutes == 0)
                    {
                        waitTime *= minutes;
                        break;
                    }
                    timestamp += waitTime;
                }
            }
            return timestamp.ToString();
        }
    }
}
