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
        _billAccepter = GlobalContext.Instance.BillAccepter;
        _jangguController = GlobalContext.Instance.JangguController;
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
