using System;
using System.Threading;
using Godot;

namespace bidrumgodot.controller;

public class KeyboardController: IBillAccepter, IJangguController, IDisposable
{
    private readonly Thread _thread;
    private bool _stopping = false;
    private bool[] _jangguKeyPresses = { false, false, false, false };
    private int _coins = 0;
    
    public KeyboardController()
    {
        _thread = new Thread(ReadKeyboardActionLoop);
        _thread.Start();
    }
    
    public JangguControllerState GetState()
    {
        bool[] currentJangguKeyPresses = { false, false, false, false };
        _jangguKeyPresses.CopyTo(currentJangguKeyPresses,0);
        Nullable<JangguFace> leftStick = null;
        Nullable<JangguFace> rightStick = null;

        if (currentJangguKeyPresses[0])
        {
            leftStick = JangguFace.Left;
        } else if (currentJangguKeyPresses[1])
        {
            leftStick = JangguFace.Right;
        }

        if (currentJangguKeyPresses[2])
        {
            rightStick = JangguFace.Left;
        }else if (currentJangguKeyPresses[3])
        {
            rightStick = JangguFace.Right;
        }

        return new JangguControllerState(leftStick, rightStick);
    }

    public void Dispose()
    {
        this._stopping = true;
        _thread.Join();
    }

    private bool[] ReadJangguActionKeyPresses()
    {
        return new bool[]{
            Input.IsActionPressed("janggu_left_stick_to_left_face"),
            Input.IsActionPressed("janggu_left_stick_to_right_face"),
            Input.IsActionPressed("janggu_right_stick_to_left_face"),
            Input.IsActionPressed("janggu_right_stick_to_right_face")
        };
    }

    private bool ReadCoinActionKeyPress()
    {
        return Input.IsActionPressed("new_coin");
    }
    
    private void ReadKeyboardActionLoop()
    {
        bool previousCoinKeyPress = false;
        while (!_stopping)
        {
            bool currentCoinKeyPress = ReadCoinActionKeyPress();
            _jangguKeyPresses = ReadJangguActionKeyPresses();
            if (!previousCoinKeyPress && currentCoinKeyPress)
            {
                Interlocked.Add(ref this._coins, 1);
            }

            previousCoinKeyPress = currentCoinKeyPress;
        }
    }

    public int GetCoins()
    {
        return _coins;
    }

    public void ConsumeCoins(int coins)
    {
        Interlocked.Add(ref _coins, -coins);
    }
}