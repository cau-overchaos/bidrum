using bidrumgodot.controller;
using Godot;

public partial class GlobalContext : Node
{
    public static GlobalContext Instance { get; private set; }

    public IJangguHardware JangguHardware { get; private set; }
    public IBillAcceptor BillAcceptor { get; private set; }

    public override void _Ready()
    {
        KeyboardHardware keyboardHardware = new KeyboardHardware();
        JangguHardware = keyboardHardware;
        BillAcceptor = keyboardHardware;

        Instance = this;
    }
}