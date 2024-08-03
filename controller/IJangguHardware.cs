using System;

namespace bidrum.controller;

public enum JangguFace
{
    Left,
    Right
}

public enum JangguStick
{
    Left,
    Right
}

public class JangguState
{
    public JangguState(Nullable<JangguFace> leftStick, Nullable<JangguFace> rightStick)
    {
        this.LeftStick = leftStick;
        this.RightStick = rightStick;
    }

    public JangguState(JangguState original)
    {
        if (original != null)
        {
            this.LeftStick = original.LeftStick;
            this.RightStick = original.RightStick;
        }
    }

    public Nullable<JangguFace> LeftStick { get; private set; }
    public Nullable<JangguFace> RightStick { get; private set; }

    public Nullable<JangguFace> GetByStick(JangguStick stick)
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
}

public interface IJangguHardware
{
    public JangguState GetState();
}