namespace bidrumgodot.controller;

public class Janggu
{
    private IJangguHardware _controller;
    private JangguState _previousHardwareState;
    
    public JangguStateWithTick State { get; private set; } = new JangguStateWithTick();
    public long Tick { get; private set; }
    public Janggu (IJangguHardware hardware)
    {
        _controller = hardware;
    }

    public void Update(long tick)
    {
        JangguState currentState = _controller.GetState();
        JangguState previousState = new JangguState(_previousHardwareState);
        
        foreach (JangguStick stick in new [] { JangguStick.Left , JangguStick.Right})
        {
            if (previousState.GetByStick(stick).Equals(currentState.GetByStick(stick)))
            {
                State.GetByStick(stick).ToggleKeydown(false);
            }
            else
            {
                if (currentState.GetByStick(stick) == null)
                {
                    // When user hit and take stick off the janggu,
                    // currentState.LeftStick is null, and State.LeftStick.Face is not null.
                    // So that state is not hit event. So call ToggleKeydown with false parameter
                    State.GetByStick(stick).ToggleKeydown(false)
                        .SetKeydownTiming(tick)
                        .SetFace(null);
                }
                else
                {
                    State.GetByStick(stick).ToggleKeydown(true)
                        .SetKeydownTiming(tick)
                        .SetFace(JangguFace.Left);
                }
            }
        }

        Tick = tick;
        _previousHardwareState = currentState;
    }
}