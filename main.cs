using Godot;
using System;
using bidrumgodot.controller;

public partial class main : Node
{
    private IBillAccepter _billAccepter;
    private IJangguController _jangguController;
    private int _coins = 0;
    public override void _Ready()
    {
        _billAccepter = GlobalContext.Instance.BillAccepter;
        _jangguController = GlobalContext.Instance.JangguController;
    }

    public override void _Process(double delta)
    {
        if (_billAccepter.GetCoins() > 0)
        {
            _billAccepter.ConsumeCoins(1);
            _coins += 1;
            ((Label)GetNode("Coin")).Text = $"COIN: {_coins}";
        }
    }
}
