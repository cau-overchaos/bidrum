using Godot;

public partial class title : Node
{
    private bool inverted = false;
    private Label messageLabel;

    public override void _Ready()
    {
        messageLabel = GetNode("Title/Message") as Label;
        startTween();
    }

    private void startTween()
    {
        Tween messageTween = GetTree().CreateTween();
        messageTween.TweenProperty(messageLabel, "modulate:a", inverted ? 1.0f : 0.0f, 1.0f)
            .SetTrans(Tween.TransitionType.Quad).SetDelay(0.1);
        messageTween.Finished += invertTweenValue;
    }

    private void invertTweenValue()
    {
        inverted = !inverted;
        startTween();
    }
}