using Godot;

public enum NoteBackgroundBarDirection
{
    Left,
    Right
}

public partial class NoteBackgroundBar : VBoxContainer
{
    [Export] public NoteBackgroundBarDirection Direction { get; set; }

    public override void _Process(double delta)
    {
        this.GetNode<HBoxContainer>("NodeBackground/HBoxContainer").Alignment =
            Direction == NoteBackgroundBarDirection.Left ? AlignmentMode.End : AlignmentMode.Begin;
    }
}