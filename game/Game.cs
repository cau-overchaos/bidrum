using Godot;

public partial class Game : Node
{
    [Signal]
    public delegate void GameEndedEventHandler();
}