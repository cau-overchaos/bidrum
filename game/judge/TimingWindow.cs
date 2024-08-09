using System;

public class TimingWindow
{
    public TimingWindow(NoteAccuracy accuracy)
    {
        switch (accuracy)
        {
            case NoteAccuracy.PPPPerfect:
                Value = 10;
                break;
            case NoteAccuracy.Perfect:
                Value = 40;
                break;
            case NoteAccuracy.Great:
                Value = 80;
                break;
            case NoteAccuracy.Good:
                Value = 160;
                break;
            case NoteAccuracy.Bad:
                Value = 320;
                break;
            default:
                throw new ArgumentOutOfRangeException(nameof(accuracy), accuracy, null);
        }
    }

    public int Value { get; private set; }

    private bool Check(long noteTiming, long hitTiming)
    {
        long difference = Math.Abs(noteTiming - hitTiming);
        return difference <= Value;
    }

    public static NoteAccuracy GetAccuracy(long noteTiming, long hitTiming)
    {
        foreach (NoteAccuracy accuracy in new[]
                 {
                     NoteAccuracy.PPPPerfect, NoteAccuracy.Perfect, NoteAccuracy.Great, NoteAccuracy.Good,
                     NoteAccuracy.Bad
                 })
        {
            TimingWindow window = new TimingWindow(accuracy);
            if (window.Check(noteTiming, hitTiming))
                return accuracy;
        }

        return NoteAccuracy.Miss;
    }
}