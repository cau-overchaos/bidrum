using System;
using System.Text.Json.Serialization;

/// <summary>
/// Simple json-serializable fraction
/// </summary>
public struct Fraction
{
    private int _numerator;
    private int _denominator;

    public Fraction(int numerator, int denominator)
    {
        _denominator = denominator;
        _numerator = numerator;
        Normalize();
    }

    private static int Gcd(int a, int b)
    {
        while (a != 0 && b != 0)
        {
            if (a > b)
                a %= b;
            else
                b %= a;
        }

        return a | b;
    }

    private void Normalize()
    {
        if (_denominator < 0)
        {
            _denominator *= -1;
            _numerator *= -1;
        }

        while (true)
        {
            int gcd = Fraction.Gcd(_numerator, _denominator);
            if (Math.Abs(gcd) <= 1) break;
            _numerator = _numerator / gcd;
            _denominator = _denominator / gcd;
        }
    }

    [JsonPropertyName("numerator")]
    public int Numerator
    {
        get => _numerator;
        set
        {
            _numerator = value;
            Normalize();
        }
    }

    [JsonPropertyName("denominator")]
    public int Denominator
    {
        get => _denominator;
        set
        {
            _denominator = value;
            Normalize();
        }
    }

    #region "Operator overriding"

    public static implicit operator double(Fraction fraction)
        => (double)fraction.Numerator / fraction.Denominator;

    public static implicit operator float(Fraction fraction)
        => (float)fraction.Numerator / fraction.Denominator;

    public static Fraction operator -(Fraction fraction)
        => new Fraction(-fraction.Numerator, fraction.Denominator);

    public static Fraction operator +(Fraction a, Fraction b)
        => new Fraction(a.Numerator * b.Denominator + b.Numerator * a.Denominator, a.Denominator * b.Denominator);

    public static Fraction operator -(Fraction a, Fraction b)
        => a + (-b);

    public static Fraction operator *(Fraction a, Fraction b)
        => new Fraction(a.Numerator * b.Numerator, a.Denominator * b.Denominator);

    public static Fraction operator /(Fraction a, Fraction b)
    {
        if (b.Numerator == 0)
        {
            throw new DivideByZeroException();
        }

        return new Fraction(a.Numerator * b.Denominator, a.Denominator * b.Numerator);
    }

    #endregion
}