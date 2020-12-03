namespace AOC2020.Models
{
    public record Point(int X, int Y)
    {
        public static implicit operator Point((int, int) point) => new Point(point.Item1, point.Item2);

        public static Point operator +(Point a, Point b)
        {
            return new Point(a.X + b.X, a.Y + b.Y);
        }

        public static Point operator -(Point a, Point b)
        {
            return new Point(a.X - b.X, a.Y - b.Y);
        }
    }
}
