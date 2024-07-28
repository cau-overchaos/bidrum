using System;

namespace bidrumgodot.controller;

public enum JangguFace
{
    Left, Right
}

public enum JangguStick
{
    Left, Right
}

public class JangguControllerState
{
    public Nullable<JangguFace> LeftStick { get; private set; }
    public Nullable<JangguFace> RightStick { get; private set; }

    public JangguControllerState(Nullable<JangguFace> leftStick, Nullable<JangguFace> rightStick)
    {
        this.LeftStick = leftStick;
        this.RightStick = rightStick;
    }
}

public interface IJangguController
{
    public JangguControllerState GetState();
}