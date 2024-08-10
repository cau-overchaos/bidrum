using System;
using System.Collections.Generic;
using System.Linq;
using bidrum.controller;
using Bidrum.DataStructLib;
using bidrum.game.judge;

/// <summary>
/// Judges accuracy of notes
/// </summary>
public class TimingJudge
{
    private uint _bpm;
    private int _combo = 0, _maxCombo = 0;
    private int _delay;
    private Health _health;
    private Score _score;
    private List<GameNote> _unprocessedNotes;

    public TimingJudge(IEnumerable<GameNote> notes, uint bpm, int delay)
    {
        _unprocessedNotes = new List<GameNote>(notes.OrderBy(note => note.TimingInMs(bpm, delay)));
        _bpm = bpm;
        _delay = delay;
        _health = new Health();
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

                // Make judgement
                NoteAccuracy accuracy = TimingWindow.GetAccuracy(preciseTiming, hitTiming);
                noteJudgements.Add(new NoteJudgement(note, accuracy));

                // Process score
                _score.Process(hitTiming, preciseTiming);
            }
        }

        foreach (NoteJudgement judgement in noteJudgements)
        {
            // Remove processed notes
            _unprocessedNotes.Remove(judgement.Note);

            // Process combo and maxCombo
            if (judgement.Accuracy == NoteAccuracy.Bad || judgement.Accuracy == NoteAccuracy.Miss)
            {
                _combo = 0;
            }
            else
            {
                _combo += 1;
                _maxCombo = Math.Max(_combo, _maxCombo);
            }

            // Process heath
            _health.Process(judgement.Accuracy);
        }

        return noteJudgements;
    }
}