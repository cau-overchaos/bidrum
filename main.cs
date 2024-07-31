using Godot;
using System;
using bidrumgodot;
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

    private int price()
    {
        return GameSettings.GetSettings().Price;
    }

    /// <summary>
    /// Avaialble credits for playing game.
    ///
    /// For example, If 7 coins are not used yet and the price is 3 coins, return value will be 2. 
    /// </summary>
    /// <returns>Available credits for playing game</returns>
    private int credits()
    {
        return _coins / price();
    }

    /// <summary>
    /// Leftover coins which cannot be a credit
    ///
    /// For example, If 7 coins are not used yet and the price is 3 coins, return value will be 1.
    /// </summary>
    /// <returns>Leftover coins</returns>
    private int leftoverCoins()
    {
        return _coins % price();
    }
    
    public override void _Process(double delta)
    {
        if (_billAccepter.GetCoins() > 0)
        {
            _billAccepter.ConsumeCoins(1);
            _coins += 1;
        }

        String coinText;
        if (GameSettings.GetSettings().Uncommericial)
        {
            coinText = "FREE PLAY (UNCOMMERCIAL)";
        }
        else if (price() == 0)
        {
            coinText = "FREE PLAY";
        }
        else if (price() == 1)
        {
            coinText = $"CREDIT: {_coins}";
        }
        else
        {
            coinText = $"CREDIT: {credits()} ({leftoverCoins()}/{price()})";
        }

        ((Label)GetNode("Coin")).Text = coinText;
    }
}
