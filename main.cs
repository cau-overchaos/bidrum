using Godot;
using System;
using bidrumgodot;
using bidrumgodot.controller;

public partial class main : Node
{
    private IBillAccepter _billAccepter;
    private Janggu _janggu;
    private DateTime _startedAt;
    private Credits _credits;
    public override void _Ready()
    {
        _billAccepter = GlobalContext.Instance.BillAccepter;
        _janggu = new Janggu(GlobalContext.Instance.JangguHardware);
        _startedAt = DateTime.Now;
        _credits = new Credits();
    }
    
    private void UpdateCoinText()
    {
        
        if (_billAccepter.GetCoins() > 0)
        {
            _billAccepter.ConsumeCoins(1);
            _credits.Coins += 1;
        }

        ((Label)GetNode("Coin")).Text = _credits.CoinText();
    }

    private bool IsGameStartKeyInputPressed()
    {
        return _janggu.State.LeftStick.IsKeydownNow || _janggu.State.RightStick.IsKeydownNow;
    }

    public override void _Process(double delta)
    {
        _janggu.Update(DateTime.Now.Subtract(_startedAt).Milliseconds);
        
        UpdateCoinText();

        if (IsGameStartKeyInputPressed() && _credits.AvailableCredits > 0)
        {
            _credits.ConsumeCredit();
            // TO-DO: impl game start logic here
        }
    }
}
