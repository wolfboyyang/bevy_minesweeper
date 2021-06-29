use crate::events::{BoardCompletedEvent, TileMarkEvent};
use crate::{Board, BoardAssets};
use bevy::log;
use bevy::prelude::*;

pub fn mark_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    board_assets: Res<BoardAssets>,
    mut tile_mark_event_rdr: EventReader<TileMarkEvent>,
    mut board_completed_event_wr: EventWriter<BoardCompletedEvent>,
    query: Query<&Children>,
) {
    for event in tile_mark_event_rdr.iter() {
        if let Some((entity, mark)) = board.try_toggle_mark(&event.0) {
            if board.is_completed() {
                log::info!("Board completed");
                board_completed_event_wr.send(BoardCompletedEvent {});
            }
            if mark {
                commands.entity(entity).with_children(|parent| {
                    parent
                        .spawn_bundle(SpriteBundle {
                            material: board_assets.flag_material.clone(),
                            sprite: Sprite::new(Vec2::splat(board.tile_size)),
                            transform: Transform::from_xyz(0., 0., 1.),
                            ..Default::default()
                        })
                        .insert(Name::new("Flag"));
                });
            } else {
                let children = match query.get(entity) {
                    Ok(c) => c,
                    Err(e) => {
                        log::error!("Failed to retrieve flag components: {}", e);
                        continue;
                    }
                };
                for child in children.iter() {
                    commands.entity(*child).despawn_recursive();
                }
            }
        }
    }
}
