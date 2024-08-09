using System.Collections.Generic;
using System.Linq;
using bidrum.controller;
using Bidrum.DataStructLib;

/// <summary>
/// Judges accuracy of notes
/// </summary>
public class TimingJudge
{
    private uint _bpm;
    private int _delay;
    private List<GameNote> _unprocessedNotes;

    public TimingJudge(IEnumerable<GameNote> notes, uint bpm, int delay)
    {
        _unprocessedNotes = new List<GameNote>(notes.OrderBy(note => note.TimingInMs(bpm, delay)));
        _bpm = bpm;
        _delay = delay;
    }

    public List<NoteJudgement> Judge(JangguStateWithTick jangguState, long tick)
    {
        List<NoteJudgement> noteJudgements = new List<NoteJudgement>();
        bool processedLeftStick = false, processedRightStick = false;

        foreach (GameNote note in _unprocessedNotes)
        {
            if (processedLeftStick && processedRightStick)
                break;

            long preciseTiming = note.TimingInMs(_bpm, _delay);
            long differenceWithNoteTiming = tick - preciseTiming;

            // Judge the missed notes
            if (differenceWithNoteTiming > new TimingWindow(NoteAccuracy.Miss).Value)
            {
                noteJudgements.Add(new NoteJudgement(note, NoteAccuracy.Miss));
                continue;
            }

            // Skip not-yet notes
            if (differenceWithNoteTiming < -new TimingWindow(NoteAccuracy.Miss).Value)
            {
                continue;
            }

            // Process note
            JangguStickStateWithTick keydownData = jangguState.GetByStick(note.Stick);
            if (keydownData.IsKeydownNow &&
                keydownData.Face == note.Face &&
                !((note.Stick == JangguStick.Left && processedLeftStick) ||
                  (note.Stick == JangguStick.Right && processedRightStick)))
            {
                if (note.Stick == JangguStick.Left)
                {
                    processedLeftStick = true;
                }
                else
                {
                    processedRightStick = true;
                }

                long hitTiming = keydownData.KeydownTimingTick;
                NoteAccuracy accuracy = TimingWindow.GetAccuracy(preciseTiming, hitTiming);
                noteJudgements.Add(new NoteJudgement(note, accuracy));

                // TO-DO: calc score here
            }
        }

        foreach (NoteJudgement judgement in noteJudgements)
        {
            _unprocessedNotes.Remove(judgement.Note);
            // TO-DO: process combo here
            // TO-DO: process health here
        }

        return noteJudgements;
    }
}