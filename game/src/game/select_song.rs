use bidrum_data_struct_lib::song::GameSong;

use super::game_common_context::GameCommonContext;

pub(crate) struct SongSelectionResult {
    pub selected_song: GameSong,
    pub selected_level: u32,
    // TO-DO: add velocity modifier (e.g. x1, x1.5, x2)
}

pub(crate) fn select_song(
    _common_context: &mut GameCommonContext,
    songs: &Vec<GameSong>,
) -> SongSelectionResult {
    // TO-DO: render screen, process user input, eventaully return selected song and options

    return SongSelectionResult {
        selected_level: songs[0].get_chart_levels().unwrap()[0],
        selected_song: songs[0].clone(),
    };
}
