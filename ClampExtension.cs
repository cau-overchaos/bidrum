namespace bidrum;

public static class ClampExtension
{
    public static int Clamp(this int value, int min, int max)
    {
        return value < min ? min : value > max ? max : value;
    }

    public static long Clamp(this long value, long min, long max)
    {
        return value < min ? min : value > max ? max : value;
    }
}