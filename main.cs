using Godot;
using System;
using bidrumgodot.controller;

public partial class main : Node
{
    private IBillAccepter _billAccepter;
    private IJangguController _jangguController;
    private int _coins = 0;
    private void _ready()
    {
        KeyboardController keyboardController = new KeyboardController();
        _billAccepter = keyboardController;
        _jangguController = keyboardController;
    }

    private void _process(float delta)
    {
        if (_billAccepter.GetCoins() > 0)
        {
            _billAccepter.ConsumeCoins(1);
            _coins += 1;
            ((Label)GetNode("Coin")).Text = $"COIN: {_coins}";
        }
    }
}
