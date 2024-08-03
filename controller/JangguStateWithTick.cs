using System;

namespace bidrum.controller;

public class JangguStickStateWithTick
{
    public JangguStickStateWithTick()
    {
        KeydownTimingTick = 0;
        Face = null;
        IsKeydownNow = false;
    }

    public JangguStickStateWithTick(long keydownTimingTick, JangguFace? face, bool isKeydownNow)
    {
        KeydownTimingTick = keydownTimingTick;
        Face = face;
        IsKeydownNow = isKeydownNow;
    }

    public JangguStickStateWithTick(JangguStickStateWithTick original) : this(original.KeydownTimingTick, original.Face,
        original.IsKeydownNow)
    {
    }

    /// <summary>
    /// Timing when the stick started to touch the face
    /// </summary>
    public long KeydownTimingTick { get; private set; }

    /// <summary>
    /// Face which the stick is touching
    ///
    /// If the stick is touching nothing, the value is null.
    /// </summary>
    public Nullable<JangguFace> Face { get; private set; }

    /// <summary>
    /// Whether it's the EXACT time the stick started to touch the face right now
    /// </summary>
    public bool IsKeydownNow { get; private set; }

    public JangguStickStateWithTick ToggleKeydown(bool newKeydown)
    {
        IsKeydownNow = newKeydown;
        return this;
    }

    public JangguStickStateWithTick SetKeydownTiming(long newTiming)
    {
        KeydownTimingTick = newTiming;
        return this;
    }

    public JangguStickStateWithTick SetFace(Nullable<JangguFace> newFace)
    {
        Face = newFace;
        return this;
    }

    public override string ToString()
    {
        return $"JangguStickStateWithTick: {Face} since {KeydownTimingTick} (IsKeydownNow: {IsKeydownNow})";
    }
}

public class JangguStateWithTick
{
    public JangguStickStateWithTick LeftStick { get; set; } = new JangguStickStateWithTick();
    public JangguStickStateWithTick RightStick { get; set; } = new JangguStickStateWithTick();

    public JangguStickStateWithTick GetByStick(JangguStick stick)
    {
        switch (stick)
        {
            case JangguStick.Left:
                return LeftStick;
            case JangguStick.Right:
                return RightStick;
            default:
                throw new ArgumentOutOfRangeException(nameof(stick), stick, null);
        }
    }

    public override string ToString()
    {
        return $"JangguStateWithTick: LeftStick: ({LeftStick}) / RightStick: ({RightStick})";
    }
}