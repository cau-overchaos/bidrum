using Godot;
using System;

public partial class ChartPlayer : Node
{
    public AudioStream Music
    {
        set => GetNode<AudioStreamPlayer>("Music").Stream = value;
        get => GetNode<AudioStreamPlayer>("Music").Stream;
    }

    public VideoStream BGA
    {
        set => GetNode<VideoStreamPlayer>("BGA").Stream = value;
        get => GetNode<VideoStreamPlayer>("BGA").Stream;
    }

    public void Start()
    {
        GetNode<VideoStreamPlayer>("BGA").
    }
}
