using System;

namespace bidrum.game.judge;

public class Score
{
    private int _badTiming, _PPPPerfectTiming;

    public Score()
    {
        _badTiming = (new TimingWindow(NoteAccuracy.Bad)).Value;
        _PPPPerfectTiming = (new TimingWindow(NoteAccuracy.PPPPerfect)).Value;
    }

    public long Value { get; private set; } = 0;

    public void Process(long hitTiming, long noteTiming)
    {
        long difference = Math.Abs(hitTiming - noteTiming);
        double delta = (double)Math.Abs(_badTiming - difference.Clamp(_PPPPerfectTiming, _badTiming)) /
            (double)(_badTiming - _PPPPerfectTiming) * 1000.0;
        Value += (long)delta;
    }
}