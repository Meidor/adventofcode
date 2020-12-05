using System;
using System.Collections.Generic;
using System.IO;
using System.Runtime.InteropServices;

namespace AOC2020
{
    public static class AOCExtensions
    {
        public static IEnumerable<int> ParseInt(this string[] input)
        {
            foreach (var i in input)
            {
                yield return int.Parse(i);
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
    }
}