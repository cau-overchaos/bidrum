using System;
using bidrum.controller;
using Godot;

public partial class ChartNoteSprite : Sprite2D
{
    private CompressedTexture2D leftStickTexture;
    private CompressedTexture2D rightStickTexture;
    [Export] public JangguStick NoteType { get; set; }

    public int MaximumTextureWidth => Math.Max(leftStickTexture.GetWidth(), rightStickTexture.GetWidth());
    public int MaximumTextureHeight => Math.Max(leftStickTexture.GetHeight(), rightStickTexture.GetHeight());

    public override void _Ready()
    {
        leftStickTexture = (GD.Load<CompressedTexture2D>("res://assets/img/chart_player_ui/left_stick.png"));
        rightStickTexture = (GD.Load<CompressedTexture2D>("res://assets/img/chart_player_ui/right_stick.png"));
    }

    public override void _Process(double delta)
    {
        switch (NoteType)
        {
            case JangguStick.Left:
                if (Texture != leftStickTexture)
                    Texture = leftStickTexture;
                break;
            case JangguStick.Right:
                if (Texture != rightStickTexture)
                    Texture = rightStickTexture;
                break;
            default:
                throw new ArgumentOutOfRangeException();
        }
    }
}