using Godot;
using System;

public partial class welcome : Node
{
    private Label messageLabel;
    private bool inverted = false;
    public void _ready()
    {
        messageLabel = GetNode("Title/Message") as Label;
        startTween();
    }

    private void startTween()
    { 
        Tween messageTween = GetTree().CreateTween();
        messageTween.TweenProperty(messageLabel, "modulate:a", inverted ? 1.0f : 0.0f, 1.0f).SetTrans(Tween.TransitionType.Quad).SetDelay(0.1);
        messageTween.Finished += invertTweenValue;
    }

    private void invertTweenValue()
    {
        inverted = !inverted;
        startTween();
    }
}
