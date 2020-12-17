using AOC2020.Helpers;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Runtime.InteropServices;

namespace AOC2020
{
    public static class AOCExtensions
    {
        private static readonly Eratosthenes eratosthenes = new Eratosthenes();

        public static string[] ReadLines(this string input) => input.Split(Environment.NewLine);

        public static void AddRange<T>(this HashSet<T> hash, IEnumerable<T> add)
        {
            foreach (var item in add)
            {
                hash.Add(item);
            }
        }

        public static IEnumerable<int> ParseInt(this string[] input)
        {
            foreach (var i in input)
            {
                yield return int.Parse(i);
            }
        }

        public static IEnumerable<long> ParseLong(this string[] input)
        {
            foreach (var i in input)
            {
                yield return long.Parse(i);
            }
        }

        public static T[] GetRow<T>(this T[,] array, int row)
        {
            if (!typeof(T).IsPrimitive)
                throw new InvalidOperationException("Not supported for managed types.");

            if (array == null)
                throw new ArgumentNullException(nameof(array));

            int cols = array.GetUpperBound(1) + 1;
            T[] result = new T[cols];

            int size;

            if (typeof(T) == typeof(bool))
                size = 1;
            else if (typeof(T) == typeof(char))
                size = 2;
            else
                size = Marshal.SizeOf<T>();

            Buffer.BlockCopy(array, row * cols * size, result, 0, cols * size);

            return result;
        }

        private const double Epsilon = 1e-10;
        public static bool IsZero(this double d) => Math.Abs(d) < Epsilon;
        public static bool AlmostEqual(this double a, double b) => Math.Abs(a - b) < Epsilon;

        public static Stream ToStream(this string s)
        {
            var stream = new MemoryStream();
            var writer = new StreamWriter(stream);
            writer.Write(s);
            writer.Flush();
            stream.Position = 0;
            return stream;
        }

        public static void Rewind(this StreamReader sr)
        {
            sr.BaseStream.Position = 0;
            sr.DiscardBufferedData();
        }

        public static void AddOrUpdate<TKey, TValue>(this Dictionary<TKey, TValue> dict, TKey key, TValue value, Func<(TValue current, TValue value), TValue> update)
            where TKey : notnull
            where TValue : notnull
        {
            if (dict.TryGetValue(key, out var current))
            {
                dict[key] = update((current, value));
            }
            else
            {
                dict.Add(key, value);
            }
        }

        public static IEnumerable<long> GetPrimeFactors(this long value)
        {
            List<long> factors = new List<long>();
            foreach (long prime in eratosthenes)
            {
                while (value % prime == 0)
                {
                    value /= prime;
                    factors.Add(prime);
                }

                if (value == 1)
                    break;
            }
            return factors;
        }

        public static long LeastCommonMultiple(this long[] numbers)
        {
            long[][] primeFactors = numbers.Select(x => x.GetPrimeFactors().ToArray()).ToArray();
            Dictionary<long, long> maxCount = new();
            Dictionary<long, long> localCount = new();
            foreach (var factor in primeFactors)
            {
                localCount.Clear();
                for (var i = 0; i < factor.Length; i++)
                {
                    if (localCount.ContainsKey(factor[i]))
                    {
                        var val = localCount[factor[i]];
                        localCount.Remove(factor[i]);
                        localCount.Add(factor[i], val + 1);
                    }
                    else
                    {
                        localCount.Add(factor[i], 1);
                    }
                }

                foreach (var keypair in localCount)
                {
                    if (!maxCount.ContainsKey(keypair.Key) || maxCount[keypair.Key] < keypair.Value)
                    {
                        maxCount[keypair.Key] = keypair.Value;
                    }
                }
            }
            return (long)maxCount.Select(kp => Math.Pow(kp.Key, kp.Value)).Aggregate((x, y) => x * y);
        }

        public static void RemoveRange<T>(this IList<T> list, IEnumerable<T> collection)
        {
            foreach(var item in collection)
            {
                list.Remove(item);
            }
        }

        public static void RemoveRange<T>(this HashSet<T> list, IEnumerable<T> collection)
        {
            foreach (var item in collection)
            {
                list.Remove(item);
            }
        }
    }
}