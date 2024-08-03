using Godot;

public partial class Game : Node
{
    [Signal]
    public delegate void GameEndedEventHandler();

    public override void _Process(double delta)
    {
    }
}