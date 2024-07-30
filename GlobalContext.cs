using Godot;
using System;
using bidrumgodot.controller;

public partial class GlobalContext : Node
{
    public static GlobalContext Instance { get; private set; }

    public IJangguController JangguController { get; private set; }
    public IBillAccepter BillAccepter { get; private set; }
    public override void _Ready()
    {
        KeyboardController keyboardController = new KeyboardController();
        JangguController = keyboardController;
        BillAccepter = keyboardController;
        
        Instance = this;
    }
}
