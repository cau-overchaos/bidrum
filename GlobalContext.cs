using Godot;
using bidrumgodot.controller;

public partial class GlobalContext : Node
{
    public static GlobalContext Instance { get; private set; }

    public IJangguHardware JangguHardware { get; private set; }
    public IBillAccepter BillAccepter { get; private set; }
    public override void _Ready()
    {
        KeyboardHardware keyboardHardware = new KeyboardHardware();
        JangguHardware = keyboardHardware;
        BillAccepter = keyboardHardware;
        
        Instance = this;
    }
}
