use bidrum_data_struct_lib::song::GameSong;

use super::game_common_context::GameCommonContext;

pub(crate) struct SongSelectionResult {
    pub selected_song: GameSong,
    pub selected_level: u32,
}

pub(crate) fn select_song(
    _common_context: &mut GameCommonContext,
    songs: &Vec<GameSong>,
) -> SongSelectionResult {
    return SongSelectionResult {
        selected_level: songs[0].get_chart_levels().unwrap()[0],
        selected_song: songs[0].clone(),
    };
}
