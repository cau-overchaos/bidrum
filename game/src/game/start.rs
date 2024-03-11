use bidrum_data_struct_lib::song::GameSong;

use super::{
    game_common_context::GameCommonContext, game_player::play_song, select_song::select_song,
};

pub(crate) fn start_game(common_context: &mut GameCommonContext) {
    let songs = GameSong::get_songs();
    let mut total_stages = 0;
    while total_stages < 3 {
        let selected = select_song(common_context, &songs);
        let _result = play_song(
            common_context,
            &selected.selected_song,
            selected.selected_level,
        );

        total_stages += 1;
    }
}
