using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;

namespace AOC2019
{
    public class Day3 : Puzzle
    {
        public Wire Wire1 { get; set; }
        public Wire Wire2 { get; set; }
        public List<Point> Crosspoints { get; set; }

        public Day3() : base()
        {
            Wire1 = GetWire(InputLines[0]);
            Wire2 = GetWire(InputLines[1]);
            Crosspoints = CalculateCrosspoints(Wire1, Wire2).Where(x => x != (0, 0)).ToList();
        }

        public Wire GetWire(string input)
        {
            return GetWire(GetCoordinates(input.Split(",")));
        }

        public override string Puzzle1()
        {
            return CalculateMinDistance(InputLines).ToString();
        }

        public override string Puzzle2()
        {
            return CalculateMinSteps(InputLines).ToString();
        }

        internal double CalculateMinDistance(string[] input)
        {
            return Crosspoints.Min(x => Math.Abs(x.X) + Math.Abs(x.Y));
        }

        internal double CalculateMinSteps(string[] input)
            => Crosspoints.Select(cp => StepsRequired(cp, Wire1, Wire2)).Min();

        private static double StepsRequired(Point crosspoint, Wire wire1, Wire wire2)
        {
            return StepsRequired(crosspoint, wire1) + StepsRequired(crosspoint, wire2);
        }

        private static double StepsRequired(Point point, Wire wire)
        {
            double steps = 0;
            foreach (var ls in wire)
            {
                if (ls.ContainsPoint(point))
                {
                    return steps + new LineSegment(ls.Start, point).Length;
                }
                else
                {
                    steps += ls.Length;
                }
            }
            throw new InvalidOperationException();
        }

        private static Wire GetWire(List<Point> coordinates)
        {
            var segments = new List<LineSegment>();
            Point start = new Point(0, 0);
            for (int i = 0; i < coordinates.Count; i++)
            {
                Point end = coordinates[i];
                segments.Add((start, end));
                start = end;
            }
            return new Wire(segments);
        }

        internal static IEnumerable<Point> CalculateCrosspoints(Wire wire1, Wire wire2)
        {
            foreach (var lsw1 in wire1)
            {
                foreach (var lsw2 in wire2)
                {
                    if (lsw1.Intersects(lsw2, out var crosspoint))
                    {
                        yield return crosspoint;
                    }
                }
            }
        }

        private static IEnumerable<Point> CalculateCrosspoints(IEnumerable<LineSegment> a, IEnumerable<LineSegment> b)
        {
            foreach (var sa in a)
            {
                foreach (var sb in b)
                {
                    if (sa.Intersects(sb, out Point crosspoint))
                    {
                        yield return crosspoint;
                    }
                }
            }
        }

        internal static List<Point> GetCoordinates(string[] wire)
        {
            var coords = new List<Point>();
            var currentPoint = new Point(0, 0);
            foreach (var instruction in wire)
            {
                var direction = instruction[0];
                var length = int.Parse(instruction.Substring(1));
                switch (direction)
                {
                    case 'L':
                        currentPoint += (-length, 0);
                        break;
                    case 'R':
                        currentPoint += (length, 0);
                        break;
                    case 'U':
                        currentPoint += (0, length);
                        break;
                    case 'D':
                        currentPoint += (0, -length);
                        break;
                }
                coords.Add(currentPoint);
            }
            return coords;
        }
    }

    public struct Wire : IEnumerable<LineSegment>
    {
        public Wire(IList<LineSegment> segments)
        {
            Segments = segments;
        }

        public IList<LineSegment> Segments { get; }

        public IEnumerator<LineSegment> GetEnumerator()
        {
            return Segments.GetEnumerator();
        }

        IEnumerator IEnumerable.GetEnumerator()
        {
            return Segments.GetEnumerator();
        }
    }

    public struct LineSegment : IEquatable<LineSegment>
    {
        public LineSegment(Point start, Point end)
        {
            Start = start;
            End = end;
        }

        public Point Start { get; }
        public Point End { get; }
        public double Length => Math.Sqrt(Math.Pow(Math.Abs(Start.X - End.X), 2) + Math.Pow(Math.Abs(Start.Y - End.Y), 2));
        public Point Vector => End - Start;

        public bool ContainsPoint(Point c)
        {
            var vectorAB = Vector;
            var vectorAC = c - Start;
            if (!vectorAB.Cross(vectorAC).IsZero())
            {
                return false;
            }
            var kac = vectorAB * vectorAC;
            var kab = vectorAB * vectorAB;
            return !(kac < 0 || kac > kab);
        }

        //ADAPTED FROM: https://www.codeproject.com/tips/862988/find-the-intersection-point-of-two-line-segments
        public bool Intersects(LineSegment other, out Point crosspoint)
        {
            crosspoint = new Point();
            var p = Start;
            var p2 = End;
            var q = other.Start;
            var q2 = other.End;
            var r = p2 - p;
            var s = q2 - q;
            var rxs = r.Cross(s);
            var qpxr = (q - p).Cross(r);

            // If r x s = 0 and (q - p) x r = 0, then the two lines are collinear.
            if (rxs.IsZero() && qpxr.IsZero())
            {
                // If neither 0 <= (q - p) * r = r * r nor 0 <= (p - q) * s <= s * s
                // then the two lines are collinear but disjoint.
                // No need to implement this expression, as it follows from the expression above.
                return false;
            }

            // 3. If r x s = 0 and (q - p) x r != 0, then the two lines are parallel and non-intersecting.
            if (rxs.IsZero() && !qpxr.IsZero())
                return false;

            // t = (q - p) x s / (r x s)
            var t = (q - p).Cross(s) / rxs;

            // u = (q - p) x r / (r x s)

            var u = (q - p).Cross(r) / rxs;

            // 4. If r x s != 0 and 0 <= t <= 1 and 0 <= u <= 1
            // the two line segments meet at the point p + t r = q + u s.
            if (!rxs.IsZero() && (0 <= t && t <= 1) && (0 <= u && u <= 1))
            {
                // We can calculate the intersection point using either t or u.
                crosspoint = p + t * r;

                // An intersection was found.
                return true;
            }

            // 5. Otherwise, the two line segments are not parallel but do not intersect.
            return false;
        }
        public static implicit operator LineSegment((Point start, Point end) segment) => new LineSegment(segment.start, segment.end);
        public static bool operator ==(LineSegment a, LineSegment b) => a.Equals(b);
        public static bool operator !=(LineSegment a, LineSegment b) => !(a == b);
        public void Deconstruct(out Point start, out Point end) => (start, end) = (Start, End);
        public bool Equals(LineSegment other) => this.Start == other.Start && this.End == other.End;

        public override int GetHashCode()
        {
            var hashCode = 1861411795;
            hashCode = hashCode * -1521134295 + Start.GetHashCode();
            hashCode = hashCode * -1521134295 + End.GetHashCode();
            return hashCode;
        }

        public override bool Equals(object obj)
        {
            return base.Equals(obj);
        }
    }

    public struct Point : IEquatable<Point>
    {
        public Point(double x, double y)
        {
            this.X = x;
            this.Y = y;
        }
        public double X { get; }
        public double Y { get; }
        public static implicit operator Point((double x, double y) point) => new Point(point.x, point.y);
        public static Point operator +(Point a, Point b) => (a.X + b.X, a.Y + b.Y);
        public static Point operator -(Point a, Point b) => (a.X - b.X, a.Y - b.Y);
        public static bool operator ==(Point a, Point b) => a.Equals(b);
        public static bool operator !=(Point a, Point b) => !(a == b);
        public static double operator *(Point v, Point w) => v.X * w.X + v.Y * w.Y;
        public static Point operator *(Point v, double mult) => new Point(v.X * mult, v.Y * mult);
        public static Point operator *(double mult, Point v) => new Point(v.X * mult, v.Y * mult);
        public double Cross(Point other) => X * other.Y - Y * other.X;
        public void Deconstruct(out double x, out double y) => (x, y) = (X, Y);

        public bool Equals(Point other) => this.X == other.X && this.Y == other.Y;

        public override int GetHashCode()
        {
            var hashCode = 1861411795;
            hashCode = hashCode * -1521134295 + X.GetHashCode();
            hashCode = hashCode * -1521134295 + Y.GetHashCode();
            return hashCode;
        }

        public override bool Equals(object obj)
        {
            return base.Equals(obj);
        }
    }
}