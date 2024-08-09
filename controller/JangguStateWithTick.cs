using System;

namespace bidrum.controller;

public class JangguStickStateWithTick : ICloneable
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

    public object Clone()
    {
        return new JangguStickStateWithTick(this);
    }

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
    private JangguStickStateWithTick _leftStick = new JangguStickStateWithTick();
    private JangguStickStateWithTick _rightStick = new JangguStickStateWithTick();

    public JangguStickStateWithTick LeftStick => new(_leftStick);

    public JangguStickStateWithTick RightStick => new(_rightStick);

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

    public void SetStickState(JangguStick stick, JangguStickStateWithTick newState)
    {
        switch (stick)
        {
            case JangguStick.Left:
                _leftStick = newState;
                break;
            case JangguStick.Right:
                _rightStick = newState;
                break;
            default:
                throw new ArgumentOutOfRangeException(nameof(stick), stick, null);
        }
    }

    public override string ToString()
    {
        return $"JangguStateWithTick: LeftStick: ({_leftStick}) / RightStick: ({_rightStick})";
    }
}