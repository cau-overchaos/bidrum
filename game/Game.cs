using bidrum.controller;
using Godot;

public partial class Game : Node
{
    [Signal]
    public delegate void GameEndedEventHandler();

    private double temp = 4.0F;

    public override void _Process(double delta)
    {
        temp -= 4.0 * delta;
        float distance = 9.0F;
        ChartLineNote[] notes = new ChartLineNote[]
        {
            new ChartLineNote() { distance = (float)temp, stick = JangguStick.Left },
            new ChartLineNote() { distance = (float)temp + distance, stick = JangguStick.Right },
            new ChartLineNote() { distance = (float)temp + distance * 2, stick = JangguStick.Left },
            new ChartLineNote() { distance = (float)temp + distance * 2, stick = JangguStick.Right },
        };

        GetNode<ChartLine>("ChartLine").SetLeftNotes(notes);
        GetNode<ChartLine>("ChartLine").SetRightNotes(notes);
    }
}