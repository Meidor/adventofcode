using System;
using System.Collections.Generic;
using System.Linq;

namespace AOC2020
{
    public sealed class Day9 : Puzzle
    {
        public static bool IsSum(long n, Queue<long> queue)
            => (from x in queue
                from y in queue
                where x + y == n
                select 1).Any();

        public static long FirstPart(long[] input, int queueSize = 25)
        {
            var queue = new Queue<long>(input[..queueSize]);
            for (var i = queueSize; i < input.Length; i++)
            {
                var n = input[i];
                if (!IsSum(n, queue))
                {
                    return n;
                }
                queue.Dequeue();
                queue.Enqueue(n);
            }
            throw new InvalidOperationException("Shouldn't be here...");
        }

        public static long EncryptionWeakness(long[] input, int queueSize = 25)
        {
            var solution = FirstPart(input, queueSize);
            var numbers = new List<long>();
            long sum = 0;
            var i = 0;
            var j = 0;
            do
            {
                sum += input[i];
                numbers.Add(input[i]);
                if (sum == solution)
                {
                    return numbers.Min() + numbers.Max();
                }
                else if (sum < solution)
                {
                    i++;
                }
                else if (sum > solution)
                {
                    numbers.Clear();
                    sum = 0;
                    j++;
                    i = j;
                }
            }
            while (i < input.Length);
            throw new InvalidOperationException("Shouldn't be here...");
        }

        public override string Puzzle1() => FirstPart(InputLines.ParseLong().ToArray()).ToString();

        public override string Puzzle2() => EncryptionWeakness(InputLines.ParseLong().ToArray()).ToString();
    }
}
