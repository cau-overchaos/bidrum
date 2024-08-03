using System;
using bidrumgodot;
using bidrumgodot.controller;
using Godot;

public partial class Main : Node
{
    private IBillAcceptor _billAcceptor;
    private Credits _credits;
    private bool _gameInProgress;
    private PackedScene _gameScene;
    private Janggu _janggu;
    private DateTime _startedAt;
    private PackedScene _titleScene;

    public override void _Ready()
    {
        _billAcceptor = GlobalContext.Instance.BillAcceptor;
        _janggu = new Janggu(GlobalContext.Instance.JangguHardware);
        _startedAt = DateTime.Now;
        _credits = new Credits();
        _gameScene = GD.Load<PackedScene>("res://game/Game.tscn");
        _titleScene = GD.Load<PackedScene>("res://title/Title.tscn");
    }

    private void UpdateCoinText()
    {
        if (_billAcceptor.GetCoins() > 0)
        {
            _billAcceptor.ConsumeCoins(1);
            _credits.Coins += 1;
        }

        ((Label)GetNode("Coin")).Text = _credits.CoinText();
    }

    private bool IsGameStartKeyInputPressed()
    {
        return _janggu.State.LeftStick.IsKeydownNow || _janggu.State.RightStick.IsKeydownNow;
    }

    private void OnGameEnded()
    {
        var titleSceneInstance = _titleScene.Instantiate();
        titleSceneInstance.Name = "Welcome";

        GetNode("Game").QueueFree();
        AddChild(titleSceneInstance);
        _gameInProgress = false;
    }

    private void StartGame()
    {
        var gameSceneInstance = _gameScene.Instantiate() as Game;
        gameSceneInstance.GameEnded += OnGameEnded;
        gameSceneInstance.Name = "Game";

        GetNode("Welcome").QueueFree();
        AddChild(gameSceneInstance);
        _gameInProgress = true;
    }

    public override void _Process(double delta)
    {
        _janggu.Update(DateTime.Now.Subtract(_startedAt).Milliseconds);

        UpdateCoinText();

        if (IsGameStartKeyInputPressed() && _credits.AvailableCredits > 0 && !_gameInProgress)
        {
            _credits.ConsumeCredit();
            StartGame();
        }
    }
}