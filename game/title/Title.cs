using Godot;

public partial class Title : Node
{
    private bool _inverted = false;
    private Label _messageLabel;

    public override void _Ready()
    {
        _messageLabel = GetNode("Title/Message") as Label;
        StartTween();
    }

    private void StartTween()
    {
        Tween messageTween = GetTree().CreateTween();
        messageTween.TweenProperty(_messageLabel, "modulate:a", _inverted ? 1.0f : 0.0f, 1.0f)
            .SetTrans(Tween.TransitionType.Quad).SetDelay(0.1);
        messageTween.Finished += InvertTweenValue;
    }

    private void InvertTweenValue()
    {
        _inverted = !_inverted;
        StartTween();
    }
}