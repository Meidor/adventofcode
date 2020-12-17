using System;
using System.Diagnostics;

namespace AOC2020.Helpers
{
    [DebuggerDisplay("(x: {X}, y: {Y}, z: {Z})")]
    public readonly struct Vector3 : IEquatable<Vector3>, IComparable<Vector3>
    {
        public double X { get; }
        public double Y { get; }
        public double Z { get; }

        public Vector3(double x, double y) : this(x, y, 0) { }
        public Vector3(double x, double y, double z)
        {
            X = x;
            Y = y;
            Z = z;
        }

        public static implicit operator Vector3((double x, double y, double z) vector)
            => new Vector3(vector.x, vector.y, vector.z);

        public static implicit operator Vector3((double x, double y) vector)
            => new Vector3(vector.x, vector.y);

        public static Vector3 operator +(Vector3 a, Vector3 b)
            => new Vector3(a.X + b.X, a.Y + b.Y, a.Z + b.Z);

        public static Vector3 operator -(Vector3 a, Vector3 b)
            => new Vector3(a.X - b.X, a.Y - b.Y, a.Z - b.Z);

        public static Vector3 operator *(Vector3 a, double n)
            => new Vector3(a.X * n, a.Y * n, a.Z * n);

        public static Vector3 operator *(Vector3 a, int n)
          => new Vector3(a.X * n, a.Y * n, a.Z * n);

        public static Vector3 operator *(Vector3 a, float n)
           => new Vector3(a.X * n, a.Y * n, a.Z * n);

        public static Vector3 operator /(Vector3 a, double n)
            => new Vector3(a.X / n, a.Y / n, a.Z / n);

        public static Vector3 operator /(Vector3 a, int n)
          => new Vector3(a.X / n, a.Y / n, a.Z / n);

        public static Vector3 operator /(Vector3 a, float n)
          => new Vector3(a.X / n, a.Y / n, a.Z / n);

        public static bool operator ==(Vector3 left, Vector3 right) => Equals(left, right);

        public static bool operator !=(Vector3 left, Vector3 right) => !Equals(left, right);

        public double Dot(Vector3 other) => Dot(this, other);

        public static double Dot(Vector3 a, Vector3 b) => (a.X * b.X) + (a.Y * b.Y) + a.Z + b.Z;

        public double Length => Math.Sqrt(SquareLength);
        public double SquareLength => (X * X) + (Y * Y) + (Z * Z);

        public Vector3 Normalize() => Length != 0 ? new Vector3(X / Length, Y / Length, Z / Length) : new Vector3();

        public override bool Equals(object? obj) => (obj is Vector3 vector) && Equals(vector);
        public override int GetHashCode() => (X, Y, Z).GetHashCode();
        public bool Equals(Vector3 other) => (X, Y, Z) == (other.X, other.Y, other.Z);
        public override string ToString() => $"({X}, {Y}, {Z})";

        public void Deconstruct(out double x, out double y, out double z)
        {
            (x, y, z) = (X, Y, Z);
        }

        public void Deconstruct(out double x, out double y)
        {
            (x, y) = (X, Y);
        }

        public int CompareTo(Vector3 other) => (X, Y, Z).CompareTo((other.X, other.Y, other.Z));
    }
}
