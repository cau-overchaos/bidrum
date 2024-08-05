using System.Collections.Generic;
using Godot;

public partial class ChartLine : Control
{
    public void SetLeftNotes(IEnumerable<ChartLineNote> notes)
    {
        GetNode<NoteBackgroundBar>("HBoxContainer/Left").SetNotes(notes);
    }

    public void SetRightNotes(IEnumerable<ChartLineNote> notes)
    {
        GetNode<NoteBackgroundBar>("HBoxContainer/Right").SetNotes(notes);
    }
}