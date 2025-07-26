use crate::GameState;
use rand::Rng;
use bevy::prelude::*;

pub struct FlockPlugin;


#[derive(Component, Default)]
struct Movement(Vec3);
#[derive(Component, Default)]
struct LastPos(Vec3);



#[derive(Component)]
pub struct Boid{
    friend: Option<Entity>,
    enemy: Option<Entity>
}

const BOID_COUNT: usize = 1000;
const SPEED_TO_CENTER: f32 = 5.0;
const SPEED_TO_FRIEND: f32 = 100.0; // How fast to move towards friend
const SPEED_TO_ENEMY: f32 = 20.0; 
const UPDATE_RELELATIONS_RATE: f64 = 0.01; 


/// This plugin handles boid related stuff like movement
/// Boid logic is only active during the State `GameState::Playing`
impl Plugin for FlockPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Playing), (spawn_boid, find_friend_and_enemy).chain())
        .add_systems(Update, update_relationships.run_if(in_state(GameState::Playing)))
        .add_systems(Update, (interact_boid, move_boid).run_if(in_state(GameState::Playing)));
    }
}

fn spawn_boid(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    
    for _ in 0..BOID_COUNT {
        let x = rng.gen_range(-400.0..-100.0);
        let y = rng.gen_range(-300.0..-200.0);
        
        commands.spawn((
            Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(4.0, 4.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(x, y, 1.)),
            Boid{
                friend: None,
                enemy: None,                
            },
            Movement(Vec3::ZERO),
            LastPos(Vec3::new(x, y, 1.)),
        ));
    }


}

fn find_friend_and_enemy(
    mut boid_query: Query<(Entity, &mut Boid)>,
) {
    let all_boids: Vec<Entity> = boid_query.iter().map(|(e, _)| e).collect();
    info!("SETUP !! Found {} boids", all_boids.len());
    
    for (entity, mut boid) in &mut boid_query {
        if all_boids.len() >= 2 {
            let mut rng = rand::thread_rng();
            
            // Pick random friend and enemy (ensuring they're different from self and each other)
            let available_boids: Vec<Entity> = all_boids.iter().filter(|&&e| e != entity).cloned().collect();
            
            if available_boids.len() >= 2 {
                let friend_idx = rng.gen_range(0..available_boids.len());
                let mut enemy_idx = rng.gen_range(0..available_boids.len());
                while enemy_idx == friend_idx {
                    enemy_idx = rng.gen_range(0..available_boids.len());
                }
                
                boid.friend = Some(available_boids[friend_idx]);
                boid.enemy = Some(available_boids[enemy_idx]);
            } else if available_boids.len() == 1 {
                // If only one other boid, make it the friend
                boid.friend = Some(available_boids[0]);
                boid.enemy = None;
            }
        }
    }
}

fn interact_boid(
    time: Res<Time>,
    mut boid_query: Query<(Entity,&Transform, &mut Movement, &Boid)>,    
) {
  
    
    let mut iter = boid_query.iter_combinations_mut(); 

     while let Some([(e1,transform1, mut movement1, boid1), (e2,transfor2, mut movement2, boid2)]) =
        iter.fetch_next()
    {
        let mut result_movement1 = Vec3::ZERO;
        let mut result_movement2 = Vec3::ZERO;
          if Some(e2) == boid1.friend {
            let vec_to_friend = transform1.translation - transfor2.translation;
            result_movement1 += vec_to_friend.normalize_or_zero() * SPEED_TO_FRIEND * time.delta_secs();
        }  
        if Some(e1) == boid2.friend {
            let vec_to_friend = transfor2.translation - transform1.translation;
            result_movement2 += vec_to_friend.normalize_or_zero() * SPEED_TO_FRIEND * time.delta_secs();
        }
        if Some(e2) == boid1.enemy {
            let vec_to_enemy = transform1.translation - transfor2.translation;
            result_movement1 -= vec_to_enemy.normalize_or_zero() * SPEED_TO_ENEMY * time.delta_secs();
        }
        if Some(e1) == boid2.enemy {
            let vec_to_enemy = transform1.translation - transfor2.translation;
            result_movement2 -= vec_to_enemy.normalize_or_zero() * SPEED_TO_ENEMY * time.delta_secs();
        }
        movement1.0 += result_movement1;
        movement2.0 += result_movement2;
    }
}

fn move_boid(
    time: Res<Time>,
    // mut boid_query: Query<(&mut Transform, &mut Movement, &mut LastPos)>,    
    mut boid_query: Query<(&mut Transform, &mut Movement, &mut LastPos, &Boid)>,    
) {
    let dt = time.delta_secs();
    
    // Calculate the geometric center of all boids
    let mut center = Vec3::ZERO;
    let mut boid_count = 0;
    
    for (transform, _, _, _) in boid_query.iter() {
        center += transform.translation;
        boid_count += 1;
    }
    
    if boid_count > 0 {
        center /= boid_count as f32;
    }
    
    info!("Number of boids: {}, Center: {:?}", boid_count, center);

    for (mut transform, mut movement, mut last_pos, _) in &mut boid_query {
        let vec_center = center - transform.translation;
        let new_pos = transform.translation + movement.0 + vec_center * SPEED_TO_CENTER * dt * 0.05;
        movement.0 = Vec3::ZERO;
        last_pos.0 = transform.translation;
        transform.translation = new_pos;
    }

  
}

fn update_relationships(
    mut boid_query: Query<(Entity, &mut Boid)>,
) {
    let all_boids: Vec<Entity> = boid_query.iter().map(|(e, _)| e).collect();
    
    // Only proceed if we have enough boids
    if all_boids.len() < 2 {
        return;
    }
    
    let mut rng = rand::thread_rng();
    
    // Randomly select a small percentage of boids to update each frame
    for (entity, mut boid) in &mut boid_query {
        // Each boid has a small chance to update its relationships each frame
        if rng.gen_bool(UPDATE_RELELATIONS_RATE) { // 0.1% chance per frame per boid
            let available_boids: Vec<Entity> = all_boids.iter().filter(|&&e| e != entity).cloned().collect();
            
            if available_boids.len() >= 2 {
                // Randomly decide what to update: friend, enemy, or both
                let update_choice = rng.gen_range(0..3);
                
                match update_choice {
                    0 => {
                        // Update friend only
                        let friend_idx = rng.gen_range(0..available_boids.len());
                        let new_friend = available_boids[friend_idx];
                        if Some(new_friend) != boid.enemy {
                            boid.friend = Some(new_friend);
                        }
                    },
                    1 => {
                        // Update enemy only
                        let enemy_idx = rng.gen_range(0..available_boids.len());
                        let new_enemy = available_boids[enemy_idx];
                        if Some(new_enemy) != boid.friend {
                            boid.enemy = Some(new_enemy);
                        }
                    },
                    _ => {
                        // Update both friend and enemy
                        let friend_idx = rng.gen_range(0..available_boids.len());
                        let mut enemy_idx = rng.gen_range(0..available_boids.len());
                        while enemy_idx == friend_idx {
                            enemy_idx = rng.gen_range(0..available_boids.len());
                        }
                        
                        boid.friend = Some(available_boids[friend_idx]);
                        boid.enemy = Some(available_boids[enemy_idx]);
                    }
                }
            } else if available_boids.len() == 1 {
                // If only one other boid, make it the friend and clear enemy
                boid.friend = Some(available_boids[0]);
                boid.enemy = None;
            }
        }
    }
}
