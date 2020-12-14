using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AOC2020.Models
{
    [DebuggerDisplay("(x: {X}, y: {Y})")]
    public readonly struct Point : IEquatable<Point>
    {
        public int X { get; }
        public int Y { get; }

        public Point(int x, int y)
        {
            X = x;
            Y = y;
        }

        public static implicit operator Point((int x, int y) point)
            => new Point(point.x, point.y);

        public static Point operator +(Point a, Point b)
            => new Point(a.X + b.X, a.Y + b.Y);

        public static Point operator -(Point a, Point b)
            => new Point(a.X - b.X, a.Y - b.Y);

        public static Point operator *(Point a, Point n)
            => new Point(a.X * n.X, a.Y * n.Y);

        public static Point operator *(Point a, int n)
          => new Point(a.X * n, a.Y * n);

        public static Point operator /(Point a, int n)
          => new Point(a.X / n, a.Y / n);
        public static bool operator ==(Point left, Point right) => Equals(left, right);
        public static bool operator !=(Point left, Point right) => !Equals(left, right);
        public int Dot(Point other) => Dot(this, other);
        public static int Dot(Point a, Point b) => (a.X * b.X) + (a.Y * b.Y);
        public override bool Equals(object? obj) => (obj is Point vector) && Equals(vector);
        public override int GetHashCode() => (X, Y).GetHashCode();
        public bool Equals(Point other) => (X, Y) == (other.X, other.Y);
        public override string ToString() => $"({X}, {Y})";

        public void Deconstruct(out double x, out double y)
        {
            (x, y) = (X, Y);
        }
    }
}
