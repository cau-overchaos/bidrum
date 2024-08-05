using System.Collections.Generic;
using System.Linq;
using bidrum.controller;
using Godot;

public enum NoteBackgroundBarDirection
{
    Left,
    Right
}

public struct ChartLineNote
{
    public float distance;
    public JangguStick stick;
}

public partial class NoteBackgroundBar : VBoxContainer
{
    [Export] public NoteBackgroundBarDirection Direction { get; set; }

    public override void _Process(double delta)
    {
        this.GetNode<HBoxContainer>("NodeBackground/HBoxContainer").Alignment =
            Direction == NoteBackgroundBarDirection.Left ? AlignmentMode.End : AlignmentMode.Begin;
    }

    public void SetNotes(IEnumerable<ChartLineNote> notes)
    {
        float? scale = null;
        Control guideline = GetNode<Control>("NodeBackground/HBoxContainer/MarginContainer/Guideline");
        foreach (JangguStick stickType in new JangguStick[] { JangguStick.Right, JangguStick.Left })
        {
            string noteGroupName = "notes-" + stickType.ToString("G");
            IEnumerable<ChartLineNote> filteredNotes = notes.Where(i => i.stick == stickType);
            Stack<Node> previousSprites = new Stack<Node>(
                GetTree().GetNodesInGroup(noteGroupName).Where(i => i.GetParent() == guideline)
            );

            foreach (ChartLineNote note in filteredNotes)
            {
                if (note.distance < -2.0F)
                    continue;

                ChartNoteSprite sprite;
                if (previousSprites.Count > 0)
                {
                    sprite = previousSprites.Pop() as ChartNoteSprite;
                }
                else
                {
                    sprite = new ChartNoteSprite();
                    sprite.NoteType = stickType;
                    sprite.AddToGroup(noteGroupName);
                    sprite.AddToGroup("notes");

                    guideline.AddChild(sprite);
                }

                if (scale == null)
                {
                    float guidelineHeight = guideline.GetRect().Size.Y;
                    scale = guidelineHeight / sprite.MaximumTextureHeight;
                }

                float scaledMaximumWidth = sprite.MaximumTextureWidth * scale.Value;
                float signByDirection = Direction == NoteBackgroundBarDirection.Left ? -1 : 1;
                // correctionValue makes notes with zero direction appear exactly at guideline
                float correctionValue = scaledMaximumWidth / 2;

                sprite.Scale = new Vector2(scale.Value, scale.Value);
                sprite.Position = new Vector2(
                    note.distance * scaledMaximumWidth * signByDirection
                    + correctionValue,
                    sprite.MaximumTextureHeight * scale.Value / 2);
            }

            while (previousSprites.Count > 0)
            {
                previousSprites.Pop().QueueFree();
            }
        }
    }
}