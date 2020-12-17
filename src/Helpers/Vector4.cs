using System;
using System.Diagnostics;

namespace AOC2020.Helpers
{
    [DebuggerDisplay("(x: {X}, y: {Y}, z: {Z}, w: {W})")]
    public readonly struct Vector4 : IEquatable<Vector4>
    {
        public double X { get; }
        public double Y { get; }
        public double Z { get; }
        public double W { get; }

        public Vector4(double x, double y) : this(x, y, 0, 0) { }
        public Vector4(double x, double y, double z) : this(x, y, z, 0) { }
        public Vector4(double x, double y, double z, double w)
        {
            X = x;
            Y = y;
            Z = z;
            W = w;
        }

        public static implicit operator Vector4((double x, double y, double z, double w) vector)
            => new Vector4(vector.x, vector.y, vector.z, vector.w);

        public static implicit operator Vector4((double x, double y, double z) vector)
            => new Vector4(vector.x, vector.y, vector.z);

        public static implicit operator Vector4((double x, double y) vector)
            => new Vector4(vector.x, vector.y);

        public static Vector4 operator +(Vector4 a, Vector4 b)
            => new Vector4(a.X + b.X, a.Y + b.Y, a.Z + b.Z, a.W + b.W);

        public static Vector4 operator -(Vector4 a, Vector4 b)
            => new Vector4(a.X - b.X, a.Y - b.Y, a.Z - b.Z, a.W - b.W);

        public static Vector4 operator *(Vector4 a, double n)
            => new Vector4(a.X * n, a.Y * n, a.Z * n, a.W * n);

        public static Vector4 operator *(Vector4 a, int n)
          => new Vector4(a.X * n, a.Y * n, a.Z * n, a.W * n);

        public static Vector4 operator *(Vector4 a, float n)
           => new Vector4(a.X * n, a.Y * n, a.Z * n, a.W * n);

        public static Vector4 operator /(Vector4 a, double n)
            => new Vector4(a.X / n, a.Y / n, a.Z / n, a.W / n);

        public static Vector4 operator /(Vector4 a, int n)
          => new Vector4(a.X / n, a.Y / n, a.Z / n, a.W / n);

        public static Vector4 operator /(Vector4 a, float n)
          => new Vector4(a.X / n, a.Y / n, a.Z / n, a.W / n);

        public static bool operator ==(Vector4 left, Vector4 right) => Equals(left, right);

        public static bool operator !=(Vector4 left, Vector4 right) => !Equals(left, right);

        public double Length => Math.Sqrt(SquareLength);
        public double SquareLength => (X * X) + (Y * Y) + (Z * Z) + (W * W);

        public Vector4 Normalize() => Length != 0 ? new Vector4(X / Length, Y / Length, Z / Length, W / Length) : new Vector4();

        public override bool Equals(object? obj) => (obj is Vector4 vector) && Equals(vector);
        public override int GetHashCode() => (X, Y, Z, W).GetHashCode();
        public bool Equals(Vector4 other) => (X, Y, Z, W) == (other.X, other.Y, other.Z, other.W);
        public override string ToString() => $"({X}, {Y}, {Z}, {W})";

        public void Deconstruct(out double x, out double y, out double z, out double w)
        {
            (x, y, z, w) = (X, Y, Z, W);
        }

        public void Deconstruct(out double x, out double y, out double z)
        {
            (x, y, z) = (X, Y, Z);
        }

        public void Deconstruct(out double x, out double y)
        {
            (x, y) = (X, Y);
        }
    }
}
