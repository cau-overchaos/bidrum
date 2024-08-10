using System;

namespace bidrum.game.judge;

public class Health
{
    private static readonly int MaxHealth = 1000;
    public int Value { get; private set; } = Health.MaxHealth;

    public void Process(NoteAccuracy accuracy)
    {
        if (Value == 0)
            return;

        switch (accuracy)
        {
            case NoteAccuracy.PPPPerfect:
            case NoteAccuracy.Perfect:
                Value += 50;
                break;
            case NoteAccuracy.Great:
                Value += 10;
                break;
            case NoteAccuracy.Good:
                break;
            case NoteAccuracy.Bad:
                Value -= 20;
                break;
            case NoteAccuracy.Miss:
                Value -= 50;
                break;
            default:
                throw new ArgumentOutOfRangeException(nameof(accuracy), accuracy, null);
        }

        Value = Value.Clamp(0, MaxHealth);
    }
}