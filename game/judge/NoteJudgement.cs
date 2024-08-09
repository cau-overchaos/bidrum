using Bidrum.DataStructLib;

public class NoteJudgement
{
    public NoteJudgement(GameNote note, NoteAccuracy accuracy)
    {
        Accuracy = accuracy;
        Note = note;
    }

    public NoteAccuracy Accuracy { get; private set; }
    public GameNote Note { get; private set; }
}